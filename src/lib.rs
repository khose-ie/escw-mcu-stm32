#![no_std]

use escw_mcu::Mcu;

pub mod hal;
pub mod peripheral;

mod memory;

pub struct Stm32 {}

impl Mcu for Stm32
{
    type Io = peripheral::io::Io;
    type Uart = peripheral::uart::Uart;

    #[cfg(any(feature = "i2c1", feature = "i2c2", feature = "i2c3"))]
    type I2cMaster = peripheral::i2c::I2cMaster;

    #[cfg(any(feature = "i2c1", feature = "i2c2", feature = "i2c3"))]
    type I2cSlave = peripheral::i2c::I2cSlave;

    #[cfg(any(feature = "spi1", feature = "spi2", feature = "spi3", feature = "spi4", feature = "spi5", feature = "spi6",))]
    type Spi = peripheral::spi::Spi;

    #[cfg(feature = "flash")]
    type Flash = peripheral::flash::Flash;
}
