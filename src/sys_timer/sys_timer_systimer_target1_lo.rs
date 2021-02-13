#[doc = "Reader of register SYS_TIMER_SYSTIMER_TARGET1_LO"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_TARGET1_LO>;
#[doc = "Writer for register SYS_TIMER_SYSTIMER_TARGET1_LO"]
pub type W = crate::W<u32, super::SYS_TIMER_SYSTIMER_TARGET1_LO>;
#[doc = "Register SYS_TIMER_SYSTIMER_TARGET1_LO `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_TIMER_SYSTIMER_TARGET1_LO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYS_TIMER_TIMER_TARGET1_LO`"]
pub type SYS_TIMER_TIMER_TARGET1_LO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_TARGET1_LO`"]
pub struct SYS_TIMER_TIMER_TARGET1_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_TARGET1_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_timer_timer_target1_lo(&self) -> SYS_TIMER_TIMER_TARGET1_LO_R {
        SYS_TIMER_TIMER_TARGET1_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_timer_timer_target1_lo(&mut self) -> SYS_TIMER_TIMER_TARGET1_LO_W {
        SYS_TIMER_TIMER_TARGET1_LO_W { w: self }
    }
}
