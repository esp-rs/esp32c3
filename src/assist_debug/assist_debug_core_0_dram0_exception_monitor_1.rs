#[doc = "Reader of register ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_PC_0`"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_PC_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn assist_debug_core_0_dram0_recording_pc_0(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_PC_0_R {
        ASSIST_DEBUG_CORE_0_DRAM0_RECORDING_PC_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
