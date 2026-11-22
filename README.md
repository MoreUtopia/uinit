# Uinit - BETA

Uinit is a system init for Utopia Kernel.
It's designed to be more functional than finit.

The goal is to be lightweight, simple, reliable and easy way to configure (and initialize ofc) Utopia kernel.

Remember that some functionality can't be added until it will be supported in Utopia kernel.

Right now Uinit supports only x86_64.

# Roadmap for 1.0-stable

## Configuration
- [X] Configuration file - `/etc/uinit.toml`
    - [X] Configure environment variables (like `PATH`).
    - [X] Configure default init executable.
    - [X] Configure mount points.

## System Initialization
- [X] Small wrapper to run from native abi
- [X] Check for being PID 1.
- [X] Mount pseudo filesystems (`proc`, `sysfs`, `devtmpfs`, `ramfs`).

## Process Management
- [X] Restart init shell when it exits.
- [ ] Track child processes.

## System
- [ ] Reboot/Shutdown handling.
    - [ ] Stop managed processes before shutdown/reboot by sending signals.
    - [ ] Unmount filesystems before shutdown/reboot.

## Reliability
- [X] Handle invalid configuration files.
- [X] Improve error handling and add more logs.
- [ ] Prevent infinite restart loops.

## Documentation
- [X] Create example configuration.
- [X] Add installation instructions.
- [ ] Document supported features and limitations.