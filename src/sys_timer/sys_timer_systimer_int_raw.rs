#[doc = "Reader of register SYS_TIMER_SYSTIMER_INT_RAW"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_INT_RAW>;
#[doc = "Writer for register SYS_TIMER_SYSTIMER_INT_RAW"]
pub type W = crate::W<u32, super::SYS_TIMER_SYSTIMER_INT_RAW>;
#[doc = "Register SYS_TIMER_SYSTIMER_INT_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_TIMER_SYSTIMER_INT_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET2_INT_RAW`"]
pub type SYS_TIMER_TARGET2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET2_INT_RAW`"]
pub struct SYS_TIMER_TARGET2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET2_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET1_INT_RAW`"]
pub type SYS_TIMER_TARGET1_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET1_INT_RAW`"]
pub struct SYS_TIMER_TARGET1_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET1_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET0_INT_RAW`"]
pub type SYS_TIMER_TARGET0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET0_INT_RAW`"]
pub struct SYS_TIMER_TARGET0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET0_INT_RAW_W<'a> {
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
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sys_timer_target2_int_raw(&self) -> SYS_TIMER_TARGET2_INT_RAW_R {
        SYS_TIMER_TARGET2_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sys_timer_target1_int_raw(&self) -> SYS_TIMER_TARGET1_INT_RAW_R {
        SYS_TIMER_TARGET1_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_timer_target0_int_raw(&self) -> SYS_TIMER_TARGET0_INT_RAW_R {
        SYS_TIMER_TARGET0_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sys_timer_target2_int_raw(&mut self) -> SYS_TIMER_TARGET2_INT_RAW_W {
        SYS_TIMER_TARGET2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sys_timer_target1_int_raw(&mut self) -> SYS_TIMER_TARGET1_INT_RAW_W {
        SYS_TIMER_TARGET1_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_timer_target0_int_raw(&mut self) -> SYS_TIMER_TARGET0_INT_RAW_W {
        SYS_TIMER_TARGET0_INT_RAW_W { w: self }
    }
}
