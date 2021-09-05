// `UBRR = f(BAUD)` formula:
// https://web.ics.purdue.edu/~jricha14/Serial_Stuff/UART_information.htm

// About the internal divider:
// https://electronics.stackexchange.com/questions/50833/atmega8-usart-transmitting-wrong-data
// https://electronics.stackexchange.com/questions/269658/serial-output-returns-wrong-ascii

const CPU_FREQUENCY_HZ: u64 = ruduino::config::CPU_FREQUENCY_HZ as u64;
const OSCILLATOR_HZ: u64 = CPU_FREQUENCY_HZ / 8;
const BAUD: u64 = 9600;
const UBRR: u16 = (OSCILLATOR_HZ / 16 / BAUD - 1) as u16;

fn is_unshiftable(symbol: u8) -> bool {
    symbol == b'\0' ||
    symbol == b'\n' ||
    symbol == b'\r'
}

pub fn transmit(symbol: u8) {
    if is_unshiftable(symbol) {
        ruduino::legacy::serial::transmit(symbol);
    } else {
        ruduino::legacy::serial::transmit(symbol + 0x40);
    }
}

pub fn receive() -> u8 {
    let it = ruduino::legacy::serial::receive();

    if is_unshiftable(it) {
        it
    } else {
        it - 0x40
    }
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
    // Idk, why 9 bits instead of 8, maybe it's just
    // supposed to mean 8 bits + 1 stopping bit

    ruduino::legacy::serial::Serial::new(UBRR)
        .character_size(ruduino::legacy::serial::CharacterSize::NineBits)
        .mode(ruduino::legacy::serial::Mode::Asynchronous)
        .parity(ruduino::legacy::serial::Parity::Disabled)
        .stop_bits(ruduino::legacy::serial::StopBits::OneBit)
        .configure();
}

pub unsafe fn get_usart_c_stream() -> *mut u8 {
    fdevopen(uart_putchar, uart_getchar)
}
