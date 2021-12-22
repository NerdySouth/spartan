#![no_std] //prevents link to Rust std lib
#![no_main] //disables default Rust entrypoints

use core::panic::PanicInfo;
mod vga_buffer;

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // -> do not mangle name of this function
pub extern "C" fn _start() -> ! {
    //Entrypoint since the Linker looks for a function
    // named _start (LLVM Linker) by default
    println!("Hello World!{}", " from the println! macro.");
    loop {}
}
