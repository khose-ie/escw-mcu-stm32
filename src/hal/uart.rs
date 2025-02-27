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
pub struct UartHandle;

#[allow(improper_ctypes)]
extern "C" {
    pub fn HAL_UART_Transmit(huart: *mut UartHandle, pData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_UART_Receive(huart: *mut UartHandle, pData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_UART_Transmit_IT(huart: *mut UartHandle, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_Receive_IT(huart: *mut UartHandle, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_Transmit_DMA(huart: *mut UartHandle, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_Receive_DMA(huart: *mut UartHandle, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_Abort(huart: *mut UartHandle) -> HalStatus;
    pub fn HAL_UART_AbortTransmit(huart: *mut UartHandle) -> HalStatus;
    pub fn HAL_UART_AbortReceive(huart: *mut UartHandle) -> HalStatus;
    pub fn HAL_UART_Abort_IT(huart: *mut UartHandle) -> HalStatus;
    pub fn HAL_UART_AbortTransmit_IT(huart: *mut UartHandle) -> HalStatus;
    pub fn HAL_UART_AbortReceive_IT(huart: *mut UartHandle) -> HalStatus;
    pub fn HAL_UARTEx_ReceiveToIdle(huart: *mut UartHandle, pData: *const u8, Size: u16, RxLen: &u16, Timeout: u32) -> HalStatus;
    pub fn HAL_UARTEx_ReceiveToIdle_IT(huart: *mut UartHandle, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UARTEx_ReceiveToIdle_DMA(huart: *mut UartHandle, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_UART_GetState(huart: *mut UartHandle) -> State;
}
