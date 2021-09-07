# Small Embedded Rust How-To

## Compiler Version

Building for AVR requires a `nightly` toolchain.
If the most recent `nightly` doesn't work, you [can use](https://dev.to/mikla/comment/1d386):

```bash
rustup override set nightly-2021-01-07
```

Note that if something doesn't work with either of the `debug` or the `release` builds,
try switching to the other one. Sometimes it suddenly *starts to work*.

## AVR-GCC Toolchain

You can get it at this [microchip.com page](https://www.microchip.com/en-us/development-tools-tools-and-software/gcc-compilers-avr-and-arm).

`avr-libc` is included, don't worry. You'll find it in the `avr` folder.

## `ruduino`

This project uses the `ruduino` crate to configure USART and access other ATmega328p registers if needed.

Without `ruduino` [it's vital to add manual declarations](https://users.rust-lang.org/t/solved-hello-world-no-std-build-problem/23122/4) for the `panic_handler` and the `eh_personality`:

```rust
#![feature(lang_items)]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
```

## Program Entry Point

I found the following approach is acceptable:

```rust
#![no_std]
#![no_main]

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    ...
}
```

`main()` doesn't have to be `pub extern`, and the parameters and the return type may be ommitted.

## General Resources
* [General Rust-AVR repos](https://github.com/orgs/avr-rust/repositories)
* [Blink repo with sample code for ATmega328p `[ruduino]`](https://github.com/avr-rust/blink)
* [Rust-AVR Book: pointers to registers](https://book.avr-rust.com/005.4-choosing-an-io-library.html)
* [UART example `[ruduino]`](https://github.com/avr-rust/ruduino/blob/master/examples/uart.rs)

## Nuances
### Proteus

Only the `release` build can be used for Proteus simulation right now. For some reason, `debug` doesn't even start.

### USART & Frequency Configuration

The base clock frequency is set to 8MHz. ATmega328p has a built-in clock prescaler `CLKPR` that allows
to further divide the frequency by a power of 2. By default it stores 3, so we manually set
it to 0 in the `board::configure()` function.

Apart from the prescaler, there is a separate build-in "by 8"-divider: `CKDIV8`. It turns out it's disabled
by default.

Funny fact: if we leave the `CLKPR` untouched, configure USART for _9 bits per character_, and
*shift some characters forward by 0x40*, we'll still get the right message! Don't ask, how I found it out...

### Linking Agains `stdout`

I didn't find a way to link against:

```rust
extern "C" {
    static mut stdout: *mut u8;
}
```

Says, "unresolved reference", although there *must* be something functions like `printf()` rely on.

## Problems

Build fails? Maybe this will help:

* https://github.com/avr-rust/blink/issues/25
* https://github.com/avr-rust/rust-legacy-fork/issues/149
