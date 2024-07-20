use core::ffi::c_void;
use escw_mcu::peripheral::io::IoState;

use crate::hal::io::*;

pub enum IoPort {
    IOA = crate::memory::GPIOA_BASE as isize,
    IOB = crate::memory::GPIOB_BASE as isize,
    IOC = crate::memory::GPIOC_BASE as isize,
    IOD = crate::memory::GPIOD_BASE as isize,
    IOE = crate::memory::GPIOE_BASE as isize,
    IOF = crate::memory::GPIOF_BASE as isize,
    IOG = crate::memory::GPIOG_BASE as isize,
    IOH = crate::memory::GPIOH_BASE as isize,
    IOI = crate::memory::GPIOI_BASE as isize,
    IOJ = crate::memory::GPIOJ_BASE as isize,
    IOK = crate::memory::GPIOK_BASE as isize,
}

impl Into<u32> for IoPort {
    fn into(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy)]
pub enum IoPin {
    P00,
    P01,
    P02,
    P03,
    P04,
    P05,
    P06,
    P07,
    P08,
    P09,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}

impl IoPin {
    pub const fn size() -> usize {
        16
    }
}

impl From<u16> for IoPin {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => Self::P00,
            0x0002 => Self::P01,
            0x0004 => Self::P02,
            0x0008 => Self::P03,
            0x0010 => Self::P04,
            0x0020 => Self::P05,
            0x0040 => Self::P06,
            0x0080 => Self::P07,
            0x0100 => Self::P08,
            0x0200 => Self::P09,
            0x0400 => Self::P10,
            0x0800 => Self::P11,
            0x1000 => Self::P12,
            0x2000 => Self::P13,
            0x4000 => Self::P14,
            0x8000 => Self::P15,
            _ => Self::P15,
        }
    }
}

const U16_TO_PIN: [u16; IoPin::size()] = [
    0x0001, 0x0002, 0x0004, 0x0008, 0x0010, 0x0020, 0x0040, 0x0080, 0x0100, 0x0200, 0x0400, 0x0800,
    0x1000, 0x2000, 0x4000, 0x8000,
];

impl Into<u16> for IoPin {
    fn into(self) -> u16 {
        U16_TO_PIN[self as usize]
    }
}

pub struct Io {
    port: u32,
    pin: IoPin,
}

impl escw_mcu::peripheral::io::Io for Io {
    type Port = IoPort;
    type Pin = IoPin;

    fn new(port: IoPort, pin: IoPin) -> Self {
        Io {
            port: port.into(),
            pin,
        }
    }

    fn with_event(&self, handle: fn()) {
        event::EventHandles::set(self.pin.into(), handle);
    }

    fn state(&self) -> IoState {
        IoState::from(unsafe { HAL_GPIO_ReadPin(self.port as *const c_void, self.pin.into()) })
    }

    fn write(&self, state: IoState) {
        unsafe {
            HAL_GPIO_WritePin(self.port as *const c_void, self.pin.into(), state.into());
        }
    }

    fn toggle(&self) {
        unsafe {
            HAL_GPIO_TogglePin(self.port as *const c_void, self.pin.into());
        }
    }
}

mod event {
    use super::IoPin;

    static mut EVENT_HANDLES: EventHandles = EventHandles::new();

    #[derive(Clone, Copy)]
    pub struct EventHandles {
        invokes: [Option<fn()>; IoPin::size()],
    }

    impl EventHandles {
        const fn new() -> Self {
            EventHandles {
                invokes: [None; IoPin::size()],
            }
        }

        pub fn set(pin: IoPin, func: fn()) {
            unsafe {
                EVENT_HANDLES.invokes[pin as usize] = Some(func);
            }
        }

        pub fn invoke(&self, pin: IoPin) {
            if let Some(invoke) = self.invokes[pin as usize].as_ref() {
                invoke();
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_GPIO_EXTI_Callback(pin: u16) {
        unsafe {
            EVENT_HANDLES.invoke(IoPin::from(pin));
        }
    }
}
