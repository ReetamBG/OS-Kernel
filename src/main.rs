#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// implementing our own panic hanlder
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// linker looks for a function named _start as the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world");

    loop {}
}
