pub const DIVISION_POWER: u8 = 0;
pub const DIVISION_FACTOR: u8 = 1 << DIVISION_POWER;

use crate::usart;
use crate::atmega328p;

use core::ptr::write_volatile;

pub fn configure() {
    unsafe {
        // About the clock prescaler, see 8.11:
        // https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

        // Configure the clock prescaler (=3 by default).
        write_volatile(atmega328p::CLKPR, atmega328p::CLKPCE);
        write_volatile(atmega328p::CLKPR, DIVISION_POWER);

        // Seems like CKDIV8 is already disabled.
        // OSCCAL is fine as well.
    }

    usart::configure();
}
