#[doc = "Reader of register RTC_CNTL_INT_RAW"]
pub type R = crate::R<u32, super::RTC_CNTL_INT_RAW>;
#[doc = "Reader of field `RTC_CNTL_BBPLL_CAL_INT_RAW`"]
pub type RTC_CNTL_BBPLL_CAL_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_GLITCH_DET_INT_RAW`"]
pub type RTC_CNTL_GLITCH_DET_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_XTAL32K_DEAD_INT_RAW`"]
pub type RTC_CNTL_XTAL32K_DEAD_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_SWD_INT_RAW`"]
pub type RTC_CNTL_SWD_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_MAIN_TIMER_INT_RAW`"]
pub type RTC_CNTL_MAIN_TIMER_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_BROWN_OUT_INT_RAW`"]
pub type RTC_CNTL_BROWN_OUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_WDT_INT_RAW`"]
pub type RTC_CNTL_WDT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_SLP_REJECT_INT_RAW`"]
pub type RTC_CNTL_SLP_REJECT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_SLP_WAKEUP_INT_RAW`"]
pub type RTC_CNTL_SLP_WAKEUP_INT_RAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_cal_int_raw(&self) -> RTC_CNTL_BBPLL_CAL_INT_RAW_R {
        RTC_CNTL_BBPLL_CAL_INT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_glitch_det_int_raw(&self) -> RTC_CNTL_GLITCH_DET_INT_RAW_R {
        RTC_CNTL_GLITCH_DET_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_dead_int_raw(&self) -> RTC_CNTL_XTAL32K_DEAD_INT_RAW_R {
        RTC_CNTL_XTAL32K_DEAD_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_swd_int_raw(&self) -> RTC_CNTL_SWD_INT_RAW_R {
        RTC_CNTL_SWD_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtc_cntl_main_timer_int_raw(&self) -> RTC_CNTL_MAIN_TIMER_INT_RAW_R {
        RTC_CNTL_MAIN_TIMER_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtc_cntl_brown_out_int_raw(&self) -> RTC_CNTL_BROWN_OUT_INT_RAW_R {
        RTC_CNTL_BROWN_OUT_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_int_raw(&self) -> RTC_CNTL_WDT_INT_RAW_R {
        RTC_CNTL_WDT_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject_int_raw(&self) -> RTC_CNTL_SLP_REJECT_INT_RAW_R {
        RTC_CNTL_SLP_REJECT_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_slp_wakeup_int_raw(&self) -> RTC_CNTL_SLP_WAKEUP_INT_RAW_R {
        RTC_CNTL_SLP_WAKEUP_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
