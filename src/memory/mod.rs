#[allow(dead_code)]
#[cfg(feature = "stm32f407xx")]
mod stm32f407xx;

#[allow(unused_imports)]
#[cfg(feature = "stm32f407xx")]
pub use stm32f407xx::*;
