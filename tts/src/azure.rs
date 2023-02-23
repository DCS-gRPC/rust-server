use std::io::Cursor;

use ogg::reading::PacketReader;
use reqwest::StatusCode;

#[derive(Debug)]
pub struct AzureConfig {
    pub voice: Option<String>,
    pub key: String,
    pub region: String,
}

/// Synthesize the `text` using AWS Polly. Returns a vec of opus frames.
pub async fn synthesize(text: &str, config: &AzureConfig) -> Result<Vec<Vec<u8>>, AzureError> {
    let client = reqwest::Client::new();

    // Acquire token
    let token_url = format!(
        "https://{}.api.cognitive.microsoft.com/sts/v1.0/issueToken",
        config.region
    );
    let ocp_apim_key = &config.key;
    let res = client
        .post(&token_url)
        .header("Ocp-Apim-Subscription-Key", ocp_apim_key)
        .header("Content-Length", "0")
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        let err = res.text().await?;
        return Err(AzureError::Azure(format!("Azure error: {err}")));
    }

    let token = res.text().await?;

    // Prepare SSML
    let voice = config
        .voice
        .as_deref()
        .unwrap_or("en-US-AriaNeural")
        .to_string();
    let (lang, _) = voice.split_at(5);

    let tts = format!(
        r#"<speak version="1.0" xml:lang="{lang}"><voice xml:lang="{lang}" name="{voice}">{text}</voice></speak>"#
    );

    // Make actual synthesize request
    let api_url = format!(
        "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
        config.region
    );
    let res = client
        .post(&api_url)
        .bearer_auth(token)
        .header("X-Microsoft-OutputFormat", "ogg-24khz-16bit-mono-opus")
        .header("Content-Type", "application/ssml+xml")
        .header("User-Agent", "DCS-gRPC")
        .body(tts)
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        let err = res.text().await?;
        return Err(AzureError::Azure(format!("Azure error: {err}")));
    }

    // Convert ogg audio data to opus frames
    let bytes = res.bytes().await?;
    let data = Cursor::new(bytes);
    let mut frames = Vec::new();
    let mut audio = PacketReader::new(data);
    while let Some(pck) = audio.read_packet()? {
        frames.push(pck.data.to_vec())
    }

    Ok(frames)
}

#[derive(Debug, thiserror::Error)]
pub enum AzureError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("received error from Azure API")]
    Azure(String),
    #[error("error reading ogg packet")]
    Ogg(#[from] ogg::OggReadError),
}
