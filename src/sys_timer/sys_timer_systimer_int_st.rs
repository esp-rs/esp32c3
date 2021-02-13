#[doc = "Reader of register SYS_TIMER_SYSTIMER_INT_ST"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_INT_ST>;
#[doc = "Reader of field `SYS_TIMER_TARGET2_INT_ST`"]
pub type SYS_TIMER_TARGET2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYS_TIMER_TARGET1_INT_ST`"]
pub type SYS_TIMER_TARGET1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYS_TIMER_TARGET0_INT_ST`"]
pub type SYS_TIMER_TARGET0_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sys_timer_target2_int_st(&self) -> SYS_TIMER_TARGET2_INT_ST_R {
        SYS_TIMER_TARGET2_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sys_timer_target1_int_st(&self) -> SYS_TIMER_TARGET1_INT_ST_R {
        SYS_TIMER_TARGET1_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_timer_target0_int_st(&self) -> SYS_TIMER_TARGET0_INT_ST_R {
        SYS_TIMER_TARGET0_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
