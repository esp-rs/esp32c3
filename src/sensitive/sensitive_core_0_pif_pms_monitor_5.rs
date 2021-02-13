#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_MONITOR_5"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_MONITOR_5>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_nonword_violate_status_hworld(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD_R::new(
            ((self.bits >> 3) & 0x03) as u8,
        )
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_nonword_violate_status_hsize(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE_R::new(
            ((self.bits >> 1) & 0x03) as u8,
        )
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_nonword_violate_intr(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
