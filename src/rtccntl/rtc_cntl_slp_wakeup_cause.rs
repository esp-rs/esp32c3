#[doc = "Reader of register RTC_CNTL_SLP_WAKEUP_CAUSE"]
pub type R = crate::R<u32, super::RTC_CNTL_SLP_WAKEUP_CAUSE>;
#[doc = "Reader of field `RTC_CNTL_WAKEUP_CAUSE`"]
pub type RTC_CNTL_WAKEUP_CAUSE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn rtc_cntl_wakeup_cause(&self) -> RTC_CNTL_WAKEUP_CAUSE_R {
        RTC_CNTL_WAKEUP_CAUSE_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
