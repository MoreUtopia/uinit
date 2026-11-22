.PHONY: all build build_uinit build_init rootfs

UINIT_TARGET := x86_64-unknown-linux-gnu
INIT_TARGET  := x86_64-unknown-none

all: build

build: build_uinit build_init

rootfs:
	mkdir -p rootfs

build_uinit: | rootfs
	cargo build --release --target $(UINIT_TARGET)
	cp target/$(UINIT_TARGET)/release/uinit rootfs/uinit

build_init: | rootfs
	cd init && \
		cargo build --release --target $(INIT_TARGET) && \
		cp target/$(INIT_TARGET)/release/init ../rootfs/init