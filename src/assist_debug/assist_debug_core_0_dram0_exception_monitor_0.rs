#[doc = "Reader of register ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_BYTEEN_0`"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_BYTEEN_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_WR_0`"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_WR_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_ADDR_0`"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_ADDR_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn assist_debug_core_0_dram0_recording_byteen_0(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_BYTEEN_0_R {
        ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_BYTEEN_0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn assist_debug_core_0_dram0_recording_wr_0(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_WR_0_R {
        ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_WR_0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn assist_debug_core_0_dram0_recording_addr_0(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_ADDR_0_R {
        ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_ADDR_0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
