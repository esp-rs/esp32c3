#[doc = "Reader of register INTERRUPT_CORE0_INTR_STATUS_0"]
pub type R = crate::R<u32, super::INTERRUPT_CORE0_INTR_STATUS_0>;
#[doc = "Reader of field `INTERRUPT_CORE0_INTR_STATUS_0`"]
pub type INTERRUPT_CORE0_INTR_STATUS_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interrupt_core0_intr_status_0(&self) -> INTERRUPT_CORE0_INTR_STATUS_0_R {
        INTERRUPT_CORE0_INTR_STATUS_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
