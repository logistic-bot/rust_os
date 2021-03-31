#![warn(missing_docs)]
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

//! This module contains the entry point for the kernel.

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
/// Kernel entry point
///
/// This function is not allowed to return
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
/// Exit code for qemu. This is primarly used for unit testing.
pub enum QemuExitCode {
    /// No problem, all test passed.
    Success = 0x10,
    /// Some problem, some test failed.
    Failed = 0x11,
}

/// Exit qemu (thus terminating the kernel) with the specified exit code.
///
/// A suitable debug-exit device is expected on port 0xf4.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[panic_handler]
/// This function is called on panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    serial_println!("Ran {} tests", tests.len());
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("Trivial assertion... ");
    assert_eq!(0, 1);
    serial_println!("[ok]");
}
