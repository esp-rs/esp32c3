#[doc = "Reader of register INTERRUPT_CORE0_CPU_INT_EIP_STATUS"]
pub type R = crate::R<u32, super::INTERRUPT_CORE0_CPU_INT_EIP_STATUS>;
#[doc = "Reader of field `INTERRUPT_CORE0_CPU_INT_EIP_STATUS`"]
pub type INTERRUPT_CORE0_CPU_INT_EIP_STATUS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interrupt_core0_cpu_int_eip_status(&self) -> INTERRUPT_CORE0_CPU_INT_EIP_STATUS_R {
        INTERRUPT_CORE0_CPU_INT_EIP_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
