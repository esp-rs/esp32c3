#[doc = "Reader of register I2C_FILTER_CFG"]
pub type R = crate::R<u32, super::I2C_FILTER_CFG>;
#[doc = "Writer for register I2C_FILTER_CFG"]
pub type W = crate::W<u32, super::I2C_FILTER_CFG>;
#[doc = "Register I2C_FILTER_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_FILTER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SDA_FILTER_EN`"]
pub type I2C_SDA_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SDA_FILTER_EN`"]
pub struct I2C_SDA_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SDA_FILTER_EN_W<'a> {
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
#[doc = "Reader of field `I2C_SCL_FILTER_EN`"]
pub type I2C_SCL_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SCL_FILTER_EN`"]
pub struct I2C_SCL_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_FILTER_EN_W<'a> {
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
#[doc = "Reader of field `I2C_SDA_FILTER_THRES`"]
pub type I2C_SDA_FILTER_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SDA_FILTER_THRES`"]
pub struct I2C_SDA_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SDA_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `I2C_SCL_FILTER_THRES`"]
pub type I2C_SCL_FILTER_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SCL_FILTER_THRES`"]
pub struct I2C_SCL_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2c_sda_filter_en(&self) -> I2C_SDA_FILTER_EN_R {
        I2C_SDA_FILTER_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_scl_filter_en(&self) -> I2C_SCL_FILTER_EN_R {
        I2C_SCL_FILTER_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn i2c_sda_filter_thres(&self) -> I2C_SDA_FILTER_THRES_R {
        I2C_SDA_FILTER_THRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn i2c_scl_filter_thres(&self) -> I2C_SCL_FILTER_THRES_R {
        I2C_SCL_FILTER_THRES_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2c_sda_filter_en(&mut self) -> I2C_SDA_FILTER_EN_W {
        I2C_SDA_FILTER_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_scl_filter_en(&mut self) -> I2C_SCL_FILTER_EN_W {
        I2C_SCL_FILTER_EN_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn i2c_sda_filter_thres(&mut self) -> I2C_SDA_FILTER_THRES_W {
        I2C_SDA_FILTER_THRES_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn i2c_scl_filter_thres(&mut self) -> I2C_SCL_FILTER_THRES_W {
        I2C_SCL_FILTER_THRES_W { w: self }
    }
}
