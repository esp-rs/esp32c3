#[doc = "Writer for register I2S_INT_CLR"]
pub type W = crate::W<u32, super::I2S_INT_CLR>;
#[doc = "Register I2S_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `I2S_TX_HUNG_INT_CLR`"]
pub struct I2S_TX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_HUNG_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `I2S_RX_HUNG_INT_CLR`"]
pub struct I2S_RX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_HUNG_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `I2S_TX_DONE_INT_CLR`"]
pub struct I2S_TX_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_DONE_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `I2S_RX_DONE_INT_CLR`"]
pub struct I2S_RX_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_DONE_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_tx_hung_int_clr(&mut self) -> I2S_TX_HUNG_INT_CLR_W {
        I2S_TX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_rx_hung_int_clr(&mut self) -> I2S_RX_HUNG_INT_CLR_W {
        I2S_RX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_tx_done_int_clr(&mut self) -> I2S_TX_DONE_INT_CLR_W {
        I2S_TX_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_rx_done_int_clr(&mut self) -> I2S_RX_DONE_INT_CLR_W {
        I2S_RX_DONE_INT_CLR_W { w: self }
    }
}
