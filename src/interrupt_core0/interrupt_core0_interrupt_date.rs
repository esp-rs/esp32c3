#[doc = "Reader of register INTERRUPT_CORE0_INTERRUPT_DATE"]
pub type R = crate::R<u32, super::INTERRUPT_CORE0_INTERRUPT_DATE>;
#[doc = "Writer for register INTERRUPT_CORE0_INTERRUPT_DATE"]
pub type W = crate::W<u32, super::INTERRUPT_CORE0_INTERRUPT_DATE>;
#[doc = "Register INTERRUPT_CORE0_INTERRUPT_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_CORE0_INTERRUPT_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERRUPT_CORE0_INTERRUPT_DATE`"]
pub type INTERRUPT_CORE0_INTERRUPT_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INTERRUPT_CORE0_INTERRUPT_DATE`"]
pub struct INTERRUPT_CORE0_INTERRUPT_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_CORE0_INTERRUPT_DATE_W<'a> {
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
    pub fn interrupt_core0_interrupt_date(&self) -> INTERRUPT_CORE0_INTERRUPT_DATE_R {
        INTERRUPT_CORE0_INTERRUPT_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn interrupt_core0_interrupt_date(&mut self) -> INTERRUPT_CORE0_INTERRUPT_DATE_W {
        INTERRUPT_CORE0_INTERRUPT_DATE_W { w: self }
    }
}
