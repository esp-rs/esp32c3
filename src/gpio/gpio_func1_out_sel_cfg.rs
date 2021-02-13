#[doc = "Reader of register GPIO_FUNC1_OUT_SEL_CFG"]
pub type R = crate::R<u32, super::GPIO_FUNC1_OUT_SEL_CFG>;
#[doc = "Writer for register GPIO_FUNC1_OUT_SEL_CFG"]
pub type W = crate::W<u32, super::GPIO_FUNC1_OUT_SEL_CFG>;
#[doc = "Register GPIO_FUNC1_OUT_SEL_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_FUNC1_OUT_SEL_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_FUNC1_OEN_INV_SEL`"]
pub type GPIO_FUNC1_OEN_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_FUNC1_OEN_INV_SEL`"]
pub struct GPIO_FUNC1_OEN_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC1_OEN_INV_SEL_W<'a> {
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
#[doc = "Reader of field `GPIO_FUNC1_OEN_SEL`"]
pub type GPIO_FUNC1_OEN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_FUNC1_OEN_SEL`"]
pub struct GPIO_FUNC1_OEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC1_OEN_SEL_W<'a> {
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
#[doc = "Reader of field `GPIO_FUNC1_OUT_INV_SEL`"]
pub type GPIO_FUNC1_OUT_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_FUNC1_OUT_INV_SEL`"]
pub struct GPIO_FUNC1_OUT_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC1_OUT_INV_SEL_W<'a> {
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
#[doc = "Reader of field `GPIO_FUNC1_OUT_SEL`"]
pub type GPIO_FUNC1_OUT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_FUNC1_OUT_SEL`"]
pub struct GPIO_FUNC1_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC1_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_func1_oen_inv_sel(&self) -> GPIO_FUNC1_OEN_INV_SEL_R {
        GPIO_FUNC1_OEN_INV_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio_func1_oen_sel(&self) -> GPIO_FUNC1_OEN_SEL_R {
        GPIO_FUNC1_OEN_SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio_func1_out_inv_sel(&self) -> GPIO_FUNC1_OUT_INV_SEL_R {
        GPIO_FUNC1_OUT_INV_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_func1_out_sel(&self) -> GPIO_FUNC1_OUT_SEL_R {
        GPIO_FUNC1_OUT_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_func1_oen_inv_sel(&mut self) -> GPIO_FUNC1_OEN_INV_SEL_W {
        GPIO_FUNC1_OEN_INV_SEL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio_func1_oen_sel(&mut self) -> GPIO_FUNC1_OEN_SEL_W {
        GPIO_FUNC1_OEN_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio_func1_out_inv_sel(&mut self) -> GPIO_FUNC1_OUT_INV_SEL_W {
        GPIO_FUNC1_OUT_INV_SEL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_func1_out_sel(&mut self) -> GPIO_FUNC1_OUT_SEL_W {
        GPIO_FUNC1_OUT_SEL_W { w: self }
    }
}
