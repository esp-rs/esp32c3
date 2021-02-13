#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_MONITOR_2"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_MONITOR_2>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_violate_status_hworld(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD_R::new(
            ((self.bits >> 6) & 0x03) as u8,
        )
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_violate_status_hwrite(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R::new(
            ((self.bits >> 5) & 0x01) != 0,
        )
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_violate_status_hsize(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R::new(
            ((self.bits >> 2) & 0x07) as u8,
        )
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_violate_status_hport_0(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0_R::new(
            ((self.bits >> 1) & 0x01) != 0,
        )
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_violate_intr(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
