#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_kernel::{init, serial_println};

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_kernel::test_panic_handler(info)
}

#[test_case]
fn test_interrupt() {
    serial_println!("Triggering breakpoint interrupt manually...");
    x86_64::instructions::interrupts::int3();

    serial_println!("It did not crash, interrupt was handled successfully!");
}
