#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();
    let left_button = pins.d5.into_floating_input();
    let right_button = pins.d6.into_floating_input();

    led.set_low();
    loop {
        let left_button_state = left_button.is_low();
        let right_button_state: bool = right_button.is_low();
        match left_button_state {
            true => led.set_high(),
            false => ()
        }
        match right_button_state {
            true => led.set_high(),
            false => ()
        }

    }
}
