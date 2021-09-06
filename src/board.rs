pub const DIVISION_POWER: u8 = 0;
pub const DIVISION_FACTOR: u8 = 1 << DIVISION_POWER;

use ruduino::{Register};
use ruduino::cores::atmega328p;

use crate::usart;

pub fn configure() {
    // About the clock prescaler, see 8.11:
    // https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

    // Configure the clock prescaler (=3 by default).
    atmega328p::CLKPR::write(atmega328p::CLKPR::CLKPCE);
    atmega328p::CLKPR::write(DIVISION_POWER);

    // Seems like CKDIV8 is already disabled.
    // OSCCAL is fine as well.

    usart::configure();
}
