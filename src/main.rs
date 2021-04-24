#![deny(missing_docs)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

//! This module contains the entry point for the kernel.

extern crate alloc;

use bootloader::BootInfo;
use core::panic::PanicInfo;
use rust_os::{println, serial_println};
use alloc::boxed::Box;

mod undoc {
    use bootloader::entry_point;
    entry_point!(crate::kernel_main);
}

//noinspection RsUnresolvedReference
// Needed because of test_main
/// Kernel entry point
///
/// This function is not allowed to return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Initialzing...");
    rust_os::init();

    let x = Box::new(42);

    #[cfg(test)]
        test_main();

    println!("Kernel initialized");
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
