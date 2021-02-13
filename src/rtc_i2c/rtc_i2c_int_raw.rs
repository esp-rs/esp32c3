#[doc = "Reader of register RTC_I2C_INT_RAW"]
pub type R = crate::R<u32, super::RTC_I2C_INT_RAW>;
#[doc = "Reader of field `RTC_I2C_DETECT_START_INT_RAW`"]
pub type RTC_I2C_DETECT_START_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_TX_DATA_INT_RAW`"]
pub type RTC_I2C_TX_DATA_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_RX_DATA_INT_RAW`"]
pub type RTC_I2C_RX_DATA_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_ACK_ERR_INT_RAW`"]
pub type RTC_I2C_ACK_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_TIMEOUT_INT_RAW`"]
pub type RTC_I2C_TIMEOUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_TRANS_COMPLETE_INT_RAW`"]
pub type RTC_I2C_TRANS_COMPLETE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_MASTER_TRAN_COMP_INT_RAW`"]
pub type RTC_I2C_MASTER_TRAN_COMP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_ARBITRATION_LOST_INT_RAW`"]
pub type RTC_I2C_ARBITRATION_LOST_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_SLAVE_TRAN_COMP_INT_RAW`"]
pub type RTC_I2C_SLAVE_TRAN_COMP_INT_RAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_i2c_detect_start_int_raw(&self) -> RTC_I2C_DETECT_START_INT_RAW_R {
        RTC_I2C_DETECT_START_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_i2c_tx_data_int_raw(&self) -> RTC_I2C_TX_DATA_INT_RAW_R {
        RTC_I2C_TX_DATA_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_i2c_rx_data_int_raw(&self) -> RTC_I2C_RX_DATA_INT_RAW_R {
        RTC_I2C_RX_DATA_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_ack_err_int_raw(&self) -> RTC_I2C_ACK_ERR_INT_RAW_R {
        RTC_I2C_ACK_ERR_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_timeout_int_raw(&self) -> RTC_I2C_TIMEOUT_INT_RAW_R {
        RTC_I2C_TIMEOUT_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_i2c_trans_complete_int_raw(&self) -> RTC_I2C_TRANS_COMPLETE_INT_RAW_R {
        RTC_I2C_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_i2c_master_tran_comp_int_raw(&self) -> RTC_I2C_MASTER_TRAN_COMP_INT_RAW_R {
        RTC_I2C_MASTER_TRAN_COMP_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_i2c_arbitration_lost_int_raw(&self) -> RTC_I2C_ARBITRATION_LOST_INT_RAW_R {
        RTC_I2C_ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_i2c_slave_tran_comp_int_raw(&self) -> RTC_I2C_SLAVE_TRAN_COMP_INT_RAW_R {
        RTC_I2C_SLAVE_TRAN_COMP_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
