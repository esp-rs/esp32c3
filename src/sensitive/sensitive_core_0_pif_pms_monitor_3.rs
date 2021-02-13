#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_MONITOR_3"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_MONITOR_3>;
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR`"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_monitor_violate_status_haddr(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR_R {
        SENSITIVE_CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR_R::new(
            (self.bits & 0xffff_ffff) as u32,
        )
    }
}
