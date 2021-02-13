#[doc = "Reader of register I2S_TX_CONF1"]
pub type R = crate::R<u32, super::I2S_TX_CONF1>;
#[doc = "Writer for register I2S_TX_CONF1"]
pub type W = crate::W<u32, super::I2S_TX_CONF1>;
#[doc = "Register I2S_TX_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_TX_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_TX_MSB_SHIFT`"]
pub type I2S_TX_MSB_SHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_MSB_SHIFT`"]
pub struct I2S_TX_MSB_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_MSB_SHIFT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_TDM_CHAN_BITS`"]
pub type I2S_TX_TDM_CHAN_BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_TDM_CHAN_BITS`"]
pub struct I2S_TX_TDM_CHAN_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_TDM_CHAN_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_HALF_SAMPLE_BITS`"]
pub type I2S_TX_HALF_SAMPLE_BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_HALF_SAMPLE_BITS`"]
pub struct I2S_TX_HALF_SAMPLE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_HALF_SAMPLE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_BITS_MOD`"]
pub type I2S_TX_BITS_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_BITS_MOD`"]
pub struct I2S_TX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | (((value as u32) & 0x1f) << 13);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_BCK_DIV_NUM`"]
pub type I2S_TX_BCK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_BCK_DIV_NUM`"]
pub struct I2S_TX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | (((value as u32) & 0x3f) << 7);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_TDM_WS_WIDTH`"]
pub type I2S_TX_TDM_WS_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_TDM_WS_WIDTH`"]
pub struct I2S_TX_TDM_WS_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_TDM_WS_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn i2s_tx_msb_shift(&self) -> I2S_TX_MSB_SHIFT_R {
        I2S_TX_MSB_SHIFT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn i2s_tx_tdm_chan_bits(&self) -> I2S_TX_TDM_CHAN_BITS_R {
        I2S_TX_TDM_CHAN_BITS_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn i2s_tx_half_sample_bits(&self) -> I2S_TX_HALF_SAMPLE_BITS_R {
        I2S_TX_HALF_SAMPLE_BITS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn i2s_tx_bits_mod(&self) -> I2S_TX_BITS_MOD_R {
        I2S_TX_BITS_MOD_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn i2s_tx_bck_div_num(&self) -> I2S_TX_BCK_DIV_NUM_R {
        I2S_TX_BCK_DIV_NUM_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn i2s_tx_tdm_ws_width(&self) -> I2S_TX_TDM_WS_WIDTH_R {
        I2S_TX_TDM_WS_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn i2s_tx_msb_shift(&mut self) -> I2S_TX_MSB_SHIFT_W {
        I2S_TX_MSB_SHIFT_W { w: self }
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn i2s_tx_tdm_chan_bits(&mut self) -> I2S_TX_TDM_CHAN_BITS_W {
        I2S_TX_TDM_CHAN_BITS_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn i2s_tx_half_sample_bits(&mut self) -> I2S_TX_HALF_SAMPLE_BITS_W {
        I2S_TX_HALF_SAMPLE_BITS_W { w: self }
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn i2s_tx_bits_mod(&mut self) -> I2S_TX_BITS_MOD_W {
        I2S_TX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn i2s_tx_bck_div_num(&mut self) -> I2S_TX_BCK_DIV_NUM_W {
        I2S_TX_BCK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn i2s_tx_tdm_ws_width(&mut self) -> I2S_TX_TDM_WS_WIDTH_W {
        I2S_TX_TDM_WS_WIDTH_W { w: self }
    }
}
