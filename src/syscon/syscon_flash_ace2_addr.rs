#[doc = "Reader of register SYSCON_FLASH_ACE2_ADDR"]
pub type R = crate::R<u32, super::SYSCON_FLASH_ACE2_ADDR>;
#[doc = "Writer for register SYSCON_FLASH_ACE2_ADDR"]
pub type W = crate::W<u32, super::SYSCON_FLASH_ACE2_ADDR>;
#[doc = "Register SYSCON_FLASH_ACE2_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_FLASH_ACE2_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_FLASH_ACE2_ADDR_S`"]
pub type SYSCON_FLASH_ACE2_ADDR_S_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYSCON_FLASH_ACE2_ADDR_S`"]
pub struct SYSCON_FLASH_ACE2_ADDR_S_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_FLASH_ACE2_ADDR_S_W<'a> {
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
    pub fn syscon_flash_ace2_addr_s(&self) -> SYSCON_FLASH_ACE2_ADDR_S_R {
        SYSCON_FLASH_ACE2_ADDR_S_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syscon_flash_ace2_addr_s(&mut self) -> SYSCON_FLASH_ACE2_ADDR_S_W {
        SYSCON_FLASH_ACE2_ADDR_S_W { w: self }
    }
}
