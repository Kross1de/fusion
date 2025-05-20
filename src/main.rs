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

static HELLO: &[u8] = b"Hello World!";

// Entry point for the kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
