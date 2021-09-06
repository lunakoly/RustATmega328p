use crate::board;

// `UBRR = f(BAUD)` formula, see 19.3.1:
// https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

// About the internal divider:
// https://electronics.stackexchange.com/questions/50833/atmega8-usart-transmitting-wrong-data
// https://electronics.stackexchange.com/questions/269658/serial-output-returns-wrong-ascii

pub const OSCILLATOR_HZ: u32 = ruduino::config::CPU_FREQUENCY_HZ / (board::DIVISION_FACTOR as u32);
pub const BAUD: u32 = 9600;
pub const UBRR: u16 = (OSCILLATOR_HZ / 16 / BAUD - 1) as u16;

pub fn transmit(symbol: u8) {
    ruduino::legacy::serial::transmit(symbol);
}

pub fn receive() -> u8 {
    ruduino::legacy::serial::receive()
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
    ruduino::legacy::serial::Serial::new(UBRR)
        .character_size(ruduino::legacy::serial::CharacterSize::EightBits)
        .mode(ruduino::legacy::serial::Mode::Asynchronous)
        .parity(ruduino::legacy::serial::Parity::Disabled)
        .stop_bits(ruduino::legacy::serial::StopBits::OneBit)
        .configure();
}

pub unsafe fn get_c_stream() -> *mut u8 {
    fdevopen(uart_putchar, uart_getchar)
}
