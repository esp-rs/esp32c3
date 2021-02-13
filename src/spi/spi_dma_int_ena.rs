#[doc = "Reader of register SPI_DMA_INT_ENA"]
pub type R = crate::R<u32, super::SPI_DMA_INT_ENA>;
#[doc = "Writer for register SPI_DMA_INT_ENA"]
pub type W = crate::W<u32, super::SPI_DMA_INT_ENA>;
#[doc = "Register SPI_DMA_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DMA_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_APP1_INT_ENA`"]
pub type SPI_APP1_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_APP1_INT_ENA`"]
pub struct SPI_APP1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_APP1_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_APP2_INT_ENA`"]
pub type SPI_APP2_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_APP2_INT_ENA`"]
pub struct SPI_APP2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_APP2_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA`"]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA`"]
pub struct SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA`"]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA`"]
pub struct SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_CMD_ERR_INT_ENA`"]
pub type SPI_SLV_CMD_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_CMD_ERR_INT_ENA`"]
pub struct SPI_SLV_CMD_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMD_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_BUF_ADDR_ERR_INT_ENA`"]
pub type SPI_SLV_BUF_ADDR_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_BUF_ADDR_ERR_INT_ENA`"]
pub struct SPI_SLV_BUF_ADDR_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_BUF_ADDR_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SEG_MAGIC_ERR_INT_ENA`"]
pub type SPI_SEG_MAGIC_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SEG_MAGIC_ERR_INT_ENA`"]
pub struct SPI_SEG_MAGIC_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SEG_MAGIC_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_DMA_SEG_TRANS_DONE_INT_ENA`"]
pub type SPI_DMA_SEG_TRANS_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_SEG_TRANS_DONE_INT_ENA`"]
pub struct SPI_DMA_SEG_TRANS_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_SEG_TRANS_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_TRANS_DONE_INT_ENA`"]
pub type SPI_TRANS_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_TRANS_DONE_INT_ENA`"]
pub struct SPI_TRANS_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TRANS_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_WR_BUF_DONE_INT_ENA`"]
pub type SPI_SLV_WR_BUF_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_WR_BUF_DONE_INT_ENA`"]
pub struct SPI_SLV_WR_BUF_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WR_BUF_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_RD_BUF_DONE_INT_ENA`"]
pub type SPI_SLV_RD_BUF_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_RD_BUF_DONE_INT_ENA`"]
pub struct SPI_SLV_RD_BUF_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RD_BUF_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_WR_DMA_DONE_INT_ENA`"]
pub type SPI_SLV_WR_DMA_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_WR_DMA_DONE_INT_ENA`"]
pub struct SPI_SLV_WR_DMA_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WR_DMA_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_RD_DMA_DONE_INT_ENA`"]
pub type SPI_SLV_RD_DMA_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_RD_DMA_DONE_INT_ENA`"]
pub struct SPI_SLV_RD_DMA_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RD_DMA_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_CMDA_INT_ENA`"]
pub type SPI_SLV_CMDA_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_CMDA_INT_ENA`"]
pub struct SPI_SLV_CMDA_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMDA_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_CMD9_INT_ENA`"]
pub type SPI_SLV_CMD9_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_CMD9_INT_ENA`"]
pub struct SPI_SLV_CMD9_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMD9_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_CMD8_INT_ENA`"]
pub type SPI_SLV_CMD8_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_CMD8_INT_ENA`"]
pub struct SPI_SLV_CMD8_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMD8_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_CMD7_INT_ENA`"]
pub type SPI_SLV_CMD7_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_CMD7_INT_ENA`"]
pub struct SPI_SLV_CMD7_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_CMD7_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_EN_QPI_INT_ENA`"]
pub type SPI_SLV_EN_QPI_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_EN_QPI_INT_ENA`"]
pub struct SPI_SLV_EN_QPI_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_EN_QPI_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_EX_QPI_INT_ENA`"]
pub type SPI_SLV_EX_QPI_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_EX_QPI_INT_ENA`"]
pub struct SPI_SLV_EX_QPI_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_EX_QPI_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA`"]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA`"]
pub struct SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_DMA_INFIFO_FULL_ERR_INT_ENA`"]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_INFIFO_FULL_ERR_INT_ENA`"]
pub struct SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W<'a> {
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
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_app1_int_ena(&self) -> SPI_APP1_INT_ENA_R {
        SPI_APP1_INT_ENA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_app2_int_ena(&self) -> SPI_APP2_INT_ENA_R {
        SPI_APP2_INT_ENA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_ena(&self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_ena(&self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_ena(&self) -> SPI_SLV_CMD_ERR_INT_ENA_R {
        SPI_SLV_CMD_ERR_INT_ENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_ena(&self) -> SPI_SLV_BUF_ADDR_ERR_INT_ENA_R {
        SPI_SLV_BUF_ADDR_ERR_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_ena(&self) -> SPI_SEG_MAGIC_ERR_INT_ENA_R {
        SPI_SEG_MAGIC_ERR_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_ena(&self) -> SPI_DMA_SEG_TRANS_DONE_INT_ENA_R {
        SPI_DMA_SEG_TRANS_DONE_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_trans_done_int_ena(&self) -> SPI_TRANS_DONE_INT_ENA_R {
        SPI_TRANS_DONE_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_ena(&self) -> SPI_SLV_WR_BUF_DONE_INT_ENA_R {
        SPI_SLV_WR_BUF_DONE_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_ena(&self) -> SPI_SLV_RD_BUF_DONE_INT_ENA_R {
        SPI_SLV_RD_BUF_DONE_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_ena(&self) -> SPI_SLV_WR_DMA_DONE_INT_ENA_R {
        SPI_SLV_WR_DMA_DONE_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_ena(&self) -> SPI_SLV_RD_DMA_DONE_INT_ENA_R {
        SPI_SLV_RD_DMA_DONE_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_slv_cmda_int_ena(&self) -> SPI_SLV_CMDA_INT_ENA_R {
        SPI_SLV_CMDA_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_ena(&self) -> SPI_SLV_CMD9_INT_ENA_R {
        SPI_SLV_CMD9_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_ena(&self) -> SPI_SLV_CMD8_INT_ENA_R {
        SPI_SLV_CMD8_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_ena(&self) -> SPI_SLV_CMD7_INT_ENA_R {
        SPI_SLV_CMD7_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_ena(&self) -> SPI_SLV_EN_QPI_INT_ENA_R {
        SPI_SLV_EN_QPI_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_ena(&self) -> SPI_SLV_EX_QPI_INT_ENA_R {
        SPI_SLV_EX_QPI_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_ena(&self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_ena(&self) -> SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R {
        SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_app1_int_ena(&mut self) -> SPI_APP1_INT_ENA_W {
        SPI_APP1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_app2_int_ena(&mut self) -> SPI_APP2_INT_ENA_W {
        SPI_APP2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_ena(&mut self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_ena(&mut self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_ena(&mut self) -> SPI_SLV_CMD_ERR_INT_ENA_W {
        SPI_SLV_CMD_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_ena(&mut self) -> SPI_SLV_BUF_ADDR_ERR_INT_ENA_W {
        SPI_SLV_BUF_ADDR_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_ena(&mut self) -> SPI_SEG_MAGIC_ERR_INT_ENA_W {
        SPI_SEG_MAGIC_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_ena(&mut self) -> SPI_DMA_SEG_TRANS_DONE_INT_ENA_W {
        SPI_DMA_SEG_TRANS_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_trans_done_int_ena(&mut self) -> SPI_TRANS_DONE_INT_ENA_W {
        SPI_TRANS_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_ena(&mut self) -> SPI_SLV_WR_BUF_DONE_INT_ENA_W {
        SPI_SLV_WR_BUF_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_ena(&mut self) -> SPI_SLV_RD_BUF_DONE_INT_ENA_W {
        SPI_SLV_RD_BUF_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_ena(&mut self) -> SPI_SLV_WR_DMA_DONE_INT_ENA_W {
        SPI_SLV_WR_DMA_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_ena(&mut self) -> SPI_SLV_RD_DMA_DONE_INT_ENA_W {
        SPI_SLV_RD_DMA_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_slv_cmda_int_ena(&mut self) -> SPI_SLV_CMDA_INT_ENA_W {
        SPI_SLV_CMDA_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_ena(&mut self) -> SPI_SLV_CMD9_INT_ENA_W {
        SPI_SLV_CMD9_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_ena(&mut self) -> SPI_SLV_CMD8_INT_ENA_W {
        SPI_SLV_CMD8_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_ena(&mut self) -> SPI_SLV_CMD7_INT_ENA_W {
        SPI_SLV_CMD7_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_ena(&mut self) -> SPI_SLV_EN_QPI_INT_ENA_W {
        SPI_SLV_EN_QPI_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_ena(&mut self) -> SPI_SLV_EX_QPI_INT_ENA_W {
        SPI_SLV_EX_QPI_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_ena(&mut self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_ena(&mut self) -> SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W {
        SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W { w: self }
    }
}
