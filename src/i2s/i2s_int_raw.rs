#[doc = "Reader of register I2S_INT_RAW"]
pub type R = crate::R<u32, super::I2S_INT_RAW>;
#[doc = "Reader of field `I2S_TX_HUNG_INT_RAW`"]
pub type I2S_TX_HUNG_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2S_RX_HUNG_INT_RAW`"]
pub type I2S_RX_HUNG_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2S_TX_DONE_INT_RAW`"]
pub type I2S_TX_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2S_RX_DONE_INT_RAW`"]
pub type I2S_RX_DONE_INT_RAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_tx_hung_int_raw(&self) -> I2S_TX_HUNG_INT_RAW_R {
        I2S_TX_HUNG_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_rx_hung_int_raw(&self) -> I2S_RX_HUNG_INT_RAW_R {
        I2S_RX_HUNG_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_tx_done_int_raw(&self) -> I2S_TX_DONE_INT_RAW_R {
        I2S_TX_DONE_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_rx_done_int_raw(&self) -> I2S_RX_DONE_INT_RAW_R {
        I2S_RX_DONE_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
