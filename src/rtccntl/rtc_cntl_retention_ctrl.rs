#[doc = "Reader of register RTC_CNTL_RETENTION_CTRL"]
pub type R = crate::R<u32, super::RTC_CNTL_RETENTION_CTRL>;
#[doc = "Writer for register RTC_CNTL_RETENTION_CTRL"]
pub type W = crate::W<u32, super::RTC_CNTL_RETENTION_CTRL>;
#[doc = "Register RTC_CNTL_RETENTION_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_RETENTION_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_RETENTION_WAIT`"]
pub type RTC_CNTL_RETENTION_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_RETENTION_WAIT`"]
pub struct RTC_CNTL_RETENTION_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RETENTION_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_RETENTION_EN`"]
pub type RTC_CNTL_RETENTION_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_RETENTION_EN`"]
pub struct RTC_CNTL_RETENTION_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RETENTION_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_RETENTION_CLKOFF_WAIT`"]
pub type RTC_CNTL_RETENTION_CLKOFF_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_RETENTION_CLKOFF_WAIT`"]
pub struct RTC_CNTL_RETENTION_CLKOFF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RETENTION_CLKOFF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_RETENTION_DONE_WAIT`"]
pub type RTC_CNTL_RETENTION_DONE_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_RETENTION_DONE_WAIT`"]
pub struct RTC_CNTL_RETENTION_DONE_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RETENTION_DONE_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_RETENTION_CLK_SEL`"]
pub type RTC_CNTL_RETENTION_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_RETENTION_CLK_SEL`"]
pub struct RTC_CNTL_RETENTION_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RETENTION_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn rtc_cntl_retention_wait(&self) -> RTC_CNTL_RETENTION_WAIT_R {
        RTC_CNTL_RETENTION_WAIT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_retention_en(&self) -> RTC_CNTL_RETENTION_EN_R {
        RTC_CNTL_RETENTION_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn rtc_cntl_retention_clkoff_wait(&self) -> RTC_CNTL_RETENTION_CLKOFF_WAIT_R {
        RTC_CNTL_RETENTION_CLKOFF_WAIT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn rtc_cntl_retention_done_wait(&self) -> RTC_CNTL_RETENTION_DONE_WAIT_R {
        RTC_CNTL_RETENTION_DONE_WAIT_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_retention_clk_sel(&self) -> RTC_CNTL_RETENTION_CLK_SEL_R {
        RTC_CNTL_RETENTION_CLK_SEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn rtc_cntl_retention_wait(&mut self) -> RTC_CNTL_RETENTION_WAIT_W {
        RTC_CNTL_RETENTION_WAIT_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_retention_en(&mut self) -> RTC_CNTL_RETENTION_EN_W {
        RTC_CNTL_RETENTION_EN_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn rtc_cntl_retention_clkoff_wait(&mut self) -> RTC_CNTL_RETENTION_CLKOFF_WAIT_W {
        RTC_CNTL_RETENTION_CLKOFF_WAIT_W { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn rtc_cntl_retention_done_wait(&mut self) -> RTC_CNTL_RETENTION_DONE_WAIT_W {
        RTC_CNTL_RETENTION_DONE_WAIT_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_retention_clk_sel(&mut self) -> RTC_CNTL_RETENTION_CLK_SEL_W {
        RTC_CNTL_RETENTION_CLK_SEL_W { w: self }
    }
}
