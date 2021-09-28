#![allow(dead_code)]

use crate::atmega328p;

use core::ptr::{write_volatile, read_volatile};

// Voltage reference: AVCC pin
const ADC_VREF_TYPE: u8 = atmega328p::REFS0;

#[inline]
pub fn ready_to_read() -> bool {
    unsafe {
        (read_volatile(atmega328p::ADCSRA) & atmega328p::ADIF0) != 0
    }
}

pub fn read(channel: u8) -> u16 {
    unsafe {
        write_volatile(atmega328p::ADMUX, channel | ADC_VREF_TYPE);

        // For the stabilization of teh ADC input voltage
        avr_delay::delay_us(10);

        // Start the conversion
        write_volatile(
            atmega328p::ADCSRA,
            read_volatile(atmega328p::ADCSRA) | atmega328p::ADSC0
        );

        // Wait for the completion
        while !ready_to_read() {}

        write_volatile(
            atmega328p::ADCSRA,
            read_volatile(atmega328p::ADCSRA) | atmega328p::ADIF0
        );

        read_volatile(atmega328p::ADC)
    }
}

pub fn configure() {
    unsafe {
        // ADC initialization
        // ADC Clock frequency: 125,000 kHz
        // ADC Voltage Reference: AVCC pin
        // ADC Auto Trigger Source: ADC Stopped
        // Digital input buffers on ADC0: On, ADC1: On, ADC2: On, ADC3: On
        // ADC4: On, ADC5: On
        write_volatile(atmega328p::DIDR0, 0);
        write_volatile(atmega328p::ADMUX, ADC_VREF_TYPE);
        write_volatile(atmega328p::ADCSRA, atmega328p::ADEN0 | atmega328p::ADPS1 | atmega328p::ADPS2);
        write_volatile(atmega328p::ADCSRB, 0);
    }
}
