// About no_std:
// https://github.com/rust-embedded/cortex-m-quickstart/issues/58

#![no_std]
#![no_main]

mod usart;

extern "C" {
    // Full avr-libc reference for looking up prototypes:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html

    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;
}

#[no_mangle]
fn main() {
    usart::configure();

    for &b in b"[Data]\r\n\0" {
        usart::transmit(b);
    }

    unsafe {
        let stdout = usart::get_usart_c_stream();
        fprintf(stdout, "Hi, I'm %d!\0".as_ptr() as *const i8, 21);
    }

    loop {}
}
