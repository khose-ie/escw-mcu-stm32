#[cfg(feature = "io")]
pub mod io;

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
pub mod uart;

#[cfg(any(feature = "i2c1", feature = "i2c2", feature = "i2c3"))]
pub mod i2c;

#[cfg(any(
    feature = "spi1",
    feature = "spi2",
    feature = "spi3",
    feature = "spi4",
    feature = "spi5",
    feature = "spi6",
))]
pub mod spi;

#[cfg(feature = "iwdg")]
pub mod iwdg;

#[cfg(feature = "wwdg")]
pub mod wwdg;
