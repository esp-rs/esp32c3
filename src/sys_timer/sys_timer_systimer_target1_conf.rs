#[doc = "Reader of register SYS_TIMER_SYSTIMER_TARGET1_CONF"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_TARGET1_CONF>;
#[doc = "Writer for register SYS_TIMER_SYSTIMER_TARGET1_CONF"]
pub type W = crate::W<u32, super::SYS_TIMER_SYSTIMER_TARGET1_CONF>;
#[doc = "Register SYS_TIMER_SYSTIMER_TARGET1_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_TIMER_SYSTIMER_TARGET1_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET1_TIMER_UNIT_SEL`"]
pub type SYS_TIMER_TARGET1_TIMER_UNIT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET1_TIMER_UNIT_SEL`"]
pub struct SYS_TIMER_TARGET1_TIMER_UNIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET1_TIMER_UNIT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET1_PERIOD_MODE`"]
pub type SYS_TIMER_TARGET1_PERIOD_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET1_PERIOD_MODE`"]
pub struct SYS_TIMER_TARGET1_PERIOD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET1_PERIOD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET1_PERIOD`"]
pub type SYS_TIMER_TARGET1_PERIOD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET1_PERIOD`"]
pub struct SYS_TIMER_TARGET1_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET1_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_timer_target1_timer_unit_sel(&self) -> SYS_TIMER_TARGET1_TIMER_UNIT_SEL_R {
        SYS_TIMER_TARGET1_TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_timer_target1_period_mode(&self) -> SYS_TIMER_TARGET1_PERIOD_MODE_R {
        SYS_TIMER_TARGET1_PERIOD_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn sys_timer_target1_period(&self) -> SYS_TIMER_TARGET1_PERIOD_R {
        SYS_TIMER_TARGET1_PERIOD_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_timer_target1_timer_unit_sel(&mut self) -> SYS_TIMER_TARGET1_TIMER_UNIT_SEL_W {
        SYS_TIMER_TARGET1_TIMER_UNIT_SEL_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_timer_target1_period_mode(&mut self) -> SYS_TIMER_TARGET1_PERIOD_MODE_W {
        SYS_TIMER_TARGET1_PERIOD_MODE_W { w: self }
    }
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn sys_timer_target1_period(&mut self) -> SYS_TIMER_TARGET1_PERIOD_W {
        SYS_TIMER_TARGET1_PERIOD_W { w: self }
    }
}
