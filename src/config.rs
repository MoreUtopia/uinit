use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExitPolicy {
    RestartApp,
    Shutdown,
    Reboot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mounts {
    Ramfs,
    Tmpdevfs,
    Sysfs,
    Proc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UinitConfig {
    pub init_executable: PathBuf,
    pub args: Vec<String>,
    pub mounts: Vec<Mounts>,
    pub env: Vec<(String, String)>,
    exit_policy: ExitPolicy,
}

impl Default for UinitConfig {
    fn default() -> Self {
        Self {
            init_executable: PathBuf::from("/bin/sh"),
            args: vec![],
            mounts: vec![Mounts::Ramfs, Mounts::Tmpdevfs, Mounts::Sysfs, Mounts::Proc],
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
