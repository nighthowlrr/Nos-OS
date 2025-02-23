#![no_std]
#![no_main]

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

    vga_println!("Hellö!");
    vga_println!("Welcome to Nos OS");
    vga_println!("The numbers are {} and {}", 42, 1.0/3.0);

    loop {}
}
