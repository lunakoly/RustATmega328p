#![feature(lang_items)]
#![no_std]
#![no_main]

const CPU_FREQUENCY_HZ: u64 = ruduino::config::CPU_FREQUENCY_HZ as u64; // 8_000_000
const OSCILLATOR_HZ: u64 = CPU_FREQUENCY_HZ / 8; // there's an internal div8
const BAUD: u64 = 9600;
const UBRR: u16 = (OSCILLATOR_HZ / 16 / BAUD - 1) as u16;

extern "C" {
    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;
    fn fdevopen(put: extern "C" fn(i8, *mut u8) -> i8, get: extern "C" fn(*mut u8) -> i8) -> *mut u8;
}

fn is_unshiftable(symbol: u8) -> bool {
    symbol == b'\0' ||
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
    ruduino::legacy::serial::Serial::new(UBRR)
        .character_size(ruduino::legacy::serial::CharacterSize::NineBits) // wtf 9, not 8
        .mode(ruduino::legacy::serial::Mode::Asynchronous)
        .parity(ruduino::legacy::serial::Parity::Disabled)
        .stop_bits(ruduino::legacy::serial::StopBits::OneBit)
        .configure();

    for &b in b"[Data]\r\n\0" {
        uart_putchar(b as i8, 0 as *mut u8);
    }

     unsafe {
        let stdout = fdevopen(uart_putchar, uart_getchar);
        fprintf(stdout, "Hi, I'm %d!\0".as_ptr() as *const i8, 21);
    }

    loop {}
}
