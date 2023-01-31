use std::io::Cursor;

use ogg::reading::PacketReader;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct GCloudConfig {
    pub voice: Option<String>,
    pub key: String,
}

/// Synthesize the `text` using AWS Polly. Returns a vec of opus frames.
pub async fn synthesize(text: &str, config: &GCloudConfig) -> Result<Vec<Vec<u8>>, GcloudError> {
    let voice = config.voice.as_deref().unwrap_or("en-GB-Neural2-A");
    let (language_code, _) = voice.split_at(5);

    let payload = TextToSpeechRequest {
        audio_config: AudioConfig {
            audio_encoding: "OGG_OPUS",
            sample_rate_hertz: 16_000,
            speaking_rate: 0.9,
        },
        input: Input {
            ssml: &format!(r#"<speak version="1.0" xml:lang="en">{text}</speak>"#),
        },
        voice: Voice {
            language_code,
            name: voice,
        },
    };

    let url = format!(
        "https://texttospeech.googleapis.com/v1/text:synthesize?key={}",
        config.key
    );
    let client = reqwest::Client::new();
    let res = client.post(&url).json(&payload).send().await?;
    if res.status() != StatusCode::OK {
        let err: Value = res.json().await?;
        return Err(GcloudError::Gcloud(err.to_string()));
    }

    // Convert ogg audio data to opus frames
    let data: TextToSpeechResponse = res.json().await?;
    let data = base64::decode(data.audio_content)?;
    let data = Cursor::new(data);
    let mut frames = Vec::new();
    let mut audio = PacketReader::new(data);
    while let Some(pck) = audio.read_packet()? {
        frames.push(pck.data.to_vec())
    }

    Ok(frames)
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AudioConfig<'a> {
    audio_encoding: &'a str,
    sample_rate_hertz: u32,
    speaking_rate: f32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Input<'a> {
    ssml: &'a str,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Voice<'a> {
    language_code: &'a str,
    name: &'a str,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TextToSpeechRequest<'a> {
    audio_config: AudioConfig<'a>,
    input: Input<'a>,
    voice: Voice<'a>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TextToSpeechResponse {
    audio_content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum GcloudError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("received error from GCloud TTS API")]
    Gcloud(String),
    #[error("error reading ogg packet")]
    Ogg(#[from] ogg::OggReadError),
    #[error("failed to base64 decode audio data")]
    Base64(#[from] base64::DecodeError),
}
