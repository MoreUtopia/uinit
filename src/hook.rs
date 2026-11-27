use mlua::{Function, Lua, RegistryKey, StdLib, Table};
use rustix::system::sethostname;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum HookEvent {
    OnBoot,
    PreSpawn,
    OnExit,
    PreShutdown,
}

impl HookEvent {
    fn as_str(&self) -> &'static str {
        match self {
            HookEvent::OnBoot => "on_boot",
            HookEvent::PreSpawn => "pre_spawn",
            HookEvent::OnExit => "on_exit",
            HookEvent::PreShutdown => "pre_shutdown",
        }
    }

    fn all() -> &'static [HookEvent] {
        &[
            HookEvent::OnBoot,
            HookEvent::PreSpawn,
            HookEvent::OnExit,
            HookEvent::PreShutdown,
        ]
    }
}

pub struct HookManager {
    lua: Lua,
    hooks: HashMap<HookEvent, Vec<RegistryKey>>,
}

impl HookManager {
    pub fn new() -> mlua::Result<Self> {
        let libs = StdLib::TABLE | StdLib::STRING | StdLib::MATH | StdLib::UTF8;
        let lua = Lua::new_with(libs, mlua::LuaOptions::default())?;

        let mgr = Self {
            lua,
            hooks: HashMap::new(),
        };

        mgr.register_api()?;

        Ok(mgr)
    }

    fn register_api(&self) -> mlua::Result<()> {
        let lua = &self.lua;
        let uinit_api = lua.create_table()?;

        // uinit.log("message")
        uinit_api.set(
            "log",
            lua.create_function(|_, msg: String| {
                println!("[hook] {msg}");
                Ok(())
            })?,
        )?;

        // uinit.sethostname("pc_utopia")
        uinit_api.set(
            "sethostname",
            lua.create_function(|_, hostname: String| {
                match sethostname(hostname.as_bytes()) {
                    Ok(()) => println!("[hook] hostname set to {hostname}"),
                    Err(error) => eprintln!("[hook] error when setting hostname: {error}"),
                }
                Ok(())
            })?,
        )?;

        lua.globals().set("uinit", uinit_api)?;
        Ok(())
    }

    pub fn load_from_dir(&mut self, dir: &str) -> mlua::Result<()> {
        let mut paths: Vec<PathBuf> = match fs::read_dir(dir) {
            Ok(entries) => entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("lua"))
                .collect(),
            Err(e) => {
                eprintln!("cannot read hooks directory {dir}: {e}");
                return Ok(());
            }
        };

        paths.sort();

        for path in paths {
            let code = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error reading hook {}: {e}", path.display());
                    continue;
                }
            };

            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            let hook_table: Table = match self.lua.load(&code).set_name(name).eval() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("error executing hook {name}: {e}");
                    continue;
                }
            };

            for &event in HookEvent::all() {
                if let Ok(func) = hook_table.get::<Function>(event.as_str()) {
                    let key = self.lua.create_registry_value(func)?;
                    self.hooks.entry(event).or_default().push(key);
                }
            }
        }

        Ok(())
    }

    pub fn trigger(&self, event: HookEvent) {
        if let Some(keys) = self.hooks.get(&event) {
            for key in keys {
                if let Ok(func) = self.lua.registry_value::<Function>(key)
                    && let Err(e) = func.call::<()>(())
                {
                    eprintln!("error running hook {:?}: {e}", event);
                }
            }
        }
    }
}
