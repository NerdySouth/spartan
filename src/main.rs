#![no_std] //prevents link to Rust std lib
#![no_main] //disables default Rust entrypoints
#![feature(custom_test_frameworks)] //allow custom test framework since Rust default needs stdlib
#![test_runner(spartan::test_runner)]
#![reexport_test_harness_main = "test_main"] //rename the exported main  from our test framework

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use spartan::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use spartan::memory;
    use x86_64::{
        structures::paging::{Page, Translate},
        VirtAddr,
    };
    //Entrypoint since the Linker looks for a function
    // named _start (LLVM Linker) by default
    println!("Hello world!");
    //initalization routines
    spartan::init();

    //set up virtual memory
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    let addresses = [
        //identity mapped VGA Buffer
        0xb8000,
        //some code page
        0x201008,
        //some stack page
        0x100_0020_1a10,
        //virt addr mapped to phys addr 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

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
