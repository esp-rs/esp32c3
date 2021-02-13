#[doc = "Reader of register RTC_CNTL_ULP_CP_TIMER_1"]
pub type R = crate::R<u32, super::RTC_CNTL_ULP_CP_TIMER_1>;
#[doc = "Writer for register RTC_CNTL_ULP_CP_TIMER_1"]
pub type W = crate::W<u32, super::RTC_CNTL_ULP_CP_TIMER_1>;
#[doc = "Register RTC_CNTL_ULP_CP_TIMER_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_ULP_CP_TIMER_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE`"]
pub type RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE`"]
pub struct RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_timer_slp_cycle(&self) -> RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE_R {
        RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_timer_slp_cycle(&mut self) -> RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE_W {
        RTC_CNTL_ULP_CP_TIMER_SLP_CYCLE_W { w: self }
    }
}
