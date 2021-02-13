#[doc = "Reader of register RTC_CNTL_DIG_PWC"]
pub type R = crate::R<u32, super::RTC_CNTL_DIG_PWC>;
#[doc = "Writer for register RTC_CNTL_DIG_PWC"]
pub type W = crate::W<u32, super::RTC_CNTL_DIG_PWC>;
#[doc = "Register RTC_CNTL_DIG_PWC `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_DIG_PWC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_PD_EN`"]
pub type RTC_CNTL_DG_WRAP_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_PD_EN`"]
pub struct RTC_CNTL_DG_WRAP_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_WIFI_PD_EN`"]
pub type RTC_CNTL_WIFI_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WIFI_PD_EN`"]
pub struct RTC_CNTL_WIFI_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WIFI_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_CPU_TOP_PD_EN`"]
pub type RTC_CNTL_CPU_TOP_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CPU_TOP_PD_EN`"]
pub struct RTC_CNTL_CPU_TOP_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPU_TOP_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PERI_PD_EN`"]
pub type RTC_CNTL_DG_PERI_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PERI_PD_EN`"]
pub struct RTC_CNTL_DG_PERI_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PERI_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BT_PD_EN`"]
pub type RTC_CNTL_BT_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BT_PD_EN`"]
pub struct RTC_CNTL_BT_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BT_PD_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_CPU_TOP_FORCE_PU`"]
pub type RTC_CNTL_CPU_TOP_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CPU_TOP_FORCE_PU`"]
pub struct RTC_CNTL_CPU_TOP_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPU_TOP_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_CPU_TOP_FORCE_PD`"]
pub type RTC_CNTL_CPU_TOP_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CPU_TOP_FORCE_PD`"]
pub struct RTC_CNTL_CPU_TOP_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPU_TOP_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_PU`"]
pub type RTC_CNTL_DG_WRAP_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_PU`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_PD`"]
pub type RTC_CNTL_DG_WRAP_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_PD`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WIFI_FORCE_PU`"]
pub type RTC_CNTL_WIFI_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WIFI_FORCE_PU`"]
pub struct RTC_CNTL_WIFI_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WIFI_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_WIFI_FORCE_PD`"]
pub type RTC_CNTL_WIFI_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_WIFI_FORCE_PD`"]
pub struct RTC_CNTL_WIFI_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WIFI_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FORCE_LPU`"]
pub type RTC_CNTL_FASTMEM_FORCE_LPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FORCE_LPU`"]
pub struct RTC_CNTL_FASTMEM_FORCE_LPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FORCE_LPU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_FASTMEM_FORCE_LPD`"]
pub type RTC_CNTL_FASTMEM_FORCE_LPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FASTMEM_FORCE_LPD`"]
pub struct RTC_CNTL_FASTMEM_FORCE_LPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FASTMEM_FORCE_LPD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PERI_FORCE_PU`"]
pub type RTC_CNTL_DG_PERI_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PERI_FORCE_PU`"]
pub struct RTC_CNTL_DG_PERI_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PERI_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_PERI_FORCE_PD`"]
pub type RTC_CNTL_DG_PERI_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PERI_FORCE_PD`"]
pub struct RTC_CNTL_DG_PERI_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PERI_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BT_FORCE_PU`"]
pub type RTC_CNTL_BT_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BT_FORCE_PU`"]
pub struct RTC_CNTL_BT_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BT_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BT_FORCE_PD`"]
pub type RTC_CNTL_BT_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BT_FORCE_PD`"]
pub struct RTC_CNTL_BT_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BT_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_LSLP_MEM_FORCE_PU`"]
pub type RTC_CNTL_LSLP_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_LSLP_MEM_FORCE_PU`"]
pub struct RTC_CNTL_LSLP_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_LSLP_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_LSLP_MEM_FORCE_PD`"]
pub type RTC_CNTL_LSLP_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_LSLP_MEM_FORCE_PD`"]
pub struct RTC_CNTL_LSLP_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_LSLP_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_VDD_SPI_PWR_FORCE`"]
pub type RTC_CNTL_VDD_SPI_PWR_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_VDD_SPI_PWR_FORCE`"]
pub struct RTC_CNTL_VDD_SPI_PWR_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_VDD_SPI_PWR_FORCE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_VDD_SPI_PWR_DRV`"]
pub type RTC_CNTL_VDD_SPI_PWR_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_VDD_SPI_PWR_DRV`"]
pub struct RTC_CNTL_VDD_SPI_PWR_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_VDD_SPI_PWR_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_pd_en(&self) -> RTC_CNTL_DG_WRAP_PD_EN_R {
        RTC_CNTL_DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_pd_en(&self) -> RTC_CNTL_WIFI_PD_EN_R {
        RTC_CNTL_WIFI_PD_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_pd_en(&self) -> RTC_CNTL_CPU_TOP_PD_EN_R {
        RTC_CNTL_CPU_TOP_PD_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_pd_en(&self) -> RTC_CNTL_DG_PERI_PD_EN_R {
        RTC_CNTL_DG_PERI_PD_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_bt_pd_en(&self) -> RTC_CNTL_BT_PD_EN_R {
        RTC_CNTL_BT_PD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_force_pu(&self) -> RTC_CNTL_CPU_TOP_FORCE_PU_R {
        RTC_CNTL_CPU_TOP_FORCE_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_force_pd(&self) -> RTC_CNTL_CPU_TOP_FORCE_PD_R {
        RTC_CNTL_CPU_TOP_FORCE_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_pu(&self) -> RTC_CNTL_DG_WRAP_FORCE_PU_R {
        RTC_CNTL_DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_pd(&self) -> RTC_CNTL_DG_WRAP_FORCE_PD_R {
        RTC_CNTL_DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_pu(&self) -> RTC_CNTL_WIFI_FORCE_PU_R {
        RTC_CNTL_WIFI_FORCE_PU_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_pd(&self) -> RTC_CNTL_WIFI_FORCE_PD_R {
        RTC_CNTL_WIFI_FORCE_PD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_lpu(&self) -> RTC_CNTL_FASTMEM_FORCE_LPU_R {
        RTC_CNTL_FASTMEM_FORCE_LPU_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_lpd(&self) -> RTC_CNTL_FASTMEM_FORCE_LPD_R {
        RTC_CNTL_FASTMEM_FORCE_LPD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_force_pu(&self) -> RTC_CNTL_DG_PERI_FORCE_PU_R {
        RTC_CNTL_DG_PERI_FORCE_PU_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_force_pd(&self) -> RTC_CNTL_DG_PERI_FORCE_PD_R {
        RTC_CNTL_DG_PERI_FORCE_PD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_bt_force_pu(&self) -> RTC_CNTL_BT_FORCE_PU_R {
        RTC_CNTL_BT_FORCE_PU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_bt_force_pd(&self) -> RTC_CNTL_BT_FORCE_PD_R {
        RTC_CNTL_BT_FORCE_PD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_lslp_mem_force_pu(&self) -> RTC_CNTL_LSLP_MEM_FORCE_PU_R {
        RTC_CNTL_LSLP_MEM_FORCE_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_lslp_mem_force_pd(&self) -> RTC_CNTL_LSLP_MEM_FORCE_PD_R {
        RTC_CNTL_LSLP_MEM_FORCE_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_vdd_spi_pwr_force(&self) -> RTC_CNTL_VDD_SPI_PWR_FORCE_R {
        RTC_CNTL_VDD_SPI_PWR_FORCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc_cntl_vdd_spi_pwr_drv(&self) -> RTC_CNTL_VDD_SPI_PWR_DRV_R {
        RTC_CNTL_VDD_SPI_PWR_DRV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_pd_en(&mut self) -> RTC_CNTL_DG_WRAP_PD_EN_W {
        RTC_CNTL_DG_WRAP_PD_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_pd_en(&mut self) -> RTC_CNTL_WIFI_PD_EN_W {
        RTC_CNTL_WIFI_PD_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_pd_en(&mut self) -> RTC_CNTL_CPU_TOP_PD_EN_W {
        RTC_CNTL_CPU_TOP_PD_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_pd_en(&mut self) -> RTC_CNTL_DG_PERI_PD_EN_W {
        RTC_CNTL_DG_PERI_PD_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_bt_pd_en(&mut self) -> RTC_CNTL_BT_PD_EN_W {
        RTC_CNTL_BT_PD_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_force_pu(&mut self) -> RTC_CNTL_CPU_TOP_FORCE_PU_W {
        RTC_CNTL_CPU_TOP_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_top_force_pd(&mut self) -> RTC_CNTL_CPU_TOP_FORCE_PD_W {
        RTC_CNTL_CPU_TOP_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_pu(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_PU_W {
        RTC_CNTL_DG_WRAP_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_pd(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_PD_W {
        RTC_CNTL_DG_WRAP_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_pu(&mut self) -> RTC_CNTL_WIFI_FORCE_PU_W {
        RTC_CNTL_WIFI_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtc_cntl_wifi_force_pd(&mut self) -> RTC_CNTL_WIFI_FORCE_PD_W {
        RTC_CNTL_WIFI_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_lpu(&mut self) -> RTC_CNTL_FASTMEM_FORCE_LPU_W {
        RTC_CNTL_FASTMEM_FORCE_LPU_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_fastmem_force_lpd(&mut self) -> RTC_CNTL_FASTMEM_FORCE_LPD_W {
        RTC_CNTL_FASTMEM_FORCE_LPD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_force_pu(&mut self) -> RTC_CNTL_DG_PERI_FORCE_PU_W {
        RTC_CNTL_DG_PERI_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_force_pd(&mut self) -> RTC_CNTL_DG_PERI_FORCE_PD_W {
        RTC_CNTL_DG_PERI_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_bt_force_pu(&mut self) -> RTC_CNTL_BT_FORCE_PU_W {
        RTC_CNTL_BT_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_bt_force_pd(&mut self) -> RTC_CNTL_BT_FORCE_PD_W {
        RTC_CNTL_BT_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_lslp_mem_force_pu(&mut self) -> RTC_CNTL_LSLP_MEM_FORCE_PU_W {
        RTC_CNTL_LSLP_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_lslp_mem_force_pd(&mut self) -> RTC_CNTL_LSLP_MEM_FORCE_PD_W {
        RTC_CNTL_LSLP_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_vdd_spi_pwr_force(&mut self) -> RTC_CNTL_VDD_SPI_PWR_FORCE_W {
        RTC_CNTL_VDD_SPI_PWR_FORCE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc_cntl_vdd_spi_pwr_drv(&mut self) -> RTC_CNTL_VDD_SPI_PWR_DRV_W {
        RTC_CNTL_VDD_SPI_PWR_DRV_W { w: self }
    }
}
