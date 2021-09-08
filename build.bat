@ECHO OFF

cargo build -Z build-std=core --target avr-atmega328p.json --release
@REM cargo build -Z build-std=core --target avr-atmega328p.json

avr-strip target\avr-atmega328p\release\simple_test.elf
