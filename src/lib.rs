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
    gdt::init();
	interrupts::init_idt();
}
