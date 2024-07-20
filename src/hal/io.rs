use core::ffi::c_void;

use super::HalStatus;

extern "C" {
    pub fn HAL_GPIO_ReadPin(GPIOx: *const c_void, GPIO_Pin: u16) -> u32;
    pub fn HAL_GPIO_WritePin(GPIOx: *const c_void, GPIO_Pin: u16, PinState: u32);
    pub fn HAL_GPIO_TogglePin(GPIOx: *const c_void, GPIO_Pin: u16);
    pub fn HAL_GPIO_LockPin(GPIOx: *const c_void, GPIO_Pin: u16) -> HalStatus;
}
