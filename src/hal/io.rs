use super::HalStatus;

#[repr(C)]
pub struct GPIO;

#[allow(improper_ctypes)]
extern "C" {
    pub fn HAL_GPIO_ReadPin(GPIOx: *mut GPIO, GPIO_Pin: u16) -> u32;
    pub fn HAL_GPIO_WritePin(GPIOx: *mut GPIO, GPIO_Pin: u16, PinState: u32);
    pub fn HAL_GPIO_TogglePin(GPIOx: *mut GPIO, GPIO_Pin: u16);
    pub fn HAL_GPIO_LockPin(GPIOx: *mut GPIO, GPIO_Pin: u16) -> HalStatus;
}
