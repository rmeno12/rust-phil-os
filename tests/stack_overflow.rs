#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use lazy_static::lazy_static;
use rust_phil_os::{exit_qemu, serial_print, serial_println, volatile::Volatile};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        let df_opts = idt.double_fault.set_handler_fn(unsafe {
            core::mem::transmute(
                test_double_fault_handler as extern "x86-interrupt" fn(InterruptStackFrame, u64),
            )
        });
        unsafe {
            df_opts.set_stack_index(rust_phil_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    serial_println!("[ok]");
    exit_qemu(rust_phil_os::QemuExitCode::Success);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    serial_print!("stack_overflow::stack_overflow...\t");

    rust_phil_os::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_phil_os::test_panic_handler(info)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    Volatile::new(0).read(); // to stop tail call optimization
}
