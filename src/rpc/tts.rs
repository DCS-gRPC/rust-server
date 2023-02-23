use std::error;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
use ::tts::WinConfig;
use ::tts::{AwsConfig, AwsRegion, AzureConfig, GCloudConfig, TtsConfig};
use dcs_module_ipc::IPC;
use futures_util::stream::{SplitSink, StreamExt};
use futures_util::SinkExt;
use srs::VoiceStream;
use stubs::common::v0::Coalition;
use stubs::mission::v0::stream_events_response::{Event, TtsEvent};
use stubs::mission::v0::StreamEventsResponse;
use stubs::tts;
use stubs::tts::v0::transmit_request;
use stubs::tts::v0::tts_service_server::TtsService;
use tokio::time::sleep;
use tonic::{Request, Response, Status};

use crate::config::TtsProvider;
use crate::fps::event_time;
use crate::shutdown::ShutdownHandle;

pub struct Tts {
    tts_config: crate::config::TtsConfig,
    srs_config: crate::config::SrsConfig,
    ipc: IPC<StreamEventsResponse>,
    shutdown_signal: ShutdownHandle,
}

impl Tts {
    pub fn new(
        tts_config: crate::config::TtsConfig,
        srs_config: crate::config::SrsConfig,
        ipc: IPC<StreamEventsResponse>,
        shutdown_signal: ShutdownHandle,
    ) -> Self {
        Self {
            tts_config,
            srs_config,
            ipc,
            shutdown_signal,
        }
    }
}

#[tonic::async_trait]
impl TtsService for Tts {
    async fn transmit(
        &self,
        request: Request<tts::v0::TransmitRequest>,
    ) -> Result<Response<tts::v0::TransmitResponse>, Status> {
        let request = request.into_inner();
        let name = request.srs_client_name.as_deref().unwrap_or("DCS-gRPC");
        let mut client = srs::Client::new(
            name,
            request.frequency,
            match Coalition::from_i32(request.coalition) {
                Some(Coalition::Red) => srs::Coalition::Red,
                _ => srs::Coalition::Blue,
            },
        );
        let position = request.position.unwrap_or_default();
        client
            .set_position(srs::Position {
                lat: position.lat,
                lon: position.lon,
                alt: position.alt,
            })
            .await;

        let addr = self
            .srs_config
            .addr
            .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5002));
        let stream = client
            .start(addr, self.shutdown_signal.signal())
            .await
            .map_err(|err| {
                Status::internal(format!("Failed to establish SRS connection: {err}"))
            })?;

        let config = match request
            .provider
            .unwrap_or(match self.tts_config.default_provider {
                TtsProvider::Aws => {
                    transmit_request::Provider::Aws(transmit_request::Aws { voice: None })
                }
                TtsProvider::Azure => {
                    transmit_request::Provider::Azure(transmit_request::Azure { voice: None })
                }
                TtsProvider::GCloud => {
                    transmit_request::Provider::Gcloud(transmit_request::GCloud { voice: None })
                }
                TtsProvider::Win => {
                    transmit_request::Provider::Win(transmit_request::Windows { voice: None })
                }
            }) {
            transmit_request::Provider::Aws(transmit_request::Aws { voice }) => {
                TtsConfig::Aws(AwsConfig {
                    voice: voice.or_else(|| {
                        self.tts_config
                            .provider
                            .as_ref()
                            .and_then(|p| p.aws.as_ref())
                            .and_then(|p| p.default_voice.clone())
                    }),
                    key: self
                        .tts_config
                        .provider
                        .as_ref()
                        .and_then(|p| p.aws.as_ref())
                        .and_then(|p| p.key.clone())
                        .ok_or_else(|| {
                            Status::failed_precondition("tts.provider.aws.key config not set")
                        })?,
                    secret: self
                        .tts_config
                        .provider
                        .as_ref()
                        .and_then(|p| p.aws.as_ref())
                        .and_then(|p| p.secret.clone())
                        .ok_or_else(|| {
                            Status::failed_precondition("tts.provider.aws.secret config not set")
                        })?,
                    region: AwsRegion::from_str(
                        self.tts_config
                            .provider
                            .as_ref()
                            .and_then(|p| p.aws.as_ref())
                            .and_then(|p| p.region.as_deref())
                            .ok_or_else(|| {
                                Status::failed_precondition(
                                    "tts.provider.aws.region config not set",
                                )
                            })?,
                    )
                    .map_err(|err| Status::internal(err.to_string()))?,
                })
            }
            transmit_request::Provider::Azure(transmit_request::Azure { voice }) => {
                TtsConfig::Azure(AzureConfig {
                    voice: voice.or_else(|| {
                        self.tts_config
                            .provider
                            .as_ref()
                            .and_then(|p| p.azure.as_ref())
                            .and_then(|p| p.default_voice.clone())
                    }),
                    key: self
                        .tts_config
                        .provider
                        .as_ref()
                        .and_then(|p| p.azure.as_ref())
                        .and_then(|p| p.key.clone())
                        .ok_or_else(|| {
                            Status::failed_precondition("tts.provider.azure.key config not set")
                        })?,
                    region: self
                        .tts_config
                        .provider
                        .as_ref()
                        .and_then(|p| p.azure.as_ref())
                        .and_then(|p| p.region.clone())
                        .ok_or_else(|| {
                            Status::failed_precondition("tts.provider.azure.region config not set")
                        })?,
                })
            }
            transmit_request::Provider::Gcloud(transmit_request::GCloud { voice }) => {
                TtsConfig::GCloud(GCloudConfig {
                    voice: voice.or_else(|| {
                        self.tts_config
                            .provider
                            .as_ref()
                            .and_then(|p| p.gcloud.as_ref())
                            .and_then(|p| p.default_voice.clone())
                    }),
                    key: self
                        .tts_config
                        .provider
                        .as_ref()
                        .and_then(|p| p.gcloud.as_ref())
                        .and_then(|p| p.key.clone())
                        .ok_or_else(|| {
                            Status::failed_precondition("tts.provider.gcloud.key config not set")
                        })?,
                })
            }
            #[cfg(target_os = "windows")]
            transmit_request::Provider::Win(transmit_request::Windows { voice }) => {
                TtsConfig::Win(WinConfig {
                    voice: voice.or_else(|| {
                        self.tts_config
                            .provider
                            .as_ref()
                            .and_then(|p| p.win.as_ref())
                            .and_then(|p| p.default_voice.clone())
                    }),
                })
            }
            #[cfg(not(target_os = "windows"))]
            transmit_request::Provider::Win(transmit_request::Windows { .. }) => {
                return Err(Status::unavailable(
                    "Windows TTS is only available on Windows",
                ));
            }
        };

        let frames = ::tts::synthesize(&request.ssml, &config)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let duration_ms = Duration::from_millis(frames.len() as u64 * 20); // ~20m per frame count

        if let Some(text) = request.plaintext {
            self.ipc
                .event(StreamEventsResponse {
                    time: event_time(),
                    event: Some(Event::Tts(TtsEvent {
                        text,
                        frequency: request.frequency,
                        coalition: request.coalition,
                        srs_client_name: request.srs_client_name,
                    })),
                })
                .await;
        }

        if request.r#async {
            let signal = self.shutdown_signal.signal();
            tokio::task::spawn(async move {
                if let Err(err) = transmit(frames, stream, signal).await {
                    log::error!("TTS transmission failed: {}", err);
                }
            });
        } else {
            transmit(frames, stream, self.shutdown_signal.signal())
                .await
                .map_err(|err| Status::internal(err.to_string()))?;
        }

        Ok(Response::new(tts::v0::TransmitResponse {
            duration_ms: duration_ms.as_millis() as u32,
        }))
    }
}

async fn transmit(
    frames: Vec<Vec<u8>>,
    stream: VoiceStream,
    mut shutdown_signal: impl Future<Output = ()> + Unpin,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let (sink, mut stream) = stream.split::<Vec<u8>>();
    let mut transmission = Box::pin(transmit_frames(frames, sink));

    loop {
        tokio::select! {
            packet = stream.next() => {
                if let Some(packet) = packet {
                    packet?;
                    // Not interested in the received voice packets, so simply discard them
                }
            }

            result = &mut transmission => {
                return result;
            }

            _ = &mut shutdown_signal => {
                break;
            }
        }
    }

    Ok(())
}

async fn transmit_frames(
    frames: Vec<Vec<u8>>,
    mut sink: SplitSink<VoiceStream, Vec<u8>>,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let start = Instant::now();
    for (i, frame) in frames.into_iter().enumerate() {
        sink.send(frame).await?;

        // wait for the current ~playtime before sending the next package
        let playtime = Duration::from_millis((i as u64 + 1) * 20); // 20m per frame count
        let elapsed = start.elapsed();
        if playtime > elapsed {
            sleep(playtime - elapsed).await;
        }
    }

    Ok(())
}
