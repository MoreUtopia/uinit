use rustix::mount::{MountFlags, mount};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MountConfig {
    pub source: String,
    pub target: PathBuf,
    pub fs_type: String,
    pub flags: MountFlags,
}

pub fn setup_mounts() {
    let dev = MountConfig {
        source: "devtmpfs".into(),
        target: "/dev".into(),
        fs_type: "devtmpfs".into(),
        flags: MountFlags::NOSUID,
    };

    let sys = MountConfig {
        source: "sysfs".into(),
        target: "/sys".into(),
        fs_type: "sysfs".into(),
        flags: MountFlags::NOSUID | MountFlags::NODEV | MountFlags::NOEXEC,
    };

    let proc = MountConfig {
        source: "proc".into(),
        target: "/proc".into(),
        fs_type: "proc".into(),
        flags: MountFlags::NOSUID | MountFlags::NODEV | MountFlags::NOEXEC,
    };

    let ramfs = MountConfig {
        source: "ramfs".into(),
        target: "/tmp".into(),
        fs_type: "ramfs".into(),
        flags: MountFlags::NOSUID | MountFlags::NODEV,
    };

    let mounts = [dev, sys, proc, ramfs];

    for cfg in mounts {
        match mount(
            cfg.source.as_str(),
            cfg.target.as_path(),
            cfg.fs_type.as_str(),
            cfg.flags,
            None,
        ) {
            Ok(()) => println!("mounted {} on {}", cfg.fs_type, cfg.target.display()),
            Err(e) => eprintln!(
                "failed to mount {} on {}: {e}",
                cfg.fs_type,
                cfg.target.display()
            ),
        }
    }
}
