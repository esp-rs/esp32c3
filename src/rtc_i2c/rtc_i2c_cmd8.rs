#[doc = "Reader of register RTC_I2C_CMD8"]
pub type R = crate::R<u32, super::RTC_I2C_CMD8>;
#[doc = "Writer for register RTC_I2C_CMD8"]
pub type W = crate::W<u32, super::RTC_I2C_CMD8>;
#[doc = "Register RTC_I2C_CMD8 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_I2C_CMD8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_COMMAND8_DONE`"]
pub type RTC_I2C_COMMAND8_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_COMMAND8`"]
pub type RTC_I2C_COMMAND8_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_I2C_COMMAND8`"]
pub struct RTC_I2C_COMMAND8_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_COMMAND8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_i2c_command8_done(&self) -> RTC_I2C_COMMAND8_DONE_R {
        RTC_I2C_COMMAND8_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn rtc_i2c_command8(&self) -> RTC_I2C_COMMAND8_R {
        RTC_I2C_COMMAND8_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn rtc_i2c_command8(&mut self) -> RTC_I2C_COMMAND8_W {
        RTC_I2C_COMMAND8_W { w: self }
    }
}
