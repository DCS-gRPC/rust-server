// Current recommendation as of
// https://github.com/tokio-rs/prost/issues/661#issuecomment-1156606409
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::large_enum_variant)]

pub mod atmosphere;
pub mod coalition;
pub mod common;
pub mod controller;
pub mod custom;
pub mod group;
pub mod hook;
pub mod mission;
pub mod net;
pub mod srs;
pub mod timer;
pub mod trigger;
pub mod unit;
mod utils;
pub mod world;

#[cfg(test)]
mod tests {
    use super::common::v0::{
        initiator, Airbase, AirbaseCategory, Coalition, Initiator, Position, Unit,
    };
    use super::mission::v0::{stream_events_response as event, StreamEventsResponse};
    use super::world::v0::GetAirbasesResponse;
    use crate::common::v0::{Orientation, Velocity};

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

    // Note that this string simulates the response from Lua. This is important as it is
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
                                    "playerName": "New callsign",
                                    "numberInGroup": 1
                                }
                            }
		                },
		                "coalition": 3,
		                "id": 42,
		                "position": {
			                "lat": 1,
			                "lon": 2,
			                "alt": 3,
                            "u": 4,
                            "v": 5
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
                            player_name: Some("New callsign".to_string()),
                            group: None,
                            number_in_group: 1,
                            position: Some(Default::default()),
                            orientation: Some(Orientation {
                                heading: Default::default(),
                                yaw: Default::default(),
                                pitch: Default::default(),
                                roll: Default::default(),
                                forward: Some(Default::default()),
                                right: Some(Default::default()),
                                up: Some(Default::default()),
                            }),
                            velocity: Some(Velocity {
                                heading: Default::default(),
                                speed: Default::default(),
                                velocity: Some(Default::default())
                            }),
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
                        u: 4.0,
                        v: 5.0,
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
                                "u": 0,
                                "v": 0
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
                    unit: None,
                    name: "Anapa-Vityazevo".to_string(),
                    callsign: "Anapa-Vityazevo".to_string(),
                    coalition: Coalition::Neutral.into(),
                    position: Some(Position {
                        lon: 37.35978347755592,
                        lat: 45.01317473377168,
                        alt: 43.00004196166992,
                        u: 0.0,
                        v: 0.0,
                    }),
                    category: AirbaseCategory::Airdrome.into(),
                    display_name: "Anapa-Vityazevo".to_string(),
                }]
            }
        );
    }
}
