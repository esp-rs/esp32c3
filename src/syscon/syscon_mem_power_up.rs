#[doc = "Reader of register SYSCON_MEM_POWER_UP"]
pub type R = crate::R<u32, super::SYSCON_MEM_POWER_UP>;
#[doc = "Writer for register SYSCON_MEM_POWER_UP"]
pub type W = crate::W<u32, super::SYSCON_MEM_POWER_UP>;
#[doc = "Register SYSCON_MEM_POWER_UP `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_MEM_POWER_UP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_SRAM_POWER_UP`"]
pub type SYSCON_SRAM_POWER_UP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_SRAM_POWER_UP`"]
pub struct SYSCON_SRAM_POWER_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_SRAM_POWER_UP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_ROM_POWER_UP`"]
pub type SYSCON_ROM_POWER_UP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_ROM_POWER_UP`"]
pub struct SYSCON_ROM_POWER_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_ROM_POWER_UP_W<'a> {
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
    pub fn syscon_sram_power_up(&self) -> SYSCON_SRAM_POWER_UP_R {
        SYSCON_SRAM_POWER_UP_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn syscon_rom_power_up(&self) -> SYSCON_ROM_POWER_UP_R {
        SYSCON_ROM_POWER_UP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn syscon_sram_power_up(&mut self) -> SYSCON_SRAM_POWER_UP_W {
        SYSCON_SRAM_POWER_UP_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn syscon_rom_power_up(&mut self) -> SYSCON_ROM_POWER_UP_W {
        SYSCON_ROM_POWER_UP_W { w: self }
    }
}
