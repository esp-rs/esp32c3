#[doc = "Reader of register SYS_TIMER_SYSTIMER_DATE"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_DATE>;
#[doc = "Writer for register SYS_TIMER_SYSTIMER_DATE"]
pub type W = crate::W<u32, super::SYS_TIMER_SYSTIMER_DATE>;
#[doc = "Register SYS_TIMER_SYSTIMER_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_TIMER_SYSTIMER_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYS_TIMER_DATE`"]
pub type SYS_TIMER_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYS_TIMER_DATE`"]
pub struct SYS_TIMER_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_DATE_W<'a> {
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
    pub fn sys_timer_date(&self) -> SYS_TIMER_DATE_R {
        SYS_TIMER_DATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_timer_date(&mut self) -> SYS_TIMER_DATE_W {
        SYS_TIMER_DATE_W { w: self }
    }
}
