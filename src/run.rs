use crate::usart;

use crate::conversion::{CPointerCompatible, C32Compatible};
use crate::{c_line, c_string};

extern "C" {
    // Full avr-libc reference for looking up prototypes:
    // https://www.nongnu.org/avr-libc/user-manual/group__avr__stdio.html

    fn fprintf(stream: *mut u8, fmt: *const i8, ...) -> i16;
}

pub fn run() {
    unsafe {
        let stdout = usart::get_c_stream();
        fprintf(stdout, c_line!("Well, 3 + 2 = %lu, but 3 * 2 = %lu."), 3 + 2, 3 * 2);
        fprintf(stdout, c_line!("And pi = %f"), 3.14159265.to_c());
    }
}
