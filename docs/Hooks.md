# Lua Hooks

## Hook Ordering

* All `.lua` files inside `/etc/uinit.d/hooks/` are loaded at boot.
* Files are sorted alphabetically before execution.

## Naming Convention
All `.lua` files in `/etc/uinit.d/hooks/` should follow the `XX-YYYY.lua` format, where `XX` is a two-digit priority number and `YYYY` is a descriptive name (e.g., `10-hostname.lua`).

## Supported Events

| Event | Triggered When |
| :--- | :--- |
| `on_boot` | Core filesystems are mounted and Lua VM is initialized. |
| `pre_spawn` | Immediately before spawning the main process in the supervisor loop. |
| `on_exit` | Immediately after the main process exits. |
| `pre_shutdown` | Before executing a `reboot` or `poweroff` command. |

More will be added later.

## Sandboxing & `uinit` API

For PID 1 safety, standard Lua `os` and `io` libraries are disabled. Instead, scripts interact with the system through the global `uinit` table:

* `uinit.log(msg)` — Prints a message to the console prefixed with `[hook]`.
* `uinit.sethostname(name)` — Sets the kernel hostname.

More will be added later.

## Example (`10-hello.lua`)

```lua
return {
    on_boot = function()
        uinit.log("Utopia booted")
        uinit.sethostname("pc_utopia_lua")
    end,

    pre_spawn = function()
        uinit.log("Spawning main process")
    end,

    on_exit = function()
        uinit.log("Main process exited")
    end,

    pre_shutdown = function()
        uinit.log("Shutting down")
    end
}
```
