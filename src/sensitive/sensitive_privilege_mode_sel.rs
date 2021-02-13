#[doc = "Reader of register SENSITIVE_PRIVILEGE_MODE_SEL"]
pub type R = crate::R<u32, super::SENSITIVE_PRIVILEGE_MODE_SEL>;
#[doc = "Writer for register SENSITIVE_PRIVILEGE_MODE_SEL"]
pub type W = crate::W<u32, super::SENSITIVE_PRIVILEGE_MODE_SEL>;
#[doc = "Register SENSITIVE_PRIVILEGE_MODE_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_PRIVILEGE_MODE_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_PRIVILEGE_MODE_SEL`"]
pub type SENSITIVE_PRIVILEGE_MODE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_PRIVILEGE_MODE_SEL`"]
pub struct SENSITIVE_PRIVILEGE_MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_PRIVILEGE_MODE_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_privilege_mode_sel(&self) -> SENSITIVE_PRIVILEGE_MODE_SEL_R {
        SENSITIVE_PRIVILEGE_MODE_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_privilege_mode_sel(&mut self) -> SENSITIVE_PRIVILEGE_MODE_SEL_W {
        SENSITIVE_PRIVILEGE_MODE_SEL_W { w: self }
    }
}
