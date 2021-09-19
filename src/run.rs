use crate::devices;

pub fn run() {
    let mut display = devices::lcd_display::configure();

    display.position(0, 0);
    display.print("Microcontrollers21");

    display.position(0, 1);
    display.print("Nikolay Lunyak");
}
