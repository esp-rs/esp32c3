#[doc = "Reader of register RTC_CNTL_SWD_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_SWD_CONF>;
#[doc = "Writer for register RTC_CNTL_SWD_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_SWD_CONF>;
#[doc = "Register RTC_CNTL_SWD_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_SWD_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_SWD_AUTO_FEED_EN`"]
pub type RTC_CNTL_SWD_AUTO_FEED_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SWD_AUTO_FEED_EN`"]
pub struct RTC_CNTL_SWD_AUTO_FEED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SWD_AUTO_FEED_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SWD_DISABLE`"]
pub type RTC_CNTL_SWD_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SWD_DISABLE`"]
pub struct RTC_CNTL_SWD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SWD_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_SWD_FEED`"]
pub struct RTC_CNTL_SWD_FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SWD_FEED_W<'a> {
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
#[doc = "Write proxy for field `RTC_CNTL_SWD_RST_FLAG_CLR`"]
pub struct RTC_CNTL_SWD_RST_FLAG_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SWD_RST_FLAG_CLR_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SWD_SIGNAL_WIDTH`"]
pub type RTC_CNTL_SWD_SIGNAL_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_SWD_SIGNAL_WIDTH`"]
pub struct RTC_CNTL_SWD_SIGNAL_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SWD_SIGNAL_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 18)) | (((value as u32) & 0x03ff) << 18);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SWD_BYPASS_RST`"]
pub type RTC_CNTL_SWD_BYPASS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SWD_BYPASS_RST`"]
pub struct RTC_CNTL_SWD_BYPASS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SWD_BYPASS_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SWD_FEED_INT`"]
pub type RTC_CNTL_SWD_FEED_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_SWD_RESET_FLAG`"]
pub type RTC_CNTL_SWD_RESET_FLAG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_swd_auto_feed_en(&self) -> RTC_CNTL_SWD_AUTO_FEED_EN_R {
        RTC_CNTL_SWD_AUTO_FEED_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_swd_disable(&self) -> RTC_CNTL_SWD_DISABLE_R {
        RTC_CNTL_SWD_DISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn rtc_cntl_swd_signal_width(&self) -> RTC_CNTL_SWD_SIGNAL_WIDTH_R {
        RTC_CNTL_SWD_SIGNAL_WIDTH_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtc_cntl_swd_bypass_rst(&self) -> RTC_CNTL_SWD_BYPASS_RST_R {
        RTC_CNTL_SWD_BYPASS_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_swd_feed_int(&self) -> RTC_CNTL_SWD_FEED_INT_R {
        RTC_CNTL_SWD_FEED_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_swd_reset_flag(&self) -> RTC_CNTL_SWD_RESET_FLAG_R {
        RTC_CNTL_SWD_RESET_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_swd_auto_feed_en(&mut self) -> RTC_CNTL_SWD_AUTO_FEED_EN_W {
        RTC_CNTL_SWD_AUTO_FEED_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_swd_disable(&mut self) -> RTC_CNTL_SWD_DISABLE_W {
        RTC_CNTL_SWD_DISABLE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_swd_feed(&mut self) -> RTC_CNTL_SWD_FEED_W {
        RTC_CNTL_SWD_FEED_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_swd_rst_flag_clr(&mut self) -> RTC_CNTL_SWD_RST_FLAG_CLR_W {
        RTC_CNTL_SWD_RST_FLAG_CLR_W { w: self }
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn rtc_cntl_swd_signal_width(&mut self) -> RTC_CNTL_SWD_SIGNAL_WIDTH_W {
        RTC_CNTL_SWD_SIGNAL_WIDTH_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtc_cntl_swd_bypass_rst(&mut self) -> RTC_CNTL_SWD_BYPASS_RST_W {
        RTC_CNTL_SWD_BYPASS_RST_W { w: self }
    }
}
