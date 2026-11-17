use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct UinitConfig {
    pub init: InitConfig,
    pub system: SystemConfig,
    #[serde(default)]
    pub mounts: Vec<MountConfig>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct MountConfig {
    pub source: String,
    pub target: PathBuf,
    pub fs_type: String,
    #[serde(default)]
    pub flags: Vec<String>,
}
