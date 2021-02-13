#[doc = "Reader of register EXTMEM_CACHE_MMU_FAULT_VADDR"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_MMU_FAULT_VADDR>;
#[doc = "Reader of field `EXTMEM_CACHE_MMU_FAULT_VADDR`"]
pub type EXTMEM_CACHE_MMU_FAULT_VADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn extmem_cache_mmu_fault_vaddr(&self) -> EXTMEM_CACHE_MMU_FAULT_VADDR_R {
        EXTMEM_CACHE_MMU_FAULT_VADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
