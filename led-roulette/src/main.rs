#![allow(unused, unused_comparisons)]
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
    hal::{prelude::*, Timer, gpio}
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    // let button_a = board.buttons.;

    let mut ch_pos: [(usize, usize); 2] = [(4, 0), (3, 0)];

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds_a = [
        [0, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 1, 0],
        [1, 0, 0, 1, 0],
    ];
    let mut leds_b = [
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
    ];

    let mut character =[ 
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ];
    // infinite loop; just so we don't leave this stack frame
    loop {
        display.show(&mut timer, character, 200);
        if let Ok(true) = board.buttons.button_b.is_low(){
            if ch_pos[0].1 <= 4 && ch_pos[1].1 <= 4{
                character[ch_pos[0].0][ch_pos[0].1] = 0;
                character[ch_pos[1].0][ch_pos[1].1] = 0;
                ch_pos[0].1 += 1;
                ch_pos[1].1 += 1;
                character[ch_pos[0].0][ch_pos[0].1] = 1;
                character[ch_pos[1].0][ch_pos[1].1] = 1;
                display.show(&mut timer, character, 200);
                rprintln!("Pos: {:?}", ch_pos)
            }
            // else {
            //     ch_pos.1 = 0;
            //     display.show(&mut timer, character, 200)
            // }
        }
        else if let Ok(true) = board.buttons.button_a.is_low() {
            //display.show(&mut timer, character, 200);
            if ch_pos[0].1 >= 0 && ch_pos[1].0 >= 0{
                character[ch_pos[0].0][ch_pos[0].1] = 0;
                character[ch_pos[1].0][ch_pos[1].1] = 0;
                ch_pos[0].1 -= 1;
                ch_pos[1].1 -= 1;
                character[ch_pos[0].0][ch_pos[1].1] = 1;
                character[ch_pos[1].0][ch_pos[1].1] = 1;
                display.show(&mut timer, character, 200);
                rprintln!("Pos: {:?}", ch_pos)
            }
        }
        //timer.delay_ms(200u16)
    }
}