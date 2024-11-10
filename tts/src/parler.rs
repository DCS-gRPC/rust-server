use std::sync::Mutex;

use candle_core::{DType, Device, IndexOp as _, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::parler_tts::{Config, Model};
use rubato::Resampler;
use tokenizers::Tokenizer;
use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct ParlerConfig {
    pub speaker: String,
}

static MODEL: OnceCell<Option<(Mutex<Model>, Tokenizer, Device)>> = OnceCell::const_new();

pub async fn synthesize(prompt: &str, config: &ParlerConfig) -> Result<Vec<Vec<u8>>, ParlerError> {
    let (model, tokenizer, device) = get_model().await.ok_or(ParlerError::NoModel)?;

    let description_tokens = tokenizer
        .encode(config.speaker.as_str(), true)?
        .get_ids()
        .to_vec();
    let description_tokens = Tensor::new(description_tokens, device)?.unsqueeze(0)?;
    let prompt_tokens = tokenizer.encode(prompt, true)?.get_ids().to_vec();
    let prompt_tokens = Tensor::new(prompt_tokens, device)?.unsqueeze(0)?;
    let lp = candle_transformers::generation::LogitsProcessor::new(0, Some(0.7), None);

    println!("generating speech ...");
    let start = std::time::Instant::now();

    let data = tokio::task::spawn_blocking(move || {
        let mut model = model.lock().unwrap();
        let codes = model.generate(&prompt_tokens, &description_tokens, lp, 512)?;
        let codes = codes.to_dtype(DType::I64)?;
        codes.save_safetensors("codes", "out.safetensors")?;
        let codes = codes.unsqueeze(0)?;
        let samples = model
            .audio_encoder
            .decode_codes(&codes.to_device(device)?)?;

        drop(model); // release lock

        let samples = samples.i((0, 0))?;
        let samples = normalize_loudness(&samples, 44_100, true)?;
        let samples = samples.to_vec1::<f32>()?;
        log::debug!("generated speech in {:?}", start.elapsed());

        // Resample audio to 16kHz
        let mut resampler = rubato::FftFixedInOut::<f32>::new(44_100, 16_000, 1024, 1)?;
        let mut data = Vec::with_capacity(resampler.output_frames_max() * 2);
        let mut buffer = vec![vec![0.0f32; resampler.output_frames_next()]];
        let chunk_size = resampler.input_frames_next();
        for chunk in samples.chunks(chunk_size) {
            log::debug!("in.len={} out.len={}", chunk.len(), buffer[0].len());
            let (_, out_len) = if chunk.len() < chunk_size {
                resampler.process_partial_into_buffer(Some(&[chunk]), &mut buffer, None)?
            } else {
                resampler.process_into_buffer(&[chunk], &mut buffer, None)?
            };
            for sample in &buffer[0][..out_len] {
                let sample = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
                data.extend(sample.to_le_bytes());
            }
        }

        let buffer = buffer.remove(0);
        for sample in buffer {
            let sample = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
            data.extend(sample.to_le_bytes());
        }

        Ok::<_, ParlerError>(data)
    })
    .await
    .unwrap()?;

    Ok(crate::wav_to_opus(data.into()).await?)
}

pub async fn get_model<'a>() -> Option<&'a (Mutex<Model>, Tokenizer, Device)> {
    MODEL
        .get_or_init(|| async {
            match load_model().await {
                Ok((model, tokenizer, device)) => Some((Mutex::new(model), tokenizer, device)),
                Err(err) => {
                    log::error!("failed to load parler model: {err}");
                    None
                }
            }
        })
        .await
        .as_ref()
}

async fn load_model() -> Result<(Model, Tokenizer, Device), ParlerError> {
    // Make other options configurable?
    // "parler-tts/parler-tts-large-v1"
    // "parler-tts/parler-tts-mini-v1"
    // "parler-tts/parler-tts-mini-expresso"
    let model_id = "parler-tts/parler-tts-large-v1";
    let revision = "main".to_string();
    log::debug!("loading {model_id} model files ...");

    let start = std::time::Instant::now();
    let api = candle_hf_hub::api::tokio::Api::new()?;

    let repo = api.repo(candle_hf_hub::Repo::with_revision(
        model_id.to_string(),
        candle_hf_hub::RepoType::Model,
        revision,
    ));
    let model_files = match model_id {
        "parler-tts/parler-tts-large-v1" => {
            hub_load_safetensors(&repo, "model.safetensors.index.json").await?
        }
        _ => vec![repo.get("model.safetensors").await?],
    };
    let config = repo.get("config.json").await?;
    let tokenizer = repo.get("tokenizer.json").await?;
    log::debug!("loaded {model_id} model files in {:?}", start.elapsed());

    log::debug!("loading {model_id} model ...");
    let start = std::time::Instant::now();
    let tokenizer = Tokenizer::from_file(tokenizer)?;
    let device = Device::new_cuda(0)?;
    let vb = tokio::task::spawn_blocking({
        let device = device.clone();
        move || unsafe { VarBuilder::from_mmaped_safetensors(&model_files, DType::F32, &device) }
    })
    .await
    .unwrap()?;
    let config: Config =
        serde_json::from_slice(&tokio::fs::read(config).await?).map_err(ParlerError::JsonConfig)?;
    let model = Model::new(&config, vb)?;
    log::debug!("loaded {model_id} model in {:?}", start.elapsed());

    Ok((model, tokenizer, device))
}

#[derive(Debug, thiserror::Error)]
pub enum ParlerError {
    #[error(transparent)]
    Huggingface(#[from] candle_hf_hub::api::tokio::ApiError),
    #[error(transparent)]
    Candle(#[from] candle_core::Error),
    #[error(transparent)]
    Tokenizer(#[from] tokenizers::Error),
    #[error("no model (loading the model failed previously)")]
    NoModel,
    #[error("failed to parse model json config")]
    JsonConfig(#[source] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to encode audio as opus")]
    Opus(#[from] audiopus::Error),
    #[error("failed to resample audio to 16kHz: {0}")]
    Resample(#[from] rubato::ResampleError),
    #[error(transparent)]
    ResamplerConstruction(#[from] rubato::ResamplerConstructionError),
}

/// Loads the safetensors files for a model from the hub based on a json index file.
async fn hub_load_safetensors(
    repo: &candle_hf_hub::api::tokio::ApiRepo,
    json_file: &str,
) -> Result<Vec<std::path::PathBuf>, candle_core::Error> {
    let json_file = repo
        .get(json_file)
        .await
        .map_err(candle_core::Error::wrap)?;
    let json_file = std::fs::File::open(json_file)?;
    let json: serde_json::Value =
        serde_json::from_reader(&json_file).map_err(candle_core::Error::wrap)?;
    let weight_map = match json.get("weight_map") {
        None => candle_core::bail!("no weight map in {json_file:?}"),
        Some(serde_json::Value::Object(map)) => map,
        Some(_) => candle_core::bail!("weight map in {json_file:?} is not a map"),
    };
    let mut files = std::collections::HashSet::new();
    for value in weight_map.values() {
        if let Some(file) = value.as_str() {
            files.insert(file.to_string());
        }
    }
    let mut safetensors_files = Vec::with_capacity(files.len());
    for file in files {
        safetensors_files.push(repo.get(&file).await.map_err(candle_core::Error::wrap)?);
    }

    Ok(safetensors_files)
}

// https://github.com/facebookresearch/audiocraft/blob/69fea8b290ad1b4b40d28f92d1dfc0ab01dbab85/audiocraft/data/audio_utils.py#L57
pub fn normalize_loudness(
    wav: &Tensor,
    sample_rate: u32,
    loudness_compressor: bool,
) -> Result<Tensor, candle_core::Error> {
    let energy = wav.sqr()?.mean_all()?.sqrt()?.to_vec0::<f32>()?;
    if energy < 2e-3 {
        return Ok(wav.clone());
    }
    let wav_array = wav.to_vec1::<f32>()?;
    let mut meter = crate::bs1770::ChannelLoudnessMeter::new(sample_rate);
    meter.push(wav_array.into_iter());
    let power = meter.as_100ms_windows();
    let loudness = match crate::bs1770::gated_mean(power) {
        None => return Ok(wav.clone()),
        Some(gp) => gp.loudness_lkfs() as f64,
    };
    let delta_loudness = -14. - loudness;
    let gain = 10f64.powf(delta_loudness / 20.);
    let wav = (wav * gain)?;
    if loudness_compressor {
        wav.tanh()
    } else {
        Ok(wav)
    }
}
