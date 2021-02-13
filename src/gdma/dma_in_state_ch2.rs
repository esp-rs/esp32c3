#[doc = "Reader of register DMA_IN_STATE_CH2"]
pub type R = crate::R<u32, super::DMA_IN_STATE_CH2>;
#[doc = "Reader of field `DMA_IN_STATE_CH2`"]
pub type DMA_IN_STATE_CH2_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_IN_DSCR_STATE_CH2`"]
pub type DMA_IN_DSCR_STATE_CH2_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_INLINK_DSCR_ADDR_CH2`"]
pub type DMA_INLINK_DSCR_ADDR_CH2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn dma_in_state_ch2(&self) -> DMA_IN_STATE_CH2_R {
        DMA_IN_STATE_CH2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn dma_in_dscr_state_ch2(&self) -> DMA_IN_DSCR_STATE_CH2_R {
        DMA_IN_DSCR_STATE_CH2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn dma_inlink_dscr_addr_ch2(&self) -> DMA_INLINK_DSCR_ADDR_CH2_R {
        DMA_INLINK_DSCR_ADDR_CH2_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
