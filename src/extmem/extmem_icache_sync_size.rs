#[doc = "Reader of register EXTMEM_ICACHE_SYNC_SIZE"]
pub type R = crate::R<u32, super::EXTMEM_ICACHE_SYNC_SIZE>;
#[doc = "Writer for register EXTMEM_ICACHE_SYNC_SIZE"]
pub type W = crate::W<u32, super::EXTMEM_ICACHE_SYNC_SIZE>;
#[doc = "Register EXTMEM_ICACHE_SYNC_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_ICACHE_SYNC_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_SYNC_SIZE`"]
pub type EXTMEM_ICACHE_SYNC_SIZE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_SYNC_SIZE`"]
pub struct EXTMEM_ICACHE_SYNC_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_SYNC_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x007f_ffff) | ((value as u32) & 0x007f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn extmem_icache_sync_size(&self) -> EXTMEM_ICACHE_SYNC_SIZE_R {
        EXTMEM_ICACHE_SYNC_SIZE_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn extmem_icache_sync_size(&mut self) -> EXTMEM_ICACHE_SYNC_SIZE_W {
        EXTMEM_ICACHE_SYNC_SIZE_W { w: self }
    }
}
