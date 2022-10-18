use std::error;
use std::future::Future;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::{Duration, Instant};

use ::tts::{AwsConfig, AwsRegion, AzureConfig, GCloudConfig, TtsConfig, WinConfig};
use futures_util::stream::{SplitSink, StreamExt};
use futures_util::SinkExt;
use srs::VoiceStream;
use stubs::tts::v0::transmit_request;
use stubs::tts::v0::tts_service_server::TtsService;
use stubs::{common::v0::Coalition, tts};
use tokio::time::sleep;
use tonic::{Request, Response, Status};

use crate::shutdown::ShutdownHandle;

pub struct Tts {
    shutdown_signal: ShutdownHandle,
}

impl Tts {
    pub fn new(shutdown_signal: ShutdownHandle) -> Self {
        Self { shutdown_signal }
    }
}

#[tonic::async_trait]
impl TtsService for Tts {
    async fn transmit(
        &self,
        request: Request<tts::v0::TransmitRequest>,
    ) -> Result<Response<tts::v0::TransmitResponse>, Status> {
        let request = request.into_inner();
        let name = request.name.as_deref().unwrap_or("DCS-gRPC");
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

        let addr = SocketAddr::from_str(request.srs_addr.as_deref().unwrap_or("127.0.0.1:5002"))
            .map_err(|_| Status::invalid_argument("`srs_addr` is not a valid socket address"))?;
        let stream = client
            .start(addr, self.shutdown_signal.signal())
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        let config = match request.provider {
            Some(transmit_request::Provider::Aws(transmit_request::Aws {
                voice,
                key,
                secret,
                region,
            })) => TtsConfig::Aws(AwsConfig {
                voice,
                key,
                secret,
                region: AwsRegion::from_str(&region)
                    .map_err(|err| Status::internal(err.to_string()))?,
            }),
            Some(transmit_request::Provider::Azure(transmit_request::Azure {
                voice,
                key,
                region,
            })) => TtsConfig::Azure(AzureConfig { voice, key, region }),
            Some(transmit_request::Provider::Gcloud(transmit_request::GCloud { voice, key })) => {
                TtsConfig::GCloud(GCloudConfig { voice, key })
            }
            Some(transmit_request::Provider::Win(transmit_request::Windows { voice })) => {
                TtsConfig::Win(WinConfig { voice })
            }
            None => TtsConfig::Win(WinConfig { voice: None }),
        };

        if request.wait {
            transmit(
                &request.text,
                &config,
                stream,
                self.shutdown_signal.signal(),
            )
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        } else {
            let signal = self.shutdown_signal.signal();
            tokio::task::spawn(async move {
                if let Err(err) = transmit(&request.text, &config, stream, signal).await {
                    log::error!("TTS transmission failed: {}", err);
                }
            });
        }

        Ok(Response::new(tts::v0::TransmitResponse {}))
    }
}

async fn transmit(
    text: &str,
    config: &TtsConfig,
    stream: VoiceStream,
    mut shutdown_signal: impl Future<Output = ()> + Unpin,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let (sink, mut stream) = stream.split::<Vec<u8>>();
    let frames = ::tts::synthesize(text, config).await?;
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
