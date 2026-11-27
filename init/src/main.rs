#![no_std]
#![no_main]

use utopia_sys::{SyscallConv, println, wrapper::becomes};

#[unsafe(no_mangle)]
extern "Rust" fn main() {
    println!("starting main uinit executable");
    let _ = becomes(c"/sbin/uinit", &[], &[], SyscallConv::Linux);
}
