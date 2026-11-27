.PHONY: all build build_uinit build_init build_uinit_ctl rootfs

all: build

build: build_uinit build_init build_uinit_ctl

rootfs:
	mkdir -p rootfs/usr/sbin rootfs/usr/bin rootfs/etc/uinit.d/hooks

build_uinit: | rootfs
	cargo build --release
	cp target/x86_64-unknown-linux-gnu/release/uinit rootfs/usr/sbin/uinit

build_init: | rootfs
	cd init && \
		cargo build --release && \
		cp target/x86_64-unknown-none/release/init ../rootfs/init

build_uinit_ctl: | rootfs
	cd uinit-ctl && \
		cargo build --release && \
		cp target/x86_64-unknown-linux-gnu/release/uinit-ctl ../rootfs/usr/bin/uinit-ctl

clean:
	cargo clean
	rm -rf rootfs
