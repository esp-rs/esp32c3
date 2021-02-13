#[doc = "Reader of register I2C_TO"]
pub type R = crate::R<u32, super::I2C_TO>;
#[doc = "Writer for register I2C_TO"]
pub type W = crate::W<u32, super::I2C_TO>;
#[doc = "Register I2C_TO `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_TO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_TIME_OUT_EN`"]
pub type I2C_TIME_OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TIME_OUT_EN`"]
pub struct I2C_TIME_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TIME_OUT_EN_W<'a> {
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
#[doc = "Reader of field `I2C_TIME_OUT_REG`"]
pub type I2C_TIME_OUT_REG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_TIME_OUT_REG`"]
pub struct I2C_TIME_OUT_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TIME_OUT_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_time_out_en(&self) -> I2C_TIME_OUT_EN_R {
        I2C_TIME_OUT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn i2c_time_out_reg(&self) -> I2C_TIME_OUT_REG_R {
        I2C_TIME_OUT_REG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_time_out_en(&mut self) -> I2C_TIME_OUT_EN_W {
        I2C_TIME_OUT_EN_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn i2c_time_out_reg(&mut self) -> I2C_TIME_OUT_REG_W {
        I2C_TIME_OUT_REG_W { w: self }
    }
}
