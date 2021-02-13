#[doc = "Reader of register SPI_DMA_INT_ST"]
pub type R = crate::R<u32, super::SPI_DMA_INT_ST>;
#[doc = "Reader of field `SPI_APP1_INT_ST`"]
pub type SPI_APP1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_APP2_INT_ST`"]
pub type SPI_APP2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST`"]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST`"]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_CMD_ERR_INT_ST`"]
pub type SPI_SLV_CMD_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_BUF_ADDR_ERR_INT_ST`"]
pub type SPI_SLV_BUF_ADDR_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SEG_MAGIC_ERR_INT_ST`"]
pub type SPI_SEG_MAGIC_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_DMA_SEG_TRANS_DONE_INT_ST`"]
pub type SPI_DMA_SEG_TRANS_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_TRANS_DONE_INT_ST`"]
pub type SPI_TRANS_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_WR_BUF_DONE_INT_ST`"]
pub type SPI_SLV_WR_BUF_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_RD_BUF_DONE_INT_ST`"]
pub type SPI_SLV_RD_BUF_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_WR_DMA_DONE_INT_ST`"]
pub type SPI_SLV_WR_DMA_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_RD_DMA_DONE_INT_ST`"]
pub type SPI_SLV_RD_DMA_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_CMDA_INT_ST`"]
pub type SPI_SLV_CMDA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_CMD9_INT_ST`"]
pub type SPI_SLV_CMD9_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_CMD8_INT_ST`"]
pub type SPI_SLV_CMD8_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_CMD7_INT_ST`"]
pub type SPI_SLV_CMD7_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_EN_QPI_INT_ST`"]
pub type SPI_SLV_EN_QPI_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_SLV_EX_QPI_INT_ST`"]
pub type SPI_SLV_EX_QPI_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST`"]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_DMA_INFIFO_FULL_ERR_INT_ST`"]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_app1_int_st(&self) -> SPI_APP1_INT_ST_R {
        SPI_APP1_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_app2_int_st(&self) -> SPI_APP2_INT_ST_R {
        SPI_APP2_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_st(&self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_st(&self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_st(&self) -> SPI_SLV_CMD_ERR_INT_ST_R {
        SPI_SLV_CMD_ERR_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_st(&self) -> SPI_SLV_BUF_ADDR_ERR_INT_ST_R {
        SPI_SLV_BUF_ADDR_ERR_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_st(&self) -> SPI_SEG_MAGIC_ERR_INT_ST_R {
        SPI_SEG_MAGIC_ERR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_st(&self) -> SPI_DMA_SEG_TRANS_DONE_INT_ST_R {
        SPI_DMA_SEG_TRANS_DONE_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_trans_done_int_st(&self) -> SPI_TRANS_DONE_INT_ST_R {
        SPI_TRANS_DONE_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_st(&self) -> SPI_SLV_WR_BUF_DONE_INT_ST_R {
        SPI_SLV_WR_BUF_DONE_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_st(&self) -> SPI_SLV_RD_BUF_DONE_INT_ST_R {
        SPI_SLV_RD_BUF_DONE_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_st(&self) -> SPI_SLV_WR_DMA_DONE_INT_ST_R {
        SPI_SLV_WR_DMA_DONE_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_st(&self) -> SPI_SLV_RD_DMA_DONE_INT_ST_R {
        SPI_SLV_RD_DMA_DONE_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_slv_cmda_int_st(&self) -> SPI_SLV_CMDA_INT_ST_R {
        SPI_SLV_CMDA_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_st(&self) -> SPI_SLV_CMD9_INT_ST_R {
        SPI_SLV_CMD9_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_st(&self) -> SPI_SLV_CMD8_INT_ST_R {
        SPI_SLV_CMD8_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_st(&self) -> SPI_SLV_CMD7_INT_ST_R {
        SPI_SLV_CMD7_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_st(&self) -> SPI_SLV_EN_QPI_INT_ST_R {
        SPI_SLV_EN_QPI_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_st(&self) -> SPI_SLV_EX_QPI_INT_ST_R {
        SPI_SLV_EX_QPI_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_st(&self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_R {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_st(&self) -> SPI_DMA_INFIFO_FULL_ERR_INT_ST_R {
        SPI_DMA_INFIFO_FULL_ERR_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
