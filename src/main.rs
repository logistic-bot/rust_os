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
use rust_os::memory::BootinfoFrameAllocator;
use rust_os::println;
#[allow(unused_imports)] // needed for tests
use rust_os::serial_println;
use rust_os::task::keyboard;
use rust_os::task::{executor::Executor, Task};
use x86_64::VirtAddr;

mod undoc {
    use bootloader::entry_point;
    entry_point!(crate::kernel_main);
}

//noinspection RsBorrowChecker
//noinspection RsUnresolvedReference
// Needed because of test_main
/// Kernel entry point
///
/// This function is not allowed to return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::allocator;
    use rust_os::memory;

    println!("Hello World!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootinfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    println!("Kernel initialized, starting task loop");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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
