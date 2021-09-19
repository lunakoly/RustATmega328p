// About no_std:
// https://github.com/rust-embedded/cortex-m-quickstart/issues/58

#![no_std]
#![no_main]
#![feature(lang_items)]

mod atmega328p;
mod conversion;
mod board;
mod usart;
mod devices;
mod run;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
fn main() {
    board::configure();
    run::run();
}
