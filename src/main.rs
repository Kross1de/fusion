// A minimal Rust kernel from Phil Opp's blog series
// Adapted from https://os.phil-opp.com/minimal-rust-kernel/

#![no_std]  // Disable the standard library
#![no_main] // Disable standard Rust entry point

use core::panic::PanicInfo;

// Define the panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point for the kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
