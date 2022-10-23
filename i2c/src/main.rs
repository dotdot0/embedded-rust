#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![allow(unused)]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::hal::{prelude::*, Timer};

#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
    Board
};

use lsm303agr::{
    AccelOutputDataRate, Lsm303agr, MagOutputDataRate
};

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[entry]
fn main() -> !{
  rtt_init_print!();
  let board = microbit::board::Board::take().unwrap();

  let mut timer = Timer::new(board.TIMER1);

  #[cfg(feature = "v2")]
  let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100)};

  let mut acc:[u8; 1] = [0];
  let mut mag:[u8; 1] = [0];
  i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc);
  i2c.write_read(MAGNETOMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut mag);

  let mut sensor = Lsm303agr::new_with_i2c(i2c);
  sensor.init().unwrap();
  sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
  sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
  let mut sensor = sensor.into_mag_continuous().ok().unwrap();

  loop {
      if sensor.accel_status().unwrap().xyz_new_data && sensor.mag_status().unwrap().xyz_new_data{
        let data = sensor.accel_data().unwrap();
        let mag_data = sensor.mag_data().unwrap();
        rprintln!("Accelerometer: x: {} y: {} z: {}", data.x, data.y, data.z);
        rprintln!("Magnetometer: x: {} y: {} z: {}", mag_data.x, mag_data.y, mag_data.z);
      }

      timer.delay_ms(1000 as u16)
      
  }
}