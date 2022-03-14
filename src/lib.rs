/*
 * Gem Research Operating System
 * Library
 * © 2022 Upper Altitude
 */

#![no_std]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;

// The central place for initialization routines.
pub fn init() {
    println!("Initializing Core Libraries...");

    gdt::init();
	interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; // Unsafe because it can cause undefined behavior if the PIC is misconfigured.
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        // This is just a thin wrapper around the assembly `hlt` instruction.
        // It is safe because there’s no way it can compromise memory safety.
        x86_64::instructions::hlt();
    }
}