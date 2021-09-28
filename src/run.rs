use crate::usart;
use crate::devices;

use crate::conversion::{CPointerCompatible};
use crate::{c_line, c_string, c_string_receiver};

use devices::lcd_display::TheDisplay;

extern "C" {
    // Full avr-libc reference for looking up prototypes:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html

    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;
    fn sprintf(buffer: *mut i8, fmt: *const i8, ...) -> i16;
}

fn print_upper_lower(
    display: &mut TheDisplay,
    pattern: *const i8,
    upper: u32,
    lower: u32,
    row: u8,
) {
    let mut buffer = [0u8; 100];

    unsafe {
        sprintf(c_string_receiver!(buffer), pattern, upper, lower);
    }

    display.position(0, row);
    display.print(core::str::from_utf8(&buffer).unwrap());

    // Magic timing
    avr_delay::delay_ms(32);
}

fn print_voltage(display: &mut TheDisplay) {
    let value = devices::adc::read(3) as u32;
    let upper = value * 5u32 / 1024u32;
    let lower = value * 1000u32 * 5u32 / 1024u32 - (upper * 1000u32);
    print_upper_lower(display, c_line!("U = %ld.%03ld V"), upper, lower, 0);
}

fn print_current(display: &mut TheDisplay) {
    let value = devices::adc::read(4) as u32;
    let upper = value * 5u32 / 1024u32;
    let lower = value * 1000u32 * 5u32 / 1024u32 - (upper * 1000u32);
    print_upper_lower(display, c_line!("I = %ld.%03ld A"), upper, lower, 1);
}

pub fn run() {
    devices::adc::configure();

    let stdout = unsafe {
        usart::get_c_stream()
    };

    let on_pressed = |key: usize| unsafe {
        fprintf(stdout, c_line!("You pressed [%d]"), key);
    };

    let mut keyboard_listener = devices::keyboard_16::configure().listen(on_pressed);
    let mut display = devices::lcd_display::configure();

    loop {
        keyboard_listener.update();
        print_voltage(&mut display);
        print_current(&mut display);
    }
}
