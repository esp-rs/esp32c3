#[doc = "Reader of register DMA_IN_CONF0_CH0"]
pub type R = crate::R<u32, super::DMA_IN_CONF0_CH0>;
#[doc = "Writer for register DMA_IN_CONF0_CH0"]
pub type W = crate::W<u32, super::DMA_IN_CONF0_CH0>;
#[doc = "Register DMA_IN_CONF0_CH0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_IN_CONF0_CH0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_MEM_TRANS_EN_CH0`"]
pub type DMA_MEM_TRANS_EN_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_MEM_TRANS_EN_CH0`"]
pub struct DMA_MEM_TRANS_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_MEM_TRANS_EN_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMA_IN_DATA_BURST_EN_CH0`"]
pub type DMA_IN_DATA_BURST_EN_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_DATA_BURST_EN_CH0`"]
pub struct DMA_IN_DATA_BURST_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DATA_BURST_EN_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DMA_INDSCR_BURST_EN_CH0`"]
pub type DMA_INDSCR_BURST_EN_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_INDSCR_BURST_EN_CH0`"]
pub struct DMA_INDSCR_BURST_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INDSCR_BURST_EN_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMA_IN_LOOP_TEST_CH0`"]
pub type DMA_IN_LOOP_TEST_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_LOOP_TEST_CH0`"]
pub struct DMA_IN_LOOP_TEST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_LOOP_TEST_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMA_IN_RST_CH0`"]
pub type DMA_IN_RST_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_RST_CH0`"]
pub struct DMA_IN_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_RST_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_mem_trans_en_ch0(&self) -> DMA_MEM_TRANS_EN_CH0_R {
        DMA_MEM_TRANS_EN_CH0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_in_data_burst_en_ch0(&self) -> DMA_IN_DATA_BURST_EN_CH0_R {
        DMA_IN_DATA_BURST_EN_CH0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_indscr_burst_en_ch0(&self) -> DMA_INDSCR_BURST_EN_CH0_R {
        DMA_INDSCR_BURST_EN_CH0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_in_loop_test_ch0(&self) -> DMA_IN_LOOP_TEST_CH0_R {
        DMA_IN_LOOP_TEST_CH0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_in_rst_ch0(&self) -> DMA_IN_RST_CH0_R {
        DMA_IN_RST_CH0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_mem_trans_en_ch0(&mut self) -> DMA_MEM_TRANS_EN_CH0_W {
        DMA_MEM_TRANS_EN_CH0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_in_data_burst_en_ch0(&mut self) -> DMA_IN_DATA_BURST_EN_CH0_W {
        DMA_IN_DATA_BURST_EN_CH0_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_indscr_burst_en_ch0(&mut self) -> DMA_INDSCR_BURST_EN_CH0_W {
        DMA_INDSCR_BURST_EN_CH0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_in_loop_test_ch0(&mut self) -> DMA_IN_LOOP_TEST_CH0_W {
        DMA_IN_LOOP_TEST_CH0_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_in_rst_ch0(&mut self) -> DMA_IN_RST_CH0_W {
        DMA_IN_RST_CH0_W { w: self }
    }
}
