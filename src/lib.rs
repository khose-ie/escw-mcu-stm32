#![no_std]

use escw_mcu::Mcu;

pub mod hal;
pub mod peripheral;

mod memory;

pub struct Stm32 {}

impl Mcu for Stm32 {
    #[cfg(feature = "io")]
    type Io = peripheral::io::Io;

    #[cfg(any(
        feature = "uart1",
        feature = "uart2",
        feature = "uart3",
        feature = "uart4",
        feature = "uart5",
        feature = "uart6",
        feature = "uart7",
        feature = "uart8",
    ))]
    type Uart = peripheral::uart::Uart;
}
