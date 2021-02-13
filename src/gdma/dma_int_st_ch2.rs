#[doc = "Reader of register DMA_INT_ST_CH2"]
pub type R = crate::R<u32, super::DMA_INT_ST_CH2>;
#[doc = "Reader of field `DMA_OUTFIFO_UDF_CH2_INT_ST`"]
pub type DMA_OUTFIFO_UDF_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUTFIFO_OVF_CH2_INT_ST`"]
pub type DMA_OUTFIFO_OVF_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_INFIFO_UDF_CH2_INT_ST`"]
pub type DMA_INFIFO_UDF_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_INFIFO_OVF_CH2_INT_ST`"]
pub type DMA_INFIFO_OVF_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUT_TOTAL_EOF_CH2_INT_ST`"]
pub type DMA_OUT_TOTAL_EOF_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_DSCR_EMPTY_CH2_INT_ST`"]
pub type DMA_IN_DSCR_EMPTY_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUT_DSCR_ERR_CH2_INT_ST`"]
pub type DMA_OUT_DSCR_ERR_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_DSCR_ERR_CH2_INT_ST`"]
pub type DMA_IN_DSCR_ERR_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUT_EOF_CH2_INT_ST`"]
pub type DMA_OUT_EOF_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUT_DONE_CH2_INT_ST`"]
pub type DMA_OUT_DONE_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_ERR_EOF_CH2_INT_ST`"]
pub type DMA_IN_ERR_EOF_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_SUC_EOF_CH2_INT_ST`"]
pub type DMA_IN_SUC_EOF_CH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_DONE_CH2_INT_ST`"]
pub type DMA_IN_DONE_CH2_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_outfifo_udf_ch2_int_st(&self) -> DMA_OUTFIFO_UDF_CH2_INT_ST_R {
        DMA_OUTFIFO_UDF_CH2_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dma_outfifo_ovf_ch2_int_st(&self) -> DMA_OUTFIFO_OVF_CH2_INT_ST_R {
        DMA_OUTFIFO_OVF_CH2_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dma_infifo_udf_ch2_int_st(&self) -> DMA_INFIFO_UDF_CH2_INT_ST_R {
        DMA_INFIFO_UDF_CH2_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dma_infifo_ovf_ch2_int_st(&self) -> DMA_INFIFO_OVF_CH2_INT_ST_R {
        DMA_INFIFO_OVF_CH2_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_out_total_eof_ch2_int_st(&self) -> DMA_OUT_TOTAL_EOF_CH2_INT_ST_R {
        DMA_OUT_TOTAL_EOF_CH2_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dma_in_dscr_empty_ch2_int_st(&self) -> DMA_IN_DSCR_EMPTY_CH2_INT_ST_R {
        DMA_IN_DSCR_EMPTY_CH2_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_out_dscr_err_ch2_int_st(&self) -> DMA_OUT_DSCR_ERR_CH2_INT_ST_R {
        DMA_OUT_DSCR_ERR_CH2_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_in_dscr_err_ch2_int_st(&self) -> DMA_IN_DSCR_ERR_CH2_INT_ST_R {
        DMA_IN_DSCR_ERR_CH2_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_out_eof_ch2_int_st(&self) -> DMA_OUT_EOF_CH2_INT_ST_R {
        DMA_OUT_EOF_CH2_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_out_done_ch2_int_st(&self) -> DMA_OUT_DONE_CH2_INT_ST_R {
        DMA_OUT_DONE_CH2_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_in_err_eof_ch2_int_st(&self) -> DMA_IN_ERR_EOF_CH2_INT_ST_R {
        DMA_IN_ERR_EOF_CH2_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_in_suc_eof_ch2_int_st(&self) -> DMA_IN_SUC_EOF_CH2_INT_ST_R {
        DMA_IN_SUC_EOF_CH2_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_in_done_ch2_int_st(&self) -> DMA_IN_DONE_CH2_INT_ST_R {
        DMA_IN_DONE_CH2_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
