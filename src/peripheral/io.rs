mod io_pin;
mod io_port;

pub use io_pin::IoPin;
pub use io_port::IoPort;

use crate::hal::io::{HAL_GPIO_ReadPin, HAL_GPIO_TogglePin, HAL_GPIO_WritePin};
use escw_mcu::peripheral::io::{IoDevice, IoState};

static mut EVENT_HANDLE: Option<fn(pin: IoPin)> = None;

pub struct Io
{
    port: IoPort,
    pin: IoPin,
}

impl Io
{
    pub const fn new(port: IoPort, pin: IoPin) -> Self
    {
        Io { port, pin }
    }
}

impl IoDevice for Io
{
    type Pin = IoPin;

    fn with_event(&self, handle: fn(pin: Self::Pin))
    {
        unsafe {
            EVENT_HANDLE = Some(handle);
        }
    }

    fn state(&self) -> IoState
    {
        IoState::from(unsafe { HAL_GPIO_ReadPin(self.port.into(), self.pin.into()) })
    }

    fn set_state(&self, state: IoState)
    {
        unsafe {
            HAL_GPIO_WritePin(self.port.into(), self.pin.into(), state.into());
        }
    }

    fn toggle(&self)
    {
        unsafe {
            HAL_GPIO_TogglePin(self.port.into(), self.pin.into());
        }
    }
}

#[no_mangle]
pub extern "C" fn HAL_GPIO_EXTI_Callback(pin: u16)
{
    unsafe {
        if let Some(event_handle) = EVENT_HANDLE
        {
            event_handle(IoPin::from(pin));
        }
    }
}
