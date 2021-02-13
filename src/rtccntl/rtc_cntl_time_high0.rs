#[doc = "Reader of register RTC_CNTL_TIME_HIGH0"]
pub type R = crate::R<u32, super::RTC_CNTL_TIME_HIGH0>;
#[doc = "Reader of field `RTC_CNTL_TIMER_VALUE0_HIGH`"]
pub type RTC_CNTL_TIMER_VALUE0_HIGH_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rtc_cntl_timer_value0_high(&self) -> RTC_CNTL_TIMER_VALUE0_HIGH_R {
        RTC_CNTL_TIMER_VALUE0_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
