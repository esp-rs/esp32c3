#[doc = "Reader of register GPIO_FUNC75_IN_SEL_CFG"]
pub type R = crate::R<u32, super::GPIO_FUNC75_IN_SEL_CFG>;
#[doc = "Writer for register GPIO_FUNC75_IN_SEL_CFG"]
pub type W = crate::W<u32, super::GPIO_FUNC75_IN_SEL_CFG>;
#[doc = "Register GPIO_FUNC75_IN_SEL_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_FUNC75_IN_SEL_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_SIG75_IN_SEL`"]
pub type GPIO_SIG75_IN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_SIG75_IN_SEL`"]
pub struct GPIO_SIG75_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SIG75_IN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GPIO_FUNC75_IN_INV_SEL`"]
pub type GPIO_FUNC75_IN_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_FUNC75_IN_INV_SEL`"]
pub struct GPIO_FUNC75_IN_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC75_IN_INV_SEL_W<'a> {
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
#[doc = "Reader of field `GPIO_FUNC75_IN_SEL`"]
pub type GPIO_FUNC75_IN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_FUNC75_IN_SEL`"]
pub struct GPIO_FUNC75_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC75_IN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_sig75_in_sel(&self) -> GPIO_SIG75_IN_SEL_R {
        GPIO_SIG75_IN_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_func75_in_inv_sel(&self) -> GPIO_FUNC75_IN_INV_SEL_R {
        GPIO_FUNC75_IN_INV_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpio_func75_in_sel(&self) -> GPIO_FUNC75_IN_SEL_R {
        GPIO_FUNC75_IN_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_sig75_in_sel(&mut self) -> GPIO_SIG75_IN_SEL_W {
        GPIO_SIG75_IN_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_func75_in_inv_sel(&mut self) -> GPIO_FUNC75_IN_INV_SEL_W {
        GPIO_FUNC75_IN_INV_SEL_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpio_func75_in_sel(&mut self) -> GPIO_FUNC75_IN_SEL_W {
        GPIO_FUNC75_IN_SEL_W { w: self }
    }
}
