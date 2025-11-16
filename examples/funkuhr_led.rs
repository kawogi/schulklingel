//! Leitet den `OUT`-Pin des DCF77-Moduls an die LED des Arduinos weiter.
//!
//! Jeder Sekunde wird ein Bit übertragen:
//!
//! - Kurzer Puls (100 ms) = 0
//! - Langer Puls (200 ms) = 1
//! - Pause (1 s) = Beginn einer neuen Minute
//!
//! Weitere Infos: <https://de.wikipedia.org/wiki/DCF77#Amplitudenumtastung>

#![warn(clippy::all, clippy::pedantic)]
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Pin, an welchem die im Arduino eingebaute LED angeschlossen ist
    let mut led = pins.d13.into_output();

    // Pin, an welchem der `OUT`-Pin des DCF77-Modul angeschlossen ist
    let dcf77 = pins.d2.into_floating_input();

    loop {
        // Übertrage den Zustand des `OUT`-Pins direkt an die LED
        if dcf77.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
