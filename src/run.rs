use crate::usart;
use crate::atmega328p;

use core::ptr::{write_volatile, read_volatile};

use crate::conversion::{CPointerCompatible};
use crate::{c_line, c_string};

extern "C" {
    // Full avr-libc reference for looking up prototypes:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html

    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;
}

const COLUMN_0: u8 = 1 << 0;
const COLUMN_1: u8 = 1 << 1;
const COLUMN_2: u8 = 1 << 2;
const COLUMN_3: u8 = 1 << 3;

const ROW_0: u8 = 1 << 4;
const ROW_1: u8 = 1 << 5;
const ROW_2: u8 = 1 << 6;
const ROW_3: u8 = 1 << 7;

pub fn run() {
    unsafe {
        // Port B initialization
        // Bit7..Bit4=In Bit3..Bit0=Out
        write_volatile(atmega328p::DDRB, 0b00001111);
        // Bit7..Bit4=T Bit3..Bit0=1
        write_volatile(atmega328p::DDRB, 0b00001111);
    }

    let mut buttons = [true; 16];

    let stdout = unsafe {
        usart::get_c_stream()
    };

    let mut reported = false;

    loop {
        for index in 0..4 {
            avr_delay::delay_ms(6);

            let zero_mask = (COLUMN_3 | COLUMN_2 | COLUMN_1 | COLUMN_0) ^ (1 << index);

            let response = unsafe {
                write_volatile(atmega328p::PORTB, zero_mask);
                avr_delay::delay_ms(3);
                read_volatile(atmega328p::PINB)
            };

            buttons[0  + index] = response & ROW_3 == 0;
            buttons[4  + index] = response & ROW_2 == 0;
            buttons[8  + index] = response & ROW_1 == 0;
            buttons[12 + index] = response & ROW_0 == 0;
        }

        let mut pressed = -1;

        for (it, &is_pressed) in buttons.iter().enumerate() {
            if is_pressed {
                pressed = it as i32;
            }
        }

        if pressed == -1 {
            reported = false;
        } else if !reported {
            unsafe {
                fprintf(stdout, c_line!("You pressed [%d]"), pressed);
            }
            reported = true;
        }
    }
}
