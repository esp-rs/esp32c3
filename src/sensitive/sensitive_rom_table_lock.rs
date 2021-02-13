#[doc = "Reader of register SENSITIVE_ROM_TABLE_LOCK"]
pub type R = crate::R<u32, super::SENSITIVE_ROM_TABLE_LOCK>;
#[doc = "Writer for register SENSITIVE_ROM_TABLE_LOCK"]
pub type W = crate::W<u32, super::SENSITIVE_ROM_TABLE_LOCK>;
#[doc = "Register SENSITIVE_ROM_TABLE_LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_ROM_TABLE_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_ROM_TABLE_LOCK`"]
pub type SENSITIVE_ROM_TABLE_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_ROM_TABLE_LOCK`"]
pub struct SENSITIVE_ROM_TABLE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_ROM_TABLE_LOCK_W<'a> {
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
    pub fn sensitive_rom_table_lock(&self) -> SENSITIVE_ROM_TABLE_LOCK_R {
        SENSITIVE_ROM_TABLE_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_rom_table_lock(&mut self) -> SENSITIVE_ROM_TABLE_LOCK_W {
        SENSITIVE_ROM_TABLE_LOCK_W { w: self }
    }
}
