#![no_std] // dont use the std library
#![no_main] // dont use the normal rust entry point

use core::panic::PanicInfo;

// override entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

// define our own panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
