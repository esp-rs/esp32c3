#[doc = "Reader of register RTC_CNTL_WAKEUP_STATE"]
pub type R = crate::R<u32, super::RTC_CNTL_WAKEUP_STATE>;
#[doc = "Writer for register RTC_CNTL_WAKEUP_STATE"]
pub type W = crate::W<u32, super::RTC_CNTL_WAKEUP_STATE>;
#[doc = "Register RTC_CNTL_WAKEUP_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_WAKEUP_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_WAKEUP_ENA`"]
pub type RTC_CNTL_WAKEUP_ENA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_CNTL_WAKEUP_ENA`"]
pub struct RTC_CNTL_WAKEUP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WAKEUP_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 15)) | (((value as u32) & 0x0001_ffff) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rtc_cntl_wakeup_ena(&self) -> RTC_CNTL_WAKEUP_ENA_R {
        RTC_CNTL_WAKEUP_ENA_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rtc_cntl_wakeup_ena(&mut self) -> RTC_CNTL_WAKEUP_ENA_W {
        RTC_CNTL_WAKEUP_ENA_W { w: self }
    }
}
