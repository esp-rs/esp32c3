#[doc = "Reader of register SENSITIVE_ROM_TABLE"]
pub type R = crate::R<u32, super::SENSITIVE_ROM_TABLE>;
#[doc = "Writer for register SENSITIVE_ROM_TABLE"]
pub type W = crate::W<u32, super::SENSITIVE_ROM_TABLE>;
#[doc = "Register SENSITIVE_ROM_TABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_ROM_TABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_ROM_TABLE`"]
pub type SENSITIVE_ROM_TABLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SENSITIVE_ROM_TABLE`"]
pub struct SENSITIVE_ROM_TABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_ROM_TABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sensitive_rom_table(&self) -> SENSITIVE_ROM_TABLE_R {
        SENSITIVE_ROM_TABLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sensitive_rom_table(&mut self) -> SENSITIVE_ROM_TABLE_W {
        SENSITIVE_ROM_TABLE_W { w: self }
    }
}
