#[doc = "Reader of register SYS_TIMER_SYSTIMER_CONF"]
pub type R = crate::R<u32, super::SYS_TIMER_SYSTIMER_CONF>;
#[doc = "Writer for register SYS_TIMER_SYSTIMER_CONF"]
pub type W = crate::W<u32, super::SYS_TIMER_SYSTIMER_CONF>;
#[doc = "Register SYS_TIMER_SYSTIMER_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SYS_TIMER_SYSTIMER_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYS_TIMER_CLK_EN`"]
pub type SYS_TIMER_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_CLK_EN`"]
pub struct SYS_TIMER_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_CLK_EN_W<'a> {
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
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT0_WORK_EN`"]
pub type SYS_TIMER_TIMER_UNIT0_WORK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_UNIT0_WORK_EN`"]
pub struct SYS_TIMER_TIMER_UNIT0_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_UNIT0_WORK_EN_W<'a> {
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
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT1_WORK_EN`"]
pub type SYS_TIMER_TIMER_UNIT1_WORK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_UNIT1_WORK_EN`"]
pub struct SYS_TIMER_TIMER_UNIT1_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_UNIT1_WORK_EN_W<'a> {
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
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN`"]
pub type SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN`"]
pub struct SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN`"]
pub type SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN`"]
pub struct SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN`"]
pub type SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN`"]
pub struct SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN`"]
pub type SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN`"]
pub struct SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET0_WORK_EN`"]
pub type SYS_TIMER_TARGET0_WORK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET0_WORK_EN`"]
pub struct SYS_TIMER_TARGET0_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET0_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET1_WORK_EN`"]
pub type SYS_TIMER_TARGET1_WORK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET1_WORK_EN`"]
pub struct SYS_TIMER_TARGET1_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET1_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_TARGET2_WORK_EN`"]
pub type SYS_TIMER_TARGET2_WORK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_TARGET2_WORK_EN`"]
pub struct SYS_TIMER_TARGET2_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_TARGET2_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SYS_TIMER_SYSTIMER_CLK_FO`"]
pub type SYS_TIMER_SYSTIMER_CLK_FO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_TIMER_SYSTIMER_CLK_FO`"]
pub struct SYS_TIMER_SYSTIMER_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_TIMER_SYSTIMER_CLK_FO_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_timer_clk_en(&self) -> SYS_TIMER_CLK_EN_R {
        SYS_TIMER_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_work_en(&self) -> SYS_TIMER_TIMER_UNIT0_WORK_EN_R {
        SYS_TIMER_TIMER_UNIT0_WORK_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sys_timer_timer_unit1_work_en(&self) -> SYS_TIMER_TIMER_UNIT1_WORK_EN_R {
        SYS_TIMER_TIMER_UNIT1_WORK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_core0_stall_en(&self) -> SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN_R {
        SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_core1_stall_en(&self) -> SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN_R {
        SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sys_timer_timer_unit1_core0_stall_en(&self) -> SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN_R {
        SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sys_timer_timer_unit1_core1_stall_en(&self) -> SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN_R {
        SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sys_timer_target0_work_en(&self) -> SYS_TIMER_TARGET0_WORK_EN_R {
        SYS_TIMER_TARGET0_WORK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sys_timer_target1_work_en(&self) -> SYS_TIMER_TARGET1_WORK_EN_R {
        SYS_TIMER_TARGET1_WORK_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sys_timer_target2_work_en(&self) -> SYS_TIMER_TARGET2_WORK_EN_R {
        SYS_TIMER_TARGET2_WORK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_timer_systimer_clk_fo(&self) -> SYS_TIMER_SYSTIMER_CLK_FO_R {
        SYS_TIMER_SYSTIMER_CLK_FO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_timer_clk_en(&mut self) -> SYS_TIMER_CLK_EN_W {
        SYS_TIMER_CLK_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_work_en(&mut self) -> SYS_TIMER_TIMER_UNIT0_WORK_EN_W {
        SYS_TIMER_TIMER_UNIT0_WORK_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sys_timer_timer_unit1_work_en(&mut self) -> SYS_TIMER_TIMER_UNIT1_WORK_EN_W {
        SYS_TIMER_TIMER_UNIT1_WORK_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_core0_stall_en(
        &mut self,
    ) -> SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN_W {
        SYS_TIMER_TIMER_UNIT0_CORE0_STALL_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sys_timer_timer_unit0_core1_stall_en(
        &mut self,
    ) -> SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN_W {
        SYS_TIMER_TIMER_UNIT0_CORE1_STALL_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sys_timer_timer_unit1_core0_stall_en(
        &mut self,
    ) -> SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN_W {
        SYS_TIMER_TIMER_UNIT1_CORE0_STALL_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sys_timer_timer_unit1_core1_stall_en(
        &mut self,
    ) -> SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN_W {
        SYS_TIMER_TIMER_UNIT1_CORE1_STALL_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sys_timer_target0_work_en(&mut self) -> SYS_TIMER_TARGET0_WORK_EN_W {
        SYS_TIMER_TARGET0_WORK_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sys_timer_target1_work_en(&mut self) -> SYS_TIMER_TARGET1_WORK_EN_W {
        SYS_TIMER_TARGET1_WORK_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sys_timer_target2_work_en(&mut self) -> SYS_TIMER_TARGET2_WORK_EN_W {
        SYS_TIMER_TARGET2_WORK_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_timer_systimer_clk_fo(&mut self) -> SYS_TIMER_SYSTIMER_CLK_FO_W {
        SYS_TIMER_SYSTIMER_CLK_FO_W { w: self }
    }
}
