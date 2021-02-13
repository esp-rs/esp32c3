#[doc = "Reader of register GPIO_CLOCK_GATE"]
pub type R = crate::R<u32, super::GPIO_CLOCK_GATE>;
#[doc = "Writer for register GPIO_CLOCK_GATE"]
pub type W = crate::W<u32, super::GPIO_CLOCK_GATE>;
#[doc = "Register GPIO_CLOCK_GATE `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_CLOCK_GATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_CLK_EN`"]
pub type GPIO_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_CLK_EN`"]
pub struct GPIO_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio_clk_en(&self) -> GPIO_CLK_EN_R {
        GPIO_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio_clk_en(&mut self) -> GPIO_CLK_EN_W {
        GPIO_CLK_EN_W { w: self }
    }
}
