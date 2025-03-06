#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(Nos_OS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use Nos_OS::vga_print;
use Nos_OS::vga_println;

/// Default Panic handler. This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_println!("{}", info);
    loop {}
}

/// Test mode Panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Nos_OS::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // entry point function, since the linker looks for a function named `_start` by default
    Nos_OS::init();

    vga_println!("Welcome to Nos OS");

    #[cfg(test)]
    test_main();

    loop {}
}
