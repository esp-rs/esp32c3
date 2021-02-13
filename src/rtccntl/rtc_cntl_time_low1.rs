#[doc = "Reader of register RTC_CNTL_TIME_LOW1"]
pub type R = crate::R<u32, super::RTC_CNTL_TIME_LOW1>;
#[doc = "Reader of field `RTC_CNTL_TIMER_VALUE1_LOW`"]
pub type RTC_CNTL_TIMER_VALUE1_LOW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_cntl_timer_value1_low(&self) -> RTC_CNTL_TIMER_VALUE1_LOW_R {
        RTC_CNTL_TIMER_VALUE1_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
