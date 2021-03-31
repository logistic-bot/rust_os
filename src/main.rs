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

/// This trait marks a function as testable. It is used for testing
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
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

#[cfg(not(test))]
#[panic_handler]
/// This function is called on panic, and prints the panic information to the vga text buffer
///
/// It also goes into an infinite loop, so we can observe what went wrong.
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
/// This function is called on panic, and prints the panic information to the vga text buffer and the serial interface
///
/// It also exits qemu with failure
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    println!("{}", info);
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    serial_println!("Ran {} tests", tests.len());
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 1);
}
