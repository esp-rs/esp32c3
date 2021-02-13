#[doc = "Reader of register I2C_INT_RAW"]
pub type R = crate::R<u32, super::I2C_INT_RAW>;
#[doc = "Writer for register I2C_INT_RAW"]
pub type W = crate::W<u32, super::I2C_INT_RAW>;
#[doc = "Register I2C_INT_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_INT_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_GENERAL_CALL_INT_RAW`"]
pub type I2C_GENERAL_CALL_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_GENERAL_CALL_INT_RAW`"]
pub struct I2C_GENERAL_CALL_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_GENERAL_CALL_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_SLAVE_STRETCH_INT_RAW`"]
pub type I2C_SLAVE_STRETCH_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SLAVE_STRETCH_INT_RAW`"]
pub struct I2C_SLAVE_STRETCH_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_STRETCH_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_DET_START_INT_RAW`"]
pub type I2C_DET_START_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_DET_START_INT_RAW`"]
pub struct I2C_DET_START_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_DET_START_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_SCL_MAIN_ST_TO_INT_RAW`"]
pub type I2C_SCL_MAIN_ST_TO_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SCL_MAIN_ST_TO_INT_RAW`"]
pub struct I2C_SCL_MAIN_ST_TO_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_MAIN_ST_TO_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_SCL_ST_TO_INT_RAW`"]
pub type I2C_SCL_ST_TO_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SCL_ST_TO_INT_RAW`"]
pub struct I2C_SCL_ST_TO_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_ST_TO_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_RXFIFO_UDF_INT_RAW`"]
pub type I2C_RXFIFO_UDF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RXFIFO_UDF_INT_RAW`"]
pub struct I2C_RXFIFO_UDF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_UDF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_TXFIFO_OVF_INT_RAW`"]
pub type I2C_TXFIFO_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TXFIFO_OVF_INT_RAW`"]
pub struct I2C_TXFIFO_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TXFIFO_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_NACK_INT_RAW`"]
pub type I2C_NACK_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_NACK_INT_RAW`"]
pub struct I2C_NACK_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_NACK_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_TRANS_START_INT_RAW`"]
pub type I2C_TRANS_START_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TRANS_START_INT_RAW`"]
pub struct I2C_TRANS_START_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TRANS_START_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_TIME_OUT_INT_RAW`"]
pub type I2C_TIME_OUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TIME_OUT_INT_RAW`"]
pub struct I2C_TIME_OUT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TIME_OUT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_TRANS_COMPLETE_INT_RAW`"]
pub type I2C_TRANS_COMPLETE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TRANS_COMPLETE_INT_RAW`"]
pub struct I2C_TRANS_COMPLETE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TRANS_COMPLETE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_MST_TXFIFO_UDF_INT_RAW`"]
pub type I2C_MST_TXFIFO_UDF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MST_TXFIFO_UDF_INT_RAW`"]
pub struct I2C_MST_TXFIFO_UDF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MST_TXFIFO_UDF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_ARBITRATION_LOST_INT_RAW`"]
pub type I2C_ARBITRATION_LOST_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ARBITRATION_LOST_INT_RAW`"]
pub struct I2C_ARBITRATION_LOST_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ARBITRATION_LOST_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_BYTE_TRANS_DONE_INT_RAW`"]
pub type I2C_BYTE_TRANS_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_BYTE_TRANS_DONE_INT_RAW`"]
pub struct I2C_BYTE_TRANS_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_BYTE_TRANS_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_END_DETECT_INT_RAW`"]
pub type I2C_END_DETECT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_END_DETECT_INT_RAW`"]
pub struct I2C_END_DETECT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_END_DETECT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_RXFIFO_OVF_INT_RAW`"]
pub type I2C_RXFIFO_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RXFIFO_OVF_INT_RAW`"]
pub struct I2C_RXFIFO_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_TXFIFO_WM_INT_RAW`"]
pub type I2C_TXFIFO_WM_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TXFIFO_WM_INT_RAW`"]
pub struct I2C_TXFIFO_WM_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TXFIFO_WM_INT_RAW_W<'a> {
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
#[doc = "Reader of field `I2C_RXFIFO_WM_INT_RAW`"]
pub type I2C_RXFIFO_WM_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RXFIFO_WM_INT_RAW`"]
pub struct I2C_RXFIFO_WM_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_WM_INT_RAW_W<'a> {
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
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2c_general_call_int_raw(&self) -> I2C_GENERAL_CALL_INT_RAW_R {
        I2C_GENERAL_CALL_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2c_slave_stretch_int_raw(&self) -> I2C_SLAVE_STRETCH_INT_RAW_R {
        I2C_SLAVE_STRETCH_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c_det_start_int_raw(&self) -> I2C_DET_START_INT_RAW_R {
        I2C_DET_START_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c_scl_main_st_to_int_raw(&self) -> I2C_SCL_MAIN_ST_TO_INT_RAW_R {
        I2C_SCL_MAIN_ST_TO_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c_scl_st_to_int_raw(&self) -> I2C_SCL_ST_TO_INT_RAW_R {
        I2C_SCL_ST_TO_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c_rxfifo_udf_int_raw(&self) -> I2C_RXFIFO_UDF_INT_RAW_R {
        I2C_RXFIFO_UDF_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c_txfifo_ovf_int_raw(&self) -> I2C_TXFIFO_OVF_INT_RAW_R {
        I2C_TXFIFO_OVF_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c_nack_int_raw(&self) -> I2C_NACK_INT_RAW_R {
        I2C_NACK_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2c_trans_start_int_raw(&self) -> I2C_TRANS_START_INT_RAW_R {
        I2C_TRANS_START_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_time_out_int_raw(&self) -> I2C_TIME_OUT_INT_RAW_R {
        I2C_TIME_OUT_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_trans_complete_int_raw(&self) -> I2C_TRANS_COMPLETE_INT_RAW_R {
        I2C_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2c_mst_txfifo_udf_int_raw(&self) -> I2C_MST_TXFIFO_UDF_INT_RAW_R {
        I2C_MST_TXFIFO_UDF_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_arbitration_lost_int_raw(&self) -> I2C_ARBITRATION_LOST_INT_RAW_R {
        I2C_ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_byte_trans_done_int_raw(&self) -> I2C_BYTE_TRANS_DONE_INT_RAW_R {
        I2C_BYTE_TRANS_DONE_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_end_detect_int_raw(&self) -> I2C_END_DETECT_INT_RAW_R {
        I2C_END_DETECT_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_rxfifo_ovf_int_raw(&self) -> I2C_RXFIFO_OVF_INT_RAW_R {
        I2C_RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_txfifo_wm_int_raw(&self) -> I2C_TXFIFO_WM_INT_RAW_R {
        I2C_TXFIFO_WM_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_rxfifo_wm_int_raw(&self) -> I2C_RXFIFO_WM_INT_RAW_R {
        I2C_RXFIFO_WM_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2c_general_call_int_raw(&mut self) -> I2C_GENERAL_CALL_INT_RAW_W {
        I2C_GENERAL_CALL_INT_RAW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2c_slave_stretch_int_raw(&mut self) -> I2C_SLAVE_STRETCH_INT_RAW_W {
        I2C_SLAVE_STRETCH_INT_RAW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c_det_start_int_raw(&mut self) -> I2C_DET_START_INT_RAW_W {
        I2C_DET_START_INT_RAW_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c_scl_main_st_to_int_raw(&mut self) -> I2C_SCL_MAIN_ST_TO_INT_RAW_W {
        I2C_SCL_MAIN_ST_TO_INT_RAW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c_scl_st_to_int_raw(&mut self) -> I2C_SCL_ST_TO_INT_RAW_W {
        I2C_SCL_ST_TO_INT_RAW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c_rxfifo_udf_int_raw(&mut self) -> I2C_RXFIFO_UDF_INT_RAW_W {
        I2C_RXFIFO_UDF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c_txfifo_ovf_int_raw(&mut self) -> I2C_TXFIFO_OVF_INT_RAW_W {
        I2C_TXFIFO_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c_nack_int_raw(&mut self) -> I2C_NACK_INT_RAW_W {
        I2C_NACK_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2c_trans_start_int_raw(&mut self) -> I2C_TRANS_START_INT_RAW_W {
        I2C_TRANS_START_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_time_out_int_raw(&mut self) -> I2C_TIME_OUT_INT_RAW_W {
        I2C_TIME_OUT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_trans_complete_int_raw(&mut self) -> I2C_TRANS_COMPLETE_INT_RAW_W {
        I2C_TRANS_COMPLETE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2c_mst_txfifo_udf_int_raw(&mut self) -> I2C_MST_TXFIFO_UDF_INT_RAW_W {
        I2C_MST_TXFIFO_UDF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_arbitration_lost_int_raw(&mut self) -> I2C_ARBITRATION_LOST_INT_RAW_W {
        I2C_ARBITRATION_LOST_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_byte_trans_done_int_raw(&mut self) -> I2C_BYTE_TRANS_DONE_INT_RAW_W {
        I2C_BYTE_TRANS_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_end_detect_int_raw(&mut self) -> I2C_END_DETECT_INT_RAW_W {
        I2C_END_DETECT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_rxfifo_ovf_int_raw(&mut self) -> I2C_RXFIFO_OVF_INT_RAW_W {
        I2C_RXFIFO_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_txfifo_wm_int_raw(&mut self) -> I2C_TXFIFO_WM_INT_RAW_W {
        I2C_TXFIFO_WM_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_rxfifo_wm_int_raw(&mut self) -> I2C_RXFIFO_WM_INT_RAW_W {
        I2C_RXFIFO_WM_INT_RAW_W { w: self }
    }
}
