# Uinit - BETA

Uinit is a system init for Utopia Kernel, designed to be more functional than `finit`.
The goal is to be a lightweight (About 1MB all tools), simple, reliable, and easy way to configure and initialize Utopia Kernel.
Note that some functionality cannot be added until it is supported by Utopia Kernel.

Right now, Uinit supports only `x86_64`.
More details in `/docs/uinit.md`.

## Elements

### Init
A small wrapper allowing Uinit to boot from the native ABI.
More details in `/docs/init.md`.

### Uinit-ctl
A simple CLI tool enabling runtime communication with `uinit` (PID 1).
More details in `/docs/uinit-ctl.md`.

### Installation
Build and rootfs deployment instructions.
More details in `/docs/installation.md`.
