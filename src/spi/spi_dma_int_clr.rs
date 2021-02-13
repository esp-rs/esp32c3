#[doc = "Writer for register SPI_DMA_INT_CLR"]
pub type W = crate::W<u32, super::SPI_DMA_INT_CLR>;
#[doc = "Register SPI_DMA_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DMA_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SPI_APP1_INT_CLR`"]
pub struct SPI_APP1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_APP1_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_APP2_INT_CLR`"]
pub struct SPI_APP2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_APP2_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR`"]
pub struct SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR`"]
pub struct SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SLV_CMD_ERR_INT_CLR`"]
pub struct SPI_SLV_CMD_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMD_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SLV_BUF_ADDR_ERR_INT_CLR`"]
pub struct SPI_SLV_BUF_ADDR_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_BUF_ADDR_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SEG_MAGIC_ERR_INT_CLR`"]
pub struct SPI_SEG_MAGIC_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SEG_MAGIC_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_DMA_SEG_TRANS_DONE_INT_CLR`"]
pub struct SPI_DMA_SEG_TRANS_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_SEG_TRANS_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_TRANS_DONE_INT_CLR`"]
pub struct SPI_TRANS_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TRANS_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_SLV_WR_BUF_DONE_INT_CLR`"]
pub struct SPI_SLV_WR_BUF_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WR_BUF_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SLV_RD_BUF_DONE_INT_CLR`"]
pub struct SPI_SLV_RD_BUF_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RD_BUF_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SLV_WR_DMA_DONE_INT_CLR`"]
pub struct SPI_SLV_WR_DMA_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WR_DMA_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SLV_RD_DMA_DONE_INT_CLR`"]
pub struct SPI_SLV_RD_DMA_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RD_DMA_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SLV_CMDA_INT_CLR`"]
pub struct SPI_SLV_CMDA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMDA_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SLV_CMD9_INT_CLR`"]
pub struct SPI_SLV_CMD9_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMD9_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SLV_CMD8_INT_CLR`"]
pub struct SPI_SLV_CMD8_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMD8_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_SLV_CMD7_INT_CLR`"]
pub struct SPI_SLV_CMD7_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMD7_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_SLV_EN_QPI_INT_CLR`"]
pub struct SPI_SLV_EN_QPI_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_EN_QPI_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_SLV_EX_QPI_INT_CLR`"]
pub struct SPI_SLV_EX_QPI_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_EX_QPI_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR`"]
pub struct SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_DMA_INFIFO_FULL_ERR_INT_CLR`"]
pub struct SPI_DMA_INFIFO_FULL_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_INFIFO_FULL_ERR_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_app1_int_clr(&mut self) -> SPI_APP1_INT_CLR_W {
        SPI_APP1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_app2_int_clr(&mut self) -> SPI_APP2_INT_CLR_W {
        SPI_APP2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_clr(&mut self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_clr(&mut self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_clr(&mut self) -> SPI_SLV_CMD_ERR_INT_CLR_W {
        SPI_SLV_CMD_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_clr(&mut self) -> SPI_SLV_BUF_ADDR_ERR_INT_CLR_W {
        SPI_SLV_BUF_ADDR_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_clr(&mut self) -> SPI_SEG_MAGIC_ERR_INT_CLR_W {
        SPI_SEG_MAGIC_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_clr(&mut self) -> SPI_DMA_SEG_TRANS_DONE_INT_CLR_W {
        SPI_DMA_SEG_TRANS_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_trans_done_int_clr(&mut self) -> SPI_TRANS_DONE_INT_CLR_W {
        SPI_TRANS_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_clr(&mut self) -> SPI_SLV_WR_BUF_DONE_INT_CLR_W {
        SPI_SLV_WR_BUF_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_clr(&mut self) -> SPI_SLV_RD_BUF_DONE_INT_CLR_W {
        SPI_SLV_RD_BUF_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_clr(&mut self) -> SPI_SLV_WR_DMA_DONE_INT_CLR_W {
        SPI_SLV_WR_DMA_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_clr(&mut self) -> SPI_SLV_RD_DMA_DONE_INT_CLR_W {
        SPI_SLV_RD_DMA_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_slv_cmda_int_clr(&mut self) -> SPI_SLV_CMDA_INT_CLR_W {
        SPI_SLV_CMDA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_clr(&mut self) -> SPI_SLV_CMD9_INT_CLR_W {
        SPI_SLV_CMD9_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_clr(&mut self) -> SPI_SLV_CMD8_INT_CLR_W {
        SPI_SLV_CMD8_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_clr(&mut self) -> SPI_SLV_CMD7_INT_CLR_W {
        SPI_SLV_CMD7_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_clr(&mut self) -> SPI_SLV_EN_QPI_INT_CLR_W {
        SPI_SLV_EN_QPI_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_clr(&mut self) -> SPI_SLV_EX_QPI_INT_CLR_W {
        SPI_SLV_EX_QPI_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_clr(&mut self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_clr(&mut self) -> SPI_DMA_INFIFO_FULL_ERR_INT_CLR_W {
        SPI_DMA_INFIFO_FULL_ERR_INT_CLR_W { w: self }
    }
}
