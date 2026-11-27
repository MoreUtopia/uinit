use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

fn print_help() {
    println!("Usage: uinit-ctl <COMMAND> [ARGS]");
    println!("Commands:");
    println!("  poweroff          Power off the system");
    println!("  reboot            Reboot the system");
}

fn send_cmd(cmd: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open("/tmp/init-rq")
        .unwrap_or_else(|e| {
            eprintln!("uinit-ctl: failed to open /tmp/init-rq: {e}");
            process::exit(1);
        });

    if let Err(e) = writeln!(file, "{cmd}") {
        eprintln!("uinit-ctl: failed to write command: {e}");
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("-h" | "--help" | "help") | None => {
            print_help();
            if args.is_empty() {
                process::exit(1);
            }
        }
        Some(cmd @ ("poweroff" | "reboot")) => {
            send_cmd(cmd);
        }
        Some(other) => {
            eprintln!("uinit-ctl: unknown command '{other}'");
            print_help();
            process::exit(1);
        }
    }
}
