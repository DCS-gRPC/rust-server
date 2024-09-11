use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub version: String,
    pub write_dir: String,
    pub dll_path: String,
    pub lua_path: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub eval_enabled: bool,
    #[serde(default)]
    pub integrity_check_disabled: bool,
    pub tts: Option<TtsConfig>,
    pub srs: Option<SrsConfig>,
    pub auth: Option<AuthConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsConfig {
    #[serde(default)]
    pub default_provider: TtsProvider,
    pub provider: Option<TtsProviderConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsProviderConfig {
    pub aws: Option<AwsConfig>,
    pub azure: Option<AzureConfig>,
    pub gcloud: Option<GCloudConfig>,
    pub win: Option<WinConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TtsProvider {
    Aws,
    Azure,
    GCloud,
    #[default]
    Win,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AwsConfig {
    pub key: Option<String>,
    pub secret: Option<String>,
    pub region: Option<String>,
    pub default_voice: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AzureConfig {
    pub key: Option<String>,
    pub region: Option<String>,
    pub default_voice: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GCloudConfig {
    pub key: Option<String>,
    pub default_voice: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WinConfig {
    pub default_voice: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SrsConfig {
    #[serde(default)]
    pub addr: Option<SocketAddr>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfig {
    #[serde(default)]
    pub enabled: bool,
    pub tokens: Vec<ApiKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    #[serde(default)]
    pub client: String,
    pub token: String,
}

fn default_host() -> String {
    String::from("127.0.0.1")
}

fn default_port() -> u16 {
    50051
}

impl<'lua> mlua::FromLua<'lua> for Config {
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        use mlua::LuaSerdeExt;
        let config: Config = lua.from_value(lua_value)?;
        Ok(config)
    }
}

impl std::fmt::Debug for AwsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AwsConfig {
            key,
            secret,
            region,
            default_voice,
        } = self;
        f.debug_struct("AwsConfig")
            .field("key", &key.as_ref().map(|_| "<REDACTED>"))
            .field("secret", &secret.as_ref().map(|_| "<REDACTED>"))
            .field("region", region)
            .field("default_voice", default_voice)
            .finish()
    }
}

impl std::fmt::Debug for AzureConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AzureConfig {
            key,
            region,
            default_voice,
        } = self;
        f.debug_struct("AzureConfig")
            .field("key", &key.as_ref().map(|_| "<REDACTED>"))
            .field("region", region)
            .field("default_voice", default_voice)
            .finish()
    }
}

impl std::fmt::Debug for GCloudConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let GCloudConfig { key, default_voice } = self;
        f.debug_struct("GCloudConfig")
            .field("key", &key.as_ref().map(|_| "<REDACTED>"))
            .field("default_voice", default_voice)
            .finish()
    }
}
