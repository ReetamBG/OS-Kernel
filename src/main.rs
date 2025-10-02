#![no_std] // dont use the std library
#![no_main] // dont use the normal rust entry point

use core::panic::PanicInfo;

// define our own panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// override entry point
// linker looks for a function named _start by default
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

mod vga_buffer;