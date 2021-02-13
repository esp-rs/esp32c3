#[doc = "Reader of register SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI>;
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT0_VALUE_HI`"]
pub type SYS_TIMER_TIMER_UNIT0_VALUE_HI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_value_hi(&self) -> SYS_TIMER_TIMER_UNIT0_VALUE_HI_R {
        SYS_TIMER_TIMER_UNIT0_VALUE_HI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
