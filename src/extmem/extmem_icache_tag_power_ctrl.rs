#[doc = "Reader of register EXTMEM_ICACHE_TAG_POWER_CTRL"]
pub type R = crate::R<u32, super::EXTMEM_ICACHE_TAG_POWER_CTRL>;
#[doc = "Writer for register EXTMEM_ICACHE_TAG_POWER_CTRL"]
pub type W = crate::W<u32, super::EXTMEM_ICACHE_TAG_POWER_CTRL>;
#[doc = "Register EXTMEM_ICACHE_TAG_POWER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_ICACHE_TAG_POWER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_TAG_MEM_FORCE_PU`"]
pub type EXTMEM_ICACHE_TAG_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_TAG_MEM_FORCE_PU`"]
pub struct EXTMEM_ICACHE_TAG_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_TAG_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `EXTMEM_ICACHE_TAG_MEM_FORCE_PD`"]
pub type EXTMEM_ICACHE_TAG_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_TAG_MEM_FORCE_PD`"]
pub struct EXTMEM_ICACHE_TAG_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_TAG_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `EXTMEM_ICACHE_TAG_MEM_FORCE_ON`"]
pub type EXTMEM_ICACHE_TAG_MEM_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_TAG_MEM_FORCE_ON`"]
pub struct EXTMEM_ICACHE_TAG_MEM_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_TAG_MEM_FORCE_ON_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_icache_tag_mem_force_pu(&self) -> EXTMEM_ICACHE_TAG_MEM_FORCE_PU_R {
        EXTMEM_ICACHE_TAG_MEM_FORCE_PU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_tag_mem_force_pd(&self) -> EXTMEM_ICACHE_TAG_MEM_FORCE_PD_R {
        EXTMEM_ICACHE_TAG_MEM_FORCE_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_tag_mem_force_on(&self) -> EXTMEM_ICACHE_TAG_MEM_FORCE_ON_R {
        EXTMEM_ICACHE_TAG_MEM_FORCE_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_icache_tag_mem_force_pu(&mut self) -> EXTMEM_ICACHE_TAG_MEM_FORCE_PU_W {
        EXTMEM_ICACHE_TAG_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_tag_mem_force_pd(&mut self) -> EXTMEM_ICACHE_TAG_MEM_FORCE_PD_W {
        EXTMEM_ICACHE_TAG_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_tag_mem_force_on(&mut self) -> EXTMEM_ICACHE_TAG_MEM_FORCE_ON_W {
        EXTMEM_ICACHE_TAG_MEM_FORCE_ON_W { w: self }
    }
}
