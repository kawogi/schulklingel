#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod lautsprecher;
mod timer;

use arduino_hal::delay_ms;
use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwriteln;

use crate::lautsprecher::Lautsprecher;
use crate::timer::date_time;
use crate::timer::{init_clock, millis};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Die Ausgabe von Statusinformationen erfolgt über diese serielle Schnittstelle.
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Start").unwrap_infallible();

    // Pin, an welchem die im Arduino eingebaute LED angeschlossen ist
    let mut led = pins.d13.into_output();

    // Pin, an welchem der `OUT`-Pin des DCF77-Modul angeschlossen ist
    let dcf77_pin = pins.d2.into_floating_input().downgrade().forget_imode();

    // Der Lautsprecher wird an Pin D6 angeschlossen.
    let mut lautsprecher = Lautsprecher::neu(pins.d6.into_output());
    pins.d4.into_output().set_low(); // extra-GND

    led.set_high();
    lautsprecher.spiele_start();
    led.set_low();

    init_clock(dp.TC0, dcf77_pin);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    loop {
        let time = millis();
        let date_time = date_time();
        let year = date_time.get_year().unwrap_or(0);
        let month = date_time.get_month().unwrap_or(0);
        let day = date_time.get_day().unwrap_or(0);
        let hour = date_time.get_hour().unwrap_or(0);
        let minute = date_time.get_minute().unwrap_or(0);
        let second = date_time.get_leap_second().unwrap_or(0);
        uwriteln!(
            serial,
            "{}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second
        )
        .unwrap_infallible();

        // if minute > 0 {
        //     lautsprecher.spiele_tonfolge();
        // }

        uwriteln!(&mut serial, "{} ms", time).unwrap_infallible();
        delay_ms(1000);
    }
}
