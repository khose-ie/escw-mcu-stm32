use escw_mcu::common::Error;
use escw_mcu::common::Result;
use escw_mcu::peripheral::spi::SpiDevice;
use escw_mcu::peripheral::spi::SpiEventHandle;

use crate::hal::spi::*;

#[derive(Clone, Copy)]
pub enum SpiIdentifies
{
    #[cfg(feature = "spi1")]
    Spi1,
    #[cfg(feature = "spi2")]
    Spi2,
    #[cfg(feature = "spi3")]
    Spi3,
    #[cfg(feature = "spi4")]
    Spi4,
    #[cfg(feature = "spi5")]
    Spi5,
    #[cfg(feature = "spi6")]
    Spi6,
}

impl SpiIdentifies
{
    pub const fn count() -> usize
    {
        6
    }
}

impl Into<usize> for SpiIdentifies
{
    fn into(self) -> usize
    {
        self as usize
    }
}

impl Into<&Hspi> for SpiIdentifies
{
    fn into(self) -> &'static Hspi
    {
        unsafe {
            match self {
                #[cfg(feature = "spi1")]
                Self::Spi1 => &hspi1,
                #[cfg(feature = "spi2")]
                Self::Spi2 => &hspi2,
                #[cfg(feature = "spi3")]
                Self::Spi3 => &hspi3,
                #[cfg(feature = "spi4")]
                Self::Spi4 => &hspi4,
                #[cfg(feature = "spi5")]
                Self::Spi5 => &hspi5,
                #[cfg(feature = "spi6")]
                Self::Spi6 => &hspi6,
            }
        }
    }
}

impl TryInto<SpiIdentifies> for &Hspi
{
    type Error = Error;

    fn try_into(self) -> core::result::Result<SpiIdentifies, Self::Error>
    {
        match self.instance {
            #[cfg(feature = "spi1")]
            crate::memory::SPI1_BASE => Ok(SpiIdentifies::Spi1),
            #[cfg(feature = "spi2")]
            crate::memory::SPI2_BASE => Ok(SpiIdentifies::Spi2),
            #[cfg(feature = "spi3")]
            crate::memory::SPI3_BASE => Ok(SpiIdentifies::Spi3),
            #[cfg(feature = "spi4")]
            crate::memory::SPI4_BASE => Ok(SpiIdentifies::Spi4),
            #[cfg(feature = "spi5")]
            crate::memory::SPI5_BASE => Ok(SpiIdentifies::Spi5),
            #[cfg(feature = "spi6")]
            crate::memory::SPI6_BASE => Ok(SpiIdentifies::Spi6),
            _ => Err(Error::Param),
        }
    }
}

pub struct Spi
{
    spi: SpiIdentifies,
}

impl Spi
{
    pub fn new(spi: SpiIdentifies) -> Self
    {
        Spi { spi }
    }
}

impl SpiDevice for Spi
{
    fn with_event(&mut self, handle: SpiEventHandle)
    {
        event::EventCenter::set(self.spi, handle)
    }

    fn send(&self, data: &[u8], timeout: u32) -> Result<()>
    {
        unsafe { HAL_SPI_Transmit(self.spi.into(), data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<()>
    {
        unsafe { HAL_SPI_Receive(self.spi.into(), data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn send_receive(&self, tx_data: &[u8], rx_data: &mut [u8], timeout: u32) -> Result<()>
    {
        unsafe { HAL_SPI_TransmitReceive(self.spi.into(), tx_data.as_ptr(), rx_data.as_ptr(), tx_data.len() as u16, timeout).into() }
    }

    fn send_with_interrupt(&self, data: &[u8]) -> Result<()>
    {
        unsafe { HAL_SPI_Transmit_IT(self.spi.into(), data.as_ptr(), data.len() as u16).into() }
    }

    fn receive_with_interrupt(&self, data: &mut [u8]) -> Result<()>
    {
        unsafe { HAL_SPI_Receive_IT(self.spi.into(), data.as_ptr(), data.len() as u16).into() }
    }

    fn send_receive_with_interrupt(&self, tx_data: &[u8], rx_data: &mut [u8]) -> Result<()>
    {
        unsafe { HAL_SPI_TransmitReceive_IT(self.spi.into(), tx_data.as_ptr(), rx_data.as_ptr(), tx_data.len() as u16).into() }
    }

    fn send_with_dma(&self, data: &[u8]) -> Result<()>
    {
        unsafe { HAL_SPI_Transmit_DMA(self.spi.into(), data.as_ptr(), data.len() as u16).into() }
    }

    fn receive_with_dma(&self, data: &mut [u8]) -> Result<()>
    {
        unsafe { HAL_SPI_Receive_DMA(self.spi.into(), data.as_ptr(), data.len() as u16).into() }
    }

    fn send_receive_with_dma(&self, tx_data: &[u8], rx_data: &mut [u8]) -> Result<()>
    {
        unsafe { HAL_SPI_TransmitReceive_DMA(self.spi.into(), tx_data.as_ptr(), rx_data.as_ptr(), tx_data.len() as u16).into() }
    }

    fn abort(&self) -> Result<()>
    {
        unsafe { HAL_SPI_Abort_IT(self.spi.into()).into() }
    }
}

mod event
{
    use escw_mcu::peripheral::spi::SpiEvent;
    use escw_mcu::peripheral::spi::SpiEventHandle;

    use crate::hal::spi::*;

    use super::SpiIdentifies;

    static mut EVENT_CENTER: EventCenter = EventCenter::new();

    pub struct EventCenter
    {
        handle: [Option<SpiEventHandle>; SpiIdentifies::count()],
    }

    impl EventCenter
    {
        const fn new() -> Self
        {
            EventCenter {
                handle: [None; SpiIdentifies::count()],
            }
        }

        pub fn set(spi: SpiIdentifies, invoke: SpiEventHandle)
        {
            unsafe {
                EVENT_CENTER.handle[spi as usize] = Some(invoke);
            }
        }

        pub fn invoke(spi: SpiIdentifies, event: SpiEvent)
        {
            unsafe {
                if let Some(invoke) = EVENT_CENTER.handle[spi as usize].as_ref() {
                    invoke(event);
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_SPI_TxCpltCallback(hspi: &Hspi)
    {
        if let Some(spi) = hspi.try_into().ok() {
            EventCenter::invoke(spi, SpiEvent::TxCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_SPI_RxCpltCallback(hspi: &Hspi)
    {
        if let Some(spi) = hspi.try_into().ok() {
            EventCenter::invoke(spi, SpiEvent::RxCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_SPI_TxRxCpltCallback(hspi: &Hspi)
    {
        if let Some(spi) = hspi.try_into().ok() {
            EventCenter::invoke(spi, SpiEvent::TxRxCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_SPI_TxHalfCpltCallback(hspi: &Hspi)
    {
        if let Some(spi) = hspi.try_into().ok() {
            EventCenter::invoke(spi, SpiEvent::TxHalf);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_SPI_RxHalfCpltCallback(hspi: &Hspi)
    {
        if let Some(spi) = hspi.try_into().ok() {
            EventCenter::invoke(spi, SpiEvent::RxHalf);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_SPI_TxRxHalfCpltCallback(hspi: &Hspi)
    {
        if let Some(spi) = hspi.try_into().ok() {
            EventCenter::invoke(spi, SpiEvent::TxRxHalf);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_SPI_ErrorCallback(hspi: &Hspi)
    {
        if let Some(spi) = hspi.try_into().ok() {
            EventCenter::invoke(spi, SpiEvent::Error);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_SPI_AbortCpltCallback(hspi: &Hspi)
    {
        if let Some(spi) = hspi.try_into().ok() {
            EventCenter::invoke(spi, SpiEvent::TxRxAborted);
        }
    }
}
