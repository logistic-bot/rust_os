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
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.0 / 3.0
    )
    .unwrap();

    loop {}
}

#[panic_handler]
/// This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
