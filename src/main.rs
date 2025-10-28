#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_kernel::{hlt_loop, init, print, println};

// linker looks for a function named _start as the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // initialize some basic things like GDT, IDT etc
    init();

    println!("Hello world");

    #[cfg(test)]
    test_main();

    print!("It did not crash!");

    hlt_loop();
}

//TODO: Remove test related stuff from main.rs as all tests are being done independently
// implementing our own panic hanlder
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

// panic handler for tests mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_kernel::test_panic_handler(info);
}
