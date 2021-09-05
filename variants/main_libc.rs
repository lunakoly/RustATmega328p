#![feature(lang_items)]
#![no_std]
#![no_main]

extern "C" {
    // See:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html
    // https://avr-rust.github.io/libc/avr_libc/

    fn printf(fmt: *const i8, ...) -> i16;
}

#[no_mangle]
pub extern fn main() {

    let text: &[u8] = &[10, 11, 12, 0];

     unsafe {
        // See:
        // https://electronix.ru/forum/index.php?app=forums&module=forums&controller=topic&id=49491&page=3
        // https://www.avrfreaks.net/forum/what-does-fdevopen-mean
        // https://habr.com/en/sandbox/101290/
        // http://www.count-zero.ru/2015/printf/

        printf("Hello World! (from Rust main())\n\0".as_ptr() as *const i8);
    }

    loop {}
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
