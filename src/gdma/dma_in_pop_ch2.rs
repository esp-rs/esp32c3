#[doc = "Reader of register DMA_IN_POP_CH2"]
pub type R = crate::R<u32, super::DMA_IN_POP_CH2>;
#[doc = "Writer for register DMA_IN_POP_CH2"]
pub type W = crate::W<u32, super::DMA_IN_POP_CH2>;
#[doc = "Register DMA_IN_POP_CH2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_IN_POP_CH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_INFIFO_POP_CH2`"]
pub type DMA_INFIFO_POP_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_INFIFO_POP_CH2`"]
pub struct DMA_INFIFO_POP_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_POP_CH2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DMA_INFIFO_RDATA_CH2`"]
pub type DMA_INFIFO_RDATA_CH2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_infifo_pop_ch2(&self) -> DMA_INFIFO_POP_CH2_R {
        DMA_INFIFO_POP_CH2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dma_infifo_rdata_ch2(&self) -> DMA_INFIFO_RDATA_CH2_R {
        DMA_INFIFO_RDATA_CH2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_infifo_pop_ch2(&mut self) -> DMA_INFIFO_POP_CH2_W {
        DMA_INFIFO_POP_CH2_W { w: self }
    }
}
