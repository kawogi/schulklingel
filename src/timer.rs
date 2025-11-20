/*!
 * A basic implementation of the `millis()` function from Arduino:
 *
 *     https://www.arduino.cc/reference/en/language/functions/time/millis/
 *
 * Uses timer TC0 and one of its interrupts to update a global millisecond
 * counter.  A walkthrough of this code is available here:
 *
 *     https://blog.rahix.de/005-avr-hal-millis/
 */

use core::{
    cell::RefCell,
    num::{Saturating, Wrapping},
};

use arduino_hal::{
    hal::port::Dynamic,
    port::{mode::Input, Pin},
};
use avr_device::interrupt::Mutex;
use dcf77_utils::DCF77Utils;
use radio_datetime_utils::RadioDateTimeUtils;

// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u32 = 125;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;

static CLOCK_STATE: Mutex<RefCell<Option<ClockState>>> = Mutex::new(RefCell::new(None));

struct ClockState {
    millis: Wrapping<u32>,
    minute: Saturating<u16>,
    dcf77: DCF77Utils,
    dcf77_pin: Pin<Input, Dynamic>,
    dcf77_pin_state: bool,
    date_time: RadioDateTimeUtils,
}

impl ClockState {
    fn new(dcf77_pin: Pin<Input, Dynamic>) -> Self {
        let dcf77_pin_state = dcf77_pin.is_high();

        let dcf77 = DCF77Utils::new();
        let date_time = dcf77.get_radio_datetime();
        Self {
            millis: Wrapping(0),
            minute: Saturating(0),
            dcf77,
            dcf77_pin,
            dcf77_pin_state,
            date_time,
        }
    }
}

pub(crate) fn init_clock(tc0: arduino_hal::pac::TC0, dcf77_pin: Pin<Input, Dynamic>) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(TIMER_COUNTS as u8));
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    let state = ClockState::new(dcf77_pin);

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        CLOCK_STATE.borrow(cs).borrow_mut().replace(state);
    });
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    timer0_compa();
}

#[inline(always)]
fn timer0_compa() {
    avr_device::interrupt::free(|cs| {
        let mut state_opt = CLOCK_STATE.borrow(cs).borrow_mut();
        let Some(ClockState {
            millis,
            minute,
            dcf77,
            dcf77_pin,
            dcf77_pin_state,
            date_time,
        }) = state_opt.as_mut()
        else {
            return;
        };

        *minute += MILLIS_INCREMENT as u16;
        *millis += MILLIS_INCREMENT;

        // if minute.0 > 60_000 {
        //     *minute -= 60_000;
        // }

        let new_pin_state = dcf77_pin.is_high();
        if new_pin_state != *dcf77_pin_state {
            *dcf77_pin_state = new_pin_state;
            dcf77.handle_new_edge(new_pin_state, millis.0 * 1_000);

            if new_pin_state {
                dcf77.increase_second();
            }

            if dcf77.is_new_second() {
                // uwriteln!(serial, "{}", dcf77.get_second()).ok();
            }

            if new_pin_state && dcf77.is_new_minute() {
                dcf77.decode_time(true, true);
                *date_time = dcf77.get_radio_datetime();
                minute.0 = 0;
            }
        }

        if minute.0 > 59_000 {
            dcf77.increase_second();
            *minute -= 59_000;
        }
    })
}

pub(crate) fn millis() -> u32 {
    avr_device::interrupt::free(|cs| {
        CLOCK_STATE
            .borrow(cs)
            .borrow()
            .as_ref()
            .map_or(0, |state| state.millis.0)
    })
}

pub(crate) fn date_time() -> RadioDateTimeUtils {
    avr_device::interrupt::free(|cs| CLOCK_STATE.borrow(cs).borrow().as_ref().unwrap().date_time)
}
