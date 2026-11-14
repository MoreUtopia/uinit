use rustix::system::uname;

pub fn print_start() {
    println!("Starting uinit");

    let info = uname();
    println!("OS:            {}", info.sysname().to_string_lossy());
    println!("Hostname:      {}", info.nodename().to_string_lossy());
    println!("Release:       {}", info.release().to_string_lossy());
    println!("Version:       {}", info.version().to_string_lossy());
    println!("Arch:          {}", info.machine().to_string_lossy());
}
