#[doc = "Reader of register RTC_CNTL_LOW_POWER_ST"]
pub type R = crate::R<u32, super::RTC_CNTL_LOW_POWER_ST>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE`"]
pub type RTC_CNTL_MAIN_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE_IN_IDLE`"]
pub type RTC_CNTL_MAIN_STATE_IN_IDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE_IN_SLP`"]
pub type RTC_CNTL_MAIN_STATE_IN_SLP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE_IN_WAIT_XTL`"]
pub type RTC_CNTL_MAIN_STATE_IN_WAIT_XTL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE_IN_WAIT_PLL`"]
pub type RTC_CNTL_MAIN_STATE_IN_WAIT_PLL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE_IN_WAIT_8M`"]
pub type RTC_CNTL_MAIN_STATE_IN_WAIT_8M_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_IN_LOW_POWER_STATE`"]
pub type RTC_CNTL_IN_LOW_POWER_STATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_IN_WAKEUP_STATE`"]
pub type RTC_CNTL_IN_WAKEUP_STATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE_WAIT_END`"]
pub type RTC_CNTL_MAIN_STATE_WAIT_END_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_RDY_FOR_WAKEUP`"]
pub type RTC_CNTL_RDY_FOR_WAKEUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE_PLL_ON`"]
pub type RTC_CNTL_MAIN_STATE_PLL_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_MAIN_STATE_XTAL_ISO`"]
pub type RTC_CNTL_MAIN_STATE_XTAL_ISO_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_COCPU_STATE_DONE`"]
pub type RTC_CNTL_COCPU_STATE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_COCPU_STATE_SLP`"]
pub type RTC_CNTL_COCPU_STATE_SLP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_COCPU_STATE_SWITCH`"]
pub type RTC_CNTL_COCPU_STATE_SWITCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_COCPU_STATE_START`"]
pub type RTC_CNTL_COCPU_STATE_START_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_TOUCH_STATE_DONE`"]
pub type RTC_CNTL_TOUCH_STATE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_TOUCH_STATE_SLP`"]
pub type RTC_CNTL_TOUCH_STATE_SLP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_TOUCH_STATE_SWITCH`"]
pub type RTC_CNTL_TOUCH_STATE_SWITCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_TOUCH_STATE_START`"]
pub type RTC_CNTL_TOUCH_STATE_START_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_XPD_DIG`"]
pub type RTC_CNTL_XPD_DIG_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_DIG_ISO`"]
pub type RTC_CNTL_DIG_ISO_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_XPD_WIFI`"]
pub type RTC_CNTL_XPD_WIFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_WIFI_ISO`"]
pub type RTC_CNTL_WIFI_ISO_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_XPD_RTC_PERI`"]
pub type RTC_CNTL_XPD_RTC_PERI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_PERI_ISO`"]
pub type RTC_CNTL_PERI_ISO_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_XPD_DIG_DCDC`"]
pub type RTC_CNTL_XPD_DIG_DCDC_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_XPD_ROM0`"]
pub type RTC_CNTL_XPD_ROM0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rtc_cntl_main_state(&self) -> RTC_CNTL_MAIN_STATE_R {
        RTC_CNTL_MAIN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_main_state_in_idle(&self) -> RTC_CNTL_MAIN_STATE_IN_IDLE_R {
        RTC_CNTL_MAIN_STATE_IN_IDLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_main_state_in_slp(&self) -> RTC_CNTL_MAIN_STATE_IN_SLP_R {
        RTC_CNTL_MAIN_STATE_IN_SLP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_main_state_in_wait_xtl(&self) -> RTC_CNTL_MAIN_STATE_IN_WAIT_XTL_R {
        RTC_CNTL_MAIN_STATE_IN_WAIT_XTL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_main_state_in_wait_pll(&self) -> RTC_CNTL_MAIN_STATE_IN_WAIT_PLL_R {
        RTC_CNTL_MAIN_STATE_IN_WAIT_PLL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_main_state_in_wait_8m(&self) -> RTC_CNTL_MAIN_STATE_IN_WAIT_8M_R {
        RTC_CNTL_MAIN_STATE_IN_WAIT_8M_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_in_low_power_state(&self) -> RTC_CNTL_IN_LOW_POWER_STATE_R {
        RTC_CNTL_IN_LOW_POWER_STATE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtc_cntl_in_wakeup_state(&self) -> RTC_CNTL_IN_WAKEUP_STATE_R {
        RTC_CNTL_IN_WAKEUP_STATE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_main_state_wait_end(&self) -> RTC_CNTL_MAIN_STATE_WAIT_END_R {
        RTC_CNTL_MAIN_STATE_WAIT_END_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_rdy_for_wakeup(&self) -> RTC_CNTL_RDY_FOR_WAKEUP_R {
        RTC_CNTL_RDY_FOR_WAKEUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_main_state_pll_on(&self) -> RTC_CNTL_MAIN_STATE_PLL_ON_R {
        RTC_CNTL_MAIN_STATE_PLL_ON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtc_cntl_main_state_xtal_iso(&self) -> RTC_CNTL_MAIN_STATE_XTAL_ISO_R {
        RTC_CNTL_MAIN_STATE_XTAL_ISO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_cocpu_state_done(&self) -> RTC_CNTL_COCPU_STATE_DONE_R {
        RTC_CNTL_COCPU_STATE_DONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_cocpu_state_slp(&self) -> RTC_CNTL_COCPU_STATE_SLP_R {
        RTC_CNTL_COCPU_STATE_SLP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtc_cntl_cocpu_state_switch(&self) -> RTC_CNTL_COCPU_STATE_SWITCH_R {
        RTC_CNTL_COCPU_STATE_SWITCH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_cocpu_state_start(&self) -> RTC_CNTL_COCPU_STATE_START_R {
        RTC_CNTL_COCPU_STATE_START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_touch_state_done(&self) -> RTC_CNTL_TOUCH_STATE_DONE_R {
        RTC_CNTL_TOUCH_STATE_DONE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_touch_state_slp(&self) -> RTC_CNTL_TOUCH_STATE_SLP_R {
        RTC_CNTL_TOUCH_STATE_SLP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtc_cntl_touch_state_switch(&self) -> RTC_CNTL_TOUCH_STATE_SWITCH_R {
        RTC_CNTL_TOUCH_STATE_SWITCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtc_cntl_touch_state_start(&self) -> RTC_CNTL_TOUCH_STATE_START_R {
        RTC_CNTL_TOUCH_STATE_START_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_dig(&self) -> RTC_CNTL_XPD_DIG_R {
        RTC_CNTL_XPD_DIG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_dig_iso(&self) -> RTC_CNTL_DIG_ISO_R {
        RTC_CNTL_DIG_ISO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_wifi(&self) -> RTC_CNTL_XPD_WIFI_R {
        RTC_CNTL_XPD_WIFI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_iso(&self) -> RTC_CNTL_WIFI_ISO_R {
        RTC_CNTL_WIFI_ISO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_rtc_peri(&self) -> RTC_CNTL_XPD_RTC_PERI_R {
        RTC_CNTL_XPD_RTC_PERI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_peri_iso(&self) -> RTC_CNTL_PERI_ISO_R {
        RTC_CNTL_PERI_ISO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_dig_dcdc(&self) -> RTC_CNTL_XPD_DIG_DCDC_R {
        RTC_CNTL_XPD_DIG_DCDC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_rom0(&self) -> RTC_CNTL_XPD_ROM0_R {
        RTC_CNTL_XPD_ROM0_R::new((self.bits & 0x01) != 0)
    }
}
