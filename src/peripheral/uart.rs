use escw_mcu::common::{AsyncKind, Error, Result, TransmitDirection};
use escw_mcu::peripheral::uart::{Uart, UartEventHandle};

use crate::hal::uart::*;
use crate::hal::HalStatus;

const UART_MAX: usize = 8;

#[derive(Clone, Copy)]
pub enum UartNum {
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

impl Into<usize> for UartNum {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<&Huart> for UartNum {
    fn into(self) -> &'static Huart {
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

impl TryInto<UartNum> for &Huart {
    type Error = Error;

    fn try_into(self) -> core::result::Result<UartNum, Self::Error> {
        match self.instance {
            #[cfg(feature = "uart1")]
            crate::memory::USART1_BASE => Ok(UartNum::U1),
            #[cfg(feature = "uart2")]
            crate::memory::USART2_BASE => Ok(UartNum::U2),
            #[cfg(feature = "uart3")]
            crate::memory::USART3_BASE => Ok(UartNum::U3),
            #[cfg(feature = "uart4")]
            crate::memory::UART4_BASE => Ok(UartNum::U4),
            #[cfg(feature = "uart5")]
            crate::memory::UART5_BASE => Ok(UartNum::U5),
            #[cfg(feature = "uart6")]
            crate::memory::USART6_BASE => Ok(UartNum::U6),
            #[cfg(feature = "uart7")]
            crate::memory::UART7_BASE => Ok(UartNum::U7),
            #[cfg(feature = "uart8")]
            crate::memory::UART8_BASE => Ok(UartNum::U8),
            #[cfg(feature = "uart1")]
            _ => Err(Error::Param),
        }
    }
}

pub struct Stm32Usart {
    uart: UartNum,
}

impl Uart for Stm32Usart {
    type UartNum = UartNum;

    fn new(uart: UartNum) -> Self {
        Stm32Usart { uart }
    }

    /// THe TransmitState::Half state will be sent as event only on DMA kind.
    fn with_event(&mut self, handle: UartEventHandle) {
        event::EventCenter::set(self.uart, 0, handle);
    }

    fn send(&self, data: &[u8], timeout: u32) -> Result<()> {
        unsafe {
            HAL_UART_Transmit(self.uart.into(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<u32> {
        unsafe {
            let mut size: u16 = 0;
            let state = HAL_UARTEx_ReceiveToIdle(
                self.uart.into(),
                data.as_ptr(),
                data.len() as u16,
                &mut size,
                timeout,
            );

            match state {
                HalStatus::Ok => Ok(size as u32),
                _ => Err(state.into()),
            }
        }
    }

    fn async_send(&mut self, kind: AsyncKind, data: &[u8]) -> Result<()> {
        unsafe {
            match kind {
                AsyncKind::Interrupt => {
                    HAL_UART_Transmit_IT(self.uart.into(), data.as_ptr(), data.len() as u16).into()
                }
                AsyncKind::Dma => {
                    HAL_UART_Transmit_DMA(self.uart.into(), data.as_ptr(), data.len() as u16).into()
                }
            }
        }
    }

    fn async_receive(&mut self, kind: AsyncKind, data: &mut [u8]) -> Result<()> {
        unsafe {
            match kind {
                AsyncKind::Interrupt => {
                    event::EventCenter::set_size(self.uart, data.len() as u16);
                    HAL_UARTEx_ReceiveToIdle_IT(self.uart.into(), data.as_ptr(), data.len() as u16)
                        .into()
                }
                AsyncKind::Dma => {
                    event::EventCenter::set_size(self.uart, data.len() as u16);
                    HAL_UARTEx_ReceiveToIdle_DMA(self.uart.into(), data.as_ptr(), data.len() as u16)
                        .into()
                }
            }
        }
    }

    fn abort_async(&mut self, kind: TransmitDirection) -> Result<()> {
        unsafe {
            match kind {
                TransmitDirection::Send => HAL_UART_AbortTransmit(self.uart.into()).into(),
                TransmitDirection::Receive => HAL_UART_AbortReceive(self.uart.into()).into(),
                TransmitDirection::Any => HAL_UART_Abort(self.uart.into()).into(),
            }
        }
    }

    fn async_abort_async(&mut self, kind: TransmitDirection) -> Result<()> {
        unsafe {
            match kind {
                TransmitDirection::Send => HAL_UART_AbortTransmit_IT(self.uart.into()).into(),
                TransmitDirection::Receive => HAL_UART_AbortReceive_IT(self.uart.into()).into(),
                TransmitDirection::Any => HAL_UART_Abort_IT(self.uart.into()).into(),
            }
        }
    }
}

mod event {
    use escw_mcu::common::{TransmitDirection, TransmitState};
    use escw_mcu::peripheral::uart::UartEventHandle;

    use crate::hal::uart::*;

    use super::{Huart, UartNum, UART_MAX};

    static mut EVENT_CENTER: EventCenter = EventCenter::new();

    #[derive(Clone, Copy)]
    struct EventHandle {
        size: u16,
        invoke: UartEventHandle,
    }

    pub struct EventCenter {
        handle: [Option<EventHandle>; UART_MAX],
    }

    impl EventCenter {
        const fn new() -> Self {
            EventCenter {
                handle: [None; UART_MAX],
            }
        }

        pub fn set(uart: UartNum, size: u16, invoke: UartEventHandle) {
            unsafe {
                EVENT_CENTER.handle[uart as usize] = Some(EventHandle { size, invoke });
            }
        }

        pub fn set_size(uart: UartNum, size: u16) {
            unsafe {
                if let Some(mut handle) = EVENT_CENTER.handle[uart as usize] {
                    handle.size = size;
                }
            }
        }

        pub fn invoke(uart: UartNum, kind: TransmitDirection, state: TransmitState, size: u16) {
            unsafe {
                if let Some(handle) = EVENT_CENTER.handle[uart as usize].as_ref() {
                    (handle.invoke)(kind, state, size);
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_TxCpltCallback(huart: &Huart) {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, TransmitDirection::Send, TransmitState::Completed, 0);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_TxHalfCpltCallback(huart: &Huart) {
        let a: Option<UartNum> = huart.try_into().ok();
        if let Some(uart) = a {
            EventCenter::invoke(uart, TransmitDirection::Send, TransmitState::Half, 0);
        }
    }

    // #[no_mangle]
    // pub extern "C" fn HAL_UART_RxCpltCallback(huart: &Huart) {
    //     if let Some(uart) = huart.try_into().ok() {
    //         EventCenter::invoke(
    //             uart,
    //             TransmitDirection::Receive,
    //             TransmitState::Completed,
    //             0,
    //         );
    //     }
    // }

    // #[no_mangle]
    // pub extern "C" fn HAL_UART_RxHalfCpltCallback(huart: &Huart) {
    //     if let Some(uart) = huart.try_into().ok() {
    //         EventCenter::invoke(uart, TransmitDirection::Receive, TransmitState::Half, 0);
    //     }
    // }

    #[no_mangle]
    pub extern "C" fn HAL_UART_ErrorCallback(huart: &Huart) {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, TransmitDirection::Any, TransmitState::Failure, 0);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_AbortCpltCallback(huart: &Huart) {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, TransmitDirection::Any, TransmitState::Aborted, 0);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_AbortTransmitCpltCallback(huart: &Huart) {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, TransmitDirection::Send, TransmitState::Aborted, 0);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UART_AbortReceiveCpltCallback(huart: &Huart) {
        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, TransmitDirection::Receive, TransmitState::Aborted, 0);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_UARTEx_RxEventCallback(huart: &Huart, size: u16) {
        let mut state = TransmitState::Completed;

        unsafe {
            if HAL_UART_GetState(huart) == State::BusyRx {
                state = TransmitState::Half;
            }
        }

        if let Some(uart) = huart.try_into().ok() {
            EventCenter::invoke(uart, TransmitDirection::Receive, state, size);
        }
    }
}
