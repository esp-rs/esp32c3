#[doc = "Reader of register SYSTEM_COMB_PVT_ERR_LVT_SITE1"]
pub type R = crate::R<u32, super::SYSTEM_COMB_PVT_ERR_LVT_SITE1>;
#[doc = "Reader of field `SYSTEM_COMB_TIMING_ERR_CNT_LVT_SITE1`"]
pub type SYSTEM_COMB_TIMING_ERR_CNT_LVT_SITE1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn system_comb_timing_err_cnt_lvt_site1(&self) -> SYSTEM_COMB_TIMING_ERR_CNT_LVT_SITE1_R {
        SYSTEM_COMB_TIMING_ERR_CNT_LVT_SITE1_R::new((self.bits & 0xffff) as u16)
    }
}
