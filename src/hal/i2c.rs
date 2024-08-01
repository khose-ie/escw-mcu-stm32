use super::HalStatus;

#[repr(C)]
pub struct Hi2c {
    pub instance: u32,
}

extern "C" {
    #[cfg(feature = "i2c1")]
    pub static hi2c1: Hi2c;
    #[cfg(feature = "i2c2")]
    pub static hi2c2: Hi2c;
    #[cfg(feature = "i2c3")]
    pub static hi2c3: Hi2c;
}

extern "C" {
    pub fn HAL_I2C_Master_Transmit(
        hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16, Timeout: u32,
    ) -> HalStatus;
    pub fn HAL_I2C_Master_Receive(hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16, Timeout: u32)
        -> HalStatus;
    pub fn HAL_I2C_Slave_Transmit(hi2c: &Hi2c, pData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_I2C_Slave_Receive(hi2c: &Hi2c, pData: *const u8, Size: u16, Timeout: u32) -> HalStatus;
    pub fn HAL_I2C_Mem_Write(
        hi2c: &Hi2c, DevAddress: u16, MemAddress: u16, MemAddSize: u16, pData: *const u8, Size: u16, Timeout: u32,
    ) -> HalStatus;
    pub fn HAL_I2C_Mem_Read(
        hi2c: &Hi2c, DevAddress: u16, MemAddress: u16, MemAddSize: u16, pData: *const u8, Size: u16, Timeout: u32,
    ) -> HalStatus;
    pub fn HAL_I2C_IsDeviceReady(hi2c: &Hi2c, DevAddress: u16, Trials: u32, Timeout: u32) -> HalStatus;
    pub fn HAL_I2C_Master_Transmit_IT(hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_I2C_Master_Receive_IT(hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_I2C_Slave_Transmit_IT(hi2c: &Hi2c, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_I2C_Slave_Receive_IT(hi2c: &Hi2c, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_I2C_Mem_Write_IT(
        hi2c: &Hi2c, DevAddress: u16, MemAddress: u16, MemAddSize: u16, pData: *const u8, Size: u16,
    ) -> HalStatus;
    pub fn HAL_I2C_Mem_Read_IT(
        hi2c: &Hi2c, DevAddress: u16, MemAddress: u16, MemAddSize: u16, pData: *const u8, Size: u16,
    ) -> HalStatus;
    pub fn HAL_I2C_Master_Seq_Transmit_IT(
        hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16, XferOptions: u32,
    ) -> HalStatus;
    pub fn HAL_I2C_Master_Seq_Receive_IT(
        hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16, XferOptions: u32,
    ) -> HalStatus;
    pub fn HAL_I2C_Slave_Seq_Transmit_IT(hi2c: &Hi2c, pData: *const u8, Size: u16, XferOptions: u32) -> HalStatus;
    pub fn HAL_I2C_Slave_Seq_Receive_IT(hi2c: &Hi2c, pData: *const u8, Size: u16, XferOptions: u32) -> HalStatus;
    pub fn HAL_I2C_EnableListen_IT(hi2c: &Hi2c) -> HalStatus;
    pub fn HAL_I2C_DisableListen_IT(hi2c: &Hi2c) -> HalStatus;
    pub fn HAL_I2C_Master_Abort_IT(hi2c: &Hi2c, DevAddress: u16) -> HalStatus;
    pub fn HAL_I2C_Master_Transmit_DMA(hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_I2C_Master_Receive_DMA(hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_I2C_Slave_Transmit_DMA(hi2c: &Hi2c, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_I2C_Slave_Receive_DMA(hi2c: &Hi2c, pData: *const u8, Size: u16) -> HalStatus;
    pub fn HAL_I2C_Mem_Write_DMA(
        hi2c: &Hi2c, DevAddress: u16, MemAddress: u16, MemAddSize: u16, pData: *const u8, Size: u16,
    ) -> HalStatus;
    pub fn HAL_I2C_Mem_Read_DMA(
        hi2c: &Hi2c, DevAddress: u16, MemAddress: u16, MemAddSize: u16, pData: *const u8, Size: u16,
    ) -> HalStatus;
    pub fn HAL_I2C_Master_Seq_Transmit_DMA(
        hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16, XferOptions: u32,
    ) -> HalStatus;
    pub fn HAL_I2C_Master_Seq_Receive_DMA(
        hi2c: &Hi2c, DevAddress: u16, pData: *const u8, Size: u16, XferOptions: u32,
    ) -> HalStatus;
    pub fn HAL_I2C_Slave_Seq_Transmit_DMA(hi2c: &Hi2c, pData: *const u8, Size: u16, XferOptions: u32) -> HalStatus;
    pub fn HAL_I2C_Slave_Seq_Receive_DMA(hi2c: &Hi2c, pData: *const u8, Size: u16, XferOptions: u32) -> HalStatus;
    pub fn HAL_I2C_EV_IRQHandler(hi2c: &Hi2c);
    pub fn HAL_I2C_ER_IRQHandler(hi2c: &Hi2c);
    pub fn HAL_I2C_MasterTxCpltCallback(hi2c: &Hi2c);
    pub fn HAL_I2C_MasterRxCpltCallback(hi2c: &Hi2c);
    pub fn HAL_I2C_SlaveTxCpltCallback(hi2c: &Hi2c);
    pub fn HAL_I2C_SlaveRxCpltCallback(hi2c: &Hi2c);
    pub fn HAL_I2C_AddrCallback(hi2c: &Hi2c, TransferDirection: u8, AddrMatchCode: u16);
    pub fn HAL_I2C_ListenCpltCallback(hi2c: &Hi2c);
    pub fn HAL_I2C_MemTxCpltCallback(hi2c: &Hi2c);
    pub fn HAL_I2C_MemRxCpltCallback(hi2c: &Hi2c);
    pub fn HAL_I2C_ErrorCallback(hi2c: &Hi2c);
    pub fn HAL_I2C_AbortCpltCallback(hi2c: &Hi2c);
}
