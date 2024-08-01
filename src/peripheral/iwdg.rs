use escw_mcu::common::Result;
use escw_mcu::peripheral::wdt::WdtDevice;

use crate::hal::iwdg::*;

pub struct Iwdg {}

impl Iwdg {
    pub fn new() -> Self {
        Iwdg {}
    }
}

impl WdtDevice for Iwdg {
    fn refresh(&self) -> Result<()> {
        unsafe { HAL_IWDG_Refresh(&hiwdg).into() }
    }
}
