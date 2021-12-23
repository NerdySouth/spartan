#![no_std] //prevents link to Rust std lib
#![no_main] //disables default Rust entrypoints
#![feature(custom_test_frameworks)] //allow custom test framework since Rust default needs stdlib
#![test_runner(spartan::test_runner)]
#![reexport_test_harness_main = "test_main"] //rename the exported main  from our test framework

use core::panic::PanicInfo;
use spartan::println;

#[no_mangle] // -> do not mangle name of this function
pub extern "C" fn _start() -> ! {
    //Entrypoint since the Linker looks for a function
    // named _start (LLVM Linker) by default
    println!("Hello world!");

    #[cfg(test)]
    test_main();

    loop {}
}

//called on panic
#[cfg(not(test))] //use this panic handler when not running tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)] // use this panic handler when running tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    spartan::test_panic_handler(_info);
}
