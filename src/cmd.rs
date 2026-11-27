use crate::hook;
use rustix::system::RebootCommand;
use std::fs::OpenOptions;
use std::io::Write;

pub fn check_file(cmd: &str, hooks: &Option<hook::HookManager>) {
    if let Ok(content) = std::fs::read_to_string(cmd) {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return;
        }

        if let Ok(mut f) = OpenOptions::new().write(true).open(cmd) {
            let _ = f.write_all(" ".repeat(content.len()).as_bytes());
        }

        match trimmed {
            "reboot" => {
                if let Some(mgr) = hooks {
                    mgr.trigger(hook::HookEvent::PreShutdown);
                }
                let _ = rustix::system::reboot(RebootCommand::Restart);
            }
            "poweroff" => {
                if let Some(mgr) = hooks {
                    mgr.trigger(hook::HookEvent::PreShutdown);
                }
                let _ = rustix::system::reboot(RebootCommand::PowerOff);
            }
            _ => eprintln!("unknown command: {trimmed}"),
        }
    }
}
