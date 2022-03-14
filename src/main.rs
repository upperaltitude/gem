/*
 * Gem Research Operating System
 * Kernel
 * © 2022 Upper Altitude
 */

#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print panic message.
    println!("{}", info);

    loop {}
}

static HELLO: &[u8] = b"Welcome to Gem.";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}
