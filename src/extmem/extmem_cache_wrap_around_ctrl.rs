#[doc = "Reader of register EXTMEM_CACHE_WRAP_AROUND_CTRL"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_WRAP_AROUND_CTRL>;
#[doc = "Writer for register EXTMEM_CACHE_WRAP_AROUND_CTRL"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_WRAP_AROUND_CTRL>;
#[doc = "Register EXTMEM_CACHE_WRAP_AROUND_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_WRAP_AROUND_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_CACHE_FLASH_WRAP_AROUND`"]
pub type EXTMEM_CACHE_FLASH_WRAP_AROUND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CACHE_FLASH_WRAP_AROUND`"]
pub struct EXTMEM_CACHE_FLASH_WRAP_AROUND_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CACHE_FLASH_WRAP_AROUND_W<'a> {
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
    pub fn extmem_cache_flash_wrap_around(&self) -> EXTMEM_CACHE_FLASH_WRAP_AROUND_R {
        EXTMEM_CACHE_FLASH_WRAP_AROUND_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_cache_flash_wrap_around(&mut self) -> EXTMEM_CACHE_FLASH_WRAP_AROUND_W {
        EXTMEM_CACHE_FLASH_WRAP_AROUND_W { w: self }
    }
}
