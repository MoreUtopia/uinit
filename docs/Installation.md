# Using Uinit
- build uinit - `cargo build --release`
- Copy output `target/release/uinit` to your initramfs.
>Make sure a standard Linux-like filesystem hierarchy (such as `/etc`, `/proc`, `/sys`, `/dev`, `/tmp`) exists in your initramfs.
- You CAN copy `example.toml` to `/etc/uinit.toml` and customize it.
- Add cmdline in config: `cmdline: init-abi=linux init=/uinit`.
- Enjoy!