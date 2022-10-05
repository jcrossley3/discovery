#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = nrf52840_dk_bsp::Board::take().unwrap();

    let mut serial = board.cdc;

    // write a byte array
    let mut buf: [u8; 1] = [b'X'; 1];
    serial.write(&buf).unwrap();

    // write a string since the UART impls Write
    let s = "The quick brown fox jumps over the lazy dog";
    write!(serial, "\r\n{}\r\n", s).unwrap();

    loop {
        serial.read(&mut buf).unwrap();
        rprintln!("{}", buf[0] as char);
        serial.write(&buf).unwrap();
    }
}
