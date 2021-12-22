#![no_std] //prevents link to Rust std lib
#![no_main] //disables default Rust entrypoints
#![feature(custom_test_frameworks)] //allow custom test framework since Rust default needs stdlib
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] //rename the exported main  from our test framework


use core::panic::PanicInfo;
mod vga_buffer;
mod serial;

//called on panic
#[cfg(not(test))]  //use this panic handler when not running tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)] // use this panic handler when running tests
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // -> do not mangle name of this function
pub extern "C" fn _start() -> ! {
    //Entrypoint since the Linker looks for a function
    // named _start (LLVM Linker) by default
    println!("Hello world!");

    #[cfg(test)]
    test_main();

    loop {}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


/*
 Test Framework functions

 */

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        //type_name of Fn() is the function's name
        serial_print!("{}...\t", core::any::type_name::<T>());
        self(); //works since we require that self implements the Fn() trait
        serial_println!("[passed]");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    if tests.len() == 0 {
        return
    }
    serial_println!("Running {} tests.", tests.len());
    for test in tests {
        test.run();
    }

    //exit after tests
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assert() {
    assert_eq!(1, 1);
}