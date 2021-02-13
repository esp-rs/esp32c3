#[doc = "Writer for register RTC_CNTL_INT_CLR"]
pub type W = crate::W<u32, super::RTC_CNTL_INT_CLR>;
#[doc = "Register RTC_CNTL_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_CAL_INT_CLR`"]
pub struct RTC_CNTL_BBPLL_CAL_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_CAL_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_GLITCH_DET_INT_CLR`"]
pub struct RTC_CNTL_GLITCH_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GLITCH_DET_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_DEAD_INT_CLR`"]
pub struct RTC_CNTL_XTAL32K_DEAD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_DEAD_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_SWD_INT_CLR`"]
pub struct RTC_CNTL_SWD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SWD_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_MAIN_TIMER_INT_CLR`"]
pub struct RTC_CNTL_MAIN_TIMER_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_MAIN_TIMER_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_BROWN_OUT_INT_CLR`"]
pub struct RTC_CNTL_BROWN_OUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BROWN_OUT_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_WDT_INT_CLR`"]
pub struct RTC_CNTL_WDT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_SLP_REJECT_INT_CLR`"]
pub struct RTC_CNTL_SLP_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_REJECT_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_SLP_WAKEUP_INT_CLR`"]
pub struct RTC_CNTL_SLP_WAKEUP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_WAKEUP_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_cal_int_clr(&mut self) -> RTC_CNTL_BBPLL_CAL_INT_CLR_W {
        RTC_CNTL_BBPLL_CAL_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_glitch_det_int_clr(&mut self) -> RTC_CNTL_GLITCH_DET_INT_CLR_W {
        RTC_CNTL_GLITCH_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_dead_int_clr(&mut self) -> RTC_CNTL_XTAL32K_DEAD_INT_CLR_W {
        RTC_CNTL_XTAL32K_DEAD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_swd_int_clr(&mut self) -> RTC_CNTL_SWD_INT_CLR_W {
        RTC_CNTL_SWD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtc_cntl_main_timer_int_clr(&mut self) -> RTC_CNTL_MAIN_TIMER_INT_CLR_W {
        RTC_CNTL_MAIN_TIMER_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtc_cntl_brown_out_int_clr(&mut self) -> RTC_CNTL_BROWN_OUT_INT_CLR_W {
        RTC_CNTL_BROWN_OUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_int_clr(&mut self) -> RTC_CNTL_WDT_INT_CLR_W {
        RTC_CNTL_WDT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject_int_clr(&mut self) -> RTC_CNTL_SLP_REJECT_INT_CLR_W {
        RTC_CNTL_SLP_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_slp_wakeup_int_clr(&mut self) -> RTC_CNTL_SLP_WAKEUP_INT_CLR_W {
        RTC_CNTL_SLP_WAKEUP_INT_CLR_W { w: self }
    }
}
