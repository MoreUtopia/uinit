use crate::config::MountConfig;
use rustix::mount::{MountFlags, mount};

fn parse_flags(flags: &[String]) -> Result<MountFlags, String> {
    let mut parsed = MountFlags::empty();
    for flag in flags {
        let value = match flag.as_str() {
            // right now don't need more
            "nosuid" => MountFlags::NOSUID,
            "nodev" => MountFlags::NODEV,
            "noexec" => MountFlags::NOEXEC,
            _ => return Err(format!("unsupported mount flag: {flag}")),
        };
        parsed |= value;
    }
    Ok(parsed)
}

pub fn setup_mounts(mounts: &[MountConfig]) {
    for config in mounts {
        let flags = match parse_flags(&config.flags) {
            Ok(flags) => flags,
            Err(e) => {
                eprintln!(
                    "failed to parse flags {} on {}: {e}",
                    config.fs_type,
                    config.target.display()
                );
                continue;
            }
        };
        match mount(
            config.source.as_str(),
            config.target.as_path(),
            config.fs_type.as_str(),
            flags,
            None,
        ) {
            Ok(()) => println!("mounted {} on {}", config.fs_type, config.target.display()),
            Err(e) => eprintln!(
                "failed to mount {} on {}: {e}",
                config.fs_type,
                config.target.display()
            ),
        }
    }
}
