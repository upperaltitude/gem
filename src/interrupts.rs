/*
 * Gem Research Operating System
 * Interrupts
 * © 2022 Upper Altitude
 */

use x86_64::structures::idt::InterruptDescriptorTable; // We are using the InterruptDescriptorTable struct of the x86_64 crate.
use x86_64::structures::idt::InterruptStackFrame;
use crate::println;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use crate::print;

/*************************
 *          PICs         *
 *************************/
// The default configuration of the PICs is not usable, because it sends interrupt vector numbers in the range 0–15 to the CPU.
// These numbers are already occupied by CPU exceptions.
// We need to remap the PIC interrupts to different numbers.
// We’re setting the offsets for the PICs to the range 32–47.
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

// Interrupts Index
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");

    unsafe {
        // Notify the PIC that the interrupt was processed and that we're ready for the next one.
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

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

        // Include Interrupts.
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);

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
