#[doc = "Reader of register DMA_OUT_DSCR_CH0"]
pub type R = crate::R<u32, super::DMA_OUT_DSCR_CH0>;
#[doc = "Reader of field `DMA_OUTLINK_DSCR_CH0`"]
pub type DMA_OUTLINK_DSCR_CH0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_outlink_dscr_ch0(&self) -> DMA_OUTLINK_DSCR_CH0_R {
        DMA_OUTLINK_DSCR_CH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
