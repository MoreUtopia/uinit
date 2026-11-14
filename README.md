# Uinit - BETA

Uinit is a system init for Utopia Kernel.
It's designed to be more functional than finit, but not to replace it. (because we are using linux abi and finit uses native abi)

The goal is to be lightweight, simple, reliable and easy way to configure (and initialize ofc) Utopia kernel.

Remember that some functionality can't be added until it will be supported in Utopia kernel.

Right now It was only checked for working on x86_64 but RISC-V64 will be supported.

# Roadmap for 1.0-stable

## Configuration
- [ ] Configuration file - `/etc/uinit.toml`
    - [ ] Configure environment variables (like `PATH`, `HOSTNAME`, `USERNAME`).
    - [ ] Configure default shell.
    - [ ] Configure mount points.

## System Initialization
- [X] Check for being PID 1.
- [X] Mount pseudo filesystems (`proc`, `sysfs`, `devtmpfs`, `ramfs`).

## Process Management
- [X] Restart init shell when it exits.
- [ ] Configurable restart policies.
- [ ] Track child processes.

## System
- [ ] Reboot/Shutdown handling.
    - [ ] Stop managed processes before shutdown/reboot by sending signals.
    - [ ] Unmount filesystems before shutdown/reboot.

## Reliability
- [ ] Handle invalid configuration files.
- [ ] Prevent infinite restart loops.
- [ ] Improve error handling and add more logs

## Documentation
- [ ] Create example configuration.
- [X] Add installation instructions.
- [ ] Document supported features and limitations.