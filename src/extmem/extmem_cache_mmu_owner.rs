#[doc = "Reader of register EXTMEM_CACHE_MMU_OWNER"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_MMU_OWNER>;
#[doc = "Writer for register EXTMEM_CACHE_MMU_OWNER"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_MMU_OWNER>;
#[doc = "Register EXTMEM_CACHE_MMU_OWNER `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_MMU_OWNER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_CACHE_MMU_OWNER`"]
pub type EXTMEM_CACHE_MMU_OWNER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTMEM_CACHE_MMU_OWNER`"]
pub struct EXTMEM_CACHE_MMU_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CACHE_MMU_OWNER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn extmem_cache_mmu_owner(&self) -> EXTMEM_CACHE_MMU_OWNER_R {
        EXTMEM_CACHE_MMU_OWNER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn extmem_cache_mmu_owner(&mut self) -> EXTMEM_CACHE_MMU_OWNER_W {
        EXTMEM_CACHE_MMU_OWNER_W { w: self }
    }
}
