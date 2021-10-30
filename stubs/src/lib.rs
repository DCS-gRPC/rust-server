pub mod atmosphere {
    tonic::include_proto!("dcs.atmosphere");
}

pub mod coalition {
    tonic::include_proto!("dcs.coalition");
}

pub mod common {
    tonic::include_proto!("dcs.common");
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

#[cfg(test)]
mod tests {
    use super::common::{
        initiator, Airbase, AirbaseCategory, Coalition, Initiator, Position, Unit,
    };
    use super::mission::{event, Event};
    use super::world::GetAirbasesResponse;

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
                                        "alt": 1,
                                        "time": 0
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
                                alt: 1.0,
                                time: 0.0
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
                        alt: 3.0,
                        time: 4.0
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
                                "alt": 43.00004196166992,
                                "time": 0.0
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
                        alt: 43.00004196166992,
                        time: 0.0
                    }),
                    category: AirbaseCategory::Airdrome.into(),
                    display_name: "Anapa-Vityazevo".to_string(),
                }]
            }
        );
    }
}
