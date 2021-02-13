#[doc = "Reader of register APB_CTRL_REDCY_SIG0"]
pub type R = crate::R<u32, super::APB_CTRL_REDCY_SIG0>;
#[doc = "Writer for register APB_CTRL_REDCY_SIG0"]
pub type W = crate::W<u32, super::APB_CTRL_REDCY_SIG0>;
#[doc = "Register APB_CTRL_REDCY_SIG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_REDCY_SIG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_REDCY_ANDOR`"]
pub type APB_CTRL_REDCY_ANDOR_R = crate::R<bool, bool>;
#[doc = "Reader of field `APB_CTRL_REDCY_SIG0`"]
pub type APB_CTRL_REDCY_SIG0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_CTRL_REDCY_SIG0`"]
pub struct APB_CTRL_REDCY_SIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_REDCY_SIG0_W<'a> {
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
    pub fn apb_ctrl_redcy_andor(&self) -> APB_CTRL_REDCY_ANDOR_R {
        APB_CTRL_REDCY_ANDOR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn apb_ctrl_redcy_sig0(&self) -> APB_CTRL_REDCY_SIG0_R {
        APB_CTRL_REDCY_SIG0_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn apb_ctrl_redcy_sig0(&mut self) -> APB_CTRL_REDCY_SIG0_W {
        APB_CTRL_REDCY_SIG0_W { w: self }
    }
}
