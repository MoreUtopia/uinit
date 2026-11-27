# Using Uinit
- build uinit and init - `make`
- Copy output `rootfs/*` to your `initramfs-x86_64/*`.
>Make sure a standard Linux-like filesystem hierarchy (such as `/etc`, `/proc`, `/sys`, `/dev`, `/tmp`) exists in your initramfs.
- You CAN copy and customize `src/default.toml` to `/etc/uinit.toml`.
- Enjoy!
# Installation

## Building

To compile `uinit`, `init` and `uinit-ctl` run
```bash
make
```

## Directory Layout

After building, make sure your Utopia OS image includes the following layout:

```text
/
├── init                      # Native ABI Wrapper
├── usr/
│   ├── sbin/
│   │   └── uinit             # Main init binary
│   └── bin/
│       └── uinit-ctl         # Control utility
└── etc/
    └── uinit.d/
        ├── uinit.toml        # Main configuration file (optional, falls back to built-in default)
        └── hooks/            # Lua hooks
```
