#[doc = "Reader of register RTC_CNTL_STATE0"]
pub type R = crate::R<u32, super::RTC_CNTL_STATE0>;
#[doc = "Writer for register RTC_CNTL_STATE0"]
pub type W = crate::W<u32, super::RTC_CNTL_STATE0>;
#[doc = "Register RTC_CNTL_STATE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_STATE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_SLEEP_EN`"]
pub type RTC_CNTL_SLEEP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLEEP_EN`"]
pub struct RTC_CNTL_SLEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLEEP_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLP_REJECT`"]
pub type RTC_CNTL_SLP_REJECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLP_REJECT`"]
pub struct RTC_CNTL_SLP_REJECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_REJECT_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLP_WAKEUP`"]
pub type RTC_CNTL_SLP_WAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLP_WAKEUP`"]
pub struct RTC_CNTL_SLP_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_WAKEUP_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SDIO_ACTIVE_IND`"]
pub type RTC_CNTL_SDIO_ACTIVE_IND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_APB2RTC_BRIDGE_SEL`"]
pub type RTC_CNTL_APB2RTC_BRIDGE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_APB2RTC_BRIDGE_SEL`"]
pub struct RTC_CNTL_APB2RTC_BRIDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_APB2RTC_BRIDGE_SEL_W<'a> {
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
#[doc = "Write proxy for field `RTC_CNTL_SLP_REJECT_CAUSE_CLR`"]
pub struct RTC_CNTL_SLP_REJECT_CAUSE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_REJECT_CAUSE_CLR_W<'a> {
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
#[doc = "Write proxy for field `RTC_CNTL_SW_CPU_INT`"]
pub struct RTC_CNTL_SW_CPU_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_CPU_INT_W<'a> {
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
    pub fn rtc_cntl_sleep_en(&self) -> RTC_CNTL_SLEEP_EN_R {
        RTC_CNTL_SLEEP_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject(&self) -> RTC_CNTL_SLP_REJECT_R {
        RTC_CNTL_SLP_REJECT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_slp_wakeup(&self) -> RTC_CNTL_SLP_WAKEUP_R {
        RTC_CNTL_SLP_WAKEUP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_active_ind(&self) -> RTC_CNTL_SDIO_ACTIVE_IND_R {
        RTC_CNTL_SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_apb2rtc_bridge_sel(&self) -> RTC_CNTL_APB2RTC_BRIDGE_SEL_R {
        RTC_CNTL_APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_sleep_en(&mut self) -> RTC_CNTL_SLEEP_EN_W {
        RTC_CNTL_SLEEP_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject(&mut self) -> RTC_CNTL_SLP_REJECT_W {
        RTC_CNTL_SLP_REJECT_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_slp_wakeup(&mut self) -> RTC_CNTL_SLP_WAKEUP_W {
        RTC_CNTL_SLP_WAKEUP_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_apb2rtc_bridge_sel(&mut self) -> RTC_CNTL_APB2RTC_BRIDGE_SEL_W {
        RTC_CNTL_APB2RTC_BRIDGE_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject_cause_clr(&mut self) -> RTC_CNTL_SLP_REJECT_CAUSE_CLR_W {
        RTC_CNTL_SLP_REJECT_CAUSE_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_sw_cpu_int(&mut self) -> RTC_CNTL_SW_CPU_INT_W {
        RTC_CNTL_SW_CPU_INT_W { w: self }
    }
}
