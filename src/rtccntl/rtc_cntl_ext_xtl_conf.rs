#[doc = "Reader of register RTC_CNTL_EXT_XTL_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_EXT_XTL_CONF>;
#[doc = "Writer for register RTC_CNTL_EXT_XTL_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_EXT_XTL_CONF>;
#[doc = "Register RTC_CNTL_EXT_XTL_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_EXT_XTL_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_XTL_EXT_CTR_EN`"]
pub type RTC_CNTL_XTL_EXT_CTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_EXT_CTR_EN`"]
pub struct RTC_CNTL_XTL_EXT_CTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_EXT_CTR_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_EXT_CTR_LV`"]
pub type RTC_CNTL_XTL_EXT_CTR_LV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_EXT_CTR_LV`"]
pub struct RTC_CNTL_XTL_EXT_CTR_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_EXT_CTR_LV_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTAL32K_GPIO_SEL`"]
pub type RTC_CNTL_XTAL32K_GPIO_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_GPIO_SEL`"]
pub struct RTC_CNTL_XTAL32K_GPIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_GPIO_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WDT_STATE`"]
pub type RTC_CNTL_WDT_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `RTC_CNTL_DAC_XTAL_32K`"]
pub type RTC_CNTL_DAC_XTAL_32K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DAC_XTAL_32K`"]
pub struct RTC_CNTL_DAC_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DAC_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XPD_XTAL_32K`"]
pub type RTC_CNTL_XPD_XTAL_32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XPD_XTAL_32K`"]
pub struct RTC_CNTL_XPD_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XPD_XTAL_32K_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DRES_XTAL_32K`"]
pub type RTC_CNTL_DRES_XTAL_32K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DRES_XTAL_32K`"]
pub struct RTC_CNTL_DRES_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DRES_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DGM_XTAL_32K`"]
pub type RTC_CNTL_DGM_XTAL_32K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DGM_XTAL_32K`"]
pub struct RTC_CNTL_DGM_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DGM_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DBUF_XTAL_32K`"]
pub type RTC_CNTL_DBUF_XTAL_32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DBUF_XTAL_32K`"]
pub struct RTC_CNTL_DBUF_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DBUF_XTAL_32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_ENCKINIT_XTAL_32K`"]
pub type RTC_CNTL_ENCKINIT_XTAL_32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ENCKINIT_XTAL_32K`"]
pub struct RTC_CNTL_ENCKINIT_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ENCKINIT_XTAL_32K_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTAL32K_XPD_FORCE`"]
pub type RTC_CNTL_XTAL32K_XPD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_XPD_FORCE`"]
pub struct RTC_CNTL_XTAL32K_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_XPD_FORCE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTAL32K_AUTO_RETURN`"]
pub type RTC_CNTL_XTAL32K_AUTO_RETURN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_AUTO_RETURN`"]
pub struct RTC_CNTL_XTAL32K_AUTO_RETURN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_AUTO_RETURN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XTAL32K_AUTO_RESTART`"]
pub type RTC_CNTL_XTAL32K_AUTO_RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_AUTO_RESTART`"]
pub struct RTC_CNTL_XTAL32K_AUTO_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_AUTO_RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XTAL32K_AUTO_BACKUP`"]
pub type RTC_CNTL_XTAL32K_AUTO_BACKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_AUTO_BACKUP`"]
pub struct RTC_CNTL_XTAL32K_AUTO_BACKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_AUTO_BACKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XTAL32K_EXT_CLK_FO`"]
pub type RTC_CNTL_XTAL32K_EXT_CLK_FO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_EXT_CLK_FO`"]
pub struct RTC_CNTL_XTAL32K_EXT_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_EXT_CLK_FO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XTAL32K_WDT_RESET`"]
pub type RTC_CNTL_XTAL32K_WDT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_WDT_RESET`"]
pub struct RTC_CNTL_XTAL32K_WDT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_WDT_RESET_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTAL32K_WDT_CLK_FO`"]
pub type RTC_CNTL_XTAL32K_WDT_CLK_FO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_WDT_CLK_FO`"]
pub struct RTC_CNTL_XTAL32K_WDT_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_WDT_CLK_FO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTAL32K_WDT_EN`"]
pub type RTC_CNTL_XTAL32K_WDT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL32K_WDT_EN`"]
pub struct RTC_CNTL_XTAL32K_WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL32K_WDT_EN_W<'a> {
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
    pub fn rtc_cntl_xtl_ext_ctr_en(&self) -> RTC_CNTL_XTL_EXT_CTR_EN_R {
        RTC_CNTL_XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_lv(&self) -> RTC_CNTL_XTL_EXT_CTR_LV_R {
        RTC_CNTL_XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_gpio_sel(&self) -> RTC_CNTL_XTAL32K_GPIO_SEL_R {
        RTC_CNTL_XTAL32K_GPIO_SEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_state(&self) -> RTC_CNTL_WDT_STATE_R {
        RTC_CNTL_WDT_STATE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn rtc_cntl_dac_xtal_32k(&self) -> RTC_CNTL_DAC_XTAL_32K_R {
        RTC_CNTL_DAC_XTAL_32K_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_xtal_32k(&self) -> RTC_CNTL_XPD_XTAL_32K_R {
        RTC_CNTL_XPD_XTAL_32K_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn rtc_cntl_dres_xtal_32k(&self) -> RTC_CNTL_DRES_XTAL_32K_R {
        RTC_CNTL_DRES_XTAL_32K_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn rtc_cntl_dgm_xtal_32k(&self) -> RTC_CNTL_DGM_XTAL_32K_R {
        RTC_CNTL_DGM_XTAL_32K_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtc_cntl_dbuf_xtal_32k(&self) -> RTC_CNTL_DBUF_XTAL_32K_R {
        RTC_CNTL_DBUF_XTAL_32K_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_enckinit_xtal_32k(&self) -> RTC_CNTL_ENCKINIT_XTAL_32K_R {
        RTC_CNTL_ENCKINIT_XTAL_32K_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_xpd_force(&self) -> RTC_CNTL_XTAL32K_XPD_FORCE_R {
        RTC_CNTL_XTAL32K_XPD_FORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_auto_return(&self) -> RTC_CNTL_XTAL32K_AUTO_RETURN_R {
        RTC_CNTL_XTAL32K_AUTO_RETURN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_auto_restart(&self) -> RTC_CNTL_XTAL32K_AUTO_RESTART_R {
        RTC_CNTL_XTAL32K_AUTO_RESTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_auto_backup(&self) -> RTC_CNTL_XTAL32K_AUTO_BACKUP_R {
        RTC_CNTL_XTAL32K_AUTO_BACKUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_ext_clk_fo(&self) -> RTC_CNTL_XTAL32K_EXT_CLK_FO_R {
        RTC_CNTL_XTAL32K_EXT_CLK_FO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_wdt_reset(&self) -> RTC_CNTL_XTAL32K_WDT_RESET_R {
        RTC_CNTL_XTAL32K_WDT_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_wdt_clk_fo(&self) -> RTC_CNTL_XTAL32K_WDT_CLK_FO_R {
        RTC_CNTL_XTAL32K_WDT_CLK_FO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_wdt_en(&self) -> RTC_CNTL_XTAL32K_WDT_EN_R {
        RTC_CNTL_XTAL32K_WDT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_en(&mut self) -> RTC_CNTL_XTL_EXT_CTR_EN_W {
        RTC_CNTL_XTL_EXT_CTR_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_lv(&mut self) -> RTC_CNTL_XTL_EXT_CTR_LV_W {
        RTC_CNTL_XTL_EXT_CTR_LV_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_gpio_sel(&mut self) -> RTC_CNTL_XTAL32K_GPIO_SEL_W {
        RTC_CNTL_XTAL32K_GPIO_SEL_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn rtc_cntl_dac_xtal_32k(&mut self) -> RTC_CNTL_DAC_XTAL_32K_W {
        RTC_CNTL_DAC_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_xpd_xtal_32k(&mut self) -> RTC_CNTL_XPD_XTAL_32K_W {
        RTC_CNTL_XPD_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn rtc_cntl_dres_xtal_32k(&mut self) -> RTC_CNTL_DRES_XTAL_32K_W {
        RTC_CNTL_DRES_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn rtc_cntl_dgm_xtal_32k(&mut self) -> RTC_CNTL_DGM_XTAL_32K_W {
        RTC_CNTL_DGM_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtc_cntl_dbuf_xtal_32k(&mut self) -> RTC_CNTL_DBUF_XTAL_32K_W {
        RTC_CNTL_DBUF_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_enckinit_xtal_32k(&mut self) -> RTC_CNTL_ENCKINIT_XTAL_32K_W {
        RTC_CNTL_ENCKINIT_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_xpd_force(&mut self) -> RTC_CNTL_XTAL32K_XPD_FORCE_W {
        RTC_CNTL_XTAL32K_XPD_FORCE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_auto_return(&mut self) -> RTC_CNTL_XTAL32K_AUTO_RETURN_W {
        RTC_CNTL_XTAL32K_AUTO_RETURN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_auto_restart(&mut self) -> RTC_CNTL_XTAL32K_AUTO_RESTART_W {
        RTC_CNTL_XTAL32K_AUTO_RESTART_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_auto_backup(&mut self) -> RTC_CNTL_XTAL32K_AUTO_BACKUP_W {
        RTC_CNTL_XTAL32K_AUTO_BACKUP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_ext_clk_fo(&mut self) -> RTC_CNTL_XTAL32K_EXT_CLK_FO_W {
        RTC_CNTL_XTAL32K_EXT_CLK_FO_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_wdt_reset(&mut self) -> RTC_CNTL_XTAL32K_WDT_RESET_W {
        RTC_CNTL_XTAL32K_WDT_RESET_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_wdt_clk_fo(&mut self) -> RTC_CNTL_XTAL32K_WDT_CLK_FO_W {
        RTC_CNTL_XTAL32K_WDT_CLK_FO_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_xtal32k_wdt_en(&mut self) -> RTC_CNTL_XTAL32K_WDT_EN_W {
        RTC_CNTL_XTAL32K_WDT_EN_W { w: self }
    }
}
