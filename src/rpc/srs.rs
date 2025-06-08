use std::error;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, Instant};

use ::srs::Sender;
#[cfg(target_os = "windows")]
use ::tts::WinConfig;
use ::tts::{AwsConfig, AwsRegion, AzureConfig, GCloudConfig, PiperConfig, TtsConfig};
use futures_util::FutureExt;
use stubs::common::v0::{Coalition, Unit};
use stubs::mission::v0::StreamEventsResponse;
use stubs::mission::v0::stream_events_response::{Event, TtsEvent};
use stubs::srs;
use stubs::srs::v0::srs_service_server::SrsService;
use stubs::srs::v0::transmit_request;
use tokio::time::sleep;
use tonic::{Request, Response, Status};

use super::MissionRpc;
use crate::config::TtsProvider;
use crate::fps::event_time;
use crate::shutdown::ShutdownHandle;
use crate::srs::SrsClients;

pub struct Srs {
    tts_config: crate::config::TtsConfig,
    srs_config: crate::config::SrsConfig,
    write_dir: PathBuf,
    rpc: MissionRpc,
    srs_clients: SrsClients,
    shutdown_signal: ShutdownHandle,
}

impl Srs {
    pub fn new(
        tts_config: crate::config::TtsConfig,
        srs_config: crate::config::SrsConfig,
        write_dir: PathBuf,
        rpc: MissionRpc,
        srs_clients: SrsClients,
        shutdown_signal: ShutdownHandle,
    ) -> Self {
        Self {
            tts_config,
            srs_config,
            write_dir,
            rpc,
            srs_clients,
            shutdown_signal,
        }
    }

    pub fn clients(&self) -> SrsClients {
        self.srs_clients.clone()
    }
}

#[tonic::async_trait]
impl SrsService for Srs {
    async fn transmit(
        &self,
        request: Request<srs::v0::TransmitRequest>,
    ) -> Result<Response<srs::v0::TransmitResponse>, Status> {
        let request = request.into_inner();
        let name = request.srs_client_name.as_deref().unwrap_or("DCS-gRPC");
        let mut client = ::srs::Client::new(
            name,
            request.frequency,
            match Coalition::try_from(request.coalition) {
                Ok(Coalition::Red) => ::srs::Coalition::Red,
                _ => ::srs::Coalition::Blue,
            },
        );
        let position = request.position.unwrap_or_default();
        client
            .set_position(::srs::Position {
                lat: position.lat,
                lon: position.lon,
                alt: position.alt,
            })
            .await;

        let addr = self
            .srs_config
            .addr
            .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5002));
        let (tx, _) = client
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
                TtsProvider::Piper => transmit_request::Provider::Piper(transmit_request::Piper {
                    voice: None,
                    speed: None,
                }),
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
            transmit_request::Provider::Piper(transmit_request::Piper { voice, speed }) => {
                TtsConfig::Piper(PiperConfig {
                    voice: voice
                        .or_else(|| {
                            self.tts_config
                                .provider
                                .as_ref()
                                .and_then(|p| p.piper.as_ref())
                                .and_then(|p| p.default_voice.clone())
                        })
                        .filter(|v| !v.is_empty())
                        .ok_or_else(|| {
                            Status::failed_precondition("tts.provider.piper.default_voice not set")
                        })?,
                    speed: speed
                        .or_else(|| {
                            self.tts_config
                                .provider
                                .as_ref()
                                .and_then(|p| p.piper.as_ref())
                                .and_then(|p| p.default_speed)
                        })
                        .unwrap_or(1.0),
                    piper_path: self.write_dir.join("Mods/tech/DCS-gRPC/piper"),
                })
            }
        };

        let frames = ::tts::synthesize(&request.ssml, &config)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let duration_ms = Duration::from_millis(frames.len() as u64 * 20); // ~20m per frame count

        if let Some(text) = request.plaintext {
            let event = StreamEventsResponse {
                time: event_time(),
                event: Some(Event::Tts(TtsEvent {
                    text,
                    frequency: request.frequency,
                    coalition: request.coalition,
                    srs_client_name: request.srs_client_name,
                })),
            };
            self.rpc.event(event).await;
        }

        if request.r#async {
            let signal = self.shutdown_signal.signal();
            tokio::task::spawn(async move {
                if let Err(err) = transmit(frames, tx, signal).await {
                    log::error!("TTS transmission failed: {}", err);
                }
            });
        } else {
            transmit(frames, tx, self.shutdown_signal.signal())
                .await
                .map_err(|err| Status::internal(err.to_string()))?;
        }

        Ok(Response::new(srs::v0::TransmitResponse {
            duration_ms: duration_ms.as_millis() as u32,
        }))
    }

    async fn get_clients(
        &self,
        _request: Request<srs::v0::GetClientsRequest>,
    ) -> Result<Response<srs::v0::GetClientsResponse>, Status> {
        #[derive(serde::Serialize)]
        struct GetUnitByIdRequest {
            id: u32,
        }
        #[derive(Debug, serde::Deserialize)]
        struct GetUnitByIdResponse {
            unit: Unit,
        }

        let clients =
            futures_util::future::join_all(self.srs_clients.clients.read().await.iter().map(
                |(id, frequencies)| {
                    let frequencies = Vec::from_iter(frequencies.iter().copied());
                    self.rpc
                        .request::<_, GetUnitByIdResponse>(
                            "getUnitById",
                            tonic::Request::new(GetUnitByIdRequest { id: *id }),
                        )
                        .map(|unit| {
                            unit.ok().map(|unit| srs::v0::get_clients_response::Client {
                                unit: Some(unit.unit),
                                frequencies,
                            })
                        })
                },
            ))
            .await
            .into_iter()
            .flatten()
            .collect();

        Ok(Response::new(srs::v0::GetClientsResponse { clients }))
    }
}

async fn transmit(
    frames: Vec<Vec<u8>>,
    tx: Sender,
    mut shutdown_signal: impl Future<Output = ()> + Unpin,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let mut transmission = Box::pin(transmit_frames(frames, tx));

    tokio::select! {
        result = &mut transmission => {
             result
        }

        _ = &mut shutdown_signal => {
            Ok(())
        }
    }
}

async fn transmit_frames(
    frames: Vec<Vec<u8>>,
    tx: Sender,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let start = Instant::now();
    for (i, frame) in frames.into_iter().enumerate() {
        tx.send(frame).await?;

        // wait for the current ~playtime before sending the next package
        let playtime = Duration::from_millis((i as u64 + 1) * 20); // 20m per frame count
        let elapsed = start.elapsed();
        if playtime > elapsed {
            sleep(playtime - elapsed).await;
        }
    }

    Ok(())
}
