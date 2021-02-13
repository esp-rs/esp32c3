#[doc = "Reader of register SENSITIVE_APB_PERIPHERAL_ACCESS_0"]
pub type R = crate::R<u32, super::SENSITIVE_APB_PERIPHERAL_ACCESS_0>;
#[doc = "Writer for register SENSITIVE_APB_PERIPHERAL_ACCESS_0"]
pub type W = crate::W<u32, super::SENSITIVE_APB_PERIPHERAL_ACCESS_0>;
#[doc = "Register SENSITIVE_APB_PERIPHERAL_ACCESS_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_APB_PERIPHERAL_ACCESS_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK`"]
pub type SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK`"]
pub struct SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK_W<'a> {
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
    pub fn sensitive_apb_peripheral_access_lock(&self) -> SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK_R {
        SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_apb_peripheral_access_lock(
        &mut self,
    ) -> SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK_W {
        SENSITIVE_APB_PERIPHERAL_ACCESS_LOCK_W { w: self }
    }
}
