#![allow(dead_code)]

pub const CLKPR: *mut u8 = 0x61 as *mut u8;
pub const CLKPCE: u8 = 1 << 7;

pub const UDR0: *mut u8 = 0xc6 as *mut u8;

pub const UCSR0A: *mut u8 = 0xc0 as *mut u8;
pub const UDRE0: u8 = 1 << 5;
pub const RXC0: u8 = 1 << 7;

pub const UCSR0B: *mut u8 = 0xc1 as *mut u8;
pub const TXEN0: u8 = 1 << 3;

pub const UCSR0C: *mut u8 = 0xc2 as *mut u8;
pub const UCSZ00: u8 = 1 << 1;
pub const UCSZ01: u8 = 1 << 2;

pub const UBRR0: *mut u16 = 0xc4 as *mut u16;

pub const PINB: *mut u8 = 0x23 as *mut u8;

pub const DDRB: *mut u8 = 0x24 as *mut u8;

pub const PORTB: *mut u8 = 0x25 as *mut u8;

pub const PINC: *mut u8 = 0x26 as *mut u8;

pub const DDRC: *mut u8 = 0x27 as *mut u8;

pub const PORTC: *mut u8 = 0x28 as *mut u8;

pub const PIND: *mut u8 = 0x29 as *mut u8;

pub const DDRD: *mut u8 = 0x2a as *mut u8;

pub const PORTD: *mut u8 = 0x2b as *mut u8;

pub const ADMUX: *mut u8 = 0x7c as *mut u8;
pub const REFS0: u8 = 1 << 6;

pub const ADCSRA: *mut u8 = 0x7a as *mut u8;
pub const ADEN0: u8 = 1 << 7;
pub const ADSC0: u8 = 1 << 6;
pub const ACO0: u8 = 1 << 5;
pub const ADIF0: u8 = 1 << 4;
pub const ADIE0: u8 = 1 << 3;
pub const ADPS2: u8 = 1 << 2;
pub const ADPS1: u8 = 1 << 1;
pub const ADPS0: u8 = 1 << 0;

pub const ADCSRB: *mut u8 = 0x7b as *mut u8;

pub const ADC: *mut u16 = 0x78 as *mut u16;

pub const ACSR: *mut u8 = 0x50 as *mut u8;
pub const ACD0: u8 = 1 << 7;

pub const DIDR0: *mut u8 = 0x7e as *mut u8;
pub const ADC0D0: u8 = 1 << 0;

pub const PRR: *mut u8 = 0x64 as *mut u8;
pub const PRADC0: u8 = 1 << 0;
