use crate::usart;
use crate::devices;

use crate::conversion::{CPointerCompatible, C32Compatible};
use crate::{c_line, c_string, c_string_receiver};

extern "C" {
    // Full avr-libc reference for looking up prototypes:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html

    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;
    fn sprintf(buffer: *mut i8, fmt: *const i8, ...) -> i16;
}

pub fn run() {
    unsafe {
        let stdout = usart::get_c_stream();
        fprintf(stdout, c_line!("Well, 3 + 2 = %lu, but 3 * 2 = %lu."), 3 + 2, 3 * 2);
        fprintf(stdout, c_line!("And pi = %f"), 3.14159265.to_c());
    }

    let mut display = devices::lcd_display::configure();
    let mut buffer = [0u8; 100];

    unsafe {
        sprintf(c_string_receiver!(buffer), c_line!("Also, 3^2 = %d"), 3 * 3);
    }

    display.clear();
    display.print(core::str::from_utf8(&buffer).unwrap());
}
