use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum Message {
    Update(UpdateMessage),
    Ping(PingMessage),
    Sync(SyncMessage),
    RadioUpdate(RadioUpdateMessage),
    ServerSettings(ServerSettingsMessage),
    ClientDisconnect(ClientDisconnectMessage),
    VersionMismatch(VersionMismatchMessage),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum MessageRequest {
    Update(UpdateMessage),
    Sync(SyncMessageRequest),
    RadioUpdate(RadioUpdateMessage),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateMessage {
    pub msg_type: MsgType<0>,
    pub client: Client,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PingMessage {
    pub msg_type: MsgType<1>,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyncMessage {
    pub msg_type: MsgType<2>,
    pub clients: Vec<Client>,
    pub server_settings: HashMap<String, String>,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyncMessageRequest {
    pub msg_type: MsgType<2>,
    pub client: Client,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioUpdateMessage {
    pub msg_type: MsgType<3>,
    pub client: Client,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerSettingsMessage {
    pub msg_type: MsgType<4>,
    pub server_settings: HashMap<String, String>,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClientDisconnectMessage {
    pub msg_type: MsgType<5>,
    pub client: Client,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersionMismatchMessage {
    pub msg_type: MsgType<6>,
    pub version: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Coalition {
    Spectator,
    Blue,
    Red,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radio {
    pub enc: bool,
    pub enc_key: u8,
    pub freq: f64,
    pub modulation: Modulation,
    pub sec_freq: f64,
    pub retransmit: bool,
}

impl Default for Radio {
    fn default() -> Self {
        Radio {
            enc: false,
            enc_key: 0,
            freq: 1.0,
            modulation: Modulation::Disabled,
            sec_freq: 1.0,
            retransmit: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[derive(Default)]
pub enum Modulation {
    Am = 0,
    Fm = 1,
    Intercom = 2,
    #[default]
    Disabled = 3,
    HaveQuick = 4,
    Satcom = 5,
    Mids = 6,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioInfo {
    pub radios: Vec<Radio>,
    pub unit: String,
    pub unit_id: u32,
    pub iff: Transponder,
}

#[derive(Debug, PartialEq, Default, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RadioSwitchControls {
    #[default]
    Hotas = 0,
    InCockpit = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Client {
    pub client_guid: String,
    pub name: String,
    pub seat: u32,
    pub coalition: Coalition,
    pub allow_record: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radio_info: Option<RadioInfo>,
    pub lat_lng_position: Position,
}

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Position {
    pub lat: f64,
    #[serde(rename = "lng")]
    pub lon: f64,
    pub alt: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transponder {
    control: IffControlMode,
    mode1: i32,
    mode3: i32,
    mode4: bool,
    mic: i32,
    status: IffStatus,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IffControlMode {
    Cockpit = 0,
    Overlay = 1,
    Disabled = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IffStatus {
    Off = 0,
    Normal = 1,
    Ident = 2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MsgType<const V: u8>;

#[derive(Debug, thiserror::Error)]
#[error("Invalid message type")]
struct MsgTypeError;

impl<const V: u8> Serialize for MsgType<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(V)
    }
}

impl<'de, const V: u8> Deserialize<'de> for MsgType<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        if value == V {
            Ok(MsgType::<V>)
        } else {
            Err(serde::de::Error::custom(MsgTypeError))
        }
    }
}

impl Default for Transponder {
    fn default() -> Self {
        Transponder {
            control: IffControlMode::Disabled,
            mode1: -1,
            mode3: -1,
            mode4: false,
            mic: -1,
            status: IffStatus::Off,
        }
    }
}

impl ::serde::Serialize for Coalition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        // Serialize the enum as a u64.
        serializer.serialize_u64(match *self {
            Coalition::Spectator => 0,
            Coalition::Red => 1,
            Coalition::Blue => 2,
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for Coalition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Coalition;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("positive integer")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Coalition, E>
            where
                E: ::serde::de::Error,
            {
                // Rust does not come with a simple way of converting a
                // number to an enum, so use a big `match`.
                match value {
                    0 => Ok(Coalition::Spectator),
                    1 => Ok(Coalition::Red),
                    2 => Ok(Coalition::Blue),
                    _ => Err(E::custom(format!(
                        "unknown {} value: {}",
                        stringify!(Coalition),
                        value
                    ))),
                }
            }
        }

        // Deserialize the enum from a u64.
        deserializer.deserialize_u64(Visitor)
    }
}

pub fn create_sguid() -> String {
    use base64::prelude::BASE64_URL_SAFE_NO_PAD;
    use base64::Engine;

    let sguid = Uuid::new_v4();
    let sguid = BASE64_URL_SAFE_NO_PAD.encode(sguid.as_bytes());
    assert_eq!(sguid.len(), 22);
    sguid
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_serde_message_update() {
        let expected = r#"{"MsgType":0,"Client":{"ClientGuid":"BCZSXySXT4WL9zlxNkJwkQ","Name":"DCS-gRPC","Seat":0,"Coalition":2,"AllowRecord":false,"LatLngPosition":{"lat":0.0,"lng":0.0,"alt":0.0}},"Version":"2.0.8.6"}"#;
        let msg: Message = serde_json::from_str(expected).unwrap();
        assert_eq!(
            msg,
            Message::Update(UpdateMessage {
                msg_type: MsgType,
                client: Client {
                    client_guid: "BCZSXySXT4WL9zlxNkJwkQ".to_string(),
                    name: "DCS-gRPC".to_string(),
                    seat: 0,
                    coalition: Coalition::Blue,
                    allow_record: false,
                    radio_info: None,
                    lat_lng_position: Position::default()
                },
                version: "2.0.8.6".to_string()
            })
        );
        assert_eq!(serde_json::to_string(&msg).unwrap(), expected);
    }

    #[test]
    fn test_serde_message_sync() {
        let expected = r#"{"MsgType":2,"Clients":[{"ClientGuid":"BCZSXySXT4WL9zlxNkJwkQ","Name":"DCS-gRPC","Seat":0,"Coalition":2,"AllowRecord":false,"LatLngPosition":{"lat":0.0,"lng":0.0,"alt":0.0}},{"ClientGuid":"7WKyf-Wc5E2xofl7IOc0wg","Name":"PILOT_220624","Seat":0,"Coalition":0,"AllowRecord":false,"RadioInfo":{"radios":[{"enc":false,"encKey":0,"freq":305000000.0,"modulation":0,"secFreq":0.0,"retransmit":false}],"unit":"FA-18C_hornet","unitId":16777472,"iff":{"control":0,"mode1":-1,"mode3":-1,"mode4":true,"mic":-1,"status":1}},"LatLngPosition":{"lat":0.0,"lng":0.0,"alt":0.0}}],"ServerSettings":{"COALITION_AUDIO_SECURITY":"False"},"Version":"2.0.8.6"}"#;
        let msg: Message = serde_json::from_str(expected).unwrap();
        assert_eq!(
            msg,
            Message::Sync(SyncMessage {
                msg_type: MsgType,
                clients: vec![
                    Client {
                        client_guid: "BCZSXySXT4WL9zlxNkJwkQ".to_string(),
                        name: "DCS-gRPC".to_string(),
                        seat: 0,
                        coalition: Coalition::Blue,
                        allow_record: false,
                        radio_info: None,
                        lat_lng_position: Position {
                            lat: 0.0,
                            lon: 0.0,
                            alt: 0.0
                        }
                    },
                    Client {
                        client_guid: "7WKyf-Wc5E2xofl7IOc0wg".to_string(),
                        name: "PILOT_220624".to_string(),
                        seat: 0,
                        coalition: Coalition::Spectator,
                        allow_record: false,
                        radio_info: Some(RadioInfo {
                            radios: vec![Radio {
                                enc: false,
                                enc_key: 0,
                                freq: 305000000.0,
                                modulation: Modulation::Am,
                                sec_freq: 0.0,
                                retransmit: false
                            }],
                            unit: "FA-18C_hornet".to_string(),
                            unit_id: 16777472,
                            iff: Transponder {
                                control: IffControlMode::Cockpit,
                                mode1: -1,
                                mode3: -1,
                                mode4: true,
                                mic: -1,
                                status: IffStatus::Normal
                            },
                        }),
                        lat_lng_position: Position::default(),
                    }
                ],
                server_settings: HashMap::from([(
                    "COALITION_AUDIO_SECURITY".to_string(),
                    "False".to_string()
                )]),
                version: "2.0.8.6".to_string()
            })
        );
        assert_eq!(serde_json::to_string(&msg).unwrap(), expected);
    }

    #[test]
    fn test_serde_message_radio_update() {
        let expected = r#"{"MsgType":3,"Client":{"ClientGuid":"BCZSXySXT4WL9zlxNkJwkQ","Name":"DCS-gRPC","Seat":0,"Coalition":2,"AllowRecord":false,"RadioInfo":{"radios":[{"enc":false,"encKey":0,"freq":1.0,"modulation":3,"secFreq":1.0,"retransmit":false}],"unit":"DCS-gRPC","unitId":0,"iff":{"control":2,"mode1":-1,"mode3":-1,"mode4":false,"mic":-1,"status":0}},"LatLngPosition":{"lat":0.0,"lng":0.0,"alt":0.0}},"Version":"2.0.8.6"}"#;
        let msg: Message = serde_json::from_str(expected).unwrap();
        assert_eq!(
            msg,
            Message::RadioUpdate(RadioUpdateMessage {
                msg_type: MsgType,
                client: Client {
                    client_guid: "BCZSXySXT4WL9zlxNkJwkQ".to_string(),
                    name: "DCS-gRPC".to_string(),
                    seat: 0,
                    coalition: Coalition::Blue,
                    allow_record: false,
                    radio_info: Some(RadioInfo {
                        radios: vec![Radio {
                            enc: false,
                            enc_key: 0,
                            freq: 1.0,
                            modulation: Modulation::Disabled,
                            sec_freq: 1.0,
                            retransmit: false
                        }],
                        unit: "DCS-gRPC".to_string(),
                        unit_id: 0,
                        iff: Transponder {
                            control: IffControlMode::Disabled,
                            mode1: -1,
                            mode3: -1,
                            mode4: false,
                            mic: -1,
                            status: IffStatus::Off
                        },
                    }),
                    lat_lng_position: Position::default(),
                },
                version: "2.0.8.6".to_string()
            })
        );
        assert_eq!(serde_json::to_string(&msg).unwrap(), expected);
    }

    #[test]
    fn test_serde_message_server_settings() {
        let expected = r#"{"MsgType":4,"ServerSettings":{"COALITION_AUDIO_SECURITY":"False"},"Version":"2.0.8.6"}"#;
        let msg: Message = serde_json::from_str(expected).unwrap();
        assert_eq!(
            msg,
            Message::ServerSettings(ServerSettingsMessage {
                msg_type: MsgType,
                server_settings: HashMap::from([(
                    "COALITION_AUDIO_SECURITY".to_string(),
                    "False".to_string()
                )]),
                version: "2.0.8.6".to_string()
            })
        );
        assert_eq!(serde_json::to_string(&msg).unwrap(), expected);
    }

    #[test]
    fn test_serde_message_client_disconnect() {
        let expected = r#"{"MsgType":5,"Client":{"ClientGuid":"OYOrf4yJdUex5tNuBYnaMQ","Name":"PILOT_220624","Seat":0,"Coalition":0,"AllowRecord":false,"RadioInfo":{"radios":[{"enc":false,"encKey":0,"freq":100.0,"modulation":2,"secFreq":0.0,"retransmit":false}],"unit":"CA","unitId":100000001,"iff":{"control":0,"mode1":0,"mode3":0,"mode4":false,"mic":-1,"status":0}},"LatLngPosition":{"lat":0.0,"lng":0.0,"alt":0.0}},"Version":"2.0.8.6"}"#;
        let msg: Message = serde_json::from_str(expected).unwrap();
        assert_eq!(
            msg,
            Message::ClientDisconnect(ClientDisconnectMessage {
                msg_type: MsgType,
                client: Client {
                    client_guid: "OYOrf4yJdUex5tNuBYnaMQ".to_string(),
                    name: "PILOT_220624".to_string(),
                    seat: 0,
                    coalition: Coalition::Spectator,
                    allow_record: false,
                    radio_info: Some(RadioInfo {
                        radios: vec![Radio {
                            enc: false,
                            enc_key: 0,
                            freq: 100.0,
                            modulation: Modulation::Intercom,
                            sec_freq: 0.0,
                            retransmit: false
                        }],
                        unit: "CA".to_string(),
                        unit_id: 100000001,
                        iff: Transponder {
                            control: IffControlMode::Cockpit,
                            mode1: 0,
                            mode3: 0,
                            mode4: false,
                            mic: -1,
                            status: IffStatus::Off
                        },
                    }),
                    lat_lng_position: Position::default(),
                },
                version: "2.0.8.6".to_string()
            })
        );
        assert_eq!(serde_json::to_string(&msg).unwrap(), expected);
    }
}
