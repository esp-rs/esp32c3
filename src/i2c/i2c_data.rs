#[doc = "Reader of register I2C_DATA"]
pub type R = crate::R<u32, super::I2C_DATA>;
#[doc = "Reader of field `I2C_FIFO_RDATA`"]
pub type I2C_FIFO_RDATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&self) -> I2C_FIFO_RDATA_R {
        I2C_FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
