#[doc = "Reader of register RTC_I2C_DATA"]
pub type R = crate::R<u32, super::RTC_I2C_DATA>;
#[doc = "Writer for register RTC_I2C_DATA"]
pub type W = crate::W<u32, super::RTC_I2C_DATA>;
#[doc = "Register RTC_I2C_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_I2C_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_DONE`"]
pub type RTC_I2C_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_SLAVE_TX_DATA`"]
pub type RTC_I2C_SLAVE_TX_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_I2C_SLAVE_TX_DATA`"]
pub struct RTC_I2C_SLAVE_TX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_TX_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_RDATA`"]
pub type RTC_I2C_RDATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_i2c_done(&self) -> RTC_I2C_DONE_R {
        RTC_I2C_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rtc_i2c_slave_tx_data(&self) -> RTC_I2C_SLAVE_TX_DATA_R {
        RTC_I2C_SLAVE_TX_DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_i2c_rdata(&self) -> RTC_I2C_RDATA_R {
        RTC_I2C_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rtc_i2c_slave_tx_data(&mut self) -> RTC_I2C_SLAVE_TX_DATA_W {
        RTC_I2C_SLAVE_TX_DATA_W { w: self }
    }
}
