#[doc = "Reader of register INTERRUPT_CORE0_INTR_STATUS_1"]
pub type R = crate::R<u32, super::INTERRUPT_CORE0_INTR_STATUS_1>;
#[doc = "Reader of field `INTERRUPT_CORE0_INTR_STATUS_1`"]
pub type INTERRUPT_CORE0_INTR_STATUS_1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interrupt_core0_intr_status_1(&self) -> INTERRUPT_CORE0_INTR_STATUS_1_R {
        INTERRUPT_CORE0_INTR_STATUS_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
