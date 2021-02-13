#[doc = "Reader of register SENSITIVE_CACHE_MMU_ACCESS_1"]
pub type R = crate::R<u32, super::SENSITIVE_CACHE_MMU_ACCESS_1>;
#[doc = "Writer for register SENSITIVE_CACHE_MMU_ACCESS_1"]
pub type W = crate::W<u32, super::SENSITIVE_CACHE_MMU_ACCESS_1>;
#[doc = "Register SENSITIVE_CACHE_MMU_ACCESS_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CACHE_MMU_ACCESS_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_PRO_MMU_WR_ACS`"]
pub type SENSITIVE_PRO_MMU_WR_ACS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_PRO_MMU_WR_ACS`"]
pub struct SENSITIVE_PRO_MMU_WR_ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_PRO_MMU_WR_ACS_W<'a> {
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
#[doc = "Reader of field `SENSITIVE_PRO_MMU_RD_ACS`"]
pub type SENSITIVE_PRO_MMU_RD_ACS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_PRO_MMU_RD_ACS`"]
pub struct SENSITIVE_PRO_MMU_RD_ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_PRO_MMU_RD_ACS_W<'a> {
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
    pub fn sensitive_pro_mmu_wr_acs(&self) -> SENSITIVE_PRO_MMU_WR_ACS_R {
        SENSITIVE_PRO_MMU_WR_ACS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_pro_mmu_rd_acs(&self) -> SENSITIVE_PRO_MMU_RD_ACS_R {
        SENSITIVE_PRO_MMU_RD_ACS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sensitive_pro_mmu_wr_acs(&mut self) -> SENSITIVE_PRO_MMU_WR_ACS_W {
        SENSITIVE_PRO_MMU_WR_ACS_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_pro_mmu_rd_acs(&mut self) -> SENSITIVE_PRO_MMU_RD_ACS_W {
        SENSITIVE_PRO_MMU_RD_ACS_W { w: self }
    }
}
