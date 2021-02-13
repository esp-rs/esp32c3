#[doc = "Reader of register SPI_DMA_CONF"]
pub type R = crate::R<u32, super::SPI_DMA_CONF>;
#[doc = "Writer for register SPI_DMA_CONF"]
pub type W = crate::W<u32, super::SPI_DMA_CONF>;
#[doc = "Register SPI_DMA_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DMA_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SPI_DMA_AFIFO_RST`"]
pub struct SPI_DMA_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_AFIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_BUF_AFIFO_RST`"]
pub struct SPI_BUF_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUF_AFIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_RX_AFIFO_RST`"]
pub struct SPI_RX_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RX_AFIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SPI_DMA_TX_ENA`"]
pub type SPI_DMA_TX_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_TX_ENA`"]
pub struct SPI_DMA_TX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_TX_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_DMA_RX_ENA`"]
pub type SPI_DMA_RX_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_RX_ENA`"]
pub struct SPI_DMA_RX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_RX_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SPI_RX_EOF_EN`"]
pub type SPI_RX_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RX_EOF_EN`"]
pub struct SPI_RX_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RX_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_TX_SEG_TRANS_CLR_EN`"]
pub type SPI_SLV_TX_SEG_TRANS_CLR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_TX_SEG_TRANS_CLR_EN`"]
pub struct SPI_SLV_TX_SEG_TRANS_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_TX_SEG_TRANS_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_RX_SEG_TRANS_CLR_EN`"]
pub type SPI_SLV_RX_SEG_TRANS_CLR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_RX_SEG_TRANS_CLR_EN`"]
pub struct SPI_SLV_RX_SEG_TRANS_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RX_SEG_TRANS_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SPI_DMA_SLV_SEG_TRANS_EN`"]
pub type SPI_DMA_SLV_SEG_TRANS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_SLV_SEG_TRANS_EN`"]
pub struct SPI_DMA_SLV_SEG_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_SLV_SEG_TRANS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_dma_tx_ena(&self) -> SPI_DMA_TX_ENA_R {
        SPI_DMA_TX_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_dma_rx_ena(&self) -> SPI_DMA_RX_ENA_R {
        SPI_DMA_RX_ENA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_rx_eof_en(&self) -> SPI_RX_EOF_EN_R {
        SPI_RX_EOF_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_slv_tx_seg_trans_clr_en(&self) -> SPI_SLV_TX_SEG_TRANS_CLR_EN_R {
        SPI_SLV_TX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_slv_rx_seg_trans_clr_en(&self) -> SPI_SLV_RX_SEG_TRANS_CLR_EN_R {
        SPI_SLV_RX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_dma_slv_seg_trans_en(&self) -> SPI_DMA_SLV_SEG_TRANS_EN_R {
        SPI_DMA_SLV_SEG_TRANS_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_dma_afifo_rst(&mut self) -> SPI_DMA_AFIFO_RST_W {
        SPI_DMA_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_buf_afifo_rst(&mut self) -> SPI_BUF_AFIFO_RST_W {
        SPI_BUF_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_rx_afifo_rst(&mut self) -> SPI_RX_AFIFO_RST_W {
        SPI_RX_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_dma_tx_ena(&mut self) -> SPI_DMA_TX_ENA_W {
        SPI_DMA_TX_ENA_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_dma_rx_ena(&mut self) -> SPI_DMA_RX_ENA_W {
        SPI_DMA_RX_ENA_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_rx_eof_en(&mut self) -> SPI_RX_EOF_EN_W {
        SPI_RX_EOF_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_slv_tx_seg_trans_clr_en(&mut self) -> SPI_SLV_TX_SEG_TRANS_CLR_EN_W {
        SPI_SLV_TX_SEG_TRANS_CLR_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_slv_rx_seg_trans_clr_en(&mut self) -> SPI_SLV_RX_SEG_TRANS_CLR_EN_W {
        SPI_SLV_RX_SEG_TRANS_CLR_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_dma_slv_seg_trans_en(&mut self) -> SPI_DMA_SLV_SEG_TRANS_EN_W {
        SPI_DMA_SLV_SEG_TRANS_EN_W { w: self }
    }
}
