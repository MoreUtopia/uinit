use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct UinitConfig {
    pub init: InitConfig,
    pub system: SystemConfig,
    #[serde(default)]
    pub env: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
pub struct InitConfig {
    pub executable: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SystemConfig {
    pub hostname: Option<String>,
}
