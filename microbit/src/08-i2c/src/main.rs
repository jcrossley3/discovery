#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52840_dk_bsp::{
    hal::{prelude::*, temp::Temp, timer::Timer},
    Board,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut sensor = Temp::new(board.TEMP);
    let mut timer = Timer::new(board.TIMER0);

    loop {
        let temp: i32 = sensor.measure().to_num();
        rprintln!("processor temp is {}°C", temp);
        timer.delay_ms(5000_u16);
    }
}
