#[doc = "Reader of register RTC_I2C_STATUS"]
pub type R = crate::R<u32, super::RTC_I2C_STATUS>;
#[doc = "Reader of field `RTC_I2C_SCL_STATE_LAST`"]
pub type RTC_I2C_SCL_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Reader of field `RTC_I2C_SCL_MAIN_STATE_LAST`"]
pub type RTC_I2C_SCL_MAIN_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Reader of field `RTC_I2C_SHIFT`"]
pub type RTC_I2C_SHIFT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RTC_I2C_OP_CNT`"]
pub type RTC_I2C_OP_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RTC_I2C_BYTE_TRANS`"]
pub type RTC_I2C_BYTE_TRANS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_SLAVE_ADDRESSED`"]
pub type RTC_I2C_SLAVE_ADDRESSED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_BUS_BUSY`"]
pub type RTC_I2C_BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_ARB_LOST`"]
pub type RTC_I2C_ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_SLAVE_RW`"]
pub type RTC_I2C_SLAVE_RW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_ACK_REC`"]
pub type RTC_I2C_ACK_REC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rtc_i2c_scl_state_last(&self) -> RTC_I2C_SCL_STATE_LAST_R {
        RTC_I2C_SCL_STATE_LAST_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rtc_i2c_scl_main_state_last(&self) -> RTC_I2C_SCL_MAIN_STATE_LAST_R {
        RTC_I2C_SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rtc_i2c_shift(&self) -> RTC_I2C_SHIFT_R {
        RTC_I2C_SHIFT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rtc_i2c_op_cnt(&self) -> RTC_I2C_OP_CNT_R {
        RTC_I2C_OP_CNT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_byte_trans(&self) -> RTC_I2C_BYTE_TRANS_R {
        RTC_I2C_BYTE_TRANS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_slave_addressed(&self) -> RTC_I2C_SLAVE_ADDRESSED_R {
        RTC_I2C_SLAVE_ADDRESSED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_i2c_bus_busy(&self) -> RTC_I2C_BUS_BUSY_R {
        RTC_I2C_BUS_BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_i2c_arb_lost(&self) -> RTC_I2C_ARB_LOST_R {
        RTC_I2C_ARB_LOST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_i2c_slave_rw(&self) -> RTC_I2C_SLAVE_RW_R {
        RTC_I2C_SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_i2c_ack_rec(&self) -> RTC_I2C_ACK_REC_R {
        RTC_I2C_ACK_REC_R::new((self.bits & 0x01) != 0)
    }
}
