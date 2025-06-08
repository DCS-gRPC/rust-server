use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use futures_util::{FutureExt, SinkExt, StreamExt, TryFutureExt};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::mpsc;
use tokio::time;
use tokio_stream::pending;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::codec::{FramedRead, FramedWrite};
use tokio_util::udp::UdpFramed;

use crate::message::{
    Client, Message, MessageRequest, MsgType, RadioInfo, RadioUpdateMessage, ServerSettingsMessage,
    SyncMessageRequest, UpdateMessage, VersionMismatchMessage,
};
use crate::messages_codec::{self, MessagesCodec, MessagesCodecError};
use crate::voice_codec::{self, Encryption, Frequency, Modulation, VoiceCodec, VoicePacket};

const SRS_VERSION: &str = "1.9.0.0";

pub type Sender = mpsc::Sender<Vec<u8>>;
pub type Receiver = mpsc::Receiver<Result<Packet, StreamError>>;

#[allow(clippy::large_enum_variant)]
pub enum Packet {
    Control(Message),
    Voice(Vec<u8>),
}

pub async fn stream(
    client: crate::Client,
    addr: SocketAddr,
    shutdown_signal: impl Future<Output = ()> + Unpin + Send + 'static,
) -> Result<(Sender, Receiver), StreamError> {
    let tcp = TcpStream::connect(addr).await?;
    let (tcp_stream, tcp_sink) = tcp.into_split();
    let mut messages_sink = FramedWrite::new(tcp_sink, MessagesCodec::new());
    let mut messages_stream = FramedRead::new(tcp_stream, MessagesCodec::new());

    let udp = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0))).await?;
    udp.connect(addr).await?;
    let (mut voice_sink, mut voice_stream) = UdpFramed::new(udp, VoiceCodec::new()).split();

    let (tx_inner, rx) = mpsc::channel::<Result<Packet, StreamError>>(128);
    let (tx, rx_inner) = mpsc::channel::<Vec<u8>>(128);
    let tx_err = tx_inner.clone();

    tokio::task::spawn(
        async move {
            // send sync message to receive server settings
            messages_sink
                .send(create_sync_message(&client).await)
                .await?;

            // send initial Update message
            messages_sink
                .send(create_radio_update_message(&client).await)
                .await?;

            let los_enabled = AtomicBool::new(false);
            let distance_enabled = AtomicBool::new(false);
            let mut old_pos = client.position().await;
            let mut position_update_interval = time::interval(Duration::from_secs(60));
            let mut voice_ping_interval = time::interval(Duration::from_secs(5));
            let mut shutdown_signal = shutdown_signal.fuse();
            let mut packet_id = 1;

            let mut sguid = [0; 22];
            sguid.clone_from_slice(client.sguid().as_bytes());

            enum Select {
                Control(Option<Result<Message, MessagesCodecError>>),
                Voice(Option<Result<(VoicePacket, SocketAddr), io::Error>>),
                Send(Option<Vec<u8>>),
                PositionUpdate,
                VoicePing,
                Shutdown,
            }

            // Never resolve once sender gets dropped.
            let mut rx = ReceiverStream::new(rx_inner).chain(pending());

            loop {
                let select = tokio::select! {
                    msg = messages_stream.next() => Select::Control(msg),
                    packet = voice_stream.next() => Select::Voice(packet),
                    data = rx.next() => Select::Send(data),
                    _ = position_update_interval.tick() => Select::PositionUpdate,
                    _ = voice_ping_interval.tick() => Select::VoicePing,
                    _ = &mut shutdown_signal => Select::Shutdown,
                };

                match select {
                    Select::Control(Some(msg)) => {
                        let msg = msg?;

                        // handle message
                        if let Message::VersionMismatch(VersionMismatchMessage {
                            version, ..
                        }) = &msg
                        {
                            return Err(StreamError::VersionMismatch {
                                expected: SRS_VERSION.to_string(),
                                encountered: version.to_string(),
                            });
                        }

                        // update server settings
                        if let Message::ServerSettings(ServerSettingsMessage {
                            server_settings,
                            ..
                        }) = &msg
                        {
                            los_enabled.store(
                                server_settings.get("LOS_ENABLED").map(|s| s.as_str())
                                    == Some("True"),
                                Ordering::Relaxed,
                            );
                            distance_enabled.store(
                                server_settings.get("DISTANCE_ENABLED").map(|s| s.as_str())
                                    == Some("true"),
                                Ordering::Relaxed,
                            );
                        }

                        tx_inner.try_send(Ok(Packet::Control(msg))).ok();
                    }
                    Select::Voice(Some(packet)) => {
                        // Not completely implemented, so might never be called for now
                        let (packet, _) = packet?;
                        tx_inner.try_send(Ok(Packet::Voice(packet.audio_part))).ok();
                    }
                    Select::Send(Some(data)) => {
                        let packet = VoicePacket {
                            audio_part: data,
                            frequencies: vec![Frequency {
                                freq: client.freq() as f64,
                                modulation: if client.freq() <= 87_995_000 {
                                    Modulation::Fm
                                } else {
                                    Modulation::Am
                                },
                                encryption: Encryption::None,
                            }],
                            unit_id: client.unit().map(|u| u.id).unwrap_or(0),
                            packet_id,
                            hop_count: 0,
                            transmission_sguid: sguid,
                            client_sguid: sguid,
                        };
                        packet_id = packet_id.wrapping_add(1);
                        voice_sink.send((packet.into(), addr)).await?;
                    }
                    Select::Control(None) | Select::Voice(None) | Select::Send(None) => {
                        return Err(StreamError::Closed);
                    }
                    Select::PositionUpdate => {
                        // keep the position of the station updated
                        let new_pos = client.position().await;
                        let los_enabled = los_enabled.load(Ordering::Relaxed);
                        let distance_enabled = distance_enabled.load(Ordering::Relaxed);
                        if (los_enabled || distance_enabled) && new_pos != old_pos {
                            log::debug!(
                                "Position of {} changed, sending a new update message",
                                client.name()
                            );
                            messages_sink
                                .send(create_update_message(&client).await)
                                .await?;
                            old_pos = new_pos;
                        }
                    }
                    Select::VoicePing => {
                        voice_sink
                            .send((voice_codec::Packet::Ping(sguid), addr))
                            .await?;
                    }
                    Select::Shutdown => {
                        messages_sink.into_inner().shutdown().await?;
                        break;
                    }
                }
            }

            Ok(())
        }
        .map_err(move |err| tx_err.try_send(Err(err)).ok()),
    );

    Ok((tx, rx))
}

#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error(transparent)]
    Net(#[from] io::Error),
    #[error(transparent)]
    MessagesCodec(#[from] messages_codec::MessagesCodecError),
    #[error("Unsupported SRS server version {encountered} (expected {expected})")]
    VersionMismatch {
        expected: String,
        encountered: String,
    },
    #[error("Stream was closed unexpectedly")]
    Closed,
}

async fn create_radio_update_message(client: &crate::Client) -> MessageRequest {
    let pos = client.position().await;
    MessageRequest::RadioUpdate(RadioUpdateMessage {
        msg_type: MsgType,
        client: Client {
            client_guid: client.sguid().to_string(),
            name: client.name().to_string(),
            seat: 0,
            coalition: client.coalition,
            allow_record: true,
            radio_info: Some(RadioInfo {
                // add a radio to receive voice
                radios: Vec::new(),
                unit: client
                    .unit()
                    .map(|u| u.name.clone())
                    .unwrap_or_else(|| client.name().to_string()),
                unit_id: client.unit().as_ref().map(|u| u.id).unwrap_or(0),
                iff: Default::default(),
            }),
            lat_lng_position: pos,
        },
        version: SRS_VERSION.to_string(),
    })
}

async fn create_update_message(client: &crate::Client) -> MessageRequest {
    let pos = client.position().await;
    MessageRequest::Update(UpdateMessage {
        msg_type: MsgType,
        client: Client {
            client_guid: client.sguid().to_string(),
            name: client.name().to_string(),
            seat: 0,
            coalition: client.coalition,
            allow_record: true,
            radio_info: None,
            lat_lng_position: pos,
        },
        version: SRS_VERSION.to_string(),
    })
}

async fn create_sync_message(client: &crate::Client) -> MessageRequest {
    let pos = client.position().await;

    MessageRequest::Sync(SyncMessageRequest {
        msg_type: MsgType,
        client: Client {
            client_guid: client.sguid().to_string(),
            name: client.name().to_string(),
            seat: 0,
            coalition: client.coalition,
            allow_record: true,
            radio_info: None,
            lat_lng_position: pos,
        },
        version: SRS_VERSION.to_string(),
    })
}
