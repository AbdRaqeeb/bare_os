// no using of Rust standard library
#![no_std]
// do not use `main` as the normal point of entry
// since we do not have access to the Rust runtime and the crt0 `("C runtime zero")`
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// Having a main function without an underlying runtime that calls it does not make sense
// we'll define a custom entry point with our own _start function.
// We disable name mangling so that the compiler doesn't generate some cryptic name for our function
// we require this so that we can give the name of our entry point to the function linker
// `extern "C"` tells the compiler to use C calling convention
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
