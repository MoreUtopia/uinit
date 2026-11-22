# Using Uinit
- build uinit and init - `make`
- Copy output `rootfs/*` to your `initramfs-x86_64/*`.
>Make sure a standard Linux-like filesystem hierarchy (such as `/etc`, `/proc`, `/sys`, `/dev`, `/tmp`) exists in your initramfs.
- You CAN copy and customize `src/default.toml` to `/etc/uinit.toml`.
- Enjoy!