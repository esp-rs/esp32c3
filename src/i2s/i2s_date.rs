#[doc = "Reader of register I2S_DATE"]
pub type R = crate::R<u32, super::I2S_DATE>;
#[doc = "Writer for register I2S_DATE"]
pub type W = crate::W<u32, super::I2S_DATE>;
#[doc = "Register I2S_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_DATE`"]
pub type I2S_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `I2S_DATE`"]
pub struct I2S_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_DATE_W<'a> {
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
    pub fn i2s_date(&self) -> I2S_DATE_R {
        I2S_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn i2s_date(&mut self) -> I2S_DATE_W {
        I2S_DATE_W { w: self }
    }
}
