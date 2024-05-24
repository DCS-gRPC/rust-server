use std::path::PathBuf;
use std::process::Stdio;

use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[derive(Debug)]
pub struct PiperConfig {
    pub voice: String,
    pub speed: f32,
    pub piper_path: PathBuf,
}

pub async fn synthesize(text: &str, config: &PiperConfig) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let mut command = Command::new(config.piper_path.join("piper.exe"));
    command
        .arg("--model")
        .arg(&config.voice)
        .arg("--length_scale")
        .arg(format!("{}", config.speed))
        .arg("--output-raw")
        .current_dir(&config.piper_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = command.spawn()?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(text.as_bytes())
        .await?;
    let output = child.wait_with_output().await?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            if output.stderr.is_empty() {
                "failed to execute piper (maybe voice model not found)".into()
            } else {
                String::from_utf8_lossy(&output.stderr)
            },
        ));
    }

    crate::wav_to_opus(output.stdout.into())
        .await
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
}
