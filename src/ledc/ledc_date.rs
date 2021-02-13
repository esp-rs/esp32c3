#[doc = "Reader of register LEDC_DATE"]
pub type R = crate::R<u32, super::LEDC_DATE>;
#[doc = "Writer for register LEDC_DATE"]
pub type W = crate::W<u32, super::LEDC_DATE>;
#[doc = "Register LEDC_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_DATE`"]
pub type LEDC_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_DATE`"]
pub struct LEDC_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ledc_date(&self) -> LEDC_DATE_R {
        LEDC_DATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ledc_date(&mut self) -> LEDC_DATE_W {
        LEDC_DATE_W { w: self }
    }
}
