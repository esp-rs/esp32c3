#[doc = "Reader of register LEDC_LSCH4_HPOINT"]
pub type R = crate::R<u32, super::LEDC_LSCH4_HPOINT>;
#[doc = "Writer for register LEDC_LSCH4_HPOINT"]
pub type W = crate::W<u32, super::LEDC_LSCH4_HPOINT>;
#[doc = "Register LEDC_LSCH4_HPOINT `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_LSCH4_HPOINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_HPOINT_LSCH4`"]
pub type LEDC_HPOINT_LSCH4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEDC_HPOINT_LSCH4`"]
pub struct LEDC_HPOINT_LSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HPOINT_LSCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn ledc_hpoint_lsch4(&self) -> LEDC_HPOINT_LSCH4_R {
        LEDC_HPOINT_LSCH4_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn ledc_hpoint_lsch4(&mut self) -> LEDC_HPOINT_LSCH4_W {
        LEDC_HPOINT_LSCH4_W { w: self }
    }
}
