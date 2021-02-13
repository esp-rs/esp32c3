#[doc = "Reader of register I2C_CLK_CONF"]
pub type R = crate::R<u32, super::I2C_CLK_CONF>;
#[doc = "Writer for register I2C_CLK_CONF"]
pub type W = crate::W<u32, super::I2C_CLK_CONF>;
#[doc = "Register I2C_CLK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_CLK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SCLK_ACTIVE`"]
pub type I2C_SCLK_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SCLK_ACTIVE`"]
pub struct I2C_SCLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCLK_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `I2C_SCLK_SEL`"]
pub type I2C_SCLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SCLK_SEL`"]
pub struct I2C_SCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `I2C_SCLK_DIV_B`"]
pub type I2C_SCLK_DIV_B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SCLK_DIV_B`"]
pub struct I2C_SCLK_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCLK_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `I2C_SCLK_DIV_A`"]
pub type I2C_SCLK_DIV_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SCLK_DIV_A`"]
pub struct I2C_SCLK_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCLK_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2C_SCLK_DIV_NUM`"]
pub type I2C_SCLK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SCLK_DIV_NUM`"]
pub struct I2C_SCLK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCLK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2c_sclk_active(&self) -> I2C_SCLK_ACTIVE_R {
        I2C_SCLK_ACTIVE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2c_sclk_sel(&self) -> I2C_SCLK_SEL_R {
        I2C_SCLK_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn i2c_sclk_div_b(&self) -> I2C_SCLK_DIV_B_R {
        I2C_SCLK_DIV_B_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn i2c_sclk_div_a(&self) -> I2C_SCLK_DIV_A_R {
        I2C_SCLK_DIV_A_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2c_sclk_div_num(&self) -> I2C_SCLK_DIV_NUM_R {
        I2C_SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2c_sclk_active(&mut self) -> I2C_SCLK_ACTIVE_W {
        I2C_SCLK_ACTIVE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2c_sclk_sel(&mut self) -> I2C_SCLK_SEL_W {
        I2C_SCLK_SEL_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn i2c_sclk_div_b(&mut self) -> I2C_SCLK_DIV_B_W {
        I2C_SCLK_DIV_B_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn i2c_sclk_div_a(&mut self) -> I2C_SCLK_DIV_A_W {
        I2C_SCLK_DIV_A_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2c_sclk_div_num(&mut self) -> I2C_SCLK_DIV_NUM_W {
        I2C_SCLK_DIV_NUM_W { w: self }
    }
}
