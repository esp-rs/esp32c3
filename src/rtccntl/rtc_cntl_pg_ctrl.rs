#[doc = "Reader of register RTC_CNTL_PG_CTRL"]
pub type R = crate::R<u32, super::RTC_CNTL_PG_CTRL>;
#[doc = "Writer for register RTC_CNTL_PG_CTRL"]
pub type W = crate::W<u32, super::RTC_CNTL_PG_CTRL>;
#[doc = "Register RTC_CNTL_PG_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_PG_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_POWER_GLITCH_EN`"]
pub type RTC_CNTL_POWER_GLITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_POWER_GLITCH_EN`"]
pub struct RTC_CNTL_POWER_GLITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_POWER_GLITCH_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_POWER_GLITCH_EFUSE_SEL`"]
pub type RTC_CNTL_POWER_GLITCH_EFUSE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_POWER_GLITCH_EFUSE_SEL`"]
pub struct RTC_CNTL_POWER_GLITCH_EFUSE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_POWER_GLITCH_EFUSE_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_POWER_GLITCH_FORCE_PU`"]
pub type RTC_CNTL_POWER_GLITCH_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_POWER_GLITCH_FORCE_PU`"]
pub struct RTC_CNTL_POWER_GLITCH_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_POWER_GLITCH_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_POWER_GLITCH_FORCE_PD`"]
pub type RTC_CNTL_POWER_GLITCH_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_POWER_GLITCH_FORCE_PD`"]
pub struct RTC_CNTL_POWER_GLITCH_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_POWER_GLITCH_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_POWER_GLITCH_DSENSE`"]
pub type RTC_CNTL_POWER_GLITCH_DSENSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_POWER_GLITCH_DSENSE`"]
pub struct RTC_CNTL_POWER_GLITCH_DSENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_POWER_GLITCH_DSENSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_en(&self) -> RTC_CNTL_POWER_GLITCH_EN_R {
        RTC_CNTL_POWER_GLITCH_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_efuse_sel(&self) -> RTC_CNTL_POWER_GLITCH_EFUSE_SEL_R {
        RTC_CNTL_POWER_GLITCH_EFUSE_SEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_force_pu(&self) -> RTC_CNTL_POWER_GLITCH_FORCE_PU_R {
        RTC_CNTL_POWER_GLITCH_FORCE_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_force_pd(&self) -> RTC_CNTL_POWER_GLITCH_FORCE_PD_R {
        RTC_CNTL_POWER_GLITCH_FORCE_PD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_dsense(&self) -> RTC_CNTL_POWER_GLITCH_DSENSE_R {
        RTC_CNTL_POWER_GLITCH_DSENSE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_en(&mut self) -> RTC_CNTL_POWER_GLITCH_EN_W {
        RTC_CNTL_POWER_GLITCH_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_efuse_sel(&mut self) -> RTC_CNTL_POWER_GLITCH_EFUSE_SEL_W {
        RTC_CNTL_POWER_GLITCH_EFUSE_SEL_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_force_pu(&mut self) -> RTC_CNTL_POWER_GLITCH_FORCE_PU_W {
        RTC_CNTL_POWER_GLITCH_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_force_pd(&mut self) -> RTC_CNTL_POWER_GLITCH_FORCE_PD_W {
        RTC_CNTL_POWER_GLITCH_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn rtc_cntl_power_glitch_dsense(&mut self) -> RTC_CNTL_POWER_GLITCH_DSENSE_W {
        RTC_CNTL_POWER_GLITCH_DSENSE_W { w: self }
    }
}
