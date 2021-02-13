#[doc = "Reader of register DMA_INT_RAW_CH2"]
pub type R = crate::R<u32, super::DMA_INT_RAW_CH2>;
#[doc = "Writer for register DMA_INT_RAW_CH2"]
pub type W = crate::W<u32, super::DMA_INT_RAW_CH2>;
#[doc = "Register DMA_INT_RAW_CH2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INT_RAW_CH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_OUTFIFO_UDF_CH2_INT_RAW`"]
pub type DMA_OUTFIFO_UDF_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUTFIFO_UDF_CH2_INT_RAW`"]
pub struct DMA_OUTFIFO_UDF_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTFIFO_UDF_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_OUTFIFO_OVF_CH2_INT_RAW`"]
pub type DMA_OUTFIFO_OVF_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUTFIFO_OVF_CH2_INT_RAW`"]
pub struct DMA_OUTFIFO_OVF_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTFIFO_OVF_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_INFIFO_UDF_CH2_INT_RAW`"]
pub type DMA_INFIFO_UDF_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_INFIFO_UDF_CH2_INT_RAW`"]
pub struct DMA_INFIFO_UDF_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_UDF_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_INFIFO_OVF_CH2_INT_RAW`"]
pub type DMA_INFIFO_OVF_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_INFIFO_OVF_CH2_INT_RAW`"]
pub struct DMA_INFIFO_OVF_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_OVF_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_OUT_TOTAL_EOF_CH2_INT_RAW`"]
pub type DMA_OUT_TOTAL_EOF_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_TOTAL_EOF_CH2_INT_RAW`"]
pub struct DMA_OUT_TOTAL_EOF_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_TOTAL_EOF_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_IN_DSCR_EMPTY_CH2_INT_RAW`"]
pub type DMA_IN_DSCR_EMPTY_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_DSCR_EMPTY_CH2_INT_RAW`"]
pub struct DMA_IN_DSCR_EMPTY_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DSCR_EMPTY_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_OUT_DSCR_ERR_CH2_INT_RAW`"]
pub type DMA_OUT_DSCR_ERR_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_DSCR_ERR_CH2_INT_RAW`"]
pub struct DMA_OUT_DSCR_ERR_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_DSCR_ERR_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_IN_DSCR_ERR_CH2_INT_RAW`"]
pub type DMA_IN_DSCR_ERR_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_DSCR_ERR_CH2_INT_RAW`"]
pub struct DMA_IN_DSCR_ERR_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DSCR_ERR_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_OUT_EOF_CH2_INT_RAW`"]
pub type DMA_OUT_EOF_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_EOF_CH2_INT_RAW`"]
pub struct DMA_OUT_EOF_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_EOF_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_OUT_DONE_CH2_INT_RAW`"]
pub type DMA_OUT_DONE_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_DONE_CH2_INT_RAW`"]
pub struct DMA_OUT_DONE_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_DONE_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_IN_ERR_EOF_CH2_INT_RAW`"]
pub type DMA_IN_ERR_EOF_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_ERR_EOF_CH2_INT_RAW`"]
pub struct DMA_IN_ERR_EOF_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_ERR_EOF_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_IN_SUC_EOF_CH2_INT_RAW`"]
pub type DMA_IN_SUC_EOF_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_SUC_EOF_CH2_INT_RAW`"]
pub struct DMA_IN_SUC_EOF_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_SUC_EOF_CH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DMA_IN_DONE_CH2_INT_RAW`"]
pub type DMA_IN_DONE_CH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_DONE_CH2_INT_RAW`"]
pub struct DMA_IN_DONE_CH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DONE_CH2_INT_RAW_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_outfifo_udf_ch2_int_raw(&self) -> DMA_OUTFIFO_UDF_CH2_INT_RAW_R {
        DMA_OUTFIFO_UDF_CH2_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dma_outfifo_ovf_ch2_int_raw(&self) -> DMA_OUTFIFO_OVF_CH2_INT_RAW_R {
        DMA_OUTFIFO_OVF_CH2_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dma_infifo_udf_ch2_int_raw(&self) -> DMA_INFIFO_UDF_CH2_INT_RAW_R {
        DMA_INFIFO_UDF_CH2_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dma_infifo_ovf_ch2_int_raw(&self) -> DMA_INFIFO_OVF_CH2_INT_RAW_R {
        DMA_INFIFO_OVF_CH2_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_out_total_eof_ch2_int_raw(&self) -> DMA_OUT_TOTAL_EOF_CH2_INT_RAW_R {
        DMA_OUT_TOTAL_EOF_CH2_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dma_in_dscr_empty_ch2_int_raw(&self) -> DMA_IN_DSCR_EMPTY_CH2_INT_RAW_R {
        DMA_IN_DSCR_EMPTY_CH2_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_out_dscr_err_ch2_int_raw(&self) -> DMA_OUT_DSCR_ERR_CH2_INT_RAW_R {
        DMA_OUT_DSCR_ERR_CH2_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_in_dscr_err_ch2_int_raw(&self) -> DMA_IN_DSCR_ERR_CH2_INT_RAW_R {
        DMA_IN_DSCR_ERR_CH2_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_out_eof_ch2_int_raw(&self) -> DMA_OUT_EOF_CH2_INT_RAW_R {
        DMA_OUT_EOF_CH2_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_out_done_ch2_int_raw(&self) -> DMA_OUT_DONE_CH2_INT_RAW_R {
        DMA_OUT_DONE_CH2_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_in_err_eof_ch2_int_raw(&self) -> DMA_IN_ERR_EOF_CH2_INT_RAW_R {
        DMA_IN_ERR_EOF_CH2_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_in_suc_eof_ch2_int_raw(&self) -> DMA_IN_SUC_EOF_CH2_INT_RAW_R {
        DMA_IN_SUC_EOF_CH2_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_in_done_ch2_int_raw(&self) -> DMA_IN_DONE_CH2_INT_RAW_R {
        DMA_IN_DONE_CH2_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_outfifo_udf_ch2_int_raw(&mut self) -> DMA_OUTFIFO_UDF_CH2_INT_RAW_W {
        DMA_OUTFIFO_UDF_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dma_outfifo_ovf_ch2_int_raw(&mut self) -> DMA_OUTFIFO_OVF_CH2_INT_RAW_W {
        DMA_OUTFIFO_OVF_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dma_infifo_udf_ch2_int_raw(&mut self) -> DMA_INFIFO_UDF_CH2_INT_RAW_W {
        DMA_INFIFO_UDF_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dma_infifo_ovf_ch2_int_raw(&mut self) -> DMA_INFIFO_OVF_CH2_INT_RAW_W {
        DMA_INFIFO_OVF_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_out_total_eof_ch2_int_raw(&mut self) -> DMA_OUT_TOTAL_EOF_CH2_INT_RAW_W {
        DMA_OUT_TOTAL_EOF_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dma_in_dscr_empty_ch2_int_raw(&mut self) -> DMA_IN_DSCR_EMPTY_CH2_INT_RAW_W {
        DMA_IN_DSCR_EMPTY_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_out_dscr_err_ch2_int_raw(&mut self) -> DMA_OUT_DSCR_ERR_CH2_INT_RAW_W {
        DMA_OUT_DSCR_ERR_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_in_dscr_err_ch2_int_raw(&mut self) -> DMA_IN_DSCR_ERR_CH2_INT_RAW_W {
        DMA_IN_DSCR_ERR_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_out_eof_ch2_int_raw(&mut self) -> DMA_OUT_EOF_CH2_INT_RAW_W {
        DMA_OUT_EOF_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_out_done_ch2_int_raw(&mut self) -> DMA_OUT_DONE_CH2_INT_RAW_W {
        DMA_OUT_DONE_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_in_err_eof_ch2_int_raw(&mut self) -> DMA_IN_ERR_EOF_CH2_INT_RAW_W {
        DMA_IN_ERR_EOF_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_in_suc_eof_ch2_int_raw(&mut self) -> DMA_IN_SUC_EOF_CH2_INT_RAW_W {
        DMA_IN_SUC_EOF_CH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_in_done_ch2_int_raw(&mut self) -> DMA_IN_DONE_CH2_INT_RAW_W {
        DMA_IN_DONE_CH2_INT_RAW_W { w: self }
    }
}
