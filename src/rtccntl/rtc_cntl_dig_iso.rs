#[doc = "Reader of register RTC_CNTL_DIG_ISO"]
pub type R = crate::R<u32, super::RTC_CNTL_DIG_ISO>;
#[doc = "Writer for register RTC_CNTL_DIG_ISO"]
pub type W = crate::W<u32, super::RTC_CNTL_DIG_ISO>;
#[doc = "Register RTC_CNTL_DIG_ISO `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_DIG_ISO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_NOISO`"]
pub type RTC_CNTL_DG_WRAP_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_NOISO`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_ISO`"]
pub type RTC_CNTL_DG_WRAP_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_ISO`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WIFI_FORCE_NOISO`"]
pub type RTC_CNTL_WIFI_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WIFI_FORCE_NOISO`"]
pub struct RTC_CNTL_WIFI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WIFI_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WIFI_FORCE_ISO`"]
pub type RTC_CNTL_WIFI_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WIFI_FORCE_ISO`"]
pub struct RTC_CNTL_WIFI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WIFI_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_CPU_TOP_FORCE_NOISO`"]
pub type RTC_CNTL_CPU_TOP_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CPU_TOP_FORCE_NOISO`"]
pub struct RTC_CNTL_CPU_TOP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPU_TOP_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_CPU_TOP_FORCE_ISO`"]
pub type RTC_CNTL_CPU_TOP_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CPU_TOP_FORCE_ISO`"]
pub struct RTC_CNTL_CPU_TOP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPU_TOP_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PERI_FORCE_NOISO`"]
pub type RTC_CNTL_DG_PERI_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PERI_FORCE_NOISO`"]
pub struct RTC_CNTL_DG_PERI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PERI_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PERI_FORCE_ISO`"]
pub type RTC_CNTL_DG_PERI_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PERI_FORCE_ISO`"]
pub struct RTC_CNTL_DG_PERI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PERI_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BT_FORCE_NOISO`"]
pub type RTC_CNTL_BT_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BT_FORCE_NOISO`"]
pub struct RTC_CNTL_BT_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BT_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BT_FORCE_ISO`"]
pub type RTC_CNTL_BT_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BT_FORCE_ISO`"]
pub struct RTC_CNTL_BT_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BT_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PAD_FORCE_HOLD`"]
pub type RTC_CNTL_DG_PAD_FORCE_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PAD_FORCE_HOLD`"]
pub struct RTC_CNTL_DG_PAD_FORCE_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PAD_FORCE_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PAD_FORCE_UNHOLD`"]
pub type RTC_CNTL_DG_PAD_FORCE_UNHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PAD_FORCE_UNHOLD`"]
pub struct RTC_CNTL_DG_PAD_FORCE_UNHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PAD_FORCE_UNHOLD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PAD_FORCE_ISO`"]
pub type RTC_CNTL_DG_PAD_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PAD_FORCE_ISO`"]
pub struct RTC_CNTL_DG_PAD_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PAD_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PAD_FORCE_NOISO`"]
pub type RTC_CNTL_DG_PAD_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PAD_FORCE_NOISO`"]
pub struct RTC_CNTL_DG_PAD_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PAD_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PAD_AUTOHOLD_EN`"]
pub type RTC_CNTL_DG_PAD_AUTOHOLD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PAD_AUTOHOLD_EN`"]
pub struct RTC_CNTL_DG_PAD_AUTOHOLD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PAD_AUTOHOLD_EN_W<'a> {
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
#[doc = "Write proxy for field `RTC_CNTL_CLR_DG_PAD_AUTOHOLD`"]
pub struct RTC_CNTL_CLR_DG_PAD_AUTOHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CLR_DG_PAD_AUTOHOLD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PAD_AUTOHOLD`"]
pub type RTC_CNTL_DG_PAD_AUTOHOLD_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_DIG_ISO_FORCE_ON`"]
pub type RTC_CNTL_DIG_ISO_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DIG_ISO_FORCE_ON`"]
pub struct RTC_CNTL_DIG_ISO_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DIG_ISO_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DIG_ISO_FORCE_OFF`"]
pub type RTC_CNTL_DIG_ISO_FORCE_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DIG_ISO_FORCE_OFF`"]
pub struct RTC_CNTL_DIG_ISO_FORCE_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DIG_ISO_FORCE_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_noiso(&self) -> RTC_CNTL_DG_WRAP_FORCE_NOISO_R {
        RTC_CNTL_DG_WRAP_FORCE_NOISO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_iso(&self) -> RTC_CNTL_DG_WRAP_FORCE_ISO_R {
        RTC_CNTL_DG_WRAP_FORCE_ISO_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_noiso(&self) -> RTC_CNTL_WIFI_FORCE_NOISO_R {
        RTC_CNTL_WIFI_FORCE_NOISO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_iso(&self) -> RTC_CNTL_WIFI_FORCE_ISO_R {
        RTC_CNTL_WIFI_FORCE_ISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_force_noiso(&self) -> RTC_CNTL_CPU_TOP_FORCE_NOISO_R {
        RTC_CNTL_CPU_TOP_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_force_iso(&self) -> RTC_CNTL_CPU_TOP_FORCE_ISO_R {
        RTC_CNTL_CPU_TOP_FORCE_ISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_force_noiso(&self) -> RTC_CNTL_DG_PERI_FORCE_NOISO_R {
        RTC_CNTL_DG_PERI_FORCE_NOISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_force_iso(&self) -> RTC_CNTL_DG_PERI_FORCE_ISO_R {
        RTC_CNTL_DG_PERI_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_bt_force_noiso(&self) -> RTC_CNTL_BT_FORCE_NOISO_R {
        RTC_CNTL_BT_FORCE_NOISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_bt_force_iso(&self) -> RTC_CNTL_BT_FORCE_ISO_R {
        RTC_CNTL_BT_FORCE_ISO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_force_hold(&self) -> RTC_CNTL_DG_PAD_FORCE_HOLD_R {
        RTC_CNTL_DG_PAD_FORCE_HOLD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_force_unhold(&self) -> RTC_CNTL_DG_PAD_FORCE_UNHOLD_R {
        RTC_CNTL_DG_PAD_FORCE_UNHOLD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_force_iso(&self) -> RTC_CNTL_DG_PAD_FORCE_ISO_R {
        RTC_CNTL_DG_PAD_FORCE_ISO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_force_noiso(&self) -> RTC_CNTL_DG_PAD_FORCE_NOISO_R {
        RTC_CNTL_DG_PAD_FORCE_NOISO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_autohold_en(&self) -> RTC_CNTL_DG_PAD_AUTOHOLD_EN_R {
        RTC_CNTL_DG_PAD_AUTOHOLD_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_autohold(&self) -> RTC_CNTL_DG_PAD_AUTOHOLD_R {
        RTC_CNTL_DG_PAD_AUTOHOLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_dig_iso_force_on(&self) -> RTC_CNTL_DIG_ISO_FORCE_ON_R {
        RTC_CNTL_DIG_ISO_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_dig_iso_force_off(&self) -> RTC_CNTL_DIG_ISO_FORCE_OFF_R {
        RTC_CNTL_DIG_ISO_FORCE_OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_noiso(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_NOISO_W {
        RTC_CNTL_DG_WRAP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_iso(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_ISO_W {
        RTC_CNTL_DG_WRAP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_noiso(&mut self) -> RTC_CNTL_WIFI_FORCE_NOISO_W {
        RTC_CNTL_WIFI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_iso(&mut self) -> RTC_CNTL_WIFI_FORCE_ISO_W {
        RTC_CNTL_WIFI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_force_noiso(&mut self) -> RTC_CNTL_CPU_TOP_FORCE_NOISO_W {
        RTC_CNTL_CPU_TOP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_force_iso(&mut self) -> RTC_CNTL_CPU_TOP_FORCE_ISO_W {
        RTC_CNTL_CPU_TOP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_force_noiso(&mut self) -> RTC_CNTL_DG_PERI_FORCE_NOISO_W {
        RTC_CNTL_DG_PERI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_force_iso(&mut self) -> RTC_CNTL_DG_PERI_FORCE_ISO_W {
        RTC_CNTL_DG_PERI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_bt_force_noiso(&mut self) -> RTC_CNTL_BT_FORCE_NOISO_W {
        RTC_CNTL_BT_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_bt_force_iso(&mut self) -> RTC_CNTL_BT_FORCE_ISO_W {
        RTC_CNTL_BT_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_force_hold(&mut self) -> RTC_CNTL_DG_PAD_FORCE_HOLD_W {
        RTC_CNTL_DG_PAD_FORCE_HOLD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_force_unhold(&mut self) -> RTC_CNTL_DG_PAD_FORCE_UNHOLD_W {
        RTC_CNTL_DG_PAD_FORCE_UNHOLD_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_force_iso(&mut self) -> RTC_CNTL_DG_PAD_FORCE_ISO_W {
        RTC_CNTL_DG_PAD_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_force_noiso(&mut self) -> RTC_CNTL_DG_PAD_FORCE_NOISO_W {
        RTC_CNTL_DG_PAD_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_dg_pad_autohold_en(&mut self) -> RTC_CNTL_DG_PAD_AUTOHOLD_EN_W {
        RTC_CNTL_DG_PAD_AUTOHOLD_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtc_cntl_clr_dg_pad_autohold(&mut self) -> RTC_CNTL_CLR_DG_PAD_AUTOHOLD_W {
        RTC_CNTL_CLR_DG_PAD_AUTOHOLD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_dig_iso_force_on(&mut self) -> RTC_CNTL_DIG_ISO_FORCE_ON_W {
        RTC_CNTL_DIG_ISO_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_dig_iso_force_off(&mut self) -> RTC_CNTL_DIG_ISO_FORCE_OFF_W {
        RTC_CNTL_DIG_ISO_FORCE_OFF_W { w: self }
    }
}
