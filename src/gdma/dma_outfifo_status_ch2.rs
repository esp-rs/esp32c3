#[doc = "Reader of register DMA_OUTFIFO_STATUS_CH2"]
pub type R = crate::R<u32, super::DMA_OUTFIFO_STATUS_CH2>;
#[doc = "Reader of field `DMA_OUT_REMAIN_UNDER_4B_CH2`"]
pub type DMA_OUT_REMAIN_UNDER_4B_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUT_REMAIN_UNDER_3B_CH2`"]
pub type DMA_OUT_REMAIN_UNDER_3B_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUT_REMAIN_UNDER_2B_CH2`"]
pub type DMA_OUT_REMAIN_UNDER_2B_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUT_REMAIN_UNDER_1B_CH2`"]
pub type DMA_OUT_REMAIN_UNDER_1B_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUTFIFO_CNT_CH2`"]
pub type DMA_OUTFIFO_CNT_CH2_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_OUTFIFO_EMPTY_CH2`"]
pub type DMA_OUTFIFO_EMPTY_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUTFIFO_FULL_CH2`"]
pub type DMA_OUTFIFO_FULL_CH2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dma_out_remain_under_4b_ch2(&self) -> DMA_OUT_REMAIN_UNDER_4B_CH2_R {
        DMA_OUT_REMAIN_UNDER_4B_CH2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dma_out_remain_under_3b_ch2(&self) -> DMA_OUT_REMAIN_UNDER_3B_CH2_R {
        DMA_OUT_REMAIN_UNDER_3B_CH2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dma_out_remain_under_2b_ch2(&self) -> DMA_OUT_REMAIN_UNDER_2B_CH2_R {
        DMA_OUT_REMAIN_UNDER_2B_CH2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dma_out_remain_under_1b_ch2(&self) -> DMA_OUT_REMAIN_UNDER_1B_CH2_R {
        DMA_OUT_REMAIN_UNDER_1B_CH2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn dma_outfifo_cnt_ch2(&self) -> DMA_OUTFIFO_CNT_CH2_R {
        DMA_OUTFIFO_CNT_CH2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_outfifo_empty_ch2(&self) -> DMA_OUTFIFO_EMPTY_CH2_R {
        DMA_OUTFIFO_EMPTY_CH2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_outfifo_full_ch2(&self) -> DMA_OUTFIFO_FULL_CH2_R {
        DMA_OUTFIFO_FULL_CH2_R::new((self.bits & 0x01) != 0)
    }
}
