use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::message::{create_sguid, Coalition, Position};
use crate::stream::{Receiver, Sender};
use crate::StreamError;

#[derive(Debug, Clone)]
pub struct UnitInfo {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    sguid: String,
    name: String,
    freq: u64,
    pos: Arc<RwLock<Position>>,
    unit: Option<UnitInfo>,
    pub coalition: Coalition,
}

impl Client {
    pub fn new(name: &str, freq: u64, coalition: Coalition) -> Self {
        Client {
            sguid: create_sguid(),
            name: name.to_string(),
            freq,
            pos: Arc::new(RwLock::new(Position::default())),
            unit: None,
            coalition,
        }
    }

    pub fn sguid(&self) -> &str {
        &self.sguid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn freq(&self) -> u64 {
        self.freq
    }

    pub async fn position(&self) -> Position {
        let p = self.pos.read().await;
        p.clone()
    }

    pub fn position_handle(&self) -> Arc<RwLock<Position>> {
        self.pos.clone()
    }

    pub fn unit(&self) -> Option<&UnitInfo> {
        self.unit.as_ref()
    }

    pub async fn set_position(&mut self, pos: Position) {
        let mut p = self.pos.write().await;
        *p = pos;
    }

    pub fn set_unit(&mut self, id: u32, name: &str) {
        self.unit = Some(UnitInfo {
            id,
            name: name.to_string(),
        });
    }

    pub async fn start(
        self,
        addr: SocketAddr,
        shutdown_signal: impl Future<Output = ()> + Unpin + Send + 'static,
    ) -> Result<(Sender, Receiver), StreamError> {
        crate::stream::stream(self, addr, shutdown_signal).await
    }
}
