use super::HalStatus;

#[repr(C)]
pub struct Hiwdg
{
    pub instance: u32,
}

extern "C" {
    #[cfg(feature = "iwdg")]
    pub static hiwdg: Hiwdg;
}

extern "C" {
    pub fn HAL_IWDG_Refresh(hiwdg: &Hiwdg) -> HalStatus;
}
