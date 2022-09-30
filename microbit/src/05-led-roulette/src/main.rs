#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52840_dk_bsp::hal::{prelude::*, timer::Timer};
use nrf52840_dk_bsp::Board;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut nrf52 = Board::take().unwrap();

    let mut timer = Timer::new(nrf52.TIMER0);

    loop {
        nrf52.leds.led_2.disable();
        rprintln!("dark!");
        timer.delay_ms(1_000_u16);

        nrf52.leds.led_2.enable();
        rprintln!("light!");
        timer.delay_ms(1_000_u16);
    }
}
