@ECHO OFF

@REM Ensure time delays are consistent with a 8MHz microcontroller.
set AVR_CPU_FREQUENCY_HZ=8000000

cargo build -Z build-std=core --target avr-atmega328p.json --release
@REM cargo build -Z build-std=core --target avr-atmega328p.json
