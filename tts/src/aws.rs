pub use rusoto_core::Region;
use rusoto_core::request::HttpClient;
use rusoto_credential::StaticProvider;
use rusoto_polly::{Polly, PollyClient, SynthesizeSpeechInput};

#[derive(Debug)]
pub struct AwsConfig {
    pub voice: Option<String>,
    pub key: String,
    pub secret: String,
    pub region: Region,
}

/// Synthesize the `text` using AWS Polly. Returns a vec of opus frames.
pub async fn synthesize(text: &str, config: &AwsConfig) -> Result<Vec<Vec<u8>>, AwsError> {
    let dispatcher = HttpClient::new()?;
    let creds = StaticProvider::new(config.key.clone(), config.secret.clone(), None, None);

    let req = SynthesizeSpeechInput {
        // TODO: allow usage of neural engine (only available for certain voices and regions!)
        engine: None,
        language_code: None,
        lexicon_names: None,
        output_format: "pcm".to_string(),
        sample_rate: None, // defaults to 16,000
        speech_mark_types: None,
        text: format!(r#"<speak version="1.0" xml:lang="en">{text}</speak>"#),
        text_type: Some("ssml".to_string()),
        voice_id: config.voice.as_deref().unwrap_or("Brian").to_string(),
    };

    let client = PollyClient::new_with(dispatcher, creds, config.region.clone());
    let response = client.synthesize_speech(req).await?;

    let wav = response.audio_stream.ok_or(AwsError::MissingAudioStream)?;
    Ok(crate::wav_to_opus(wav).await?)
}

#[derive(Debug, thiserror::Error)]
pub enum AwsError {
    #[error(transparent)]
    Tls(#[from] rusoto_core::request::TlsError),
    #[error("AWS Polly response did not contain an audio stream")]
    MissingAudioStream,
    #[error("failed to encode audio data as opus")]
    Opus(#[from] audiopus::Error),
    #[error("failed to synthesize text to speech")]
    Synthesize(#[from] rusoto_core::RusotoError<rusoto_polly::SynthesizeSpeechError>),
}
