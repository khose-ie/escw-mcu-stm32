use super::HalStatus;

#[repr(C)]
pub struct Hwwdg
{
    pub instance: u32,
}

extern "C" {
    #[cfg(feature = "wwdg")]
    pub static hwwdg: Hwwdg;
}

extern "C" {
    pub fn HAL_WWDG_Refresh(hwwdg: &Hwwdg) -> HalStatus;
}
