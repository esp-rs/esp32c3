#[doc = "Writer for register SYS_TIMER_SYSTIMER_COMP0_LOAD"]
pub type W = crate::W<u32, super::SYS_TIMER_SYSTIMER_COMP0_LOAD>;
#[doc = "Register SYS_TIMER_SYSTIMER_COMP0_LOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_TIMER_SYSTIMER_COMP0_LOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SYS_TIMER_TIMER_COMP0_LOAD`"]
pub struct SYS_TIMER_TIMER_COMP0_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_COMP0_LOAD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_timer_timer_comp0_load(&mut self) -> SYS_TIMER_TIMER_COMP0_LOAD_W {
        SYS_TIMER_TIMER_COMP0_LOAD_W { w: self }
    }
}
