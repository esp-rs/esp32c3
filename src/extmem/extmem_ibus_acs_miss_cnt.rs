#[doc = "Reader of register EXTMEM_IBUS_ACS_MISS_CNT"]
pub type R = crate::R<u32, super::EXTMEM_IBUS_ACS_MISS_CNT>;
#[doc = "Reader of field `EXTMEM_IBUS_ACS_MISS_CNT`"]
pub type EXTMEM_IBUS_ACS_MISS_CNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn extmem_ibus_acs_miss_cnt(&self) -> EXTMEM_IBUS_ACS_MISS_CNT_R {
        EXTMEM_IBUS_ACS_MISS_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
