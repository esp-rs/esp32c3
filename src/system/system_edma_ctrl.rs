#[doc = "Reader of register SYSTEM_EDMA_CTRL"]
pub type R = crate::R<u32, super::SYSTEM_EDMA_CTRL>;
#[doc = "Writer for register SYSTEM_EDMA_CTRL"]
pub type W = crate::W<u32, super::SYSTEM_EDMA_CTRL>;
#[doc = "Register SYSTEM_EDMA_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_EDMA_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_EDMA_RESET`"]
pub type SYSTEM_EDMA_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_EDMA_RESET`"]
pub struct SYSTEM_EDMA_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_EDMA_RESET_W<'a> {
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
#[doc = "Reader of field `SYSTEM_EDMA_CLK_ON`"]
pub type SYSTEM_EDMA_CLK_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_EDMA_CLK_ON`"]
pub struct SYSTEM_EDMA_CLK_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_EDMA_CLK_ON_W<'a> {
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
    pub fn system_edma_reset(&self) -> SYSTEM_EDMA_RESET_R {
        SYSTEM_EDMA_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_edma_clk_on(&self) -> SYSTEM_EDMA_CLK_ON_R {
        SYSTEM_EDMA_CLK_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_edma_reset(&mut self) -> SYSTEM_EDMA_RESET_W {
        SYSTEM_EDMA_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_edma_clk_on(&mut self) -> SYSTEM_EDMA_CLK_ON_W {
        SYSTEM_EDMA_CLK_ON_W { w: self }
    }
}
