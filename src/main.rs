#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // entry point function, since the linker looks for a function named `_start` by default

    vga_println!("Welcome to Nos OS\n");

    #[cfg(test)]
    test_main();

    loop {}
}


// Testing code

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    vga_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    vga_print!("trivial assertion... ");
    assert_eq!(1, 1);
    vga_println!("[ok]");
}