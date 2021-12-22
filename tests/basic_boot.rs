#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(spartan::test_runner)]
#![reexport_test_harness_main= "test_main"]

use core::panic::PanicInfo;
use spartan::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

///test cases
#[test_case]
fn test_println_after_boot() {
    spartan::println!("test_println output");
}

#[test_case]
fn test_println_many_after_boot() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}