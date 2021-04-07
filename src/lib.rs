#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

/// Initialize hardware and software
pub fn init() {
    interrupts::init_idt();
}

/// This trait marks a function as testable. It is used for testing
pub trait Testable {
    fn run(&self);
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

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    serial_println!("Ran {} tests", tests.len());
    exit_qemu(QemuExitCode::Success);
}

/// This function is called on panic, and prints the panic information to the vga text buffer and the serial interface
///
/// It also exits qemu with failure
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    println!("{}", info);
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Failed);

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Entry point for tests
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
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
