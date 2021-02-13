#[doc = "Reader of register SYS_TIMER_SYSTIMER_TARGET2_HI"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_TARGET2_HI>;
#[doc = "Writer for register SYS_TIMER_SYSTIMER_TARGET2_HI"]
pub type W = crate::W<u32, super::SYS_TIMER_SYSTIMER_TARGET2_HI>;
#[doc = "Register SYS_TIMER_SYSTIMER_TARGET2_HI `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_TIMER_SYSTIMER_TARGET2_HI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYS_TIMER_TIMER_TARGET2_HI`"]
pub type SYS_TIMER_TIMER_TARGET2_HI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_TARGET2_HI`"]
pub struct SYS_TIMER_TIMER_TARGET2_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_TARGET2_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn sys_timer_timer_target2_hi(&self) -> SYS_TIMER_TIMER_TARGET2_HI_R {
        SYS_TIMER_TIMER_TARGET2_HI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn sys_timer_timer_target2_hi(&mut self) -> SYS_TIMER_TIMER_TARGET2_HI_W {
        SYS_TIMER_TIMER_TARGET2_HI_W { w: self }
    }
}
