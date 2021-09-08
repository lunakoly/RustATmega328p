// About no_std:
// https://github.com/rust-embedded/cortex-m-quickstart/issues/58

#![no_std]
#![no_main]
#![feature(lang_items)]

mod atmega328p;
mod board;
mod usart;

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

    for &b in b"[Data]\r\n" {
        usart::transmit(b);
    }

    unsafe {
        let stdout = usart::get_c_stream();
        fprintf(stdout, "3 + 2 = %d\0".as_ptr() as *const i8, 3 + 2);
    }

    loop {}
}
