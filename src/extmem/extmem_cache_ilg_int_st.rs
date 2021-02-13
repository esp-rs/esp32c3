#[doc = "Reader of register EXTMEM_CACHE_ILG_INT_ST"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_ILG_INT_ST>;
#[doc = "Reader of field `EXTMEM_DBUS_ACS_FLASH_MISS_CNT_OVF_ST`"]
pub type EXTMEM_DBUS_ACS_FLASH_MISS_CNT_OVF_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_DBUS_ACS_CNT_OVF_ST`"]
pub type EXTMEM_DBUS_ACS_CNT_OVF_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_IBUS_ACS_MISS_CNT_OVF_ST`"]
pub type EXTMEM_IBUS_ACS_MISS_CNT_OVF_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_IBUS_ACS_CNT_OVF_ST`"]
pub type EXTMEM_IBUS_ACS_CNT_OVF_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_MMU_ENTRY_FAULT_ST`"]
pub type EXTMEM_MMU_ENTRY_FAULT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_ICACHE_PRELOAD_OP_FAULT_ST`"]
pub type EXTMEM_ICACHE_PRELOAD_OP_FAULT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_ICACHE_SYNC_OP_FAULT_ST`"]
pub type EXTMEM_ICACHE_SYNC_OP_FAULT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extmem_dbus_acs_flash_miss_cnt_ovf_st(&self) -> EXTMEM_DBUS_ACS_FLASH_MISS_CNT_OVF_ST_R {
        EXTMEM_DBUS_ACS_FLASH_MISS_CNT_OVF_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn extmem_dbus_acs_cnt_ovf_st(&self) -> EXTMEM_DBUS_ACS_CNT_OVF_ST_R {
        EXTMEM_DBUS_ACS_CNT_OVF_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn extmem_ibus_acs_miss_cnt_ovf_st(&self) -> EXTMEM_IBUS_ACS_MISS_CNT_OVF_ST_R {
        EXTMEM_IBUS_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn extmem_ibus_acs_cnt_ovf_st(&self) -> EXTMEM_IBUS_ACS_CNT_OVF_ST_R {
        EXTMEM_IBUS_ACS_CNT_OVF_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn extmem_mmu_entry_fault_st(&self) -> EXTMEM_MMU_ENTRY_FAULT_ST_R {
        EXTMEM_MMU_ENTRY_FAULT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_preload_op_fault_st(&self) -> EXTMEM_ICACHE_PRELOAD_OP_FAULT_ST_R {
        EXTMEM_ICACHE_PRELOAD_OP_FAULT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_sync_op_fault_st(&self) -> EXTMEM_ICACHE_SYNC_OP_FAULT_ST_R {
        EXTMEM_ICACHE_SYNC_OP_FAULT_ST_R::new((self.bits & 0x01) != 0)
    }
}
