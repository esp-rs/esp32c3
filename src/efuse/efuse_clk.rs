#[doc = "Reader of register EFUSE_CLK"]
pub type R = crate::R<u32, super::EFUSE_CLK>;
#[doc = "Writer for register EFUSE_CLK"]
pub type W = crate::W<u32, super::EFUSE_CLK>;
#[doc = "Register EFUSE_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_CLK_EN`"]
pub type EFUSE_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_CLK_EN`"]
pub struct EFUSE_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_MEM_FORCE_PU`"]
pub type EFUSE_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_MEM_FORCE_PU`"]
pub struct EFUSE_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_MEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_MEM_CLK_FORCE_ON`"]
pub type EFUSE_MEM_CLK_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_MEM_CLK_FORCE_ON`"]
pub struct EFUSE_MEM_CLK_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_MEM_CLK_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `EFUSE_MEM_FORCE_PD`"]
pub type EFUSE_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_MEM_FORCE_PD`"]
pub struct EFUSE_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_MEM_FORCE_PD_W<'a> {
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_mem_force_pu(&self) -> EFUSE_MEM_FORCE_PU_R {
        EFUSE_MEM_FORCE_PU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_mem_clk_force_on(&self) -> EFUSE_MEM_CLK_FORCE_ON_R {
        EFUSE_MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_mem_force_pd(&self) -> EFUSE_MEM_FORCE_PD_R {
        EFUSE_MEM_FORCE_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W {
        EFUSE_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_mem_force_pu(&mut self) -> EFUSE_MEM_FORCE_PU_W {
        EFUSE_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_mem_clk_force_on(&mut self) -> EFUSE_MEM_CLK_FORCE_ON_W {
        EFUSE_MEM_CLK_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_mem_force_pd(&mut self) -> EFUSE_MEM_FORCE_PD_W {
        EFUSE_MEM_FORCE_PD_W { w: self }
    }
}
