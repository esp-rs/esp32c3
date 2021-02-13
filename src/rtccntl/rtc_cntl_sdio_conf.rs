#[doc = "Reader of register RTC_CNTL_SDIO_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_SDIO_CONF>;
#[doc = "Writer for register RTC_CNTL_SDIO_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_SDIO_CONF>;
#[doc = "Register RTC_CNTL_SDIO_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_SDIO_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_XPD_SDIO_REG`"]
pub type RTC_CNTL_XPD_SDIO_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XPD_SDIO_REG`"]
pub struct RTC_CNTL_XPD_SDIO_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XPD_SDIO_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DREFH_SDIO`"]
pub type RTC_CNTL_DREFH_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DREFH_SDIO`"]
pub struct RTC_CNTL_DREFH_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DREFH_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DREFM_SDIO`"]
pub type RTC_CNTL_DREFM_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DREFM_SDIO`"]
pub struct RTC_CNTL_DREFM_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DREFM_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DREFL_SDIO`"]
pub type RTC_CNTL_DREFL_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DREFL_SDIO`"]
pub struct RTC_CNTL_DREFL_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DREFL_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_REG1P8_READY`"]
pub type RTC_CNTL_REG1P8_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_SDIO_TIEH`"]
pub type RTC_CNTL_SDIO_TIEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_TIEH`"]
pub struct RTC_CNTL_SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_TIEH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_FORCE`"]
pub type RTC_CNTL_SDIO_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_FORCE`"]
pub struct RTC_CNTL_SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_PD_EN`"]
pub type RTC_CNTL_SDIO_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_PD_EN`"]
pub struct RTC_CNTL_SDIO_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_PD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_ENCURLIM`"]
pub type RTC_CNTL_SDIO_ENCURLIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_ENCURLIM`"]
pub struct RTC_CNTL_SDIO_ENCURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_ENCURLIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_MODECURLIM`"]
pub type RTC_CNTL_SDIO_MODECURLIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_MODECURLIM`"]
pub struct RTC_CNTL_SDIO_MODECURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_MODECURLIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_DCURLIM`"]
pub type RTC_CNTL_SDIO_DCURLIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_DCURLIM`"]
pub struct RTC_CNTL_SDIO_DCURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_DCURLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_EN_INITI`"]
pub type RTC_CNTL_SDIO_EN_INITI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_EN_INITI`"]
pub struct RTC_CNTL_SDIO_EN_INITI_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_EN_INITI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_INITI`"]
pub type RTC_CNTL_SDIO_INITI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_INITI`"]
pub struct RTC_CNTL_SDIO_INITI_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_INITI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_DCAP`"]
pub type RTC_CNTL_SDIO_DCAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_DCAP`"]
pub struct RTC_CNTL_SDIO_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_DTHDRV`"]
pub type RTC_CNTL_SDIO_DTHDRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_DTHDRV`"]
pub struct RTC_CNTL_SDIO_DTHDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_DTHDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SDIO_TIMER_TARGET`"]
pub type RTC_CNTL_SDIO_TIMER_TARGET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_TIMER_TARGET`"]
pub struct RTC_CNTL_SDIO_TIMER_TARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_TIMER_TARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_sdio_reg(&self) -> RTC_CNTL_XPD_SDIO_REG_R {
        RTC_CNTL_XPD_SDIO_REG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn rtc_cntl_drefh_sdio(&self) -> RTC_CNTL_DREFH_SDIO_R {
        RTC_CNTL_DREFH_SDIO_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn rtc_cntl_drefm_sdio(&self) -> RTC_CNTL_DREFM_SDIO_R {
        RTC_CNTL_DREFM_SDIO_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn rtc_cntl_drefl_sdio(&self) -> RTC_CNTL_DREFL_SDIO_R {
        RTC_CNTL_DREFL_SDIO_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_reg1p8_ready(&self) -> RTC_CNTL_REG1P8_READY_R {
        RTC_CNTL_REG1P8_READY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_tieh(&self) -> RTC_CNTL_SDIO_TIEH_R {
        RTC_CNTL_SDIO_TIEH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_force(&self) -> RTC_CNTL_SDIO_FORCE_R {
        RTC_CNTL_SDIO_FORCE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_pd_en(&self) -> RTC_CNTL_SDIO_PD_EN_R {
        RTC_CNTL_SDIO_PD_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_encurlim(&self) -> RTC_CNTL_SDIO_ENCURLIM_R {
        RTC_CNTL_SDIO_ENCURLIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_modecurlim(&self) -> RTC_CNTL_SDIO_MODECURLIM_R {
        RTC_CNTL_SDIO_MODECURLIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_dcurlim(&self) -> RTC_CNTL_SDIO_DCURLIM_R {
        RTC_CNTL_SDIO_DCURLIM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_en_initi(&self) -> RTC_CNTL_SDIO_EN_INITI_R {
        RTC_CNTL_SDIO_EN_INITI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_initi(&self) -> RTC_CNTL_SDIO_INITI_R {
        RTC_CNTL_SDIO_INITI_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_dcap(&self) -> RTC_CNTL_SDIO_DCAP_R {
        RTC_CNTL_SDIO_DCAP_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_dthdrv(&self) -> RTC_CNTL_SDIO_DTHDRV_R {
        RTC_CNTL_SDIO_DTHDRV_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_timer_target(&self) -> RTC_CNTL_SDIO_TIMER_TARGET_R {
        RTC_CNTL_SDIO_TIMER_TARGET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_sdio_reg(&mut self) -> RTC_CNTL_XPD_SDIO_REG_W {
        RTC_CNTL_XPD_SDIO_REG_W { w: self }
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn rtc_cntl_drefh_sdio(&mut self) -> RTC_CNTL_DREFH_SDIO_W {
        RTC_CNTL_DREFH_SDIO_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn rtc_cntl_drefm_sdio(&mut self) -> RTC_CNTL_DREFM_SDIO_W {
        RTC_CNTL_DREFM_SDIO_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn rtc_cntl_drefl_sdio(&mut self) -> RTC_CNTL_DREFL_SDIO_W {
        RTC_CNTL_DREFL_SDIO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_tieh(&mut self) -> RTC_CNTL_SDIO_TIEH_W {
        RTC_CNTL_SDIO_TIEH_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_force(&mut self) -> RTC_CNTL_SDIO_FORCE_W {
        RTC_CNTL_SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_pd_en(&mut self) -> RTC_CNTL_SDIO_PD_EN_W {
        RTC_CNTL_SDIO_PD_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_encurlim(&mut self) -> RTC_CNTL_SDIO_ENCURLIM_W {
        RTC_CNTL_SDIO_ENCURLIM_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_modecurlim(&mut self) -> RTC_CNTL_SDIO_MODECURLIM_W {
        RTC_CNTL_SDIO_MODECURLIM_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_dcurlim(&mut self) -> RTC_CNTL_SDIO_DCURLIM_W {
        RTC_CNTL_SDIO_DCURLIM_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_en_initi(&mut self) -> RTC_CNTL_SDIO_EN_INITI_W {
        RTC_CNTL_SDIO_EN_INITI_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_initi(&mut self) -> RTC_CNTL_SDIO_INITI_W {
        RTC_CNTL_SDIO_INITI_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_dcap(&mut self) -> RTC_CNTL_SDIO_DCAP_W {
        RTC_CNTL_SDIO_DCAP_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_dthdrv(&mut self) -> RTC_CNTL_SDIO_DTHDRV_W {
        RTC_CNTL_SDIO_DTHDRV_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_timer_target(&mut self) -> RTC_CNTL_SDIO_TIMER_TARGET_W {
        RTC_CNTL_SDIO_TIMER_TARGET_W { w: self }
    }
}
