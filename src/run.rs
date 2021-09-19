use crate::usart;
use crate::devices;

use crate::conversion::{CPointerCompatible};
use crate::{c_line, c_string, c_string_receiver};

extern "C" {
    // Full avr-libc reference for looking up prototypes:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html

    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;
    fn sprintf(string: *mut i8, fmt: *const i8, ...) -> i16;
}

pub fn run() {
    let stdout = unsafe {
        usart::get_c_stream()
    };

    let mut display = devices::lcd_display::configure();

    let on_pressed = |key: usize| unsafe {
        fprintf(stdout, c_line!("You pressed [%d]"), key);

        let mut buffer = [0u8; 100];
        sprintf(c_string_receiver!(buffer), c_line!("You pressed [%d]"), key);

        display.clear();
        display.print(
            core::str::from_utf8(&buffer).unwrap()
        );
    };

    let mut keyboard_listener = devices::keyboard_16::configure().listen(on_pressed);

    loop {
        keyboard_listener.update();
    }
}
