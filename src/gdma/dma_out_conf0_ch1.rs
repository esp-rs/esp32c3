#[doc = "Reader of register DMA_OUT_CONF0_CH1"]
pub type R = crate::R<u32, super::DMA_OUT_CONF0_CH1>;
#[doc = "Writer for register DMA_OUT_CONF0_CH1"]
pub type W = crate::W<u32, super::DMA_OUT_CONF0_CH1>;
#[doc = "Register DMA_OUT_CONF0_CH1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_OUT_CONF0_CH1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_OUT_DATA_BURST_EN_CH1`"]
pub type DMA_OUT_DATA_BURST_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_DATA_BURST_EN_CH1`"]
pub struct DMA_OUT_DATA_BURST_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_DATA_BURST_EN_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DMA_OUTDSCR_BURST_EN_CH1`"]
pub type DMA_OUTDSCR_BURST_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUTDSCR_BURST_EN_CH1`"]
pub struct DMA_OUTDSCR_BURST_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTDSCR_BURST_EN_CH1_W<'a> {
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
#[doc = "Reader of field `DMA_OUT_EOF_MODE_CH1`"]
pub type DMA_OUT_EOF_MODE_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_EOF_MODE_CH1`"]
pub struct DMA_OUT_EOF_MODE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_EOF_MODE_CH1_W<'a> {
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
#[doc = "Reader of field `DMA_OUT_AUTO_WRBACK_CH1`"]
pub type DMA_OUT_AUTO_WRBACK_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_AUTO_WRBACK_CH1`"]
pub struct DMA_OUT_AUTO_WRBACK_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_AUTO_WRBACK_CH1_W<'a> {
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
#[doc = "Reader of field `DMA_OUT_LOOP_TEST_CH1`"]
pub type DMA_OUT_LOOP_TEST_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_LOOP_TEST_CH1`"]
pub struct DMA_OUT_LOOP_TEST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_LOOP_TEST_CH1_W<'a> {
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
#[doc = "Reader of field `DMA_OUT_RST_CH1`"]
pub type DMA_OUT_RST_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_RST_CH1`"]
pub struct DMA_OUT_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_RST_CH1_W<'a> {
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_out_data_burst_en_ch1(&self) -> DMA_OUT_DATA_BURST_EN_CH1_R {
        DMA_OUT_DATA_BURST_EN_CH1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_outdscr_burst_en_ch1(&self) -> DMA_OUTDSCR_BURST_EN_CH1_R {
        DMA_OUTDSCR_BURST_EN_CH1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_out_eof_mode_ch1(&self) -> DMA_OUT_EOF_MODE_CH1_R {
        DMA_OUT_EOF_MODE_CH1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_out_auto_wrback_ch1(&self) -> DMA_OUT_AUTO_WRBACK_CH1_R {
        DMA_OUT_AUTO_WRBACK_CH1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_out_loop_test_ch1(&self) -> DMA_OUT_LOOP_TEST_CH1_R {
        DMA_OUT_LOOP_TEST_CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_out_rst_ch1(&self) -> DMA_OUT_RST_CH1_R {
        DMA_OUT_RST_CH1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_out_data_burst_en_ch1(&mut self) -> DMA_OUT_DATA_BURST_EN_CH1_W {
        DMA_OUT_DATA_BURST_EN_CH1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_outdscr_burst_en_ch1(&mut self) -> DMA_OUTDSCR_BURST_EN_CH1_W {
        DMA_OUTDSCR_BURST_EN_CH1_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_out_eof_mode_ch1(&mut self) -> DMA_OUT_EOF_MODE_CH1_W {
        DMA_OUT_EOF_MODE_CH1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_out_auto_wrback_ch1(&mut self) -> DMA_OUT_AUTO_WRBACK_CH1_W {
        DMA_OUT_AUTO_WRBACK_CH1_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_out_loop_test_ch1(&mut self) -> DMA_OUT_LOOP_TEST_CH1_W {
        DMA_OUT_LOOP_TEST_CH1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_out_rst_ch1(&mut self) -> DMA_OUT_RST_CH1_W {
        DMA_OUT_RST_CH1_W { w: self }
    }
}
