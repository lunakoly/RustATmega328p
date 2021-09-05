#![feature(lang_items)]
#![no_std]
#![no_main]

// See:
// https://electronics.stackexchange.com/questions/50833/atmega8-usart-transmitting-wrong-data
// https://electronics.stackexchange.com/questions/269658/serial-output-returns-wrong-ascii

// Alernative Approach:
// https://stackoverflow.com/questions/60626086/how-do-i-configure-a-uart-in-rust-using-the-embedded-hal

const CPU_FREQUENCY_HZ: u64 = ruduino::config::CPU_FREQUENCY_HZ as u64; // 8_000_000
const OSCILLATOR_HZ: u64 = CPU_FREQUENCY_HZ / 8; // there's an internal div8
const BAUD: u64 = 9600;
const UBRR: u16 = (OSCILLATOR_HZ / 16 / BAUD - 1) as u16;

// use ruduino::{Register};
// use ruduino::cores::atmega328p;

extern "C" {
    // See:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html
    // https://avr-rust.github.io/libc/avr_libc/

    // fn printf(fmt: *const i8, ...) -> i16;

    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;

    fn fdevopen(put: extern "C" fn(i8, *mut u8) -> i8, get: extern "C" fn(*mut u8) -> i8) -> *mut u8;

    // static mut stdout: *mut u8;
}

fn is_unshiftable(symbol: u8) -> bool {
    // It should've been here to prevent
    // a space, but this leads to access violation
    // error in Proteus
    // (probably there's some string unclosed).
    // symbol == b'\0' ||
    symbol == b'\n' ||
    symbol == b'\r'
}

#[no_mangle]
pub extern "C" fn uart_putchar(symbol: i8, _: *mut u8) -> i8 {
    if is_unshiftable(symbol as u8) {
        ruduino::legacy::serial::transmit(symbol as u8);
    } else {
        ruduino::legacy::serial::transmit(symbol as u8 + 0x40);
    }

    return 0;
}

#[no_mangle]
pub extern "C" fn uart_getchar(_: *mut u8) -> i8 {
    let it = ruduino::legacy::serial::receive() as i8;

    if is_unshiftable(it as u8) {
        it
    } else {
        it - 0x40
    }
}

#[no_mangle]
fn main() {
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

    ruduino::legacy::serial::Serial::new(UBRR)
        // .character_size(ruduino::legacy::serial::CharacterSize::EightBits)
        .character_size(ruduino::legacy::serial::CharacterSize::NineBits)
        .mode(ruduino::legacy::serial::Mode::Asynchronous)
        .parity(ruduino::legacy::serial::Parity::Disabled)
        .stop_bits(ruduino::legacy::serial::StopBits::TwoBits)
        .configure();

    // atmega328p::UBRR0::write(0x33u16);
    // atmega328p::UCSR0A::write(32);
    // atmega328p::UCSR0B::write(8);
    // atmega328p::UCSR0C::write(6);

    for &b in b"[Data]\r\n\0" {
        uart_putchar(b as i8, 0 as *mut u8);
    }

     unsafe {
        // See:
        // https://electronix.ru/forum/index.php?app=forums&module=forums&controller=topic&id=49491&page=3
        // https://www.avrfreaks.net/forum/what-does-fdevopen-mean
        // https://habr.com/en/sandbox/101290/
        // http://www.count-zero.ru/2015/printf/

        let stdout = fdevopen(uart_putchar, uart_getchar);

        // fprintf(stdout, "Hello World! (from Rust main())\n\0".as_ptr() as *const i8);
        fprintf(stdout, "Hi, I'm %d!\0".as_ptr() as *const i8, 21);
    }

    loop {}
}
