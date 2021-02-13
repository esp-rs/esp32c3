#[doc = "Reader of register DMA_IN_PRI_CH2"]
pub type R = crate::R<u32, super::DMA_IN_PRI_CH2>;
#[doc = "Writer for register DMA_IN_PRI_CH2"]
pub type W = crate::W<u32, super::DMA_IN_PRI_CH2>;
#[doc = "Register DMA_IN_PRI_CH2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_IN_PRI_CH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_RX_PRI_CH2`"]
pub type DMA_RX_PRI_CH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_RX_PRI_CH2`"]
pub struct DMA_RX_PRI_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_PRI_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dma_rx_pri_ch2(&self) -> DMA_RX_PRI_CH2_R {
        DMA_RX_PRI_CH2_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dma_rx_pri_ch2(&mut self) -> DMA_RX_PRI_CH2_W {
        DMA_RX_PRI_CH2_W { w: self }
    }
}
