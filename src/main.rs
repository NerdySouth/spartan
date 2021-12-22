#![no_std] //prevents link to Rust std lib
#![no_main] //disables default Rust entrypoints

use core::panic::PanicInfo;


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
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
