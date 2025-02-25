use escw_mcu::common::Result;
use escw_mcu::peripheral::wdt::WdtDevice;

use crate::hal::wwdg::*;

pub struct Wwdg {}

impl Wwdg
{
    pub fn new() -> Self
    {
        Wwdg {}
    }
}

impl WdtDevice for Wwdg
{
    fn refresh(&self) -> Result<()>
    {
        unsafe { HAL_WWDG_Refresh(&hwwdg).into() }
    }
}
