# Uinit

`uinit` is the primary PID 1 process responsible for early userspace initialization, filesystem mounting, system configuration, and supervising the main userspace executable on Utopia Kernel.

## Configuration (`uinit.toml`)

Located at `/etc/uinit.d/uinit.toml`:

```toml
[init]
executable = "/bin/bash"
args = []

[system]
hostname = "pc_utopia"
env = [
    ["PATH", "/bin:/sbin:/usr/bin:/usr/sbin:/"],
    ["TERM", "linux"],
]
```
