#[doc = "Reader of register RTC_CNTL_DBG_SEL"]
pub type R = crate::R<u32, super::RTC_CNTL_DBG_SEL>;
#[doc = "Writer for register RTC_CNTL_DBG_SEL"]
pub type W = crate::W<u32, super::RTC_CNTL_DBG_SEL>;
#[doc = "Register RTC_CNTL_DBG_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_DBG_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DEBUG_SEL4`"]
pub type RTC_CNTL_DEBUG_SEL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DEBUG_SEL4`"]
pub struct RTC_CNTL_DEBUG_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEBUG_SEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DEBUG_SEL3`"]
pub type RTC_CNTL_DEBUG_SEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DEBUG_SEL3`"]
pub struct RTC_CNTL_DEBUG_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEBUG_SEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | (((value as u32) & 0x1f) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DEBUG_SEL2`"]
pub type RTC_CNTL_DEBUG_SEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DEBUG_SEL2`"]
pub struct RTC_CNTL_DEBUG_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEBUG_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DEBUG_SEL1`"]
pub type RTC_CNTL_DEBUG_SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DEBUG_SEL1`"]
pub struct RTC_CNTL_DEBUG_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEBUG_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DEBUG_SEL0`"]
pub type RTC_CNTL_DEBUG_SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DEBUG_SEL0`"]
pub struct RTC_CNTL_DEBUG_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEBUG_SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DEBUG_BIT_SEL`"]
pub type RTC_CNTL_DEBUG_BIT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DEBUG_BIT_SEL`"]
pub struct RTC_CNTL_DEBUG_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEBUG_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DEBUG_12M_NO_GATING`"]
pub type RTC_CNTL_DEBUG_12M_NO_GATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DEBUG_12M_NO_GATING`"]
pub struct RTC_CNTL_DEBUG_12M_NO_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DEBUG_12M_NO_GATING_W<'a> {
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
impl R {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel4(&self) -> RTC_CNTL_DEBUG_SEL4_R {
        RTC_CNTL_DEBUG_SEL4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel3(&self) -> RTC_CNTL_DEBUG_SEL3_R {
        RTC_CNTL_DEBUG_SEL3_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel2(&self) -> RTC_CNTL_DEBUG_SEL2_R {
        RTC_CNTL_DEBUG_SEL2_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel1(&self) -> RTC_CNTL_DEBUG_SEL1_R {
        RTC_CNTL_DEBUG_SEL1_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel0(&self) -> RTC_CNTL_DEBUG_SEL0_R {
        RTC_CNTL_DEBUG_SEL0_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn rtc_cntl_debug_bit_sel(&self) -> RTC_CNTL_DEBUG_BIT_SEL_R {
        RTC_CNTL_DEBUG_BIT_SEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_debug_12m_no_gating(&self) -> RTC_CNTL_DEBUG_12M_NO_GATING_R {
        RTC_CNTL_DEBUG_12M_NO_GATING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel4(&mut self) -> RTC_CNTL_DEBUG_SEL4_W {
        RTC_CNTL_DEBUG_SEL4_W { w: self }
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel3(&mut self) -> RTC_CNTL_DEBUG_SEL3_W {
        RTC_CNTL_DEBUG_SEL3_W { w: self }
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel2(&mut self) -> RTC_CNTL_DEBUG_SEL2_W {
        RTC_CNTL_DEBUG_SEL2_W { w: self }
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel1(&mut self) -> RTC_CNTL_DEBUG_SEL1_W {
        RTC_CNTL_DEBUG_SEL1_W { w: self }
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn rtc_cntl_debug_sel0(&mut self) -> RTC_CNTL_DEBUG_SEL0_W {
        RTC_CNTL_DEBUG_SEL0_W { w: self }
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn rtc_cntl_debug_bit_sel(&mut self) -> RTC_CNTL_DEBUG_BIT_SEL_W {
        RTC_CNTL_DEBUG_BIT_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_debug_12m_no_gating(&mut self) -> RTC_CNTL_DEBUG_12M_NO_GATING_W {
        RTC_CNTL_DEBUG_12M_NO_GATING_W { w: self }
    }
}
