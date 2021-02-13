#[doc = "Reader of register SENSITIVE_INTERNAL_SRAM_USAGE_3"]
pub type R = crate::R<u32, super::SENSITIVE_INTERNAL_SRAM_USAGE_3>;
#[doc = "Writer for register SENSITIVE_INTERNAL_SRAM_USAGE_3"]
pub type W = crate::W<u32, super::SENSITIVE_INTERNAL_SRAM_USAGE_3>;
#[doc = "Register SENSITIVE_INTERNAL_SRAM_USAGE_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_INTERNAL_SRAM_USAGE_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP`"]
pub type SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP`"]
pub struct SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM`"]
pub type SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM`"]
pub struct SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sensitive_internal_sram_alloc_mac_dump(
        &self,
    ) -> SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP_R {
        SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sensitive_internal_sram_usage_mac_dump_sram(
        &self,
    ) -> SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R {
        SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sensitive_internal_sram_alloc_mac_dump(
        &mut self,
    ) -> SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP_W {
        SENSITIVE_INTERNAL_SRAM_ALLOC_MAC_DUMP_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sensitive_internal_sram_usage_mac_dump_sram(
        &mut self,
    ) -> SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W {
        SENSITIVE_INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W { w: self }
    }
}
