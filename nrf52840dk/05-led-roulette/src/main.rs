#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52840_dk_bsp::{
    hal::{pac::TIMER0, prelude::*, timer::Timer},
    Board, Led,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut nrf52 = Board::take().unwrap();
    let mut timer = Timer::new(nrf52.TIMER0);
    rprintln!("blinking");
    loop {
        blink(&mut nrf52.leds.led_1, &mut timer);
        blink(&mut nrf52.leds.led_2, &mut timer);
        blink(&mut nrf52.leds.led_4, &mut timer);
        blink(&mut nrf52.leds.led_3, &mut timer);
    }
}

fn blink(led: &mut Led, t: &mut Timer<TIMER0>) {
    led.enable();
    t.delay_ms(100_u16);
    led.disable();
}
