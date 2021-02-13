#[doc = "Reader of register SYSCON_FLASH_ACE2_ATTR"]
pub type R = crate::R<u32, super::SYSCON_FLASH_ACE2_ATTR>;
#[doc = "Writer for register SYSCON_FLASH_ACE2_ATTR"]
pub type W = crate::W<u32, super::SYSCON_FLASH_ACE2_ATTR>;
#[doc = "Register SYSCON_FLASH_ACE2_ATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_FLASH_ACE2_ATTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_FLASH_ACE2_ATTR`"]
pub type SYSCON_FLASH_ACE2_ATTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_FLASH_ACE2_ATTR`"]
pub struct SYSCON_FLASH_ACE2_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_FLASH_ACE2_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn syscon_flash_ace2_attr(&self) -> SYSCON_FLASH_ACE2_ATTR_R {
        SYSCON_FLASH_ACE2_ATTR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn syscon_flash_ace2_attr(&mut self) -> SYSCON_FLASH_ACE2_ATTR_W {
        SYSCON_FLASH_ACE2_ATTR_W { w: self }
    }
}
