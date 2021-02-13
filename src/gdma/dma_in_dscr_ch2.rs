#[doc = "Reader of register DMA_IN_DSCR_CH2"]
pub type R = crate::R<u32, super::DMA_IN_DSCR_CH2>;
#[doc = "Reader of field `DMA_INLINK_DSCR_CH2`"]
pub type DMA_INLINK_DSCR_CH2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_inlink_dscr_ch2(&self) -> DMA_INLINK_DSCR_CH2_R {
        DMA_INLINK_DSCR_CH2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
