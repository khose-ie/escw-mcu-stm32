use super::HalStatus;

#[repr(C)]
pub struct Hspi
{
    pub instance: u32,
}

extern "C" {
    #[cfg(feature = "spi1")]
    pub static hspi1: Hspi;
    #[cfg(feature = "spi2")]
    pub static hspi2: Hspi;
    #[cfg(feature = "spi3")]
    pub static hspi3: Hspi;
    #[cfg(feature = "spi4")]
    pub static hspi4: Hspi;
    #[cfg(feature = "spi5")]
    pub static hspi5: Hspi;
    #[cfg(feature = "spi6")]
    pub static hspi6: Hspi;
}

extern "C" {
    pub fn HAL_SPI_Transmit(hspi: &Hspi, pData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_SPI_Receive(hspi: &Hspi, pData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_SPI_TransmitReceive(hspi: &Hspi, pTxData: *const u8, pRxData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_SPI_Transmit_IT(hspi: &Hspi, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_SPI_Receive_IT(hspi: &Hspi, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_SPI_TransmitReceive_IT(hspi: &Hspi, pTxData: *const u8, pRxData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_SPI_Transmit_DMA(hspi: &Hspi, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_SPI_Receive_DMA(hspi: &Hspi, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_SPI_TransmitReceive_DMA(hspi: &Hspi, pTxData: *const u8, pRxData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_SPI_Abort(hspi: &Hspi) -> HalStatus;
    pub fn HAL_SPI_Abort_IT(hspi: &Hspi) -> HalStatus;
}
