use crate::board;
use crate::atmega328p;

use core::ptr::{read_volatile, write_volatile};

// `UBRR = f(BAUD)` formula, see 19.3.1:
// https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

// About the internal divider:
// https://electronics.stackexchange.com/questions/50833/atmega8-usart-transmitting-wrong-data
// https://electronics.stackexchange.com/questions/269658/serial-output-returns-wrong-ascii

pub const OSCILLATOR_HZ: u32 = avr_config::CPU_FREQUENCY_HZ / (board::DIVISION_FACTOR as u32);
pub const BAUD: u32 = 9600;
pub const UBRR: u16 = (OSCILLATOR_HZ / 16 / BAUD - 1) as u16;

#[inline]
pub fn ready_to_transmit() -> bool {
    unsafe {
        (read_volatile(atmega328p::UCSR0A) & atmega328p::UDRE0) != 0
    }
}

#[inline]
pub fn transmit(symbol: u8) {
    while !ready_to_transmit() {}
    unsafe {
        write_volatile(atmega328p::UDR0, symbol);
    }
}

#[inline]
pub fn ready_to_receive() -> bool {
    unsafe {
        (read_volatile(atmega328p::UCSR0A) & atmega328p::RXC0) != 0
    }
}

#[inline]
pub fn receive() -> u8 {
    while !ready_to_receive() {}
    unsafe { read_volatile(atmega328p::UDR0) }
}

#[no_mangle]
pub extern "C" fn uart_putchar(symbol: i8, _: *mut u8) -> i8 {
    transmit(symbol as u8);
    return 0;
}

#[no_mangle]
pub extern "C" fn uart_getchar(_: *mut u8) -> i8 {
    receive() as i8
}

extern "C" {
    // NB: `*mut u8` is a dirty hack for `FILE *`

    // About fdevopen:
    // https://habr.com/en/sandbox/101290/

    fn fdevopen(put: extern "C" fn(i8, *mut u8) -> i8, get: extern "C" fn(*mut u8) -> i8) -> *mut u8;
}

pub fn configure() {
    unsafe {
        // Configure USART as "async transmitter 8N1"
        write_volatile(atmega328p::UCSR0A, 0);
        write_volatile(atmega328p::UCSR0B, atmega328p::TXEN0);
        write_volatile(atmega328p::UCSR0C, atmega328p::UCSZ00 | atmega328p::UCSZ01);
        write_volatile(atmega328p::UBRR0, UBRR);
    }
}

pub unsafe fn get_c_stream() -> *mut u8 {
    fdevopen(uart_putchar, uart_getchar)
}
