use std::borrow::Cow;

use tokio::sync::Mutex;
use windows::core::HSTRING;
use windows::Media::SpeechSynthesis::SpeechSynthesizer;
use windows::Storage::Streams::DataReader;

#[derive(Debug)]
pub struct WinConfig {
    pub voice: Option<String>,
}

static MUTEX: Mutex<()> = Mutex::const_new(());

pub async fn synthesize(text: &str, config: &WinConfig) -> Result<Vec<Vec<u8>>, WinError> {
    // Note, there does not seem to be a way to explicitly set 16000kHz, 16 audio bits per
    // sample and mono channel.

    // Prevent concurrent Windows TTS synthesis, as this might cause a crash.
    let lock = MUTEX.lock().await;

    let mut voice_info = None;
    if let Some(voice) = &config.voice {
        let all_voices = SpeechSynthesizer::AllVoices()?;
        let len = all_voices.Size()? as usize;
        for i in 0..len {
            let v = all_voices.GetAt(i as u32)?;
            let lang = v.Language()?.to_string();
            if !lang.starts_with("en-") {
                continue;
            }

            let name = v.DisplayName()?.to_string();
            if name.ends_with(voice) {
                voice_info = Some(v);
                break;
            }
        }
    } else {
        // default to the first english voice in the list
        let all_voices = SpeechSynthesizer::AllVoices()?;
        let len = all_voices.Size()? as usize;
        for i in 0..len {
            let v = all_voices.GetAt(i as u32)?;
            let lang = v.Language()?.to_string();
            if lang.starts_with("en-") {
                let name = v.DisplayName()?.to_string();
                log::debug!("Using WIN voice: {}", name);
                voice_info = Some(v);
                break;
            }
        }

        if voice_info.is_none() {
            log::warn!("Could not find any english Windows TTS voice");
        }
    }

    if voice_info.is_none() {
        let all_voices = SpeechSynthesizer::AllVoices()?;
        let len = all_voices.Size()? as usize;
        log::info!(
            "Available WIN voices are (you don't have to include the `Microsoft` prefix in \
                the name):"
        );
        for i in 0..len {
            let v = all_voices.GetAt(i as u32)?;
            let lang = v.Language()?.to_string();
            if !lang.starts_with("en-") {
                continue;
            }

            let name = v.DisplayName()?.to_string();
            log::info!("- {} ({})", name, lang);
        }
    }

    let synth = SpeechSynthesizer::new()?;
    let lang = if let Some(info) = voice_info {
        synth.SetVoice(&info)?;
        info.Language()?.to_string().into()
    } else {
        Cow::Borrowed("en")
    };

    // the DataReader is !Send, which is why we have to process it in a local set
    let stream = synth
        .SynthesizeSsmlToStreamAsync(&HSTRING::from(&format!(
            r#"<speak version="1.0" xml:lang="{lang}">{text}</speak>"#
        )))?
        .await?;
    let size = stream.Size()?;

    let rd = DataReader::CreateDataReader(&stream.GetInputStreamAt(0)?)?;
    rd.LoadAsync(size as u32)?.await?;

    let mut wav = vec![0u8; size as usize];
    rd.ReadBytes(wav.as_mut_slice())?;

    drop(lock);

    Ok(crate::wav_to_opus(wav.into()).await?)
}

#[derive(Debug, thiserror::Error)]
pub enum WinError {
    #[error("Calling WinRT API failed with error code {0}: {1}")]
    Win(i32, String),
    #[error("Runtime error")]
    Io(#[from] std::io::Error),
    #[error("failed to encode audio data as opus")]
    Opus(#[from] audiopus::Error),
}

impl From<windows::core::Error> for WinError {
    fn from(err: windows::core::Error) -> Self {
        WinError::Win(err.code().0, err.message().to_string())
    }
}
