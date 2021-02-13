#[doc = "Reader of register SENSITIVE_DMA_APBPERI_PMS_MONITOR_3"]
pub type R = crate::R<u32, super::SENSITIVE_DMA_APBPERI_PMS_MONITOR_3>;
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN`"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR`"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_pms_monitor_violate_status_byteen(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
        SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R::new(
            ((self.bits >> 1) & 0x0f) as u8,
        )
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_pms_monitor_violate_status_wr(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R::new((self.bits & 0x01) != 0)
    }
}
