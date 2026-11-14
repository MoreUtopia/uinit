# Using Uinit
- build uinit - `cargo build --release`
- Copy output `target/release/uinit` to your initramfs.
> You must ensure that `/dev`, `/sys`, `/proc`, `/tmp` exists in initramfs!
- Add cmdline in config: `cmdline: init-abi=linux init=/uinit`.
- Enjoy!