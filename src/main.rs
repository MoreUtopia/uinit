use rustix::{
    process::{WaitOptions, getpid, waitpid},
    system::{sethostname, uname},
};
use std::process::{Command, exit};

mod cmd;
mod config;
mod hook;
mod info;
mod mount;

use crate::config::UinitConfig;
use crate::info::print_start;

const PATH: &str = "/etc/uinit.d/uinit.toml";
const DEFCONFIG: &str = include_str!("default.toml");
const CMD_PATH: &str = "/tmp/init-rq";
const HOOKS_DIR: &str = "/etc/uinit.d/hooks";

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

    mount::setup_mounts();
    let _ = std::fs::write(CMD_PATH, "");

    let hook_mgr = match hook::HookManager::new() {
        Ok(mut mgr) => {
            if let Err(e) = mgr.load_from_dir(HOOKS_DIR) {
                eprintln!("hooks load error: {e}");
            }
            Some(mgr)
        }
        Err(e) => {
            eprintln!("hooks init error: {e}");
            None
        }
    };

    let trigger_hook = |event: hook::HookEvent| {
        if let Some(mgr) = &hook_mgr {
            mgr.trigger(event);
        }
    };

    trigger_hook(hook::HookEvent::OnBoot);

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
        trigger_hook(hook::HookEvent::PreSpawn);
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
            Ok(mut app) => loop {
                match app.try_wait() {
                    Ok(Some(status)) => {
                        if let Some(exit_code) = status.code() {
                            eprintln!("exited: {exit_code}");
                        }
                        trigger_hook(hook::HookEvent::OnExit);
                        break;
                    }
                    Ok(None) => {}
                    Err(e) => eprintln!("error waiting for main process: {e}"),
                }

                while waitpid(None, WaitOptions::NOHANG).ok().flatten().is_some() {}

                cmd::check_file(CMD_PATH, &hook_mgr);

                std::thread::sleep(std::time::Duration::from_millis(100));
            },
            Err(e) => {
                eprintln!("init executable failed: {e}");
                println!("starting again");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
