use crate::config::Mounts;
use rustix::mount::{MountFlags, mount};
use std::ffi::CStr;

struct MountSpec {
    src: &'static str,
    target: &'static str,
    fstype: &'static str,
    flags: MountFlags,
    data: Option<&'static CStr>,
}

impl Mounts {
    fn spec(self) -> MountSpec {
        match self {
            Mounts::Ramfs => MountSpec {
                src: "ramfs",
                target: "/tmp",
                fstype: "ramfs",
                flags: MountFlags::NOSUID | MountFlags::NODEV,
                data: None,
            },
            Mounts::Proc => MountSpec {
                src: "proc",
                target: "/proc",
                fstype: "proc",
                flags: MountFlags::NOSUID | MountFlags::NODEV | MountFlags::NOEXEC,
                data: None,
            },
            Mounts::Sysfs => MountSpec {
                src: "sysfs",
                target: "/sys",
                fstype: "sysfs",
                flags: MountFlags::NOSUID | MountFlags::NODEV | MountFlags::NOEXEC,
                data: None,
            },
            Mounts::Tmpdevfs => MountSpec {
                src: "devtmpfs",
                target: "/dev",
                fstype: "devtmpfs",
                flags: MountFlags::NOSUID,
                data: None,
            },
        }
    }
}

pub fn setup_mounts(mounts: &[Mounts]) {
    for &m in mounts {
        let spec = m.spec();
        match mount(spec.src, spec.target, spec.fstype, spec.flags, spec.data) {
            Ok(()) => println!("mounted {} on {}", spec.fstype, spec.target),
            Err(e) => eprintln!("failed to mount {} on {}: {e}", spec.fstype, spec.target),
        }
    }
}
