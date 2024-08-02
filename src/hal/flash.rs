use super::HalStatus;

pub const FLASH_ERASE_KIND_SECTORS: u32 = 0x0000_0000;
pub const FLASH_ERASE_KIND_MASS: u32 = 0x0000_0001;

pub const FLASH_VOLTAGE_RANGE1: u32 = 0x0000_0000;
pub const FLASH_VOLTAGE_RANGE2: u32 = 0x0000_0001;
pub const FLASH_VOLTAGE_RANGE3: u32 = 0x0000_0002;
pub const FLASH_VOLTAGE_RANGE4: u32 = 0x0000_0003;

pub const FLASH_TYPEPROGRAM_BYTE: u32 = 0x0000_0000;
pub const FLASH_TYPEPROGRAM_HALFWORD: u32 = 0x0000_0001;
pub const FLASH_TYPEPROGRAM_WORD: u32 = 0x0000_0002;
pub const FLASH_TYPEPROGRAM_DOUBLEWORD: u32 = 0x0000_0003;

#[repr(C)]
pub struct FlashEraseInitTypeDef {
    pub type_erase: u32,
    pub banks: u32,
    pub sector: u32,
    pub count: u32,
    pub voltage_range: u32,
}

impl FlashEraseInitTypeDef {
    pub fn new(banks: u32, sector: u32, count: u32) -> Self {
        FlashEraseInitTypeDef {
            type_erase: FLASH_ERASE_KIND_SECTORS,
            banks,
            sector,
            count,
            voltage_range: FLASH_VOLTAGE_RANGE1,
        }
    }
}

extern "C" {
    pub fn HAL_FLASH_Unlock() -> HalStatus;
    pub fn HAL_FLASH_Lock() -> HalStatus;
    pub fn HAL_FLASH_Program(TypeProgram: u32, Address: u32, Data: u64) -> HalStatus;
    pub fn HAL_FLASH_Program_IT(TypeProgram: u32, Address: u32, Data: u64) -> HalStatus;
    pub fn HAL_FLASHEx_Erase(pEraseInit: &FlashEraseInitTypeDef, SectorError: &u32) -> HalStatus;
    pub fn HAL_FLASHEx_Erase_IT(pEraseInit: &FlashEraseInitTypeDef) -> HalStatus;

}
