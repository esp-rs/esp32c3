#[doc = "Writer for register RTC_I2C_INT_CLR"]
pub type W = crate::W<u32, super::RTC_I2C_INT_CLR>;
#[doc = "Register RTC_I2C_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_I2C_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RTC_I2C_DETECT_START_INT_CLR`"]
pub struct RTC_I2C_DETECT_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_DETECT_START_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_I2C_TX_DATA_INT_CLR`"]
pub struct RTC_I2C_TX_DATA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TX_DATA_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_I2C_RX_DATA_INT_CLR`"]
pub struct RTC_I2C_RX_DATA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_RX_DATA_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_I2C_ACK_ERR_INT_CLR`"]
pub struct RTC_I2C_ACK_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_ACK_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_I2C_TIMEOUT_INT_CLR`"]
pub struct RTC_I2C_TIMEOUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TIMEOUT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_I2C_TRANS_COMPLETE_INT_CLR`"]
pub struct RTC_I2C_TRANS_COMPLETE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TRANS_COMPLETE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_I2C_MASTER_TRAN_COMP_INT_CLR`"]
pub struct RTC_I2C_MASTER_TRAN_COMP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_MASTER_TRAN_COMP_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_I2C_ARBITRATION_LOST_INT_CLR`"]
pub struct RTC_I2C_ARBITRATION_LOST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_ARBITRATION_LOST_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_I2C_SLAVE_TRAN_COMP_INT_CLR`"]
pub struct RTC_I2C_SLAVE_TRAN_COMP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_TRAN_COMP_INT_CLR_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_i2c_detect_start_int_clr(&mut self) -> RTC_I2C_DETECT_START_INT_CLR_W {
        RTC_I2C_DETECT_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_i2c_tx_data_int_clr(&mut self) -> RTC_I2C_TX_DATA_INT_CLR_W {
        RTC_I2C_TX_DATA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_i2c_rx_data_int_clr(&mut self) -> RTC_I2C_RX_DATA_INT_CLR_W {
        RTC_I2C_RX_DATA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_ack_err_int_clr(&mut self) -> RTC_I2C_ACK_ERR_INT_CLR_W {
        RTC_I2C_ACK_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_timeout_int_clr(&mut self) -> RTC_I2C_TIMEOUT_INT_CLR_W {
        RTC_I2C_TIMEOUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_i2c_trans_complete_int_clr(&mut self) -> RTC_I2C_TRANS_COMPLETE_INT_CLR_W {
        RTC_I2C_TRANS_COMPLETE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_i2c_master_tran_comp_int_clr(&mut self) -> RTC_I2C_MASTER_TRAN_COMP_INT_CLR_W {
        RTC_I2C_MASTER_TRAN_COMP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_i2c_arbitration_lost_int_clr(&mut self) -> RTC_I2C_ARBITRATION_LOST_INT_CLR_W {
        RTC_I2C_ARBITRATION_LOST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_i2c_slave_tran_comp_int_clr(&mut self) -> RTC_I2C_SLAVE_TRAN_COMP_INT_CLR_W {
        RTC_I2C_SLAVE_TRAN_COMP_INT_CLR_W { w: self }
    }
}
