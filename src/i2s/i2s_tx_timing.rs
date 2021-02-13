#[doc = "Reader of register I2S_TX_TIMING"]
pub type R = crate::R<u32, super::I2S_TX_TIMING>;
#[doc = "Writer for register I2S_TX_TIMING"]
pub type W = crate::W<u32, super::I2S_TX_TIMING>;
#[doc = "Register I2S_TX_TIMING `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_TX_TIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_TX_BCK_IN_DM`"]
pub type I2S_TX_BCK_IN_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_BCK_IN_DM`"]
pub struct I2S_TX_BCK_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_BCK_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_WS_IN_DM`"]
pub type I2S_TX_WS_IN_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_WS_IN_DM`"]
pub struct I2S_TX_WS_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_WS_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_BCK_OUT_DM`"]
pub type I2S_TX_BCK_OUT_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_BCK_OUT_DM`"]
pub struct I2S_TX_BCK_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_BCK_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_WS_OUT_DM`"]
pub type I2S_TX_WS_OUT_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_WS_OUT_DM`"]
pub struct I2S_TX_WS_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_WS_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_SD1_OUT_DM`"]
pub type I2S_TX_SD1_OUT_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_SD1_OUT_DM`"]
pub struct I2S_TX_SD1_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_SD1_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_SD_OUT_DM`"]
pub type I2S_TX_SD_OUT_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_SD_OUT_DM`"]
pub struct I2S_TX_SD_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_SD_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn i2s_tx_bck_in_dm(&self) -> I2S_TX_BCK_IN_DM_R {
        I2S_TX_BCK_IN_DM_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn i2s_tx_ws_in_dm(&self) -> I2S_TX_WS_IN_DM_R {
        I2S_TX_WS_IN_DM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn i2s_tx_bck_out_dm(&self) -> I2S_TX_BCK_OUT_DM_R {
        I2S_TX_BCK_OUT_DM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2s_tx_ws_out_dm(&self) -> I2S_TX_WS_OUT_DM_R {
        I2S_TX_WS_OUT_DM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn i2s_tx_sd1_out_dm(&self) -> I2S_TX_SD1_OUT_DM_R {
        I2S_TX_SD1_OUT_DM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn i2s_tx_sd_out_dm(&self) -> I2S_TX_SD_OUT_DM_R {
        I2S_TX_SD_OUT_DM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn i2s_tx_bck_in_dm(&mut self) -> I2S_TX_BCK_IN_DM_W {
        I2S_TX_BCK_IN_DM_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn i2s_tx_ws_in_dm(&mut self) -> I2S_TX_WS_IN_DM_W {
        I2S_TX_WS_IN_DM_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn i2s_tx_bck_out_dm(&mut self) -> I2S_TX_BCK_OUT_DM_W {
        I2S_TX_BCK_OUT_DM_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2s_tx_ws_out_dm(&mut self) -> I2S_TX_WS_OUT_DM_W {
        I2S_TX_WS_OUT_DM_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn i2s_tx_sd1_out_dm(&mut self) -> I2S_TX_SD1_OUT_DM_W {
        I2S_TX_SD1_OUT_DM_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn i2s_tx_sd_out_dm(&mut self) -> I2S_TX_SD_OUT_DM_W {
        I2S_TX_SD_OUT_DM_W { w: self }
    }
}
