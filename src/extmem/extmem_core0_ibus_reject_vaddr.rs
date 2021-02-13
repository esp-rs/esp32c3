#[doc = "Reader of register EXTMEM_CORE0_IBUS_REJECT_VADDR"]
pub type R = crate::R<u32, super::EXTMEM_CORE0_IBUS_REJECT_VADDR>;
#[doc = "Reader of field `EXTMEM_CORE0_IBUS_VADDR`"]
pub type EXTMEM_CORE0_IBUS_VADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn extmem_core0_ibus_vaddr(&self) -> EXTMEM_CORE0_IBUS_VADDR_R {
        EXTMEM_CORE0_IBUS_VADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
