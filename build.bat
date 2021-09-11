@ECHO OFF

set AVR_CPU_FREQUENCY_HZ=8000000

cargo build -Z build-std=core --target avr-atmega328p.json --release
@REM cargo build -Z build-std=core --target avr-atmega328p.json

avr-strip target\avr-atmega328p\release\simple_test.elf
