#[doc = "Reader of register ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_LOADSTORE_1`"]
pub type ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_LOADSTORE_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_WR_1`"]
pub type ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_WR_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_ADDR_1`"]
pub type ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_ADDR_1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn assist_debug_core_0_iram0_recording_loadstore_1(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_LOADSTORE_1_R {
        ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_LOADSTORE_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn assist_debug_core_0_iram0_recording_wr_1(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_WR_1_R {
        ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_WR_1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn assist_debug_core_0_iram0_recording_addr_1(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_ADDR_1_R {
        ASSIST_DEBUG_CORE_0_IRAM0_RECORDING_ADDR_1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
