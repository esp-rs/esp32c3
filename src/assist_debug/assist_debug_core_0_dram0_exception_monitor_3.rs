#[doc = "Reader of register ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_PC_1`"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_PC_1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn assist_debug_core_0_dram0_recording_pc_1(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_PC_1_R {
        ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_PC_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
