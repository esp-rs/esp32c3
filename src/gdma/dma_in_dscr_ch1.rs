#[doc = "Reader of register DMA_IN_DSCR_CH1"]
pub type R = crate::R<u32, super::DMA_IN_DSCR_CH1>;
#[doc = "Reader of field `DMA_INLINK_DSCR_CH1`"]
pub type DMA_INLINK_DSCR_CH1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_inlink_dscr_ch1(&self) -> DMA_INLINK_DSCR_CH1_R {
        DMA_INLINK_DSCR_CH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
