#[doc = "Reader of register RTC_CNTL_TIME_UPDATE"]
pub type R = crate::R<u32, super::RTC_CNTL_TIME_UPDATE>;
#[doc = "Writer for register RTC_CNTL_TIME_UPDATE"]
pub type W = crate::W<u32, super::RTC_CNTL_TIME_UPDATE>;
#[doc = "Register RTC_CNTL_TIME_UPDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_TIME_UPDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RTC_CNTL_TIME_UPDATE`"]
pub struct RTC_CNTL_TIME_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TIME_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_TIMER_SYS_RST`"]
pub type RTC_CNTL_TIMER_SYS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TIMER_SYS_RST`"]
pub struct RTC_CNTL_TIMER_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TIMER_SYS_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_TIMER_XTL_OFF`"]
pub type RTC_CNTL_TIMER_XTL_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TIMER_XTL_OFF`"]
pub struct RTC_CNTL_TIMER_XTL_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TIMER_XTL_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_TIMER_SYS_STALL`"]
pub type RTC_CNTL_TIMER_SYS_STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TIMER_SYS_STALL`"]
pub struct RTC_CNTL_TIMER_SYS_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TIMER_SYS_STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_timer_sys_rst(&self) -> RTC_CNTL_TIMER_SYS_RST_R {
        RTC_CNTL_TIMER_SYS_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_timer_xtl_off(&self) -> RTC_CNTL_TIMER_XTL_OFF_R {
        RTC_CNTL_TIMER_XTL_OFF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_timer_sys_stall(&self) -> RTC_CNTL_TIMER_SYS_STALL_R {
        RTC_CNTL_TIMER_SYS_STALL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_time_update(&mut self) -> RTC_CNTL_TIME_UPDATE_W {
        RTC_CNTL_TIME_UPDATE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_timer_sys_rst(&mut self) -> RTC_CNTL_TIMER_SYS_RST_W {
        RTC_CNTL_TIMER_SYS_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_timer_xtl_off(&mut self) -> RTC_CNTL_TIMER_XTL_OFF_W {
        RTC_CNTL_TIMER_XTL_OFF_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_timer_sys_stall(&mut self) -> RTC_CNTL_TIMER_SYS_STALL_W {
        RTC_CNTL_TIMER_SYS_STALL_W { w: self }
    }
}
