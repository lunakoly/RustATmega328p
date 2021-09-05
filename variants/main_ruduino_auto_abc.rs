#![no_std]
#![no_main]

const BAUD: u32 = 9600;
const UBRR: u16 = (ruduino::config::CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

#[no_mangle]
pub extern fn main() {
    ruduino::legacy::serial::Serial::new(UBRR)
        .character_size(ruduino::legacy::serial::CharacterSize::EightBits)
        .mode(ruduino::legacy::serial::Mode::Asynchronous)
        .parity(ruduino::legacy::serial::Parity::Disabled)
        .stop_bits(ruduino::legacy::serial::StopBits::OneBit)
        .configure();

    let text: &[u8] = &[10, 11, 12, 0];

    for &b in text {
        ruduino::legacy::serial::transmit(b);
    }

    loop {}
}
