#![deny(unsafe_code)]
#![no_main]
#![no_std]

//use core::ops::BitXorAssign;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer}
};

const PIXEL: [(usize, usize); 16] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (2,4), (1,4), (3,4), (0,4),
    (4,3), (4,2), (4,1), (4,0), (3,0), (2,0), (1,0)
];


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut second_last = (1, 1);
    let mut last_led = (0, 0);
    // infinite loop; just so we don't leave this stack frame
    loop {
        for current_led in PIXEL.iter(){
            leds[second_last.0][second_last.1] = 0;
            leds[last_led.0][last_led.1] = 1;
            leds[current_led.0][current_led.1] = 1;
            display.show(&mut timer, leds, 500);
            rprintln!("{:?}, {:?}", current_led, last_led);
            second_last = last_led;
            last_led = *current_led;
        }
    }
}