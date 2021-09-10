pub trait CPointerCompatible {
    fn to_c(&self) -> *const i8;
}

impl CPointerCompatible for &str {
    fn to_c(&self) -> *const i8 {
        self.as_ptr() as *const i8
    }
}

#[macro_export]
macro_rules! c_string {
    ( $it:expr ) => {
        concat!($it, "\0").to_c()
    };
}

#[macro_export]
macro_rules! c_line {
    ( $it:literal ) => {
        c_string!(concat!($it, "\r\n"))
    };
}

pub trait C32Compatible {
    fn to_c(self) -> i32;
}

impl C32Compatible for f32 {
    fn to_c(self) -> i32 {
        let parts = self.to_le_bytes();
        let mut result = 0;

        result += (parts[0] as i32) << 0;
        result += (parts[1] as i32) << 8;
        result += (parts[2] as i32) << 16;
        result += (parts[3] as i32) << 24;

        result
    }
}
