#[doc = "Reader of register EXTMEM_CACHE_SYNC_INT_CTRL"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_SYNC_INT_CTRL>;
#[doc = "Writer for register EXTMEM_CACHE_SYNC_INT_CTRL"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_SYNC_INT_CTRL>;
#[doc = "Register EXTMEM_CACHE_SYNC_INT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_SYNC_INT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EXTMEM_ICACHE_SYNC_INT_CLR`"]
pub struct EXTMEM_ICACHE_SYNC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_SYNC_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_SYNC_INT_ENA`"]
pub type EXTMEM_ICACHE_SYNC_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_SYNC_INT_ENA`"]
pub struct EXTMEM_ICACHE_SYNC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_SYNC_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_SYNC_INT_ST`"]
pub type EXTMEM_ICACHE_SYNC_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_sync_int_ena(&self) -> EXTMEM_ICACHE_SYNC_INT_ENA_R {
        EXTMEM_ICACHE_SYNC_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_sync_int_st(&self) -> EXTMEM_ICACHE_SYNC_INT_ST_R {
        EXTMEM_ICACHE_SYNC_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_icache_sync_int_clr(&mut self) -> EXTMEM_ICACHE_SYNC_INT_CLR_W {
        EXTMEM_ICACHE_SYNC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_sync_int_ena(&mut self) -> EXTMEM_ICACHE_SYNC_INT_ENA_W {
        EXTMEM_ICACHE_SYNC_INT_ENA_W { w: self }
    }
}
