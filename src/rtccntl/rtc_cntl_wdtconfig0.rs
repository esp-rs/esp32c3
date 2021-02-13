#[doc = "Reader of register RTC_CNTL_WDTCONFIG0"]
pub type R = crate::R<u32, super::RTC_CNTL_WDTCONFIG0>;
#[doc = "Writer for register RTC_CNTL_WDTCONFIG0"]
pub type W = crate::W<u32, super::RTC_CNTL_WDTCONFIG0>;
#[doc = "Register RTC_CNTL_WDTCONFIG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_WDTCONFIG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_EN`"]
pub type RTC_CNTL_WDT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_EN`"]
pub struct RTC_CNTL_WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WDT_STG0`"]
pub type RTC_CNTL_WDT_STG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_STG0`"]
pub struct RTC_CNTL_WDT_STG0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_STG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_STG1`"]
pub type RTC_CNTL_WDT_STG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_STG1`"]
pub struct RTC_CNTL_WDT_STG1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_STG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_STG2`"]
pub type RTC_CNTL_WDT_STG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_STG2`"]
pub struct RTC_CNTL_WDT_STG2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_STG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_STG3`"]
pub type RTC_CNTL_WDT_STG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_STG3`"]
pub struct RTC_CNTL_WDT_STG3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_STG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_en(&self) -> RTC_CNTL_WDT_EN_R {
        RTC_CNTL_WDT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg0(&self) -> RTC_CNTL_WDT_STG0_R {
        RTC_CNTL_WDT_STG0_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg1(&self) -> RTC_CNTL_WDT_STG1_R {
        RTC_CNTL_WDT_STG1_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg2(&self) -> RTC_CNTL_WDT_STG2_R {
        RTC_CNTL_WDT_STG2_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg3(&self) -> RTC_CNTL_WDT_STG3_R {
        RTC_CNTL_WDT_STG3_R::new(((self.bits >> 19) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_en(&mut self) -> RTC_CNTL_WDT_EN_W {
        RTC_CNTL_WDT_EN_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg0(&mut self) -> RTC_CNTL_WDT_STG0_W {
        RTC_CNTL_WDT_STG0_W { w: self }
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg1(&mut self) -> RTC_CNTL_WDT_STG1_W {
        RTC_CNTL_WDT_STG1_W { w: self }
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg2(&mut self) -> RTC_CNTL_WDT_STG2_W {
        RTC_CNTL_WDT_STG2_W { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_stg3(&mut self) -> RTC_CNTL_WDT_STG3_W {
        RTC_CNTL_WDT_STG3_W { w: self }
    }
}
