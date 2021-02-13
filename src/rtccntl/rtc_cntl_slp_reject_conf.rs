#[doc = "Reader of register RTC_CNTL_SLP_REJECT_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_SLP_REJECT_CONF>;
#[doc = "Writer for register RTC_CNTL_SLP_REJECT_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_SLP_REJECT_CONF>;
#[doc = "Register RTC_CNTL_SLP_REJECT_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_SLP_REJECT_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DEEP_SLP_REJECT_EN`"]
pub type RTC_CNTL_DEEP_SLP_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DEEP_SLP_REJECT_EN`"]
pub struct RTC_CNTL_DEEP_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEEP_SLP_REJECT_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_LIGHT_SLP_REJECT_EN`"]
pub type RTC_CNTL_LIGHT_SLP_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_LIGHT_SLP_REJECT_EN`"]
pub struct RTC_CNTL_LIGHT_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_LIGHT_SLP_REJECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SLEEP_REJECT_ENA`"]
pub type RTC_CNTL_SLEEP_REJECT_ENA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_CNTL_SLEEP_REJECT_ENA`"]
pub struct RTC_CNTL_SLEEP_REJECT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLEEP_REJECT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 12)) | (((value as u32) & 0x0003_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_deep_slp_reject_en(&self) -> RTC_CNTL_DEEP_SLP_REJECT_EN_R {
        RTC_CNTL_DEEP_SLP_REJECT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_light_slp_reject_en(&self) -> RTC_CNTL_LIGHT_SLP_REJECT_EN_R {
        RTC_CNTL_LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 12:29"]
    #[inline(always)]
    pub fn rtc_cntl_sleep_reject_ena(&self) -> RTC_CNTL_SLEEP_REJECT_ENA_R {
        RTC_CNTL_SLEEP_REJECT_ENA_R::new(((self.bits >> 12) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_deep_slp_reject_en(&mut self) -> RTC_CNTL_DEEP_SLP_REJECT_EN_W {
        RTC_CNTL_DEEP_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_light_slp_reject_en(&mut self) -> RTC_CNTL_LIGHT_SLP_REJECT_EN_W {
        RTC_CNTL_LIGHT_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bits 12:29"]
    #[inline(always)]
    pub fn rtc_cntl_sleep_reject_ena(&mut self) -> RTC_CNTL_SLEEP_REJECT_ENA_W {
        RTC_CNTL_SLEEP_REJECT_ENA_W { w: self }
    }
}
