#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
/// Kernel entry point
///
/// This function is not allowed to return
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
/// This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
