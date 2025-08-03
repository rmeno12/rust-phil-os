#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_phil_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_phil_os::{hlt_loop, println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_phil_os::test_panic_handler(info)
}


#[test_case]
fn test_println() {
    println!("test_println output");
}
