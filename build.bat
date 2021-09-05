@ECHO OFF

@REM cls

@REM set VARIANT=main_ruduino_auto_abc
@REM set VARIANT=main_ruduino_manual_abc
@REM set VARIANT=start_ruduino_auto_abc
set VARIANT=many_things

set VARIANT_FILE=variants\%VARIANT%.rs
set MAIN_FILE=src\main.rs
@REM copy %VARIANT_FILE% %MAIN_FILE%

@REM Ensure time delays are consistent with a 8MHz microcontroller.
set AVR_CPU_FREQUENCY_HZ=8000000

@REM cargo build -Z build-std=core --target avr-unknown-gnu-atmega328 --release --verbose
@REM cargo build -Z build-std=core --target avr-unknown-gnu-atmega328
cargo build -Z build-std=core --target avr-atmega328p.json --release
@REM cargo build -Z build-std=core --target avr-atmega328p.json

@REM -nostartfiles
@REM -nostdlib
