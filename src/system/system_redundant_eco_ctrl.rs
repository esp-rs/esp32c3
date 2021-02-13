#[doc = "Reader of register SYSTEM_REDUNDANT_ECO_CTRL"]
pub type R = crate::R<u32, super::SYSTEM_REDUNDANT_ECO_CTRL>;
#[doc = "Writer for register SYSTEM_REDUNDANT_ECO_CTRL"]
pub type W = crate::W<u32, super::SYSTEM_REDUNDANT_ECO_CTRL>;
#[doc = "Register SYSTEM_REDUNDANT_ECO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_REDUNDANT_ECO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_REDUNDANT_ECO_RESULT`"]
pub type SYSTEM_REDUNDANT_ECO_RESULT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSTEM_REDUNDANT_ECO_DRIVE`"]
pub type SYSTEM_REDUNDANT_ECO_DRIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_REDUNDANT_ECO_DRIVE`"]
pub struct SYSTEM_REDUNDANT_ECO_DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_REDUNDANT_ECO_DRIVE_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_redundant_eco_result(&self) -> SYSTEM_REDUNDANT_ECO_RESULT_R {
        SYSTEM_REDUNDANT_ECO_RESULT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_redundant_eco_drive(&self) -> SYSTEM_REDUNDANT_ECO_DRIVE_R {
        SYSTEM_REDUNDANT_ECO_DRIVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_redundant_eco_drive(&mut self) -> SYSTEM_REDUNDANT_ECO_DRIVE_W {
        SYSTEM_REDUNDANT_ECO_DRIVE_W { w: self }
    }
}
