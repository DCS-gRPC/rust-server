use std::error;

pub use aws::{AwsConfig, Region as AwsRegion};
pub use azure::AzureConfig;
pub use gcloud::GCloudConfig;
#[cfg(target_os = "windows")]
pub use win::WinConfig;

mod aws;
mod azure;
mod gcloud;
#[cfg(target_os = "windows")]
mod win;

#[derive(Debug)]
pub enum TtsConfig {
    Aws(aws::AwsConfig),
    Azure(azure::AzureConfig),
    GCloud(gcloud::GCloudConfig),
    #[cfg(target_os = "windows")]
    Win(win::WinConfig),
}

/// Synthesize the `text` to speech. Returns a vec of opus frames.
pub async fn synthesize(
    text: &str,
    config: &TtsConfig,
) -> Result<Vec<Vec<u8>>, Box<dyn error::Error + Send + Sync + 'static>> {
    Ok(match config {
        TtsConfig::Aws(config) => aws::synthesize(text, config).await?,
        TtsConfig::Azure(config) => azure::synthesize(text, config).await?,
        TtsConfig::GCloud(config) => gcloud::synthesize(text, config).await?,
        #[cfg(target_os = "windows")]
        TtsConfig::Win(config) => win::synthesize(text, config).await?,
    })
}

async fn wav_to_opus(wav: bytes::Bytes) -> Result<Vec<Vec<u8>>, audiopus::Error> {
    use audiopus::coder::Encoder;
    use audiopus::{Application, Channels, SampleRate};

    tokio::task::spawn_blocking(move || {
        let audio_stream = wav
            .chunks(2)
            .map(|bytes| i16::from_le_bytes(bytes.try_into().unwrap()))
            .collect::<Vec<_>>();

        const MONO_20MS: usize = 16000 /* 1 channel */ * 20 / 1000;
        let enc = Encoder::new(SampleRate::Hz16000, Channels::Mono, Application::Voip)?;
        let mut pos = 0;
        let mut output = [0; 256];
        let mut frames = Vec::new();

        while pos + MONO_20MS < audio_stream.len() {
            let len = enc.encode(&audio_stream[pos..(pos + MONO_20MS)], &mut output)?;
            frames.push(output[..len].to_vec());

            pos += MONO_20MS;
        }

        Ok::<_, audiopus::Error>(frames)
    })
    .await
    .unwrap()
}
