#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use heapless::Vec;


#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    }; 

    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();
        let byte = nb::block!(serial.read()).unwrap();
        if buffer.push(byte).is_err(){
            for b in b"error: buffer full\r\n".iter(){
                nb::block!(serial.write(*b)).unwrap();
            }
        }
        
        if byte==13{
            for byte in buffer.iter().rev().chain(&[b'\n', b'\r']){
                nb::block!(serial.write(*byte)).unwrap();
            }
           
        }
        nb::block!(serial.flush()).unwrap()

    }
}