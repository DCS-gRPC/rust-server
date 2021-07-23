use std::pin::Pin;

use crate::shutdown::{AbortableStream, ShutdownHandle};
use dcs::coalitions_server::Coalitions;
use dcs::custom_server::Custom;
use dcs::mission_server::Mission;
use dcs::triggers_server::Triggers;
use dcs::units_server::Units;
use dcs::world_server::World;
use dcs::*;
use dcs_module_ipc::IPC;
use futures_util::{Stream, StreamExt};
use tonic::{Request, Response, Status};

pub mod dcs {
    tonic::include_proto!("dcs");
}

#[derive(Clone)]
pub struct RPC {
    ipc: IPC<Event>,
    shutdown_signal: ShutdownHandle,
}

impl RPC {
    pub fn new(ipc: IPC<Event>, shutdown_signal: ShutdownHandle) -> Self {
        RPC {
            ipc,
            shutdown_signal,
        }
    }

    pub async fn request<I, O>(&self, method: &str, request: Request<I>) -> Result<O, Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
        for<'de> O: serde::Deserialize<'de> + Send + Sync + std::fmt::Debug + 'static,
    {
        self.ipc
            .request(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }

    pub async fn notification<I>(&self, method: &str, request: Request<I>) -> Result<(), Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
    {
        self.ipc
            .notification(method, Some(request.into_inner()))
            .await
            .map_err(to_status)
    }
}

#[tonic::async_trait]
impl Mission for RPC {
    type StreamEventsStream =
        Pin<Box<dyn Stream<Item = Result<Event, tonic::Status>> + Send + Sync + 'static>>;

    async fn stream_events(
        &self,
        _request: Request<StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let events = self.ipc.events().await;
        let stream = AbortableStream::new(self.shutdown_signal.signal(), events.map(Ok));
        Ok(Response::new(Box::pin(stream)))
    }
}

#[tonic::async_trait]
impl Triggers for RPC {
    async fn out_text(
        &self,
        request: Request<OutTextRequest>,
    ) -> Result<Response<OutTextResponse>, Status> {
        self.notification("outText", request).await?;
        Ok(Response::new(OutTextResponse {}))
    }

    async fn get_user_flag(
        &self,
        request: Request<GetUserFlagRequest>,
    ) -> Result<Response<GetUserFlagResponse>, Status> {
        let res: GetUserFlagResponse = self.request("getUserFlag", request).await?;
        Ok(Response::new(res))
    }

    async fn set_user_flag(
        &self,
        request: Request<SetUserFlagRequest>,
    ) -> Result<Response<SetUserFlagResponse>, Status> {
        self.notification("setUserFlag", request).await?;
        Ok(Response::new(SetUserFlagResponse {}))
    }
}

#[tonic::async_trait]
impl World for RPC {
    async fn get_airbases(
        &self,
        request: Request<GetAirbasesRequest>,
    ) -> Result<Response<GetAirbasesResponse>, Status> {
        let res: GetAirbasesResponse = self.request("getAirbases", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl Coalitions for RPC {
    async fn get_players(
        &self,
        request: Request<GetPlayersRequest>,
    ) -> Result<Response<GetPlayersResponse>, Status> {
        let res: GetPlayersResponse = self.request("getPlayers", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl Units for RPC {
    async fn get_radar(
        &self,
        request: Request<GetRadarRequest>,
    ) -> Result<Response<GetRadarResponse>, Status> {
        let res: GetRadarResponse = self.request("getRadar", request).await?;
        Ok(Response::new(res))
    }
}

#[tonic::async_trait]
impl Custom for RPC {
    async fn request_mission_assignment(
        &self,
        request: Request<MissionAssignmentRequest>,
    ) -> Result<Response<MissionAssignmentResponse>, Status> {
        self.notification("requestMissionAssignment", request)
            .await?;
        Ok(Response::new(MissionAssignmentResponse {}))
    }

    async fn join_mission(
        &self,
        request: Request<MissionJoinRequest>,
    ) -> Result<Response<MissionJoinResponse>, Status> {
        self.notification("joinMission", request).await?;
        Ok(Response::new(MissionJoinResponse {}))
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

#[cfg(test)]
mod tests {
    use super::dcs::{
        event, Airbase, AirbaseCategory, Coalition, Event, GetAirbasesResponse, Position, Unit,
    };

    #[test]
    fn test_event_deserialization() {
        let event: Event =
            serde_json::from_str(r#"{"time":4.2,"event":{"type":"missionStart"}}"#).unwrap();
        assert_eq!(
            event,
            Event {
                time: 4.2,
                event: Some(event::Event::MissionStart(event::MissionStartEvent {})),
            }
        );
    }

    #[test]
    fn test_enum_deserialization() {
        let event: Event = serde_json::from_str(
            r#"
                {
	                "time": 4.2,
	                "event": {
		                "type": "markAdd",
		                "initiator": {
			                "id": 1,
			                "name": "Aerial-1-1",
			                "callsign": "Enfield11",
			                "coalition": 2,
			                "type": "FA-18C_hornet",
			                "position": {
				                "lat": 3,
				                "lon": 2,
				                "alt": 1
			                },
			                "playerName": "New callsign"
		                },
		                "coalition": 2,
		                "id": 42,
		                "pos": {
			                "lat": 1,
			                "lon": 2,
			                "alt": 3
		                },
		                "text": "Test"
	                }
                }
            "#,
        )
        .unwrap();
        assert_eq!(
            event,
            Event {
                time: 4.2,
                event: Some(event::Event::MarkAdd(event::MarkAddEvent {
                    initiator: Some(Unit {
                        id: 1,
                        name: "Aerial-1-1".to_string(),
                        callsign: "Enfield11".to_string(),
                        r#type: "FA-18C_hornet".to_string(),
                        coalition: Coalition::Blue.into(),
                        position: Some(Position {
                            lat: 3.0,
                            lon: 2.0,
                            alt: 1.0
                        }),
                        player_name: Some("New callsign".to_string())
                    }),
                    visibility: Some(event::mark_add_event::Visibility::Coalition(
                        Coalition::Blue.into()
                    )),
                    id: 42,
                    pos: Some(Position {
                        lat: 1.0,
                        lon: 2.0,
                        alt: 3.0
                    }),
                    text: "Test".to_string(),
                })),
            }
        );
    }

    #[test]
    fn test_optional_field_deserialization() {
        let resp: GetAirbasesResponse = serde_json::from_str(
            r#"

                {
                    "airbases": [
                        {
                            "coalition": 0,
                            "name": "Anapa-Vityazevo",
                            "callsign": "Anapa-Vityazevo",
                            "position": {
                                "lon": 37.35978347755592,
                                "lat": 45.01317473377168,
                                "alt": 43.00004196166992
                            },
                            "category": 0,
                            "displayName": "Anapa-Vityazevo"
                        }
                    ]
                }
            "#,
        )
        .unwrap();
        assert_eq!(
            resp,
            GetAirbasesResponse {
                airbases: vec![Airbase {
                    id: None,
                    name: "Anapa-Vityazevo".to_string(),
                    callsign: "Anapa-Vityazevo".to_string(),
                    coalition: Coalition::Neutral.into(),
                    position: Some(Position {
                        lon: 37.35978347755592,
                        lat: 45.01317473377168,
                        alt: 43.00004196166992
                    }),
                    category: AirbaseCategory::Airdrome.into(),
                    display_name: "Anapa-Vityazevo".to_string(),
                }]
            }
        );
    }
}
