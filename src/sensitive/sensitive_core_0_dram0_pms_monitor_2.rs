#[doc = "Reader of register SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2>;
#[doc = "Reader of field `SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR`"]
pub type SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R = crate::R<u32, u32>;
#[doc = "Reader of field `SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD`"]
pub type SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK`"]
pub type SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR`"]
pub type SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 4:27"]
    #[inline(always)]
    pub fn sensitive_core_0_dram0_pms_monitor_violate_status_addr(
        &self,
    ) -> SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new(
            ((self.bits >> 4) & 0x00ff_ffff) as u32,
        )
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_core_0_dram0_pms_monitor_violate_status_world(
        &self,
    ) -> SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(
            ((self.bits >> 2) & 0x03) as u8,
        )
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sensitive_core_0_dram0_pms_monitor_violate_status_lock(
        &self,
    ) -> SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R {
        SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R::new(
            ((self.bits >> 1) & 0x01) != 0,
        )
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_core_0_dram0_pms_monitor_violate_intr(
        &self,
    ) -> SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R {
        SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
