#![no_std]
#![no_main]

const BAUD: u32 = 9600;
const UBRR: u16 = (ruduino::config::CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

use ruduino::{Register};
use ruduino::cores::atmega328p;

#[no_mangle]
pub extern fn main() {
    atmega328p::UCSR0A::write(0);

    atmega328p::UCSR0B::write(
        atmega328p::UCSR0B::RXEN0 |
        atmega328p::UCSR0B::TXEN0
    );

    atmega328p::UCSR0C::write(
        atmega328p::UCSR0C::UCSZ01 |
        atmega328p::UCSR0C::UCSZ00
    );

    atmega328p::UBRR0::write(UBRR);

    // atmega328p::UCSR0A::write(0x32);
    // // atmega328p::UCSR0A::write(0);
    // atmega328p::UCSR0B::write(0x8);
    // atmega328p::UCSR0C::write(0x6);
    // atmega328p::UBRR0::write(UBRR);

    // atmega328p::UCSR0A::write(atmega328p::UCSR0A::read() | atmega328p::UCSR0A::U2X0);
    // atmega328p::UCSR0B::write(atmega328p::UCSR0B::TXEN0);
    // atmega328p::UCSR0C::write(atmega328p::UCSR0C::UCSZ01 | atmega328p::UCSR0C::UCSZ00);
    // atmega328p::UBRR0::write(UBRR);

    // atmega328p::CLKPR::write(0);
    // atmega328p::OSCCAL::write(0x9Eu8);
    // atmega328p::OSCCAL::write(0xC2u8);
    // atmega328p::OSCCAL::write(128u8);

    // atmega328p::UBRR0::write(0x33u16);
    // atmega328p::UCSR0A::write(32);
    // atmega328p::UCSR0B::write(8);
    // atmega328p::UCSR0C::write(6);

    let text: &[u8] = &[10, 11, 12, 0];

    for &b in text {
        ruduino::legacy::serial::transmit(b);
    }

    loop {}
}
