/*
 * Gem Research Operating System
 * Interrupts
 * © 2022 Upper Altitude
 */

use x86_64::structures::idt::InterruptDescriptorTable; // We are using the InterruptDescriptorTable struct of the x86_64 crate.
use x86_64::structures::idt::InterruptStackFrame;
use crate::println;
use lazy_static::lazy_static;

/*************************
 *          IDT          *
 *************************/

// Load the IDT so that the CPU uses it.
// The IDT.load() method expects a static self,  a reference that is valid for the complete runtime of the program.
// The reason is that the CPU will access this table on every interrupt until we load a different IDT.
// Instead of evaluating a static at compile time, `lazy_static` performs the initialization when the static is referenced the first time.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

// Exposes the IDT.load() method.
pub fn init_idt() {
    IDT.load();
}

/*************************
 *   Breakpoint Handler  *
 *************************/
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
    println!("Exception: Breakpoint\n{:#?}", stack_frame);
}

/*************************
 *     Fault Handler     *
 *************************/
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("Exception: Double Fault\n{:#?}", stack_frame);
}
