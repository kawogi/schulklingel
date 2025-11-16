#![warn(clippy::all, clippy::pedantic)]
#![no_std]
#![no_main]
#![expect(unused, reason = "// TODO")]

mod lautsprecher;

use core::num::Wrapping;

use arduino_hal::{delay_ms, delay_us};
use dcf77_utils::DCF77Utils;
use panic_halt as _;
use radio_datetime_utils::RadioDateTimeUtils;
use ufmt::uwriteln;

use crate::lautsprecher::Lautsprecher;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Die Ausgabe von Statusinformationen erfolgt über diese serielle Schnittstelle.
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Pin, an welchem die im Arduino eingebaute LED angeschlossen ist
    let mut led = pins.d13.into_output();

    // Pin, an welchem der `OUT`-Pin des DCF77-Modul angeschlossen ist
    let dcf77_pin = pins.d2.into_floating_input();

    // Der Lautsprecher wird an Pin D6 angeschlossen.
    let mut lautsprecher = Lautsprecher::neu(pins.d6.into_output());

    let mut dcf77 = DCF77Utils::new();

    let mut prev_state = dcf77_pin.is_high();

    let mut micros = Wrapping::<u32>(0);
    let mut minute = Wrapping::<u32>(0);
    loop {
        let state = dcf77_pin.is_high();
        if state != prev_state {
            dcf77.handle_new_edge(state, micros.0);

            if state {
                dcf77.increase_second();
            }

            if dcf77.is_new_second() {
                uwriteln!(serial, "{}", dcf77.get_second()).ok();
            }

            if state && dcf77.is_new_minute() {
                dcf77.decode_time(true, true);
                {
                    let date_time: RadioDateTimeUtils = dcf77.get_radio_datetime();
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
                    .ok();
                }
                minute.0 = 0;
            }

            if state {
                led.set_high();
            } else {
                led.set_low();
            }

            prev_state = state;
        }
        if minute.0 >= 59_000_000 {
            uwriteln!(serial, "full minute");
            dcf77.increase_second();
            minute -= 59_000_000;
        }

        micros += 10_000;
        minute += 10_000;
        delay_us(10_000);
    }

    // loop {
    //     uwriteln!(serial, "Spiele Tonfolge").ok();
    //     lautsprecher.spiele_tonfolge();
    //     delay_ms(2000);
    // }
}
