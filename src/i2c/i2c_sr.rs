#[doc = "Reader of register I2C_SR"]
pub type R = crate::R<u32, super::I2C_SR>;
#[doc = "Reader of field `I2C_SCL_STATE_LAST`"]
pub type I2C_SCL_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_SCL_MAIN_STATE_LAST`"]
pub type I2C_SCL_MAIN_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_TXFIFO_CNT`"]
pub type I2C_TXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_STRETCH_CAUSE`"]
pub type I2C_STRETCH_CAUSE_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_RXFIFO_CNT`"]
pub type I2C_RXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_SLAVE_ADDRESSED`"]
pub type I2C_SLAVE_ADDRESSED_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_BUS_BUSY`"]
pub type I2C_BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_ARB_LOST`"]
pub type I2C_ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_SLAVE_RW`"]
pub type I2C_SLAVE_RW_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_RESP_REC`"]
pub type I2C_RESP_REC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn i2c_scl_state_last(&self) -> I2C_SCL_STATE_LAST_R {
        I2C_SCL_STATE_LAST_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn i2c_scl_main_state_last(&self) -> I2C_SCL_MAIN_STATE_LAST_R {
        I2C_SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn i2c_txfifo_cnt(&self) -> I2C_TXFIFO_CNT_R {
        I2C_TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn i2c_stretch_cause(&self) -> I2C_STRETCH_CAUSE_R {
        I2C_STRETCH_CAUSE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn i2c_rxfifo_cnt(&self) -> I2C_RXFIFO_CNT_R {
        I2C_RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_slave_addressed(&self) -> I2C_SLAVE_ADDRESSED_R {
        I2C_SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_bus_busy(&self) -> I2C_BUS_BUSY_R {
        I2C_BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_slave_rw(&self) -> I2C_SLAVE_RW_R {
        I2C_SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_resp_rec(&self) -> I2C_RESP_REC_R {
        I2C_RESP_REC_R::new((self.bits & 0x01) != 0)
    }
}
