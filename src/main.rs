#![no_std]
#![no_main]

use core::panic::PanicInfo;
use os_kernel::{init, println};

// linker looks for a function named _start as the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // initialize interrupt descriptor table
    init();
    
    println!("Hello world");

    loop {}
}

// implementing our own panic hanlder
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
