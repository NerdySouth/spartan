#![no_std] //prevents link to Rust std lib
#![no_main] //disables default Rust entrypoints
#![feature(custom_test_frameworks)] //allow custom test framework since Rust default needs stdlib
#![test_runner(spartan::test_runner)]
#![reexport_test_harness_main = "test_main"] //rename the exported main  from our test framework

use core::panic::PanicInfo;
use spartan::println;
use x86_64::registers::control::Cr3;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    //Entrypoint since the Linker looks for a function
    // named _start (LLVM Linker) by default
    println!("Hello world!");
    //initalization routines
    spartan::init();



    #[cfg(test)]
    test_main();

    spartan::halt();
}

//called on panic
#[cfg(not(test))] //use this panic handler when not running tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    spartan::halt();
}

#[cfg(test)] // use this panic handler when running tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    spartan::test_panic_handler(_info);
    spartan::halt();
}
