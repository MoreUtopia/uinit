use rustix::mount::{MountFlags, mount};
use std::process::Command;

fn setup_mounts() {
    let proc_common = MountFlags::NOSUID | MountFlags::NOEXEC | MountFlags::NODEV;
    let sys_common = MountFlags::NOSUID | MountFlags::NOEXEC | MountFlags::NODEV;
    let dev_common = MountFlags::NOSUID;
    let ram_flags = MountFlags::NOSUID | MountFlags::NODEV;

    let mounts = [
        ("ramfs",    "/tmp",  "ramfs",    ram_flags),
        ("proc",     "/proc", "proc",     proc_common),
        ("sysfs",    "/sys",  "sysfs",    sys_common),
        ("devtmpfs", "/dev",  "devtmpfs", dev_common),
    ];

    for (src, target, fstype, flags) in mounts {
        match mount(src, target, fstype, flags, None) {
            Ok(()) => println!("mounted {fstype}"),
            Err(e) => println!("failed to mount {fstype}: {e}"),
        }
    }
}

fn main() {
    let init_app = "/usr/bin/bash";
    let path = "/usr/bin:/sbin:/bin:/";

    setup_mounts();

    loop {
        let child = Command::new(init_app)
            .env("PATH", path)
            .spawn();

        match child {
            Ok(mut app) => {
                match app.wait() {
                    Ok(status) => {
                        if let Some(exit_code) = status.code() {
                            eprintln!("exited: {exit_code}");
                        }
                    }
                    Err(e) => {
                        eprintln!("error: {e}");
                    }
                }
            }
            Err(e) => {
                eprintln!("init shell failed: {e}");
                println!("starting again");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
