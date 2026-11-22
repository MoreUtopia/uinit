#![no_std]
#![no_main]

use utopia_sys::{println, wrapper::{becomes}, SyscallConv};

#[unsafe(no_mangle)]
extern "Rust" fn main() {
    println!("starting main uinit executable");
    let _ = becomes(c"/uinit", &[], &[], SyscallConv::Linux);
}
