// About no_std:
// https://github.com/rust-embedded/cortex-m-quickstart/issues/58

#![no_std]
#![no_main]
#![feature(lang_items)]

mod atmega328p;
mod conversion;
mod board;
mod usart;

use conversion::{CPointerCompatible, C32Compatible};

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

extern "C" {
    // Full avr-libc reference for looking up prototypes:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html

    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;
}

#[no_mangle]
fn main() {
    board::configure();

    unsafe {
        let stdout = usart::get_c_stream();
        fprintf(stdout, c_line!("Well, 3 + 2 = %lu, but 3 * 2 = %lu."), 3 + 2, 3 * 2);
        fprintf(stdout, c_line!("And pi = %f\r\n"), 3.14159265.to_c());
    }

    loop {}
}
