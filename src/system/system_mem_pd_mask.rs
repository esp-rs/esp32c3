#[doc = "Reader of register SYSTEM_MEM_PD_MASK"]
pub type R = crate::R<u32, super::SYSTEM_MEM_PD_MASK>;
#[doc = "Writer for register SYSTEM_MEM_PD_MASK"]
pub type W = crate::W<u32, super::SYSTEM_MEM_PD_MASK>;
#[doc = "Register SYSTEM_MEM_PD_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_MEM_PD_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_LSLP_MEM_PD_MASK`"]
pub type SYSTEM_LSLP_MEM_PD_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_LSLP_MEM_PD_MASK`"]
pub struct SYSTEM_LSLP_MEM_PD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_LSLP_MEM_PD_MASK_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_lslp_mem_pd_mask(&self) -> SYSTEM_LSLP_MEM_PD_MASK_R {
        SYSTEM_LSLP_MEM_PD_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_lslp_mem_pd_mask(&mut self) -> SYSTEM_LSLP_MEM_PD_MASK_W {
        SYSTEM_LSLP_MEM_PD_MASK_W { w: self }
    }
}
