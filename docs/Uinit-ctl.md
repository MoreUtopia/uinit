# Uinit-ctl

`uinit-ctl` is a lightweight CLI utility used to send runtime commands to `uinit` (PID 1).

## Usage

```bash
uinit-ctl [OPTIONS]
```

### Available Commands

| Command | Description |
| :--- | :--- |
| `reboot` | Triggers `pre_shutdown` hooks and restarts the machine via `reboot(Restart)`. |
| `poweroff` | Triggers `pre_shutdown` hooks and powers off the machine via `reboot(PowerOff)`. |
| `help` | Prints help information or help for a specific subcommand. |

## IPC Mechanism (`/tmp/init-rq`)

Because Utopia Kernel does not yet support any type of IPC, `uinit-ctl` communicates with PID 1 through a plain control file at `/tmp/init-rq`:

1. `uinit-ctl` opens `/tmp/init-rq` and writes the requested command string.
2. `uinit` polls `/tmp/init-rq` every `100ms` inside the supervisor loop.
3. Upon reading a non-empty command, `uinit` overwrites the file with spaces and executes the corresponding action.
