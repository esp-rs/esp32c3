#[doc = "Reader of register I2C_INT_STATUS"]
pub type R = crate::R<u32, super::I2C_INT_STATUS>;
#[doc = "Reader of field `I2C_GENERAL_CALL_INT_ST`"]
pub type I2C_GENERAL_CALL_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_SLAVE_STRETCH_INT_ST`"]
pub type I2C_SLAVE_STRETCH_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_DET_START_INT_ST`"]
pub type I2C_DET_START_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_SCL_MAIN_ST_TO_INT_ST`"]
pub type I2C_SCL_MAIN_ST_TO_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_SCL_ST_TO_INT_ST`"]
pub type I2C_SCL_ST_TO_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_RXFIFO_UDF_INT_ST`"]
pub type I2C_RXFIFO_UDF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_TXFIFO_OVF_INT_ST`"]
pub type I2C_TXFIFO_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_NACK_INT_ST`"]
pub type I2C_NACK_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_TRANS_START_INT_ST`"]
pub type I2C_TRANS_START_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_TIME_OUT_INT_ST`"]
pub type I2C_TIME_OUT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_TRANS_COMPLETE_INT_ST`"]
pub type I2C_TRANS_COMPLETE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_MST_TXFIFO_UDF_INT_ST`"]
pub type I2C_MST_TXFIFO_UDF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_ARBITRATION_LOST_INT_ST`"]
pub type I2C_ARBITRATION_LOST_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_BYTE_TRANS_DONE_INT_ST`"]
pub type I2C_BYTE_TRANS_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_END_DETECT_INT_ST`"]
pub type I2C_END_DETECT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_RXFIFO_OVF_INT_ST`"]
pub type I2C_RXFIFO_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_TXFIFO_WM_INT_ST`"]
pub type I2C_TXFIFO_WM_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_RXFIFO_WM_INT_ST`"]
pub type I2C_RXFIFO_WM_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2c_general_call_int_st(&self) -> I2C_GENERAL_CALL_INT_ST_R {
        I2C_GENERAL_CALL_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2c_slave_stretch_int_st(&self) -> I2C_SLAVE_STRETCH_INT_ST_R {
        I2C_SLAVE_STRETCH_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c_det_start_int_st(&self) -> I2C_DET_START_INT_ST_R {
        I2C_DET_START_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c_scl_main_st_to_int_st(&self) -> I2C_SCL_MAIN_ST_TO_INT_ST_R {
        I2C_SCL_MAIN_ST_TO_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c_scl_st_to_int_st(&self) -> I2C_SCL_ST_TO_INT_ST_R {
        I2C_SCL_ST_TO_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c_rxfifo_udf_int_st(&self) -> I2C_RXFIFO_UDF_INT_ST_R {
        I2C_RXFIFO_UDF_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c_txfifo_ovf_int_st(&self) -> I2C_TXFIFO_OVF_INT_ST_R {
        I2C_TXFIFO_OVF_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c_nack_int_st(&self) -> I2C_NACK_INT_ST_R {
        I2C_NACK_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2c_trans_start_int_st(&self) -> I2C_TRANS_START_INT_ST_R {
        I2C_TRANS_START_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_time_out_int_st(&self) -> I2C_TIME_OUT_INT_ST_R {
        I2C_TIME_OUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_trans_complete_int_st(&self) -> I2C_TRANS_COMPLETE_INT_ST_R {
        I2C_TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2c_mst_txfifo_udf_int_st(&self) -> I2C_MST_TXFIFO_UDF_INT_ST_R {
        I2C_MST_TXFIFO_UDF_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_arbitration_lost_int_st(&self) -> I2C_ARBITRATION_LOST_INT_ST_R {
        I2C_ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_byte_trans_done_int_st(&self) -> I2C_BYTE_TRANS_DONE_INT_ST_R {
        I2C_BYTE_TRANS_DONE_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_end_detect_int_st(&self) -> I2C_END_DETECT_INT_ST_R {
        I2C_END_DETECT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_rxfifo_ovf_int_st(&self) -> I2C_RXFIFO_OVF_INT_ST_R {
        I2C_RXFIFO_OVF_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_txfifo_wm_int_st(&self) -> I2C_TXFIFO_WM_INT_ST_R {
        I2C_TXFIFO_WM_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_rxfifo_wm_int_st(&self) -> I2C_RXFIFO_WM_INT_ST_R {
        I2C_RXFIFO_WM_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
