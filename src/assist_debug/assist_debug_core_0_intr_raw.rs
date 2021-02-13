#[doc = "Reader of register ASSIST_DEBUG_CORE_0_INTR_RAW"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_INTR_RAW>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_RAW`"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_RAW`"]
pub type ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_RAW`"]
pub type ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_RAW`"]
pub type ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_RAW`"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_RAW`"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_RAW`"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_RAW`"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_RAW`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_RAW`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_RAW`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_RAW`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_RAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn assist_debug_core_0_dram0_exception_monitor_raw(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R {
        ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn assist_debug_core_0_iram0_exception_monitor_raw(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R {
        ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn assist_debug_core_0_sp_spill_max_raw(&self) -> ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_RAW_R {
        ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn assist_debug_core_0_sp_spill_min_raw(&self) -> ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_RAW_R {
        ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_1_wr_raw(&self) -> ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_RAW_R {
        ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_1_rd_raw(&self) -> ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_RAW_R {
        ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_0_wr_raw(&self) -> ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_RAW_R {
        ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_0_rd_raw(&self) -> ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_RAW_R {
        ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_1_wr_raw(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_RAW_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_1_rd_raw(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_RAW_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_0_wr_raw(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_RAW_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_0_rd_raw(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_RAW_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_RAW_R::new((self.bits & 0x01) != 0)
    }
}
