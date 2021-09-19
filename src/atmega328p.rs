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
