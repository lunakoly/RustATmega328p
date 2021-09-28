#![allow(dead_code)]

use crate::atmega328p;

use core::ptr::{write_volatile, read_volatile};

use lcd::{
    Hardware,
    Delay,
    Display,
    FunctionLine,
    FunctionDots,
    DisplayMode,
    DisplayCursor,
    DisplayBlink,
    EntryModeDirection,
    EntryModeShift,
};

// 0 -> commands, 1 -> data
const RS_BIT: u8 = 1 << 0;
// 0 -> write, 1 -> read
const RW_BIT: u8 = 1 << 1;
// strobe
const E_BIT: u8 = 1 << 2;

const DATA_MASK: u8 = 0b00001111;

pub struct HW {}

fn set_control_flag(flag: u8, value: bool) {
    unsafe {
        let current = read_volatile(atmega328p::PINC);

        write_volatile(
            atmega328p::PORTC,
            if value {
                current | flag
            } else {
                current & !flag
            }
        );
    }
}

impl Hardware for HW {
    fn rs(&self, bit: bool) {
        set_control_flag(RS_BIT, bit);
    }

    fn enable(&self, bit: bool) {
        set_control_flag(E_BIT, bit);
    }

    fn data(&self, data: u8) {
        unsafe {
            let cleared = read_volatile(atmega328p::PIND) & DATA_MASK;

            write_volatile(
                atmega328p::PORTD,
                cleared | (data << 4)
            )
        }
    }

    fn mode(&self) -> lcd::FunctionMode {
        lcd::FunctionMode::Bit4
    }
}

impl Delay for HW {
    fn delay_us(&self, delay_usec: u32) {
        avr_delay::delay_us(delay_usec);
    }
}

const HARDWARE: HW = HW {};

pub type TheDisplay = Display<'static, HW>;

pub fn configure() -> TheDisplay {
    unsafe {
        // Port C initialization
        // Bit2..Bit0=Out
        write_volatile(atmega328p::DDRC, read_volatile(atmega328p::DDRC) | 0b00000111);
        // Bit2..Bit0=1
        write_volatile(atmega328p::PORTC, read_volatile(atmega328p::PORTC) | 0b00000111);

        // Port D initialization
        // Bit7..Bit4=Out
        write_volatile(atmega328p::DDRD, read_volatile(atmega328p::DDRC) | 0b11110000);
        // Bit7..Bit4=1
        write_volatile(atmega328p::PORTD, read_volatile(atmega328p::DDRC) | 0b11110000);
    }

    // Write-only
    set_control_flag(RW_BIT, false);

    let mut lcd = Display::new(&HARDWARE);

    lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);
    lcd.display(DisplayMode::DisplayOn, DisplayCursor::CursorOn, DisplayBlink::BlinkOn);
    lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::NoShift);

    lcd.print("Welcome to LCD!");
    lcd
}

