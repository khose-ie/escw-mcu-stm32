pub mod i2c;
pub mod io;
pub mod uart;

use escw_mcu::common::{Error, Result};

#[repr(C)]
#[derive(Debug)]
pub enum HalStatus {
    Ok = 0,
    Error = 1,
    Busy = 2,
    Timeout = 3,
    Unknown = 4,
}

impl From<u32> for HalStatus {
    fn from(value: u32) -> HalStatus {
        match value {
            0 => HalStatus::Ok,
            1 => HalStatus::Error,
            2 => HalStatus::Busy,
            3 => HalStatus::Timeout,
            _ => HalStatus::Unknown,
        }
    }
}

impl Into<Result<()>> for HalStatus {
    fn into(self) -> Result<()> {
        match self {
            Self::Ok => Ok(()),
            Self::Error => Err(Error::Param),
            Self::Busy => Err(Error::PeripheralBusy),
            Self::Timeout => Err(Error::WaitTimeout),
            Self::Unknown => Err(Error::Unknown),
        }
    }
}

impl Into<Error> for HalStatus {
    fn into(self) -> Error {
        match self {
            Self::Ok => Error::Param,
            Self::Error => Error::Param,
            Self::Busy => Error::PeripheralBusy,
            Self::Timeout => Error::WaitTimeout,
            Self::Unknown => Error::Unknown,
        }
    }
}
