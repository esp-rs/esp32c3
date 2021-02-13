#[doc = "Reader of register SYS_TIMER_SYSTIMER_UNIT0_OP"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_UNIT0_OP>;
#[doc = "Writer for register SYS_TIMER_SYSTIMER_UNIT0_OP"]
pub type W = crate::W<u32, super::SYS_TIMER_SYSTIMER_UNIT0_OP>;
#[doc = "Register SYS_TIMER_SYSTIMER_UNIT0_OP `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_TIMER_SYSTIMER_UNIT0_OP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SYS_TIMER_TIMER_UNIT0_UPDATE`"]
pub struct SYS_TIMER_TIMER_UNIT0_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_UNIT0_UPDATE_W<'a> {
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
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT0_VALUE_VALID`"]
pub type SYS_TIMER_TIMER_UNIT0_VALUE_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_UNIT0_VALUE_VALID`"]
pub struct SYS_TIMER_TIMER_UNIT0_VALUE_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_UNIT0_VALUE_VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_value_valid(&self) -> SYS_TIMER_TIMER_UNIT0_VALUE_VALID_R {
        SYS_TIMER_TIMER_UNIT0_VALUE_VALID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_update(&mut self) -> SYS_TIMER_TIMER_UNIT0_UPDATE_W {
        SYS_TIMER_TIMER_UNIT0_UPDATE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_value_valid(&mut self) -> SYS_TIMER_TIMER_UNIT0_VALUE_VALID_W {
        SYS_TIMER_TIMER_UNIT0_VALUE_VALID_W { w: self }
    }
}
