#[doc = "Reader of register RTC_I2C_INT_ENA"]
pub type R = crate::R<u32, super::RTC_I2C_INT_ENA>;
#[doc = "Writer for register RTC_I2C_INT_ENA"]
pub type W = crate::W<u32, super::RTC_I2C_INT_ENA>;
#[doc = "Register RTC_I2C_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_I2C_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_DETECT_START_INT_ENA`"]
pub type RTC_I2C_DETECT_START_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_DETECT_START_INT_ENA`"]
pub struct RTC_I2C_DETECT_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_DETECT_START_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_TX_DATA_INT_ENA`"]
pub type RTC_I2C_TX_DATA_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TX_DATA_INT_ENA`"]
pub struct RTC_I2C_TX_DATA_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TX_DATA_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_RX_DATA_INT_ENA`"]
pub type RTC_I2C_RX_DATA_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_RX_DATA_INT_ENA`"]
pub struct RTC_I2C_RX_DATA_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_RX_DATA_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_ACK_ERR_INT_ENA`"]
pub type RTC_I2C_ACK_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_ACK_ERR_INT_ENA`"]
pub struct RTC_I2C_ACK_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_ACK_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_TIMEOUT_INT_ENA`"]
pub type RTC_I2C_TIMEOUT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TIMEOUT_INT_ENA`"]
pub struct RTC_I2C_TIMEOUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TIMEOUT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_TRANS_COMPLETE_INT_ENA`"]
pub type RTC_I2C_TRANS_COMPLETE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TRANS_COMPLETE_INT_ENA`"]
pub struct RTC_I2C_TRANS_COMPLETE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TRANS_COMPLETE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_MASTER_TRAN_COMP_INT_ENA`"]
pub type RTC_I2C_MASTER_TRAN_COMP_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_MASTER_TRAN_COMP_INT_ENA`"]
pub struct RTC_I2C_MASTER_TRAN_COMP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_MASTER_TRAN_COMP_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_ARBITRATION_LOST_INT_ENA`"]
pub type RTC_I2C_ARBITRATION_LOST_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_ARBITRATION_LOST_INT_ENA`"]
pub struct RTC_I2C_ARBITRATION_LOST_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_ARBITRATION_LOST_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_SLAVE_TRAN_COMP_INT_ENA`"]
pub type RTC_I2C_SLAVE_TRAN_COMP_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SLAVE_TRAN_COMP_INT_ENA`"]
pub struct RTC_I2C_SLAVE_TRAN_COMP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_TRAN_COMP_INT_ENA_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_i2c_detect_start_int_ena(&self) -> RTC_I2C_DETECT_START_INT_ENA_R {
        RTC_I2C_DETECT_START_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_i2c_tx_data_int_ena(&self) -> RTC_I2C_TX_DATA_INT_ENA_R {
        RTC_I2C_TX_DATA_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_i2c_rx_data_int_ena(&self) -> RTC_I2C_RX_DATA_INT_ENA_R {
        RTC_I2C_RX_DATA_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_ack_err_int_ena(&self) -> RTC_I2C_ACK_ERR_INT_ENA_R {
        RTC_I2C_ACK_ERR_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_timeout_int_ena(&self) -> RTC_I2C_TIMEOUT_INT_ENA_R {
        RTC_I2C_TIMEOUT_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_i2c_trans_complete_int_ena(&self) -> RTC_I2C_TRANS_COMPLETE_INT_ENA_R {
        RTC_I2C_TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_i2c_master_tran_comp_int_ena(&self) -> RTC_I2C_MASTER_TRAN_COMP_INT_ENA_R {
        RTC_I2C_MASTER_TRAN_COMP_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_i2c_arbitration_lost_int_ena(&self) -> RTC_I2C_ARBITRATION_LOST_INT_ENA_R {
        RTC_I2C_ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_i2c_slave_tran_comp_int_ena(&self) -> RTC_I2C_SLAVE_TRAN_COMP_INT_ENA_R {
        RTC_I2C_SLAVE_TRAN_COMP_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_i2c_detect_start_int_ena(&mut self) -> RTC_I2C_DETECT_START_INT_ENA_W {
        RTC_I2C_DETECT_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_i2c_tx_data_int_ena(&mut self) -> RTC_I2C_TX_DATA_INT_ENA_W {
        RTC_I2C_TX_DATA_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_i2c_rx_data_int_ena(&mut self) -> RTC_I2C_RX_DATA_INT_ENA_W {
        RTC_I2C_RX_DATA_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_ack_err_int_ena(&mut self) -> RTC_I2C_ACK_ERR_INT_ENA_W {
        RTC_I2C_ACK_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_timeout_int_ena(&mut self) -> RTC_I2C_TIMEOUT_INT_ENA_W {
        RTC_I2C_TIMEOUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_i2c_trans_complete_int_ena(&mut self) -> RTC_I2C_TRANS_COMPLETE_INT_ENA_W {
        RTC_I2C_TRANS_COMPLETE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_i2c_master_tran_comp_int_ena(&mut self) -> RTC_I2C_MASTER_TRAN_COMP_INT_ENA_W {
        RTC_I2C_MASTER_TRAN_COMP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_i2c_arbitration_lost_int_ena(&mut self) -> RTC_I2C_ARBITRATION_LOST_INT_ENA_W {
        RTC_I2C_ARBITRATION_LOST_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_i2c_slave_tran_comp_int_ena(&mut self) -> RTC_I2C_SLAVE_TRAN_COMP_INT_ENA_W {
        RTC_I2C_SLAVE_TRAN_COMP_INT_ENA_W { w: self }
    }
}
