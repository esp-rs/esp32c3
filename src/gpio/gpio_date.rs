#[doc = "Reader of register GPIO_DATE"]
pub type R = crate::R<u32, super::GPIO_DATE>;
#[doc = "Writer for register GPIO_DATE"]
pub type W = crate::W<u32, super::GPIO_DATE>;
#[doc = "Register GPIO_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_DATE`"]
pub type GPIO_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_DATE`"]
pub struct GPIO_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn gpio_date(&self) -> GPIO_DATE_R {
        GPIO_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn gpio_date(&mut self) -> GPIO_DATE_W {
        GPIO_DATE_W { w: self }
    }
}
