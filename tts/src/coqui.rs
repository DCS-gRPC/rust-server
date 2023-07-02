use std::str::FromStr;

use reqwest::Url;

#[derive(Debug)]
pub struct CoquiConfig {
    pub addr: Option<String>,
    pub voice: Option<String>,
}

/// Synthesize the `text` using Coqui (server). Returns a vec of opus frames.
pub async fn synthesize(text: &str, config: &CoquiConfig) -> Result<Vec<Vec<u8>>, CoquiError> {
    let url = config.addr.as_deref().unwrap_or("http://localhost:4000");
    let mut url = Url::from_str(url).map_err(|_| CoquiError::InvalidAddr(url.to_string()))?;
    url.set_path("api/tts");
    url.query_pairs_mut()
        .append_pair("speaker_id", config.voice.as_deref().unwrap_or("p250"))
        .append_pair("text", text);

    let res = reqwest::get(url).await?.error_for_status()?;
    let wav = res.bytes().await?;
    Ok(crate::wav_to_opus(wav, 22_050).await?)
}

#[derive(Debug, thiserror::Error)]
pub enum CoquiError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("error reading ogg packet")]
    Ogg(#[from] ogg::OggReadError),
    #[error("failed to base64 decode audio data")]
    Base64(#[from] base64::DecodeError),
    #[error("invalid coqui address: {0}")]
    InvalidAddr(String),
    #[error("failed to encode audio data as opus")]
    Encode(#[from] crate::WaveToOpsError),
}
