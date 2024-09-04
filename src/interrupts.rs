use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

// Using lazy static here, because we need the
// Interrupt Descriptor Table to have a lifetime
// longer than the init function. So we can opt
// to use the heap, tho not created in our OS yet.
// `static mut` is prone to data races so we can
// just lazily create the variable upon first access
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}