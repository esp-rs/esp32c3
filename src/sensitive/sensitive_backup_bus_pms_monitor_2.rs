#[doc = "Reader of register SENSITIVE_BACKUP_BUS_PMS_MONITOR_2"]
pub type R = crate::R<u32, super::SENSITIVE_BACKUP_BUS_PMS_MONITOR_2>;
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE`"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE`"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS`"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR`"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_monitor_violate_status_hwrite(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
        SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R::new(
            ((self.bits >> 6) & 0x01) != 0,
        )
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_monitor_violate_status_hsize(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
        SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R::new(
            ((self.bits >> 3) & 0x07) as u8,
        )
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_monitor_violate_status_htrans(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R {
        SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R::new(
            ((self.bits >> 1) & 0x03) as u8,
        )
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_monitor_violate_intr(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R {
        SENSITIVE_BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
