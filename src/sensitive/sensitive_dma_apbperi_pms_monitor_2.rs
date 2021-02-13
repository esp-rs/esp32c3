#[doc = "Reader of register SENSITIVE_DMA_APBPERI_PMS_MONITOR_2"]
pub type R = crate::R<u32, super::SENSITIVE_DMA_APBPERI_PMS_MONITOR_2>;
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR`"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R = crate::R<u32, u32>;
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD`"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R = crate::R<u8, u8>;
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR`"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 3:26"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_pms_monitor_violate_status_addr(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new(
            ((self.bits >> 3) & 0x00ff_ffff) as u32,
        )
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_pms_monitor_violate_status_world(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(
            ((self.bits >> 1) & 0x03) as u8,
        )
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_pms_monitor_violate_intr(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R {
        SENSITIVE_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
