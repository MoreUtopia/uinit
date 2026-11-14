use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ExitPolicy {
    #[default]
    RestartApp,
    Shutdown,
    Reboot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UinitConfig {
    pub init_executable: PathBuf,
    pub args: Vec<String>,
    #[serde(default)]
    pub env: Vec<(String, String)>,
    #[serde(default)]
    exit_policy: ExitPolicy,
}

impl Default for UinitConfig {
    fn default() -> Self {
        Self {
            init_executable: PathBuf::from("/bin/sh"),
            args: vec![],
            env: vec![
                (
                    "PATH".into(),
                    "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/".into(),
                ),
                ("TERM".into(), "linux".into()),
            ],
            exit_policy: ExitPolicy::RestartApp,
        }
    }
}
