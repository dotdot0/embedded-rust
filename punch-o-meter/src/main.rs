#![no_main]
#![no_std]
#![allow(unused)]

mod dir;
use dir::{Direction, dir};
use heapless::Vec;

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::{hal::{prelude::*, Timer}, display::blocking::Display};

#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
    Board
};

use lsm303agr::{
    AccelOutputDataRate, Lsm303agr
};

#[entry]
fn main() -> !{
  rtt_init_print!();
  let board = microbit::board::Board::take().unwrap();

  let mut timer = Timer::new(board.TIMER1);

  let mut display = Display::new(board.display_pins);

  #[cfg(feature = "v2")]
  let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100)};

  let mut sensor = Lsm303agr::new_with_i2c(i2c);
  sensor.init().unwrap();
  sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

  let mut ch_pos: (usize, usize) = (4, 0);
  
  let mut character =[ 
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ]; 

  loop {
      display.show(&mut timer, character, 200);
      if sensor.accel_status().unwrap().xyz_new_data{
        let data = sensor.accel_data().unwrap();
        if data.x > 400 {
          character[ch_pos.0][ch_pos.1] = 0;
          ch_pos.1 += 1;
          character[ch_pos.0][ch_pos.1] = 1;
          display.show(&mut timer, character, 200);
          rprintln!("Pos: {:?}", ch_pos);
                // if ch_pos[0].1 > 4 && ch_pos[1].1 > 4{
                //   ch_pos[0].1 = 0;
                //   ch_pos[1].1 = 0;
                // }
        }
        else if data.x < -400{
          character[ch_pos.0][ch_pos.1] = 0;
          ch_pos.1 -= 1;
          character[ch_pos.0][ch_pos.1] = 1;
          display.show(&mut timer, character, 200);
          rprintln!("Pos: {:?}", ch_pos);
                // if ch_pos[0].1 < 0 && ch_pos[1].1 < 0{
                //   ch_pos[0].1 = 0;
                //   ch_pos[1].1 = 0;
                // }
        }
        else if data.y < -200{
          character[ch_pos.0][ch_pos.1] = 0;
          ch_pos.0 += 1;
          character[ch_pos.0][ch_pos.1] = 1;
          display.show(&mut timer, character, 200);
          rprintln!("Pos: {:?}", ch_pos);
        }
        else if data.y > 200{
          character[ch_pos.0][ch_pos.1] = 0;
          ch_pos.0 -= 1;
          character[ch_pos.0][ch_pos.1] = 1;
          display.show(&mut timer, character, 200);
          rprintln!("Pos: {:?}", ch_pos)
        } 
      }

      //timer.delay_ms(1000 as u16)
      
  }
}