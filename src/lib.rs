#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![deny(missing_docs)]

//! rust_os Library

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

use core::panic::PanicInfo;

extern crate alloc;

/// Memory allocations
pub mod allocator;
/// Global Descriptor Table
pub mod gdt;
/// Interrupts
pub mod interrupts;
/// Memory managment
pub mod memory;
/// Serial io
pub mod serial;
/// Task struct for async stuff
pub mod task;
pub mod vga_buffer;

/// Halt the cpu in a loop, never returns.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}

/// Initialize hardware and software
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    };
    x86_64::instructions::interrupts::enable();
}

/// This trait marks a function as testable. It is used for testing
pub trait Testable {
    /// Runnable function
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

/// Run each test sequentially.
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

    hlt_loop();
}

//noinspection RsUnresolvedReference
// needed because of test_main
/// Entry point for tests
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();

    hlt_loop();
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
