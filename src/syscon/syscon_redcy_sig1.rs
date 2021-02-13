#[doc = "Reader of register SYSCON_REDCY_SIG1"]
pub type R = crate::R<u32, super::SYSCON_REDCY_SIG1>;
#[doc = "Writer for register SYSCON_REDCY_SIG1"]
pub type W = crate::W<u32, super::SYSCON_REDCY_SIG1>;
#[doc = "Register SYSCON_REDCY_SIG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_REDCY_SIG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_REDCY_NANDOR`"]
pub type SYSCON_REDCY_NANDOR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSCON_REDCY_SIG1`"]
pub type SYSCON_REDCY_SIG1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYSCON_REDCY_SIG1`"]
pub struct SYSCON_REDCY_SIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_REDCY_SIG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn syscon_redcy_nandor(&self) -> SYSCON_REDCY_NANDOR_R {
        SYSCON_REDCY_NANDOR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn syscon_redcy_sig1(&self) -> SYSCON_REDCY_SIG1_R {
        SYSCON_REDCY_SIG1_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn syscon_redcy_sig1(&mut self) -> SYSCON_REDCY_SIG1_W {
        SYSCON_REDCY_SIG1_W { w: self }
    }
}
