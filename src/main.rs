#![warn(missing_docs)]
#![no_std]
#![no_main]

//! This module contains the entry point for the kernel.

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
/// Kernel entry point
///
/// This function is not allowed to return
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    panic!("End of kernel!");

    loop {}
}

#[panic_handler]
/// This function is called on panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
