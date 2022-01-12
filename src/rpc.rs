use std::sync::Arc;

use crate::shutdown::ShutdownHandle;
use crate::stats::Stats;
use dcs_module_ipc::IPC;
use futures_util::Stream;
use stubs::mission::v0::StreamEventsResponse;
use tokio::sync::RwLock;
use tonic::{Request, Status};

mod atmosphere;
mod coalition;
mod controller;
mod custom;
mod group;
mod hook;
mod mission;
mod net;
mod timer;
mod trigger;
mod unit;
mod world;

#[derive(Clone)]
pub struct MissionRpc {
    ipc: IPC<StreamEventsResponse>,
    stats: Stats,
    eval_enabled: bool,
    shutdown_signal: ShutdownHandle,
    cache: Arc<RwLock<Cache>>,
}

#[derive(Default)]
struct Cache {
    scenario_start_time: Option<time::OffsetDateTime>,
}

#[derive(Clone)]
pub struct HookRpc {
    ipc: IPC<()>,
    stats: Stats,
    eval_enabled: bool,
    shutdown_signal: ShutdownHandle,
}

impl MissionRpc {
    pub fn new(
        ipc: IPC<StreamEventsResponse>,
        stats: Stats,
        shutdown_signal: ShutdownHandle,
    ) -> Self {
        MissionRpc {
            ipc,
            stats,
            eval_enabled: false,
            shutdown_signal,
            cache: Default::default(),
        }
    }

    pub fn enable_eval(&mut self) {
        self.eval_enabled = true;
    }

    pub async fn request<I, O>(&self, method: &str, request: Request<I>) -> Result<O, Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
        for<'de> O: serde::Deserialize<'de> + Send + Sync + std::fmt::Debug + 'static,
    {
        let _guard = self.stats.track_queue_size();
        self.ipc
            .request(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }

    pub async fn notification<I>(&self, method: &str, request: Request<I>) -> Result<(), Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
    {
        let _guard = self.stats.track_queue_size();
        self.ipc
            .notification(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }

    pub async fn events(&self) -> impl Stream<Item = StreamEventsResponse> {
        self.ipc.events().await
    }
}

impl HookRpc {
    pub fn new(ipc: IPC<()>, stats: Stats, shutdown_signal: ShutdownHandle) -> Self {
        HookRpc {
            ipc,
            stats,
            eval_enabled: false,
            shutdown_signal,
        }
    }

    pub fn enable_eval(&mut self) {
        self.eval_enabled = true;
    }

    pub async fn request<I, O>(&self, method: &str, request: Request<I>) -> Result<O, Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
        for<'de> O: serde::Deserialize<'de> + Send + Sync + std::fmt::Debug + 'static,
    {
        let _guard = self.stats.track_queue_size();
        self.ipc
            .request(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }

    pub async fn notification<I>(&self, method: &str, request: Request<I>) -> Result<(), Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
    {
        let _guard = self.stats.track_queue_size();
        self.ipc
            .notification(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }
}

fn to_status(err: dcs_module_ipc::Error) -> Status {
    use dcs_module_ipc::Error;
    match err {
        Error::Script { kind, message } => match kind.as_deref() {
            Some("INVALID_ARGUMENT") => Status::invalid_argument(message),
            Some("NOT_FOUND") => Status::not_found(message),
            Some("ALREADY_EXISTS") => Status::already_exists(message),
            Some("UNIMPLEMENTED") => Status::unimplemented(message),
            _ => Status::internal(message),
        },
        err => Status::internal(err.to_string()),
    }
}
