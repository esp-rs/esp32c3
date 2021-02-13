#[doc = "Reader of register EXTMEM_DATE"]
pub type R = crate::R<u32, super::EXTMEM_DATE>;
#[doc = "Writer for register EXTMEM_DATE"]
pub type W = crate::W<u32, super::EXTMEM_DATE>;
#[doc = "Register EXTMEM_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_DATE`"]
pub type EXTMEM_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EXTMEM_DATE`"]
pub struct EXTMEM_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_DATE_W<'a> {
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
    pub fn extmem_date(&self) -> EXTMEM_DATE_R {
        EXTMEM_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn extmem_date(&mut self) -> EXTMEM_DATE_W {
        EXTMEM_DATE_W { w: self }
    }
}
