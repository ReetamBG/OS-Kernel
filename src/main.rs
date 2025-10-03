#![no_std] // dont use the std library
#![no_main] // dont use the normal rust entry point

use core::panic::PanicInfo;

// define our own panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// override entry point
// linker looks for a function named _start by default
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world finally easy");
    println!("YOYO {}", 2.001);

    loop {}
}

mod vga_buffer;