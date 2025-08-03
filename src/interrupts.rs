use lazy_static::lazy_static;
use x86_64::structures::idt::{
    DivergingHandlerFuncWithErrCode, InterruptDescriptorTable, InterruptStackFrame,
};

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // temporary workaround to fix regression in nightly compiler with externed functions with
        // return types: https://github.com/rust-lang/rust/pull/143075
        let double_fault_handler_ptr = unsafe { *(double_fault_handler as *mut DivergingHandlerFuncWithErrCode) };
        idt.double_fault.set_handler_fn(double_fault_handler_ptr);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
