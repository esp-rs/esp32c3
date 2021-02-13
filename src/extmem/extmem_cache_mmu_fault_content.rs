#[doc = "Reader of register EXTMEM_CACHE_MMU_FAULT_CONTENT"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_MMU_FAULT_CONTENT>;
#[doc = "Reader of field `EXTMEM_CACHE_MMU_FAULT_CODE`"]
pub type EXTMEM_CACHE_MMU_FAULT_CODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXTMEM_CACHE_MMU_FAULT_CONTENT`"]
pub type EXTMEM_CACHE_MMU_FAULT_CONTENT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn extmem_cache_mmu_fault_code(&self) -> EXTMEM_CACHE_MMU_FAULT_CODE_R {
        EXTMEM_CACHE_MMU_FAULT_CODE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn extmem_cache_mmu_fault_content(&self) -> EXTMEM_CACHE_MMU_FAULT_CONTENT_R {
        EXTMEM_CACHE_MMU_FAULT_CONTENT_R::new((self.bits & 0x03ff) as u16)
    }
}
