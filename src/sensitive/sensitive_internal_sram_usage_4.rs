#[doc = "Reader of register SENSITIVE_INTERNAL_SRAM_USAGE_4"]
pub type R = crate::R<u32, super::SENSITIVE_INTERNAL_SRAM_USAGE_4>;
#[doc = "Writer for register SENSITIVE_INTERNAL_SRAM_USAGE_4"]
pub type W = crate::W<u32, super::SENSITIVE_INTERNAL_SRAM_USAGE_4>;
#[doc = "Register SENSITIVE_INTERNAL_SRAM_USAGE_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_INTERNAL_SRAM_USAGE_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM`"]
pub type SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM`"]
pub struct SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM_W<'a> {
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
    pub fn sensitive_internal_sram_usage_log_sram(
        &self,
    ) -> SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM_R {
        SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_internal_sram_usage_log_sram(
        &mut self,
    ) -> SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM_W {
        SENSITIVE_INTERNAL_SRAM_USAGE_LOG_SRAM_W { w: self }
    }
}
