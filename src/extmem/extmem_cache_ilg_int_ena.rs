#[doc = "Reader of register EXTMEM_CACHE_ILG_INT_ENA"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_ILG_INT_ENA>;
#[doc = "Writer for register EXTMEM_CACHE_ILG_INT_ENA"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_ILG_INT_ENA>;
#[doc = "Register EXTMEM_CACHE_ILG_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_ILG_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_DBUS_CNT_OVF_INT_ENA`"]
pub type EXTMEM_DBUS_CNT_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_DBUS_CNT_OVF_INT_ENA`"]
pub struct EXTMEM_DBUS_CNT_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_DBUS_CNT_OVF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_IBUS_CNT_OVF_INT_ENA`"]
pub type EXTMEM_IBUS_CNT_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_IBUS_CNT_OVF_INT_ENA`"]
pub struct EXTMEM_IBUS_CNT_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_IBUS_CNT_OVF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_MMU_ENTRY_FAULT_INT_ENA`"]
pub type EXTMEM_MMU_ENTRY_FAULT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_MMU_ENTRY_FAULT_INT_ENA`"]
pub struct EXTMEM_MMU_ENTRY_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_MMU_ENTRY_FAULT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA`"]
pub type EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA`"]
pub struct EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA`"]
pub type EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA`"]
pub struct EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn extmem_dbus_cnt_ovf_int_ena(&self) -> EXTMEM_DBUS_CNT_OVF_INT_ENA_R {
        EXTMEM_DBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn extmem_ibus_cnt_ovf_int_ena(&self) -> EXTMEM_IBUS_CNT_OVF_INT_ENA_R {
        EXTMEM_IBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn extmem_mmu_entry_fault_int_ena(&self) -> EXTMEM_MMU_ENTRY_FAULT_INT_ENA_R {
        EXTMEM_MMU_ENTRY_FAULT_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_preload_op_fault_int_ena(
        &self,
    ) -> EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA_R {
        EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_sync_op_fault_int_ena(&self) -> EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA_R {
        EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn extmem_dbus_cnt_ovf_int_ena(&mut self) -> EXTMEM_DBUS_CNT_OVF_INT_ENA_W {
        EXTMEM_DBUS_CNT_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn extmem_ibus_cnt_ovf_int_ena(&mut self) -> EXTMEM_IBUS_CNT_OVF_INT_ENA_W {
        EXTMEM_IBUS_CNT_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn extmem_mmu_entry_fault_int_ena(&mut self) -> EXTMEM_MMU_ENTRY_FAULT_INT_ENA_W {
        EXTMEM_MMU_ENTRY_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_preload_op_fault_int_ena(
        &mut self,
    ) -> EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA_W {
        EXTMEM_ICACHE_PRELOAD_OP_FAULT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_sync_op_fault_int_ena(&mut self) -> EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA_W {
        EXTMEM_ICACHE_SYNC_OP_FAULT_INT_ENA_W { w: self }
    }
}
