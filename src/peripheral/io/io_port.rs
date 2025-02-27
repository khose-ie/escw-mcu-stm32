use crate::hal::io::GPIO;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum IoPort
{
    A = crate::memory::GPIOA_BASE,
    B = crate::memory::GPIOB_BASE,
    C = crate::memory::GPIOC_BASE,
    D = crate::memory::GPIOD_BASE,
    E = crate::memory::GPIOE_BASE,
    F = crate::memory::GPIOF_BASE,
    G = crate::memory::GPIOG_BASE,
    H = crate::memory::GPIOH_BASE,
    I = crate::memory::GPIOI_BASE,
    J = crate::memory::GPIOJ_BASE,
    K = crate::memory::GPIOK_BASE,
}

impl Into<u32> for IoPort
{
    fn into(self) -> u32
    {
        self as u32
    }
}

impl Into<*mut GPIO> for IoPort
{
    fn into(self) -> *mut GPIO
    {
        self as u32 as *mut GPIO
    }
}

