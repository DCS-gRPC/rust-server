mod utils;

pub mod atmosphere {
    pub mod v0 {
        tonic::include_proto!("dcs.atmosphere.v0");
    }
}

pub mod coalition {
    pub mod v0 {
        tonic::include_proto!("dcs.coalition.v0");
    }
}

pub mod common {
    pub mod v0 {
        tonic::include_proto!("dcs.common.v0");
    }
}

pub mod controller {
    pub mod v0 {
        tonic::include_proto!("dcs.controller.v0");
    }
}

pub mod custom {
    pub mod v0 {
        tonic::include_proto!("dcs.custom.v0");
    }
}

pub mod group {
    pub mod v0 {
        tonic::include_proto!("dcs.group.v0");
    }
}

pub mod hook {
    pub mod v0 {
        tonic::include_proto!("dcs.hook.v0");
    }
}

pub mod mission {
    pub mod v0 {
        tonic::include_proto!("dcs.mission.v0");
    }
}

pub mod net {
    pub mod v0 {
        tonic::include_proto!("dcs.net.v0");
    }
}

pub mod timer {
    pub mod v0 {
        tonic::include_proto!("dcs.timer.v0");
    }
}

pub mod trigger {
    pub mod v0 {
        tonic::include_proto!("dcs.trigger.v0");
    }
}

pub mod unit {
    pub mod v0 {
        tonic::include_proto!("dcs.unit.v0");
    }
}

pub mod world {
    pub mod v0 {
        tonic::include_proto!("dcs.world.v0");
    }
}

#[cfg(test)]
mod tests {
    use super::common::v0::{
        initiator, Airbase, AirbaseCategory, Coalition, Initiator, Position, Unit,
    };
    use super::mission::v0::{stream_events_response as event, StreamEventsResponse};
    use super::world::v0::GetAirbasesResponse;

    #[test]
    fn test_event_deserialization() {
        let event: StreamEventsResponse =
            serde_json::from_str(r#"{"time":4.2,"event":{"type":"missionStart"}}"#).unwrap();
        assert_eq!(
            event,
            StreamEventsResponse {
                time: 4.2,
                event: Some(event::Event::MissionStart(event::MissionStartEvent {})),
            }
        );
    }

    // Note that this string sumulates the response from Lua. This is important as it is
    // _after_ increment changes to enums to cater to gRPC enum indexing where 0 is not allowed
    // for responses.
    #[test]
    fn test_enum_deserialization() {
        let event: StreamEventsResponse = serde_json::from_str(
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
                                    "coalition": 3,
                                    "type": "FA-18C_hornet",
                                    "position": {
                                        "lat": 3,
                                        "lon": 2,
                                        "alt": 1,
                                        "time": 0
                                    },
                                    "playerName": "New callsign",
                                    "groupName": "Group 1",
                                    "numberInGroup": 1,
                                    "heading": 0.5,
                                    "speed": 0.8,
                                    "category": 0
                                }
                            }
		                },
		                "coalition": 3,
		                "id": 42,
		                "position": {
			                "lat": 1,
			                "lon": 2,
			                "alt": 3,
                            "time": 4
		                },
		                "text": "Test"
	                }
                }
            "#,
        )
        .unwrap();
        assert_eq!(
            event,
            StreamEventsResponse {
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
                                alt: 1.0,
                            }),
                            player_name: Some("New callsign".to_string()),
                            group_name: "Group 1".to_string(),
                            number_in_group: 1,
                            heading: 0.5,
                            speed: 0.8,
                            category: 0,
                        }))
                    }),
                    visibility: Some(event::mark_add_event::Visibility::Coalition(
                        Coalition::Blue.into()
                    )),
                    id: 42,
                    position: Some(Position {
                        lat: 1.0,
                        lon: 2.0,
                        alt: 3.0,
                    }),
                    text: "Test".to_string(),
                })),
            }
        );
    }

    // Note that this string sumulates the response from Lua. This is important as it is
    // _after_ increment changes to enums to cater to gRPC enum indexing where 0 is not allowed
    // for responses.
    #[test]
    fn test_optional_field_deserialization() {
        let resp: GetAirbasesResponse = serde_json::from_str(
            r#"

                {
                    "airbases": [
                        {
                            "coalition": 1,
                            "name": "Anapa-Vityazevo",
                            "callsign": "Anapa-Vityazevo",
                            "position": {
                                "lon": 37.35978347755592,
                                "lat": 45.01317473377168,
                                "alt": 43.00004196166992,
                                "time": 0.0
                            },
                            "category": 1,
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
                        alt: 43.00004196166992,
                    }),
                    category: AirbaseCategory::Airdrome.into(),
                    display_name: "Anapa-Vityazevo".to_string(),
                }]
            }
        );
    }
}
