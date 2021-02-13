#[doc = "Reader of register DMA_IN_DSCR_BF1_CH0"]
pub type R = crate::R<u32, super::DMA_IN_DSCR_BF1_CH0>;
#[doc = "Reader of field `DMA_INLINK_DSCR_BF1_CH0`"]
pub type DMA_INLINK_DSCR_BF1_CH0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_inlink_dscr_bf1_ch0(&self) -> DMA_INLINK_DSCR_BF1_CH0_R {
        DMA_INLINK_DSCR_BF1_CH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
