#[doc = "Reader of register I2S_STATE"]
pub type R = crate::R<u32, super::I2S_STATE>;
#[doc = "Reader of field `I2S_TX_IDLE`"]
pub type I2S_TX_IDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_tx_idle(&self) -> I2S_TX_IDLE_R {
        I2S_TX_IDLE_R::new((self.bits & 0x01) != 0)
    }
}
