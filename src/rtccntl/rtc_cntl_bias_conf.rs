#[doc = "Reader of register RTC_CNTL_BIAS_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_BIAS_CONF>;
#[doc = "Writer for register RTC_CNTL_BIAS_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_BIAS_CONF>;
#[doc = "Register RTC_CNTL_BIAS_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_BIAS_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DBG_ATTEN_MONITOR`"]
pub type RTC_CNTL_DBG_ATTEN_MONITOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DBG_ATTEN_MONITOR`"]
pub struct RTC_CNTL_DBG_ATTEN_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DBG_ATTEN_MONITOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DBG_ATTEN_DEEP_SLP`"]
pub type RTC_CNTL_DBG_ATTEN_DEEP_SLP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DBG_ATTEN_DEEP_SLP`"]
pub struct RTC_CNTL_DBG_ATTEN_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DBG_ATTEN_DEEP_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_BIAS_SLEEP_MONITOR`"]
pub type RTC_CNTL_BIAS_SLEEP_MONITOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_SLEEP_MONITOR`"]
pub struct RTC_CNTL_BIAS_SLEEP_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_SLEEP_MONITOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_BIAS_SLEEP_DEEP_SLP`"]
pub type RTC_CNTL_BIAS_SLEEP_DEEP_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_SLEEP_DEEP_SLP`"]
pub struct RTC_CNTL_BIAS_SLEEP_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_SLEEP_DEEP_SLP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_PD_CUR_MONITOR`"]
pub type RTC_CNTL_PD_CUR_MONITOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PD_CUR_MONITOR`"]
pub struct RTC_CNTL_PD_CUR_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PD_CUR_MONITOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_PD_CUR_DEEP_SLP`"]
pub type RTC_CNTL_PD_CUR_DEEP_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PD_CUR_DEEP_SLP`"]
pub struct RTC_CNTL_PD_CUR_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PD_CUR_DEEP_SLP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_BIAS_BUF_MONITOR`"]
pub type RTC_CNTL_BIAS_BUF_MONITOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_BUF_MONITOR`"]
pub struct RTC_CNTL_BIAS_BUF_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_BUF_MONITOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_BIAS_BUF_DEEP_SLP`"]
pub type RTC_CNTL_BIAS_BUF_DEEP_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_BUF_DEEP_SLP`"]
pub struct RTC_CNTL_BIAS_BUF_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_BUF_DEEP_SLP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_BIAS_BUF_WAKE`"]
pub type RTC_CNTL_BIAS_BUF_WAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_BUF_WAKE`"]
pub struct RTC_CNTL_BIAS_BUF_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_BUF_WAKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_BIAS_BUF_IDLE`"]
pub type RTC_CNTL_BIAS_BUF_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BIAS_BUF_IDLE`"]
pub struct RTC_CNTL_BIAS_BUF_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BIAS_BUF_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_VDD_DRV_B_SLP_EN`"]
pub type RTC_CNTL_DG_VDD_DRV_B_SLP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_VDD_DRV_B_SLP_EN`"]
pub struct RTC_CNTL_DG_VDD_DRV_B_SLP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_VDD_DRV_B_SLP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_VDD_DRV_B_SLP`"]
pub type RTC_CNTL_DG_VDD_DRV_B_SLP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DG_VDD_DRV_B_SLP`"]
pub struct RTC_CNTL_DG_VDD_DRV_B_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_VDD_DRV_B_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn rtc_cntl_dbg_atten_monitor(&self) -> RTC_CNTL_DBG_ATTEN_MONITOR_R {
        RTC_CNTL_DBG_ATTEN_MONITOR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn rtc_cntl_dbg_atten_deep_slp(&self) -> RTC_CNTL_DBG_ATTEN_DEEP_SLP_R {
        RTC_CNTL_DBG_ATTEN_DEEP_SLP_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtc_cntl_bias_sleep_monitor(&self) -> RTC_CNTL_BIAS_SLEEP_MONITOR_R {
        RTC_CNTL_BIAS_SLEEP_MONITOR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_bias_sleep_deep_slp(&self) -> RTC_CNTL_BIAS_SLEEP_DEEP_SLP_R {
        RTC_CNTL_BIAS_SLEEP_DEEP_SLP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_pd_cur_monitor(&self) -> RTC_CNTL_PD_CUR_MONITOR_R {
        RTC_CNTL_PD_CUR_MONITOR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtc_cntl_pd_cur_deep_slp(&self) -> RTC_CNTL_PD_CUR_DEEP_SLP_R {
        RTC_CNTL_PD_CUR_DEEP_SLP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_bias_buf_monitor(&self) -> RTC_CNTL_BIAS_BUF_MONITOR_R {
        RTC_CNTL_BIAS_BUF_MONITOR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_bias_buf_deep_slp(&self) -> RTC_CNTL_BIAS_BUF_DEEP_SLP_R {
        RTC_CNTL_BIAS_BUF_DEEP_SLP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_bias_buf_wake(&self) -> RTC_CNTL_BIAS_BUF_WAKE_R {
        RTC_CNTL_BIAS_BUF_WAKE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtc_cntl_bias_buf_idle(&self) -> RTC_CNTL_BIAS_BUF_IDLE_R {
        RTC_CNTL_BIAS_BUF_IDLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_dg_vdd_drv_b_slp_en(&self) -> RTC_CNTL_DG_VDD_DRV_B_SLP_EN_R {
        RTC_CNTL_DG_VDD_DRV_B_SLP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_cntl_dg_vdd_drv_b_slp(&self) -> RTC_CNTL_DG_VDD_DRV_B_SLP_R {
        RTC_CNTL_DG_VDD_DRV_B_SLP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn rtc_cntl_dbg_atten_monitor(&mut self) -> RTC_CNTL_DBG_ATTEN_MONITOR_W {
        RTC_CNTL_DBG_ATTEN_MONITOR_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn rtc_cntl_dbg_atten_deep_slp(&mut self) -> RTC_CNTL_DBG_ATTEN_DEEP_SLP_W {
        RTC_CNTL_DBG_ATTEN_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtc_cntl_bias_sleep_monitor(&mut self) -> RTC_CNTL_BIAS_SLEEP_MONITOR_W {
        RTC_CNTL_BIAS_SLEEP_MONITOR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_bias_sleep_deep_slp(&mut self) -> RTC_CNTL_BIAS_SLEEP_DEEP_SLP_W {
        RTC_CNTL_BIAS_SLEEP_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_pd_cur_monitor(&mut self) -> RTC_CNTL_PD_CUR_MONITOR_W {
        RTC_CNTL_PD_CUR_MONITOR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtc_cntl_pd_cur_deep_slp(&mut self) -> RTC_CNTL_PD_CUR_DEEP_SLP_W {
        RTC_CNTL_PD_CUR_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_bias_buf_monitor(&mut self) -> RTC_CNTL_BIAS_BUF_MONITOR_W {
        RTC_CNTL_BIAS_BUF_MONITOR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_bias_buf_deep_slp(&mut self) -> RTC_CNTL_BIAS_BUF_DEEP_SLP_W {
        RTC_CNTL_BIAS_BUF_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_bias_buf_wake(&mut self) -> RTC_CNTL_BIAS_BUF_WAKE_W {
        RTC_CNTL_BIAS_BUF_WAKE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtc_cntl_bias_buf_idle(&mut self) -> RTC_CNTL_BIAS_BUF_IDLE_W {
        RTC_CNTL_BIAS_BUF_IDLE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_dg_vdd_drv_b_slp_en(&mut self) -> RTC_CNTL_DG_VDD_DRV_B_SLP_EN_W {
        RTC_CNTL_DG_VDD_DRV_B_SLP_EN_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_cntl_dg_vdd_drv_b_slp(&mut self) -> RTC_CNTL_DG_VDD_DRV_B_SLP_W {
        RTC_CNTL_DG_VDD_DRV_B_SLP_W { w: self }
    }
}
