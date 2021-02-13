#[doc = "Reader of register ASSIST_DEBUG_DATE"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_DATE>;
#[doc = "Writer for register ASSIST_DEBUG_DATE"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_DATE>;
#[doc = "Register ASSIST_DEBUG_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_DATE`"]
pub type ASSIST_DEBUG_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ASSIST_DEBUG_DATE`"]
pub struct ASSIST_DEBUG_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_DATE_W<'a> {
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
    pub fn assist_debug_date(&self) -> ASSIST_DEBUG_DATE_R {
        ASSIST_DEBUG_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn assist_debug_date(&mut self) -> ASSIST_DEBUG_DATE_W {
        ASSIST_DEBUG_DATE_W { w: self }
    }
}
