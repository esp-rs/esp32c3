#[doc = "Reader of register EXTMEM_CACHE_STATE"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_STATE>;
#[doc = "Reader of field `EXTMEM_ICACHE_STATE`"]
pub type EXTMEM_ICACHE_STATE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn extmem_icache_state(&self) -> EXTMEM_ICACHE_STATE_R {
        EXTMEM_ICACHE_STATE_R::new((self.bits & 0x0fff) as u16)
    }
}
