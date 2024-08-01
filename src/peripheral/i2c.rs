use escw_mcu::common::Error;
use escw_mcu::common::Result;
use escw_mcu::peripheral::i2c::I2cEventHandle;
use escw_mcu::peripheral::i2c::I2cMasterDevice;
use escw_mcu::peripheral::i2c::I2cSlaveDevice;

use crate::hal::i2c::*;

#[derive(Clone, Copy)]
pub enum I2cIdentifies {
    #[cfg(feature = "i2c1")]
    I2c1,
    #[cfg(feature = "i2c2")]
    I2c2,
    #[cfg(feature = "i2c3")]
    I2c3,
}

impl I2cIdentifies {
    pub const fn count() -> usize {
        3
    }
}

impl Into<&Hi2c> for I2cIdentifies {
    fn into(self) -> &'static Hi2c {
        unsafe {
            match self {
                #[cfg(feature = "i2c1")]
                Self::I2c1 => &hi2c1,
                #[cfg(feature = "i2c2")]
                Self::I2c2 => &hi2c2,
                #[cfg(feature = "i2c3")]
                Self::I2c3 => &hi2c3,
            }
        }
    }
}

impl TryInto<I2cIdentifies> for &Hi2c {
    type Error = Error;

    fn try_into(self) -> core::result::Result<I2cIdentifies, Self::Error> {
        match self.instance {
            #[cfg(feature = "i2c1")]
            crate::memory::I2C1_BASE => Ok(I2cIdentifies::I2c1),
            #[cfg(feature = "i2c2")]
            crate::memory::I2C2_BASE => Ok(I2cIdentifies::I2c2),
            #[cfg(feature = "i2c3")]
            crate::memory::I2C3_BASE => Ok(I2cIdentifies::I2c3),
            _ => Err(Error::Param),
        }
    }
}

pub struct I2cMaster {
    i2c: I2cIdentifies,
}

impl I2cMasterDevice for I2cMaster {
    type Identifies = I2cIdentifies;

    fn new(i2c: Self::Identifies) -> Self {
        I2cMaster { i2c }
    }

    fn with_event(&mut self, handle: I2cEventHandle) {
        event::EventCenter::set(self.i2c, handle)
    }

    fn device_state(&self, device: u16, trails: u32, timeout: u32) -> Result<()> {
        unsafe { HAL_I2C_IsDeviceReady(self.i2c.into(), device, trails, timeout).into() }
    }

    fn send(&self, device: u16, data: &[u8], timeout: u32) -> Result<()> {
        unsafe {
            HAL_I2C_Master_Transmit(
                self.i2c.into(),
                device,
                data.as_ptr(),
                data.len() as u16,
                timeout,
            )
            .into()
        }
    }

    fn receive(&self, device: u16, data: &mut [u8], timeout: u32) -> Result<()> {
        unsafe {
            HAL_I2C_Master_Receive(
                self.i2c.into(),
                device,
                data.as_ptr(),
                data.len() as u16,
                timeout,
            )
            .into()
        }
    }

    fn memory_write(
        &self, device: u16, address: u16, wide: u16, data: &[u8], timeout: u32,
    ) -> Result<()> {
        unsafe {
            HAL_I2C_Mem_Write(
                self.i2c.into(),
                device,
                address,
                wide,
                data.as_ptr(),
                data.len() as u16,
                timeout,
            )
            .into()
        }
    }

    fn memory_read(
        &self, device: u16, address: u16, wide: u16, data: &mut [u8], timeout: u32,
    ) -> Result<()> {
        unsafe {
            HAL_I2C_Mem_Read(
                self.i2c.into(),
                device,
                address,
                wide,
                data.as_ptr(),
                data.len() as u16,
                timeout,
            )
            .into()
        }
    }

    fn send_with_interrupt(&self, device: u16, data: &[u8]) -> Result<()> {
        unsafe {
            HAL_I2C_Master_Transmit_IT(self.i2c.into(), device, data.as_ptr(), data.len() as u16)
                .into()
        }
    }

    fn receive_with_interrupt(&self, device: u16, data: &mut [u8]) -> Result<()> {
        unsafe {
            HAL_I2C_Master_Receive_IT(self.i2c.into(), device, data.as_ptr(), data.len() as u16)
                .into()
        }
    }

    fn memory_write_with_interrupt(
        &self, device: u16, address: u16, wide: u16, data: &[u8],
    ) -> Result<()> {
        unsafe {
            HAL_I2C_Mem_Write_IT(
                self.i2c.into(),
                device,
                address,
                wide,
                data.as_ptr(),
                data.len() as u16,
            )
            .into()
        }
    }

    fn memory_read_with_interrupt(
        &self, device: u16, address: u16, wide: u16, data: &mut [u8],
    ) -> Result<()> {
        unsafe {
            HAL_I2C_Mem_Read_IT(
                self.i2c.into(),
                device,
                address,
                wide,
                data.as_ptr(),
                data.len() as u16,
            )
            .into()
        }
    }

    fn send_with_dma(&self, device: u16, data: &[u8]) -> Result<()> {
        unsafe {
            HAL_I2C_Master_Transmit_DMA(self.i2c.into(), device, data.as_ptr(), data.len() as u16)
                .into()
        }
    }

    fn receive_with_dma(&self, device: u16, data: &mut [u8]) -> Result<()> {
        unsafe {
            HAL_I2C_Master_Receive_DMA(self.i2c.into(), device, data.as_ptr(), data.len() as u16)
                .into()
        }
    }

    fn memory_write_with_dma(
        &self, device: u16, address: u16, wide: u16, data: &[u8],
    ) -> Result<()> {
        unsafe {
            HAL_I2C_Mem_Write_DMA(
                self.i2c.into(),
                device,
                address,
                wide,
                data.as_ptr(),
                data.len() as u16,
            )
            .into()
        }
    }

    fn memory_read_with_dma(
        &self, device: u16, address: u16, wide: u16, data: &mut [u8],
    ) -> Result<()> {
        unsafe {
            HAL_I2C_Mem_Read_DMA(
                self.i2c.into(),
                device,
                address,
                wide,
                data.as_ptr(),
                data.len() as u16,
            )
            .into()
        }
    }

    fn abort(&self, device: u16) -> Result<()> {
        unsafe { HAL_I2C_Master_Abort_IT(self.i2c.into(), device).into() }
    }
}

pub struct I2cSlave {
    i2c: I2cIdentifies,
}

impl I2cSlaveDevice for I2cSlave {
    type Identifies = I2cIdentifies;

    fn new(i2c: Self::Identifies) -> Self {
        I2cSlave { i2c }
    }

    fn with_event(&mut self, handle: I2cEventHandle) {
        event::EventCenter::set(self.i2c, handle)
    }

    fn send(&self, data: &[u8], timeout: u32) -> Result<()> {
        unsafe {
            HAL_I2C_Slave_Transmit(self.i2c.into(), data.as_ptr(), data.len() as u16, timeout)
                .into()
        }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> Result<()> {
        unsafe {
            HAL_I2C_Slave_Receive(self.i2c.into(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn send_with_interrupt(&self, data: &[u8]) -> Result<()> {
        unsafe {
            HAL_I2C_Slave_Transmit_IT(self.i2c.into(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn receive_with_interrupt(&self, data: &mut [u8]) -> Result<()> {
        unsafe {
            HAL_I2C_Slave_Receive_IT(self.i2c.into(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn send_with_dma(&self, data: &[u8]) -> Result<()> {
        unsafe {
            HAL_I2C_Slave_Transmit_DMA(self.i2c.into(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn receive_with_dma(&self, data: &mut [u8]) -> Result<()> {
        unsafe {
            HAL_I2C_Slave_Receive_DMA(self.i2c.into(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn listen(&self) -> Result<()> {
        unsafe { HAL_I2C_EnableListen_IT(self.i2c.into()).into() }
    }
}

mod event {
    use super::I2cIdentifies;
    use crate::hal::i2c::Hi2c;
    use escw_mcu::peripheral::i2c::I2cDirection;
    use escw_mcu::peripheral::i2c::I2cEvent;
    use escw_mcu::peripheral::i2c::I2cEventHandle;

    static mut EVENT_CENTER: EventCenter = EventCenter::new();

    pub struct EventCenter {
        handle: [Option<I2cEventHandle>; I2cIdentifies::count()],
    }

    impl EventCenter {
        const fn new() -> Self {
            EventCenter {
                handle: [None; I2cIdentifies::count()],
            }
        }

        pub fn set(i2c: I2cIdentifies, invoke: I2cEventHandle) {
            unsafe {
                EVENT_CENTER.handle[i2c as usize] = Some(invoke);
            }
        }

        pub fn invoke(i2c: I2cIdentifies, event: I2cEvent) {
            unsafe {
                if let Some(invoke) = EVENT_CENTER.handle[i2c as usize].as_ref() {
                    invoke(event);
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_I2C_MasterTxCpltCallback(hi2c: &Hi2c) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(i2c, I2cEvent::TxCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_I2C_MasterRxCpltCallback(hi2c: &Hi2c) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(i2c, I2cEvent::RxCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_I2C_SlaveTxCpltCallback(hi2c: &Hi2c) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(i2c, I2cEvent::TxCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_I2C_SlaveRxCpltCallback(hi2c: &Hi2c) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(i2c, I2cEvent::RxCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_I2C_AddrCallback(
        hi2c: &Hi2c, transfer_direction: u8, addr_match_codee: u16,
    ) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(
                i2c,
                I2cEvent::Awakened((
                    if transfer_direction == 0 {
                        I2cDirection::Rx
                    } else {
                        I2cDirection::Tx
                    },
                    addr_match_codee,
                )),
            );
        }
    }

    // #[no_mangle]
    // pub extern "C" fn HAL_I2C_ListenCpltCallback(hi2c: &Hi2c) {}

    #[no_mangle]
    pub extern "C" fn HAL_I2C_MemTxCpltCallback(hi2c: &Hi2c) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(i2c, I2cEvent::MemoryWriteCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_I2C_MemRxCpltCallback(hi2c: &Hi2c) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(i2c, I2cEvent::MemoryReadCompleted);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_I2C_ErrorCallback(hi2c: &Hi2c) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(i2c, I2cEvent::Error);
        }
    }

    #[no_mangle]
    pub extern "C" fn HAL_I2C_AbortCpltCallback(hi2c: &Hi2c) {
        if let Some(i2c) = hi2c.try_into().ok() {
            EventCenter::invoke(i2c, I2cEvent::TxRxAborted);
        }
    }
}
