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

        let a = 1;
        let b = 2;
        let c = 3;
        let d = 4;

        let pair1 = a + (b << 16);
        let pair2 = c + (d << 16);

        fprintf(stdout, "And let the numbers be: %d, %d, %d, %d\r\n\0".as_ptr() as *const i8, pair1, pair2);

        let pi = 3.14159265f32;
        let pi_parts = pi.to_le_bytes();
        let new_pi =
            ((pi_parts[0] as i32) << 0) +
            ((pi_parts[1] as i32) << 8) +
            ((pi_parts[2] as i32) << 16) +
            ((pi_parts[3] as i32) << 24);

        fprintf(stdout, "And pi = %f\r\n\0".as_ptr() as *const i8, new_pi);
    }

    loop {}
}
