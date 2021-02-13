#[doc = "Reader of register RTC_CNTL_DIAG0"]
pub type R = crate::R<u32, super::RTC_CNTL_DIAG0>;
#[doc = "Reader of field `RTC_CNTL_LOW_POWER_DIAG1`"]
pub type RTC_CNTL_LOW_POWER_DIAG1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_cntl_low_power_diag1(&self) -> RTC_CNTL_LOW_POWER_DIAG1_R {
        RTC_CNTL_LOW_POWER_DIAG1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
