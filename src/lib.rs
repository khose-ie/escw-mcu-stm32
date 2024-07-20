#![no_std]

use escw_mcu::Mcu;

pub mod hal;
pub mod peripheral;

mod memory;

pub struct Stm32 {}

impl Mcu for Stm32 {
    #[cfg(feature = "io")]
    type Io = peripheral::io::Io;
}
