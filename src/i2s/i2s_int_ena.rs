#[doc = "Reader of register I2S_INT_ENA"]
pub type R = crate::R<u32, super::I2S_INT_ENA>;
#[doc = "Writer for register I2S_INT_ENA"]
pub type W = crate::W<u32, super::I2S_INT_ENA>;
#[doc = "Register I2S_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_TX_HUNG_INT_ENA`"]
pub type I2S_TX_HUNG_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_HUNG_INT_ENA`"]
pub struct I2S_TX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Reader of field `I2S_RX_HUNG_INT_ENA`"]
pub type I2S_RX_HUNG_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_HUNG_INT_ENA`"]
pub struct I2S_RX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Reader of field `I2S_TX_DONE_INT_ENA`"]
pub type I2S_TX_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_DONE_INT_ENA`"]
pub struct I2S_TX_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `I2S_RX_DONE_INT_ENA`"]
pub type I2S_RX_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_DONE_INT_ENA`"]
pub struct I2S_RX_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_DONE_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_tx_hung_int_ena(&self) -> I2S_TX_HUNG_INT_ENA_R {
        I2S_TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_rx_hung_int_ena(&self) -> I2S_RX_HUNG_INT_ENA_R {
        I2S_RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_tx_done_int_ena(&self) -> I2S_TX_DONE_INT_ENA_R {
        I2S_TX_DONE_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_rx_done_int_ena(&self) -> I2S_RX_DONE_INT_ENA_R {
        I2S_RX_DONE_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_tx_hung_int_ena(&mut self) -> I2S_TX_HUNG_INT_ENA_W {
        I2S_TX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_rx_hung_int_ena(&mut self) -> I2S_RX_HUNG_INT_ENA_W {
        I2S_RX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_tx_done_int_ena(&mut self) -> I2S_TX_DONE_INT_ENA_W {
        I2S_TX_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_rx_done_int_ena(&mut self) -> I2S_RX_DONE_INT_ENA_W {
        I2S_RX_DONE_INT_ENA_W { w: self }
    }
}
