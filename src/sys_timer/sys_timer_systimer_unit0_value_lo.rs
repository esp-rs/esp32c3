#[doc = "Reader of register SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO>;
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT0_VALUE_LO`"]
pub type SYS_TIMER_TIMER_UNIT0_VALUE_LO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_value_lo(&self) -> SYS_TIMER_TIMER_UNIT0_VALUE_LO_R {
        SYS_TIMER_TIMER_UNIT0_VALUE_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
