#[doc = "Reader of register SYSCON_CLKGATE_FORCE_ON"]
pub type R = crate::R<u32, super::SYSCON_CLKGATE_FORCE_ON>;
#[doc = "Writer for register SYSCON_CLKGATE_FORCE_ON"]
pub type W = crate::W<u32, super::SYSCON_CLKGATE_FORCE_ON>;
#[doc = "Register SYSCON_CLKGATE_FORCE_ON `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_CLKGATE_FORCE_ON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_SRAM_CLKGATE_FORCE_ON`"]
pub type SYSCON_SRAM_CLKGATE_FORCE_ON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_SRAM_CLKGATE_FORCE_ON`"]
pub struct SYSCON_SRAM_CLKGATE_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_SRAM_CLKGATE_FORCE_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_ROM_CLKGATE_FORCE_ON`"]
pub type SYSCON_ROM_CLKGATE_FORCE_ON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_ROM_CLKGATE_FORCE_ON`"]
pub struct SYSCON_ROM_CLKGATE_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_ROM_CLKGATE_FORCE_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn syscon_sram_clkgate_force_on(&self) -> SYSCON_SRAM_CLKGATE_FORCE_ON_R {
        SYSCON_SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn syscon_rom_clkgate_force_on(&self) -> SYSCON_ROM_CLKGATE_FORCE_ON_R {
        SYSCON_ROM_CLKGATE_FORCE_ON_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn syscon_sram_clkgate_force_on(&mut self) -> SYSCON_SRAM_CLKGATE_FORCE_ON_W {
        SYSCON_SRAM_CLKGATE_FORCE_ON_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn syscon_rom_clkgate_force_on(&mut self) -> SYSCON_ROM_CLKGATE_FORCE_ON_W {
        SYSCON_ROM_CLKGATE_FORCE_ON_W { w: self }
    }
}
