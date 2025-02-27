mod uart_event;

pub use uart_event::UartEvent;

use escw_mcu::common::Result;
use escw_mcu::peripheral::uart::UartDevice;

use crate::hal::uart::*;
use crate::hal::HalStatus;

static mut EVENT_HANDLE: Option<fn(&mut UartHandle, UartEvent)> = None;

pub struct Uart
{
    uart: *mut UartHandle,
}

impl Uart
{
    pub fn new(uart: *mut UartHandle) -> Self
    {
        Uart { uart }
    }
}

impl UartDevice for Uart
{
    type Handle = UartHandle;
    type EventCode = UartEvent;

    /// THe TransmitState::Half state will be sent as event only on DMA kind.
    fn with_event(event_handle: fn(&mut Self::Handle, Self::EventCode))
    {
        unsafe {
            EVENT_HANDLE = Some(event_handle);
        }
    }

    fn transmit(&self, data: &[u8], timeout: u32) -> Result<()>
    {
        unsafe { HAL_UART_Transmit(self.uart, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<u32>
    {
        let mut size: u16 = 0;

        unsafe {
            let state = HAL_UARTEx_ReceiveToIdle(self.uart, data.as_ptr(), data.len() as u16, &mut size, timeout);

            match state
            {
                HalStatus::Ok => Ok(size as u32),
                _ => Err(state.into()),
            }
        }
    }

    fn transmit_async_int(&self, data: &[u8]) -> Result<()>
    {
        unsafe { HAL_UART_Transmit_IT(self.uart, data.as_ptr(), data.len() as u16).into() }
    }

    fn receive_async_int(&self, data: &mut [u8]) -> Result<()>
    {
        unsafe { HAL_UARTEx_ReceiveToIdle_IT(self.uart, data.as_ptr(), data.len() as u16).into() }
    }

    fn transmit_async_dma(&self, data: &[u8]) -> Result<()>
    {
        unsafe { HAL_UART_Transmit_DMA(self.uart, data.as_ptr(), data.len() as u16).into() }
    }

    fn receive_async_dma(&self, data: &mut [u8]) -> Result<()>
    {
        unsafe { HAL_UARTEx_ReceiveToIdle_DMA(self.uart, data.as_ptr(), data.len() as u16).into() }
    }

    fn abort(&self) -> Result<()>
    {
        unsafe { HAL_UART_Abort_IT(self.uart).into() }
    }

    fn abort_transmit(&self) -> Result<()>
    {
        unsafe { HAL_UART_AbortTransmit_IT(self.uart).into() }
    }

    fn abort_receive(&self) -> Result<()>
    {
        unsafe { HAL_UART_AbortReceive_IT(self.uart).into() }
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_UART_TxCpltCallback(uart: *mut UartHandle)
{
    if let Some(event_handle) = EVENT_HANDLE
    {
        event_handle(uart.as_mut().unwrap_unchecked(), UartEvent::TxCompleted);
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_UART_TxHalfCpltCallback(uart: *mut UartHandle)
{
    if let Some(event_handle) = EVENT_HANDLE
    {
        event_handle(uart.as_mut().unwrap_unchecked(), UartEvent::TxHalf);
    }
}

// #[no_mangle]
// pub extern "C" fn HAL_UART_RxCpltCallback(huart: &Huart) {
//     if let Some(uart) = huart.try_into().ok() {
//         EventCenter::invoke(uart, UartEvent::RxCompleted);
//     }
// }

// #[no_mangle]
// pub extern "C" fn HAL_UART_RxHalfCpltCallback(huart: &Huart) {
//     if let Some(uart) = huart.try_into().ok() {
//         EventCenter::invoke(uart, UartEvent::RxHalf);
//     }
// }

#[no_mangle]
pub unsafe extern "C" fn HAL_UART_ErrorCallback(uart: *mut UartHandle)
{
    if let Some(event_handle) = EVENT_HANDLE
    {
        event_handle(uart.as_mut().unwrap_unchecked(), UartEvent::Error);
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_UART_AbortCpltCallback(uart: *mut UartHandle)
{
    if let Some(event_handle) = EVENT_HANDLE
    {
        event_handle(uart.as_mut().unwrap_unchecked(), UartEvent::TxRxAborted);
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_UART_AbortTransmitCpltCallback(uart: *mut UartHandle)
{
    if let Some(event_handle) = EVENT_HANDLE
    {
        event_handle(uart.as_mut().unwrap_unchecked(), UartEvent::TxAborted);
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_UART_AbortReceiveCpltCallback(uart: *mut UartHandle)
{
    if let Some(event_handle) = EVENT_HANDLE
    {
        event_handle(uart.as_mut().unwrap_unchecked(), UartEvent::RxAborted);
    }
}

#[no_mangle]
pub unsafe extern "C" fn HAL_UARTEx_RxEventCallback(uart: *mut UartHandle, size: u16)
{
    let mut state = UartEvent::RxCompleted(size);

    if HAL_UART_GetState(uart) == State::BusyRx
    {
        state = UartEvent::RxHalf;
    }

    if let Some(event_handle) = EVENT_HANDLE
    {
        event_handle(uart.as_mut().unwrap_unchecked(), state);
    }
}
