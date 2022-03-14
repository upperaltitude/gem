/*
 * Gem Research Operating System
 * Kernel
 * © 2022 Upper Altitude
 */

#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo;
use gem::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print panic message.
    println!("{}", info);
    
    // Halt CPU until the next interrupt arrives.
    gem::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Gem.");
    println!("");

    gem::init();

    // Invoke a breakpoint exception.
    // x86_64::instructions::interrupts::int3();

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // panic!("Some panic message");

    println!("Gem is still running...");

    // Halt CPU until the next interrupt arrives.
    gem::hlt_loop();
}
