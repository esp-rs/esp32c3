#[doc = "Reader of register RTC_CNTL_SENSOR_CTRL"]
pub type R = crate::R<u32, super::RTC_CNTL_SENSOR_CTRL>;
#[doc = "Writer for register RTC_CNTL_SENSOR_CTRL"]
pub type W = crate::W<u32, super::RTC_CNTL_SENSOR_CTRL>;
#[doc = "Register RTC_CNTL_SENSOR_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_SENSOR_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_FORCE_XPD_SAR`"]
pub type RTC_CNTL_FORCE_XPD_SAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_FORCE_XPD_SAR`"]
pub struct RTC_CNTL_FORCE_XPD_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FORCE_XPD_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SAR2_PWDET_CCT`"]
pub type RTC_CNTL_SAR2_PWDET_CCT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SAR2_PWDET_CCT`"]
pub struct RTC_CNTL_SAR2_PWDET_CCT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SAR2_PWDET_CCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rtc_cntl_force_xpd_sar(&self) -> RTC_CNTL_FORCE_XPD_SAR_R {
        RTC_CNTL_FORCE_XPD_SAR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn rtc_cntl_sar2_pwdet_cct(&self) -> RTC_CNTL_SAR2_PWDET_CCT_R {
        RTC_CNTL_SAR2_PWDET_CCT_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rtc_cntl_force_xpd_sar(&mut self) -> RTC_CNTL_FORCE_XPD_SAR_W {
        RTC_CNTL_FORCE_XPD_SAR_W { w: self }
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn rtc_cntl_sar2_pwdet_cct(&mut self) -> RTC_CNTL_SAR2_PWDET_CCT_W {
        RTC_CNTL_SAR2_PWDET_CCT_W { w: self }
    }
}
