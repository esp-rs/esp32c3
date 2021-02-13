#[doc = "Writer for register EXTMEM_CACHE_ILG_INT_CLR"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_ILG_INT_CLR>;
#[doc = "Register EXTMEM_CACHE_ILG_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_ILG_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EXTMEM_DBUS_CNT_OVF_INT_CLR`"]
pub struct EXTMEM_DBUS_CNT_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_DBUS_CNT_OVF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `EXTMEM_IBUS_CNT_OVF_INT_CLR`"]
pub struct EXTMEM_IBUS_CNT_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_IBUS_CNT_OVF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `EXTMEM_MMU_ENTRY_FAULT_INT_CLR`"]
pub struct EXTMEM_MMU_ENTRY_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_MMU_ENTRY_FAULT_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_CLR`"]
pub struct EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `EXTMEM_ICACHE_SYNC_OP_FAULT_INT_CLR`"]
pub struct EXTMEM_ICACHE_SYNC_OP_FAULT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_SYNC_OP_FAULT_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn extmem_dbus_cnt_ovf_int_clr(&mut self) -> EXTMEM_DBUS_CNT_OVF_INT_CLR_W {
        EXTMEM_DBUS_CNT_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn extmem_ibus_cnt_ovf_int_clr(&mut self) -> EXTMEM_IBUS_CNT_OVF_INT_CLR_W {
        EXTMEM_IBUS_CNT_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn extmem_mmu_entry_fault_int_clr(&mut self) -> EXTMEM_MMU_ENTRY_FAULT_INT_CLR_W {
        EXTMEM_MMU_ENTRY_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_preload_op_fault_int_clr(
        &mut self,
    ) -> EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_CLR_W {
        EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_sync_op_fault_int_clr(&mut self) -> EXTMEM_ICACHE_SYNC_OP_FAULT_INT_CLR_W {
        EXTMEM_ICACHE_SYNC_OP_FAULT_INT_CLR_W { w: self }
    }
}
