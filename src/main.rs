// A minimal Rust kernel from Phil Opp's blog series
// Adapted from https://os.phil-opp.com/minimal-rust-kernel/

#![no_std]  // Disable the standard library
#![no_main] // Disable standard Rust entry point

mod vgabfr;

use core::panic::PanicInfo;

// Define the panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point for the kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vgabfr::WRITER.lock().write_str("Hello again").unwrap();
    write!(vgabfr::WRITER.lock(), ", some numbers: {} {}\n", 42, 1.337).unwrap();

    println!("And... Hello World{}", "!");

    loop {}
}
