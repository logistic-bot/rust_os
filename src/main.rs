#![warn(missing_docs)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

//! This module contains the entry point for the kernel.

use bootloader::BootInfo;
use core::panic::PanicInfo;
use rust_os::{println, serial_println};

mod undoc {
    use bootloader::entry_point;
    entry_point!(crate::kernel_main);
}

/// Kernel entry point
///
/// This function is not allowed to return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Initialzing...");
    rust_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(physical_memory_offset) };

    let addresses = [
        // the identity-mapped vga_buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("[  OK  ] Kernel initialized");
    rust_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
/// This function is called on panic, and prints the panic information to the vga text buffer
///
/// It also goes into an infinite loop, so we can observe what went wrong.
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("{}", info);

    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
/// This function is called on panic during test mode, and defers to the test panic handler
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}
