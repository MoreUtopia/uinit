use rustix::process::getpid;
use std::process::{Command, exit};

mod config;
mod info;
mod mount;

use crate::config::UinitConfig;
use crate::info::print_start;

const PATH: &str = "/etc/uinit.toml";

fn main() {
    if !getpid().is_init() {
        eprintln!("process is not running as pid 1");
        exit(1);
    }

    print_start();

    let config: UinitConfig = std::fs::read_to_string(PATH)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_else(|| {
            println!("couldn't read config file: {PATH}, using default configuration");
            UinitConfig::default()
        });

    mount::setup_mounts(&config.mounts);

    loop {
        println!(
            "running {} with args {}",
            config.init_executable.display(),
            config.args.join(" ")
        );
        let child = Command::new(&config.init_executable)
            .args(&config.args)
            .envs(config.env.iter().map(|(k, v)| (k, v)))
            .spawn();

        match child {
            Ok(mut app) => match app.wait() {
                Ok(status) => {
                    if let Some(exit_code) = status.code() {
                        eprintln!("exited: {exit_code}");
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                }
            },
            Err(e) => {
                eprintln!("init executable failed: {e}");
                println!("starting again");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
