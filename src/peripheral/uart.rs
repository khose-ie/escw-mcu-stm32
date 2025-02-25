use escw_mcu::common::Error;
use escw_mcu::common::Result;
use escw_mcu::peripheral::uart::UartDevice;
use escw_mcu::peripheral::uart::UartEventHandle;

use crate::hal::uart::*;
use crate::hal::HalStatus;

#[derive(Clone, Copy)]
pub enum UartIdentifies
{
    #[cfg(feature = "uart1")]
    U1,
    #[cfg(feature = "uart2")]
    U2,
    #[cfg(feature = "uart3")]
    U3,
    #[cfg(feature = "uart4")]
    U4,
    #[cfg(feature = "uart5")]
    U5,
    #[cfg(feature = "uart6")]
    U6,
    #[cfg(feature = "uart7")]
    U7,
    #[cfg(feature = "uart8")]
    U8,
}

impl UartIdentifies
{
    pub const fn count() -> usize
    {
        8
    }
}

impl Into<usize> for UartIdentifies
{
    fn into(self) -> usize
    {
        self as usize
    }
}

impl Into<&Huart> for UartIdentifies
{
    fn into(self) -> &'static Huart
    {
        unsafe {
            match self {
                #[cfg(feature = "uart1")]
                Self::U1 => &huart1,
                #[cfg(feature = "uart2")]
                Self::U2 => &huart2,
                #[cfg(feature = "uart3")]
                Self::U3 => &huart3,
                #[cfg(feature = "uart4")]
                Self::U4 => &huart4,
                #[cfg(feature = "uart5")]
                Self::U5 => &huart5,
                #[cfg(feature = "uart6")]
                Self::U6 => &huart6,
                #[cfg(feature = "uart7")]
                Self::U7 => &huart7,
                #[cfg(feature = "uart8")]
                Self::U8 => &huart8,
            }
        }
    }
}

impl TryInto<UartIdentifies> for &Huart
{
    type Error = Error;

    fn try_into(self) -> core::result::Result<UartIdentifies, Self::Error>
    {
        match self.instance {
            #[cfg(feature = "uart1")]
            crate::memory::USART1_BASE => Ok(UartIdentifies::U1),
            #[cfg(feature = "uart2")]
            crate::memory::USART2_BASE => Ok(UartIdentifies::U2),
            #[cfg(feature = "uart3")]
            crate::memory::USART3_BASE => Ok(UartIdentifies::U3),
            #[cfg(feature = "uart4")]
            crate::memory::UART4_BASE => Ok(UartIdentifies::U4),
            #[cfg(feature = "uart5")]
            crate::memory::UART5_BASE => Ok(UartIdentifies::U5),
            #[cfg(feature = "uart6")]
            crate::memory::USART6_BASE => Ok(UartIdentifies::U6),
            #[cfg(feature = "uart7")]
            crate::memory::UART7_BASE => Ok(UartIdentifies::U7),
            #[cfg(feature = "uart8")]
            crate::memory::UART8_BASE => Ok(UartIdentifies::U8),
            #[cfg(feature = "uart1")]
            _ => Err(Error::Param),
        }
    }
}

pub struct Uart
{
    uart: UartIdentifies,
}

impl Uart
{
    pub fn new(uart: UartIdentifies) -> Self
    {
        Uart { uart }
    }
}

impl UartDevice for Uart
{
    /// THe TransmitState::Half state will be sent as event only on DMA kind.
    fn with_event(&mut self, handle: UartEventHandle)
    {
        event::EventCenter::set(self.uart, handle);
    }

    fn send(&self, data: &[u8], timeout: u32) -> Result<()>
    {
        unsafe { HAL_UART_Transmit(self.uart.into(), data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<u32>
    {
        let mut size: u16 = 0;

        unsafe {
            let state = HAL_UARTEx_ReceiveToIdle(self.uart.into(), data.as_ptr(), data.len() as u16, &mut size, timeout);

            match state {
                HalStatus::Ok => Ok(size as u32),
                _ => Err(state.into()),
            }
        }
    }

    fn send_with_interrupt(&self, data: &[u8]) -> Result<()>
    {
        unsafe { HAL_UART_Transmit_IT(self.uart.into(), data.as_ptr(), data.len() as u16).into() }
    }

    fn receive_with_interrupt(&self, data: &mut [u8]) -> Result<()>
    {
        unsafe { HAL_UARTEx_ReceiveToIdle_IT(self.uart.into(), data.as_ptr(), data.len() as u16).into() }
    }

    fn send_with_dma(&self, data: &[u8]) -> Result<()>
    {
        unsafe { HAL_UART_Transmit_DMA(self.uart.into(), data.as_ptr(), data.len() as u16).into() }
    }

    fn receive_with_dma(&self, data: &mut [u8]) -> Result<()>
    {
        unsafe { HAL_UARTEx_ReceiveToIdle_DMA(self.uart.into(), data.as_ptr(), data.len() as u16).into() }
    }

    fn abort(&self) -> Result<()>
    {
        unsafe { HAL_UART_Abort_IT(self.uart.into()).into() }
    }

    fn abort_send(&self) -> Result<()>
    {
        unsafe { HAL_UART_AbortTransmit_IT(self.uart.into()).into() }
    }

    fn abort_receive(&self) -> Result<()>
    {
        unsafe { HAL_UART_AbortReceive_IT(self.uart.into()).into() }
    }
}

mod event
{
    use escw_mcu::peripheral::uart::UartEvent;
    use escw_mcu::peripheral::uart::UartEventHandle;

    use crate::hal::uart::*;

    use super::{Huart, UartIdentifies};

    static mut EVENT_CENTER: EventCenter = EventCenter::new();

    pub struct EventCenter
    {
        handle: [Option<UartEventHandle>; UartIdentifies::count()],
    }

    impl EventCenter
    {
        const fn new() -> Self
        {
            EventCenter {
                handle: [None; UartIdentifies::count()],
            }
        }

        pub fn set(uart: UartIdentifies, invoke: UartEventHandle)
        {
            unsafe {
                EVENT_CENTER.handle[uart as usize] = Some(invoke);
            }
        }

        pub fn invoke(uart: UartIdentifies, event: UartEvent)
        {
            unsafe {
                if let Some(invoke) = EVENT_CENTER.handle[uart as usize].as_ref() {
                    invoke(event);
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_TxCpltCallback(huart: &Huart)
    {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, UartEvent::TxCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_TxHalfCpltCallback(huart: &Huart)
    {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, UartEvent::TxHalf);
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
    pub extern "C" fn HAL_UART_ErrorCallback(huart: &Huart)
    {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, UartEvent::Error);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_AbortCpltCallback(huart: &Huart)
    {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, UartEvent::TxRxAborted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_AbortTransmitCpltCallback(huart: &Huart)
    {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, UartEvent::TxAborted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_AbortReceiveCpltCallback(huart: &Huart)
    {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, UartEvent::RxAborted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UARTEx_RxEventCallback(huart: &Huart, size: u16)
    {
        let mut state = UartEvent::RxCompleted(size);

        unsafe {
            if HAL_UART_GetState(huart) == State::BusyRx {
                state = UartEvent::RxHalf;
            }
        }

        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, state);
        }
    }
}
