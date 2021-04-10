#![warn(missing_docs)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

//! This module contains the entry point for the kernel.

use bootloader::BootInfo;
use core::panic::PanicInfo;
use rust_os::println;

mod undoc {
    use bootloader::entry_point;
    entry_point!(crate::kernel_main);
}

/// Kernel entry point
///
/// This function is not allowed to return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::active_level_4_table;
    use x86_64::structures::paging::PageTable;
    use x86_64::VirtAddr;

    println!("         Initialzing...");
    rust_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(physical_memory_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            rust_os::serial_println!("L4 {}: {:?}", i, entry);

            // Get the physical address from the entry and convert it
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // print non-empty entries of level 3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    rust_os::serial_println!("  L3 {}: {:?}", i, entry);
                }
            }
        }
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

    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
/// This function is called on panic during test mode, and defers to the test panic handler
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}
