use super::HalStatus;

#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum State
{
    Reset = 0x00,
    Ready = 0x20,
    Busy = 0x24,
    BusyTx = 0x21,
    BusyRx = 0x22,
    BusyTxRx = 0x23,
    Timeout = 0xA0,
    Error = 0xE0,
}

#[repr(C)]
pub struct Huart
{
    pub instance: u32,
}

extern "C" {
    #[cfg(feature = "uart1")]
    pub static huart1: Huart;
    #[cfg(feature = "uart2")]
    pub static huart2: Huart;
    #[cfg(feature = "uart3")]
    pub static huart3: Huart;
    #[cfg(feature = "uart4")]
    pub static huart4: Huart;
    #[cfg(feature = "uart5")]
    pub static huart5: Huart;
    #[cfg(feature = "uart6")]
    pub static huart6: Huart;
    #[cfg(feature = "uart7")]
    pub static huart7: Huart;
    #[cfg(feature = "uart8")]
    pub static huart8: Huart;
}

extern "C" {
    pub fn HAL_UART_Transmit(huart: &Huart, pData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_UART_Receive(huart: &Huart, pData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_UART_Transmit_IT(huart: &Huart, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_Receive_IT(huart: &Huart, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_Transmit_DMA(huart: &Huart, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_Receive_DMA(huart: &Huart, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_Abort(huart: &Huart) -> HalStatus;
    pub fn HAL_UART_AbortTransmit(huart: &Huart) -> HalStatus;
    pub fn HAL_UART_AbortReceive(huart: &Huart) -> HalStatus;
    pub fn HAL_UART_Abort_IT(huart: &Huart) -> HalStatus;
    pub fn HAL_UART_AbortTransmit_IT(huart: &Huart) -> HalStatus;
    pub fn HAL_UART_AbortReceive_IT(huart: &Huart) -> HalStatus;
    pub fn HAL_UARTEx_ReceiveToIdle(huart: &Huart, pData: *const u8, Size: u16, RxLen: &u16, Timeout: u32) -> HalStatus;
    pub fn HAL_UARTEx_ReceiveToIdle_IT(huart: &Huart, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UARTEx_ReceiveToIdle_DMA(huart: &Huart, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_GetState(huart: &Huart) -> State;
}
