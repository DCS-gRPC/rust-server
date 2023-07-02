use std::error;

pub use aws::{AwsConfig, Region as AwsRegion};
pub use azure::AzureConfig;
pub use coqui::CoquiConfig;
pub use gcloud::GCloudConfig;
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
#[cfg(target_os = "windows")]
pub use win::WinConfig;

mod aws;
mod azure;
mod coqui;
mod gcloud;
#[cfg(target_os = "windows")]
mod win;

#[derive(Debug)]
pub enum TtsConfig {
    Aws(aws::AwsConfig),
    Azure(azure::AzureConfig),
    Coqui(coqui::CoquiConfig),
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
        TtsConfig::Coqui(config) => coqui::synthesize(text, config).await?,
        TtsConfig::GCloud(config) => gcloud::synthesize(text, config).await?,
        #[cfg(target_os = "windows")]
        TtsConfig::Win(config) => win::synthesize(text, config).await?,
    })
}

async fn wav_to_opus(
    wav: bytes::Bytes,
    in_sample_rate: u32,
) -> Result<Vec<Vec<u8>>, WaveToOpsError> {
    use audiopus::coder::Encoder;
    use audiopus::{Application, Channels, SampleRate};

    tokio::task::spawn_blocking(move || {
        const PCM_MAX: i16 = 0x7FFF;
        const PCM_DIV: f32 = 1.0 / PCM_MAX as f32;
        let mut audio_stream = wav
            .chunks(2)
            .map(|bytes| PCM_DIV * i16::from_le_bytes(bytes.try_into().unwrap()) as f32)
            .collect::<Vec<_>>();

        if in_sample_rate != 16_000 {
            let params = SincInterpolationParameters {
                sinc_len: 256,
                f_cutoff: 0.95,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 256,
                window: WindowFunction::BlackmanHarris2,
            };
            audio_stream = SincFixedIn::<f32>::new(
                16_000.0 / in_sample_rate as f64,
                2.0,
                params,
                audio_stream.len(),
                1,
            )?
            .process(&[&audio_stream], None)?
            .into_iter()
            .next()
            .unwrap()
        }

        log::error!("ASDF {}", audio_stream.len());

        const MONO_20MS: usize = 16000 /* 1 channel */ * 20 / 1000;
        let enc = Encoder::new(SampleRate::Hz16000, Channels::Mono, Application::Voip)?;
        let mut pos = 0;
        let mut output = [0; 256];
        let mut frames = Vec::new();

        while pos + MONO_20MS < audio_stream.len() {
            let len = enc.encode_float(&audio_stream[pos..(pos + MONO_20MS)], &mut output)?;
            frames.push(output[..len].to_vec());

            pos += MONO_20MS;
        }

        Ok::<_, WaveToOpsError>(frames)
    })
    .await
    .unwrap()
}

#[derive(Debug, thiserror::Error)]
pub enum WaveToOpsError {
    #[error(transparent)]
    Opus(#[from] audiopus::Error),
    #[error(transparent)]
    Sample(#[from] rubato::ResamplerConstructionError),
    #[error(transparent)]
    Resample(#[from] rubato::ResampleError),
}
