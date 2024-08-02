use escw_mcu::common::{Error, Result};
use escw_mcu::peripheral::flash::FlashDevice;

use crate::hal::flash::*;

pub struct Flash {}

impl Flash {
    pub fn new() -> Self {
        Flash {}
    }
}
impl FlashDevice for Flash {
    fn erase(&self, bank: u32, sector: u32, count: u32) -> Result<()> {
        let error: u32 = 0;

        unsafe {
            HAL_FLASH_Unlock().ok()?;
            HAL_FLASHEx_Erase(&FlashEraseInitTypeDef::new(bank, sector, count), &error).ok()?;
            HAL_FLASH_Lock().ok()?;
        }

        if error != 0xFFFF_FFFF {
            return Err(Error::Unknown);
        }

        Ok(())
    }

    fn program(&self, address: u32, data: &[u8]) -> Result<()> {
        unsafe {
            HAL_FLASH_Unlock().ok()?;

            for idx in 0..data.len() {
                HAL_FLASH_Program(
                    FLASH_TYPEPROGRAM_BYTE,
                    address + idx as u32,
                    data[idx] as u64,
                );
            }

            HAL_FLASH_Lock().ok()?;
        }

        Ok(())
    }
}
