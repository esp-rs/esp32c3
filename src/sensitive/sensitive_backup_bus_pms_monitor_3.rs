#[doc = "Reader of register SENSITIVE_BACKUP_BUS_PMS_MONITOR_3"]
pub type R = crate::R<u32, super::SENSITIVE_BACKUP_BUS_PMS_MONITOR_3>;
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR`"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_monitor_violate_haddr(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR_R {
        SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
