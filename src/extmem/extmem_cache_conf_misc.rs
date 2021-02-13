#[doc = "Reader of register EXTMEM_CACHE_CONF_MISC"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_CONF_MISC>;
#[doc = "Writer for register EXTMEM_CACHE_CONF_MISC"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_CONF_MISC>;
#[doc = "Register EXTMEM_CACHE_CONF_MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_CONF_MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_CACHE_TRACE_ENA`"]
pub type EXTMEM_CACHE_TRACE_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CACHE_TRACE_ENA`"]
pub struct EXTMEM_CACHE_TRACE_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CACHE_TRACE_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT`"]
pub type EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT`"]
pub struct EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W<'a> {
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
#[doc = "Reader of field `EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT`"]
pub type EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT`"]
pub struct EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_cache_trace_ena(&self) -> EXTMEM_CACHE_TRACE_ENA_R {
        EXTMEM_CACHE_TRACE_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_cache_ignore_sync_mmu_entry_fault(
        &self,
    ) -> EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R {
        EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_cache_ignore_preload_mmu_entry_fault(
        &self,
    ) -> EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R {
        EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_cache_trace_ena(&mut self) -> EXTMEM_CACHE_TRACE_ENA_W {
        EXTMEM_CACHE_TRACE_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_cache_ignore_sync_mmu_entry_fault(
        &mut self,
    ) -> EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W {
        EXTMEM_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_cache_ignore_preload_mmu_entry_fault(
        &mut self,
    ) -> EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W {
        EXTMEM_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W { w: self }
    }
}
