#[doc = "Reader of register I2C_SCL_STRETCH_CONF"]
pub type R = crate::R<u32, super::I2C_SCL_STRETCH_CONF>;
#[doc = "Writer for register I2C_SCL_STRETCH_CONF"]
pub type W = crate::W<u32, super::I2C_SCL_STRETCH_CONF>;
#[doc = "Register I2C_SCL_STRETCH_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_SCL_STRETCH_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SLAVE_BYTE_ACK_LVL`"]
pub type I2C_SLAVE_BYTE_ACK_LVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SLAVE_BYTE_ACK_LVL`"]
pub struct I2C_SLAVE_BYTE_ACK_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_BYTE_ACK_LVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `I2C_SLAVE_BYTE_ACK_CTL_EN`"]
pub type I2C_SLAVE_BYTE_ACK_CTL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SLAVE_BYTE_ACK_CTL_EN`"]
pub struct I2C_SLAVE_BYTE_ACK_CTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_BYTE_ACK_CTL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `I2C_SLAVE_SCL_STRETCH_CLR`"]
pub struct I2C_SLAVE_SCL_STRETCH_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_SCL_STRETCH_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `I2C_SLAVE_SCL_STRETCH_EN`"]
pub type I2C_SLAVE_SCL_STRETCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SLAVE_SCL_STRETCH_EN`"]
pub struct I2C_SLAVE_SCL_STRETCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_SCL_STRETCH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `I2C_STRETCH_PROTECT_NUM`"]
pub type I2C_STRETCH_PROTECT_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_STRETCH_PROTECT_NUM`"]
pub struct I2C_STRETCH_PROTECT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_STRETCH_PROTECT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c_slave_byte_ack_lvl(&self) -> I2C_SLAVE_BYTE_ACK_LVL_R {
        I2C_SLAVE_BYTE_ACK_LVL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c_slave_byte_ack_ctl_en(&self) -> I2C_SLAVE_BYTE_ACK_CTL_EN_R {
        I2C_SLAVE_BYTE_ACK_CTL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c_slave_scl_stretch_en(&self) -> I2C_SLAVE_SCL_STRETCH_EN_R {
        I2C_SLAVE_SCL_STRETCH_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2c_stretch_protect_num(&self) -> I2C_STRETCH_PROTECT_NUM_R {
        I2C_STRETCH_PROTECT_NUM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c_slave_byte_ack_lvl(&mut self) -> I2C_SLAVE_BYTE_ACK_LVL_W {
        I2C_SLAVE_BYTE_ACK_LVL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c_slave_byte_ack_ctl_en(&mut self) -> I2C_SLAVE_BYTE_ACK_CTL_EN_W {
        I2C_SLAVE_BYTE_ACK_CTL_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c_slave_scl_stretch_clr(&mut self) -> I2C_SLAVE_SCL_STRETCH_CLR_W {
        I2C_SLAVE_SCL_STRETCH_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c_slave_scl_stretch_en(&mut self) -> I2C_SLAVE_SCL_STRETCH_EN_W {
        I2C_SLAVE_SCL_STRETCH_EN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2c_stretch_protect_num(&mut self) -> I2C_STRETCH_PROTECT_NUM_W {
        I2C_STRETCH_PROTECT_NUM_W { w: self }
    }
}
