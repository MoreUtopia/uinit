use rustix::mount::{MountFlags, mount};

pub fn setup_mounts() {
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