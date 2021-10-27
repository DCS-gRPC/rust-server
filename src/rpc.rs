use crate::chat::Chat;
use crate::shutdown::ShutdownHandle;
use crate::stats::Stats;
use dcs::mission::Event;
use dcs_module_ipc::IPC;
use futures_util::Stream;
use tonic::{Request, Status};

pub mod dcs {
    tonic::include_proto!("dcs");

    pub mod atmosphere {
        tonic::include_proto!("dcs.atmosphere");
    }

    pub mod coalition {
        tonic::include_proto!("dcs.coalition");
    }

    pub mod controller {
        tonic::include_proto!("dcs.controller");
    }

    pub mod custom {
        tonic::include_proto!("dcs.custom");
    }

    pub mod group {
        tonic::include_proto!("dcs.group");
    }

    pub mod hook {
        tonic::include_proto!("dcs.hook");
    }

    pub mod mission {
        tonic::include_proto!("dcs.mission");
    }

    pub mod timer {
        tonic::include_proto!("dcs.timer");
    }

    pub mod trigger {
        tonic::include_proto!("dcs.trigger");
    }

    pub mod unit {
        tonic::include_proto!("dcs.unit");
    }

    pub mod world {
        tonic::include_proto!("dcs.world");
    }
}

mod atmosphere;
mod coalition;
mod controller;
mod custom;
mod group;
mod hook;
mod mission;
mod timer;
mod trigger;
mod unit;
mod world;

#[derive(Clone)]
pub struct MissionRpc {
    ipc: IPC<Event>,
    stats: Stats,
    eval_enabled: bool,
    shutdown_signal: ShutdownHandle,
}

#[derive(Clone)]
pub struct HookRpc {
    ipc: IPC<()>,
    chat: Chat,
    stats: Stats,
    eval_enabled: bool,
    shutdown_signal: ShutdownHandle,
}

impl MissionRpc {
    pub fn new(ipc: IPC<Event>, stats: Stats, shutdown_signal: ShutdownHandle) -> Self {
        MissionRpc {
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

    pub async fn events(&self) -> impl Stream<Item = Event> {
        self.ipc.events().await
    }
}

impl HookRpc {
    pub fn new(ipc: IPC<()>, chat: Chat, stats: Stats, shutdown_signal: ShutdownHandle) -> Self {
        HookRpc {
            ipc,
            chat,
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

#[cfg(test)]
mod tests {
    use super::dcs::mission::{event, Event};
    use super::dcs::world::GetAirbasesResponse;
    use super::dcs::{initiator, Airbase, AirbaseCategory, Coalition, Initiator, Position, Unit};

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
                            "initiator": {
                                "unit": {
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
                                    "playerName": "New callsign",
                                    "groupName": "Group 1",
                                    "numberInGroup": 1,
                                    "heading": 0.5,
                                    "speed": 0.8
                                }
                            }
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
                    initiator: Some(Initiator {
                        initiator: Some(initiator::Initiator::Unit(Unit {
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
                            player_name: Some("New callsign".to_string()),
                            group_name: "Group 1".to_string(),
                            number_in_group: 1,
                            heading: 0.5,
                            speed: 0.8
                        }))
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
