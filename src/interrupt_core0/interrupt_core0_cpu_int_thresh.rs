#[doc = "Reader of register INTERRUPT_CORE0_CPU_INT_THRESH"]
pub type R = crate::R<u32, super::INTERRUPT_CORE0_CPU_INT_THRESH>;
#[doc = "Writer for register INTERRUPT_CORE0_CPU_INT_THRESH"]
pub type W = crate::W<u32, super::INTERRUPT_CORE0_CPU_INT_THRESH>;
#[doc = "Register INTERRUPT_CORE0_CPU_INT_THRESH `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_CORE0_CPU_INT_THRESH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERRUPT_CORE0_CPU_INT_THRESH`"]
pub type INTERRUPT_CORE0_CPU_INT_THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTERRUPT_CORE0_CPU_INT_THRESH`"]
pub struct INTERRUPT_CORE0_CPU_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_CORE0_CPU_INT_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn interrupt_core0_cpu_int_thresh(&self) -> INTERRUPT_CORE0_CPU_INT_THRESH_R {
        INTERRUPT_CORE0_CPU_INT_THRESH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn interrupt_core0_cpu_int_thresh(&mut self) -> INTERRUPT_CORE0_CPU_INT_THRESH_W {
        INTERRUPT_CORE0_CPU_INT_THRESH_W { w: self }
    }
}
