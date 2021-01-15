use std::pin::Pin;

use dcs::mission_server::{Mission, MissionServer};
use dcs::*;
use dcs_module_ipc::IPC;
use futures::{Stream, StreamExt};
use tonic::transport::server::Router;
use tonic::transport::{self, Server};
use tonic::{Request, Response, Status};

pub mod dcs {
    tonic::include_proto!("dcs");
}

pub struct RPC {
    ipc: IPC<Event>,
}

impl RPC {
    pub fn builder(
        ipc: IPC<Event>,
    ) -> Router<MissionServer<RPC>, transport::server::Unimplemented> {
        Server::builder().add_service(MissionServer::new(RPC { ipc }))
    }
}

#[tonic::async_trait]
impl Mission for RPC {
    type StreamEventsStream =
        Pin<Box<dyn Stream<Item = Result<Event, tonic::Status>> + Send + Sync + 'static>>;

    async fn out_text(
        &self,
        request: Request<OutTextRequest>,
    ) -> Result<Response<OutTextResponse>, Status> {
        self.ipc
            .notification("outText", Some(request.into_inner()))
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(OutTextResponse {}))
    }

    async fn get_user_flag(
        &self,
        request: Request<GetUserFlagRequest>,
    ) -> Result<Response<GetUserFlagResponse>, Status> {
        let res: GetUserFlagResponse = self
            .ipc
            .request("getUserFlag", Some(request.into_inner()))
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(res))
    }

    async fn set_user_flag(
        &self,
        request: Request<SetUserFlagRequest>,
    ) -> Result<Response<SetUserFlagResponse>, Status> {
        self.ipc
            .notification("setUserFlag", Some(request.into_inner()))
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(SetUserFlagResponse {}))
    }

    async fn stream_events(
        &self,
        _request: Request<StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        Ok(Response::new(Box::pin(
            self.ipc.events().await.map(|e| Ok(e)),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::dcs::{event, Event};

    #[test]
    fn test_event_deserialization() {
        let event: Event = serde_json::from_str(r#"{"time":4.2,"missionStart":{}}"#).unwrap();
        assert_eq!(
            event,
            Event {
                time: 4.2,
                event: Some(event::Event::MissionStart(event::MissionStartEvent {})),
            }
        );
    }
}
