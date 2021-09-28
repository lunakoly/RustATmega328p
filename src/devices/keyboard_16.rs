#![allow(dead_code)]

use crate::atmega328p;

use core::ptr::{write_volatile, read_volatile};

const COLUMN_0: u8 = 1 << 4;
const COLUMN_1: u8 = 1 << 5;
const COLUMN_2: u8 = 1 << 6;
const COLUMN_3: u8 = 1 << 7;

const ROW_0: u8 = 1 << 0;
const ROW_1: u8 = 1 << 1;
const ROW_2: u8 = 1 << 2;
const ROW_3: u8 = 1 << 3;

pub struct Keyboard {
    buttons: [bool; 16],
}

pub struct KeyboardListener<F: FnMut(usize)> {
    keyboard: Keyboard,
    reported: bool,
    on_pressed: F,
}

impl Keyboard {
    pub fn read_keys_state(&mut self) {
        for index in 0..4 {
            avr_delay::delay_ms(6);

            let zero_mask = (ROW_3 | ROW_2 | ROW_1 | ROW_0) ^ (1 << index);

            let response = unsafe {
                write_volatile(atmega328p::PORTB, zero_mask);
                avr_delay::delay_ms(3);
                read_volatile(atmega328p::PINB)
            };

            let offset = 12 - 4 * index;

            self.buttons[offset + 0] = response & COLUMN_0 == 0;
            self.buttons[offset + 1] = response & COLUMN_1 == 0;
            self.buttons[offset + 2] = response & COLUMN_2 == 0;
            self.buttons[offset + 3] = response & COLUMN_3 == 0;
        }
    }

    pub fn find_first_pressed(&self) -> Option<usize> {
        for (it, &is_pressed) in self.buttons.iter().enumerate() {
            if is_pressed {
                return Some(it);
            }
        }

        None
    }

    pub fn listen<F: FnMut(usize)>(self, on_pressed: F) -> KeyboardListener<F> {
        KeyboardListener {
            keyboard: self,
            reported: false,
            on_pressed: on_pressed,
        }
    }
}

impl<F: FnMut(usize)> KeyboardListener<F> {
    pub fn update(&mut self) {
        self.keyboard.read_keys_state();
        let pressed = self.keyboard.find_first_pressed();

        match (pressed, self.reported) {
            (None, _) => self.reported = false,
            (Some(key), false) => {
                self.reported = true;
                (self.on_pressed)(key);
            },
            _ => {}
        }
    }
}

pub fn configure() -> Keyboard {
    unsafe {
        // Port B initialization
        // Bit7..Bit4=In Bit3..Bit0=Out
        write_volatile(atmega328p::DDRB, 0b00001111);
        // Bit7..Bit4=T Bit3..Bit0=1
        write_volatile(atmega328p::PORTB, 0b00001111);
    }

    Keyboard {
        buttons: [false; 16],
    }
}
