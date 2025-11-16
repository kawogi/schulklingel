//! Dieses Programm spielt die Schulglocken-Tonfolge auf dem Lautsprecher ab.

#![warn(clippy::all, clippy::pedantic)]
#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;
use ufmt::uwriteln;

use crate::lautsprecher::Lautsprecher;

#[path = "../src/lautsprecher.rs"]
mod lautsprecher;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Die Ausgabe von Statusinformationen erfolgt über diese serielle Schnittstelle.
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Der Lautsprecher wird an Pin D6 angeschlossen.
    let mut lautsprecher = Lautsprecher::neu(pins.d6.into_output());

    loop {
        uwriteln!(serial, "Spiele Tonfolge").ok();
        lautsprecher.spiele_tonfolge();
        delay_ms(2000);
    }
}
