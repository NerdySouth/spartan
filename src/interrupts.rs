use crate::gdt;
use crate::println;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// IDT initialization (lazy static loaded at first reference, not compile time)
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        //set up double fault handler
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.divide_error.set_handler_fn(divide_by_zero_handler);
        idt
    };
}

// intialization function for use in  the kernal _start entrypoint
pub fn init_idt() {
    IDT.load();
}

// Handler for breakpoint exceptions. Prints stack frame, returns execution to caller.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame)
}

// handler for double fault exceptions. Diverging function, never returns since a double fault is  not fixable
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

// handler for a divide by zero error. Prints stack frame, r eturns execution to caller.
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame)
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

// Hardware interrupts

// // initialize our 8259 PIC
// pub const PIC_1_OFFSET: u8 = 32;
// pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

// // use a mutex on the PIC for mutable access via the lock method.
// pub static PICS: spin::Mutex<ChainedPics> =
//     spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
