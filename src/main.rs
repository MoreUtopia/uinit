use rustix::{process::getpid, system::sethostname, system::uname};
use std::process::{Command, exit};

mod config;
mod info;
mod mount;

use crate::config::UinitConfig;
use crate::info::print_start;

const PATH: &str = "/etc/uinit.toml";
const DEFCONFIG: &str = include_str!("default.toml");

fn main() {
    if !getpid().is_init() {
        eprintln!("process is not running as pid 1");
        exit(1);
    }

    let config: UinitConfig = match std::fs::read_to_string(PATH) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => config,
            Err(error) => {
                eprintln!("invalid config file {PATH}: {error}; using default configuration");
                toml::from_str(DEFCONFIG).expect("embedded default configuration is invalid")
            }
        },
        Err(error) => {
            eprintln!("couldn't read config file {PATH}: {error}; using default configuration");
            toml::from_str(DEFCONFIG).expect("embedded default configuration is invalid")
        }
    };

    mount::setup_mounts(&config.mounts);

    if let Some(hostname) = config.system.hostname.as_deref() {
        match sethostname(hostname.as_bytes()) {
            Ok(()) => println!("hostname set to {hostname}"),
            Err(error) => println!("error when setting hostname: {error}"),
        }
    } else {
        println!(
            "hostname set to kernel default: {}",
            uname().nodename().to_string_lossy()
        );
    }

    print_start();

    loop {
        println!(
            "running {} with args {}",
            config.init.executable.display(),
            config.init.args.join(" ")
        );
        let child = Command::new(&config.init.executable)
            .args(&config.init.args)
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
