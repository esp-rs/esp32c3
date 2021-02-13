#[doc = "Reader of register SYSCON_FLASH_ACE2_SIZE"]
pub type R = crate::R<u32, super::SYSCON_FLASH_ACE2_SIZE>;
#[doc = "Writer for register SYSCON_FLASH_ACE2_SIZE"]
pub type W = crate::W<u32, super::SYSCON_FLASH_ACE2_SIZE>;
#[doc = "Register SYSCON_FLASH_ACE2_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_FLASH_ACE2_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_FLASH_ACE2_SIZE`"]
pub type SYSCON_FLASH_ACE2_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSCON_FLASH_ACE2_SIZE`"]
pub struct SYSCON_FLASH_ACE2_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_FLASH_ACE2_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn syscon_flash_ace2_size(&self) -> SYSCON_FLASH_ACE2_SIZE_R {
        SYSCON_FLASH_ACE2_SIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn syscon_flash_ace2_size(&mut self) -> SYSCON_FLASH_ACE2_SIZE_W {
        SYSCON_FLASH_ACE2_SIZE_W { w: self }
    }
}
