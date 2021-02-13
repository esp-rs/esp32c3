#[doc = "Reader of register ASSIST_DEBUG_CORE_0_RCD_EN"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_RCD_EN>;
#[doc = "Writer for register ASSIST_DEBUG_CORE_0_RCD_EN"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_CORE_0_RCD_EN>;
#[doc = "Register ASSIST_DEBUG_CORE_0_RCD_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_CORE_0_RCD_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN`"]
pub type ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN`"]
pub struct ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN_W<'a> {
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
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_RCD_RECORDEN`"]
pub type ASSIST_DEBUG_CORE_0_RCD_RECORDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_RCD_RECORDEN`"]
pub struct ASSIST_DEBUG_CORE_0_RCD_RECORDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_RCD_RECORDEN_W<'a> {
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
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn assist_debug_core_0_rcd_pdebugen(&self) -> ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN_R {
        ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn assist_debug_core_0_rcd_recorden(&self) -> ASSIST_DEBUG_CORE_0_RCD_RECORDEN_R {
        ASSIST_DEBUG_CORE_0_RCD_RECORDEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn assist_debug_core_0_rcd_pdebugen(&mut self) -> ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN_W {
        ASSIST_DEBUG_CORE_0_RCD_PDEBUGEN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn assist_debug_core_0_rcd_recorden(&mut self) -> ASSIST_DEBUG_CORE_0_RCD_RECORDEN_W {
        ASSIST_DEBUG_CORE_0_RCD_RECORDEN_W { w: self }
    }
}
