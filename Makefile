.PHONY: build build_uinit build_init

build: build_uinit build_init

rootfs:
	mkdir -p rootfs

build_uinit: rootfs
	cargo build --release
	cp target/x86_64-unknown-linux-gnu/release/uinit rootfs/

build_init: rootfs
	cargo build --release
	cp init/target/x86_64-unknown-none/release/init rootfs/
