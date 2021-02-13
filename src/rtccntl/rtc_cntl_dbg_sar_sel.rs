#[doc = "Reader of register RTC_CNTL_DBG_SAR_SEL"]
pub type R = crate::R<u32, super::RTC_CNTL_DBG_SAR_SEL>;
#[doc = "Writer for register RTC_CNTL_DBG_SAR_SEL"]
pub type W = crate::W<u32, super::RTC_CNTL_DBG_SAR_SEL>;
#[doc = "Register RTC_CNTL_DBG_SAR_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_DBG_SAR_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_SAR_DEBUG_SEL`"]
pub type RTC_CNTL_SAR_DEBUG_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SAR_DEBUG_SEL`"]
pub struct RTC_CNTL_SAR_DEBUG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SAR_DEBUG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn rtc_cntl_sar_debug_sel(&self) -> RTC_CNTL_SAR_DEBUG_SEL_R {
        RTC_CNTL_SAR_DEBUG_SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn rtc_cntl_sar_debug_sel(&mut self) -> RTC_CNTL_SAR_DEBUG_SEL_W {
        RTC_CNTL_SAR_DEBUG_SEL_W { w: self }
    }
}
