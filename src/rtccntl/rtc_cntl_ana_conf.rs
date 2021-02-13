#[doc = "Reader of register RTC_CNTL_ANA_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_ANA_CONF>;
#[doc = "Writer for register RTC_CNTL_ANA_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_ANA_CONF>;
#[doc = "Register RTC_CNTL_ANA_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_ANA_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_PLL_I2C_PU`"]
pub type RTC_CNTL_PLL_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PLL_I2C_PU`"]
pub struct RTC_CNTL_PLL_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PLL_I2C_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_CKGEN_I2C_PU`"]
pub type RTC_CNTL_CKGEN_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CKGEN_I2C_PU`"]
pub struct RTC_CNTL_CKGEN_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CKGEN_I2C_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_RFRX_PBUS_PU`"]
pub type RTC_CNTL_RFRX_PBUS_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_RFRX_PBUS_PU`"]
pub struct RTC_CNTL_RFRX_PBUS_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_RFRX_PBUS_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_TXRF_I2C_PU`"]
pub type RTC_CNTL_TXRF_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TXRF_I2C_PU`"]
pub struct RTC_CNTL_TXRF_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TXRF_I2C_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_PVTMON_PU`"]
pub type RTC_CNTL_PVTMON_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PVTMON_PU`"]
pub struct RTC_CNTL_PVTMON_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PVTMON_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_CAL_SLP_START`"]
pub type RTC_CNTL_BBPLL_CAL_SLP_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_CAL_SLP_START`"]
pub struct RTC_CNTL_BBPLL_CAL_SLP_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_CAL_SLP_START_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_PLLA_FORCE_PU`"]
pub type RTC_CNTL_PLLA_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PLLA_FORCE_PU`"]
pub struct RTC_CNTL_PLLA_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PLLA_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_PLLA_FORCE_PD`"]
pub type RTC_CNTL_PLLA_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PLLA_FORCE_PD`"]
pub struct RTC_CNTL_PLLA_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PLLA_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SAR_I2C_PU`"]
pub type RTC_CNTL_SAR_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SAR_I2C_PU`"]
pub struct RTC_CNTL_SAR_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SAR_I2C_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GLITCH_RST_EN`"]
pub type RTC_CNTL_GLITCH_RST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GLITCH_RST_EN`"]
pub struct RTC_CNTL_GLITCH_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GLITCH_RST_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_I2C_RESET_POR_FORCE_PU`"]
pub type RTC_CNTL_I2C_RESET_POR_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_I2C_RESET_POR_FORCE_PU`"]
pub struct RTC_CNTL_I2C_RESET_POR_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_I2C_RESET_POR_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_I2C_RESET_POR_FORCE_PD`"]
pub type RTC_CNTL_I2C_RESET_POR_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_I2C_RESET_POR_FORCE_PD`"]
pub struct RTC_CNTL_I2C_RESET_POR_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_I2C_RESET_POR_FORCE_PD_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_pll_i2c_pu(&self) -> RTC_CNTL_PLL_I2C_PU_R {
        RTC_CNTL_PLL_I2C_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_ckgen_i2c_pu(&self) -> RTC_CNTL_CKGEN_I2C_PU_R {
        RTC_CNTL_CKGEN_I2C_PU_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_rfrx_pbus_pu(&self) -> RTC_CNTL_RFRX_PBUS_PU_R {
        RTC_CNTL_RFRX_PBUS_PU_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_txrf_i2c_pu(&self) -> RTC_CNTL_TXRF_I2C_PU_R {
        RTC_CNTL_TXRF_I2C_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_pvtmon_pu(&self) -> RTC_CNTL_PVTMON_PU_R {
        RTC_CNTL_PVTMON_PU_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_cal_slp_start(&self) -> RTC_CNTL_BBPLL_CAL_SLP_START_R {
        RTC_CNTL_BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_plla_force_pu(&self) -> RTC_CNTL_PLLA_FORCE_PU_R {
        RTC_CNTL_PLLA_FORCE_PU_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_plla_force_pd(&self) -> RTC_CNTL_PLLA_FORCE_PD_R {
        RTC_CNTL_PLLA_FORCE_PD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_sar_i2c_pu(&self) -> RTC_CNTL_SAR_I2C_PU_R {
        RTC_CNTL_SAR_I2C_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_glitch_rst_en(&self) -> RTC_CNTL_GLITCH_RST_EN_R {
        RTC_CNTL_GLITCH_RST_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_i2c_reset_por_force_pu(&self) -> RTC_CNTL_I2C_RESET_POR_FORCE_PU_R {
        RTC_CNTL_I2C_RESET_POR_FORCE_PU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_i2c_reset_por_force_pd(&self) -> RTC_CNTL_I2C_RESET_POR_FORCE_PD_R {
        RTC_CNTL_I2C_RESET_POR_FORCE_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_pll_i2c_pu(&mut self) -> RTC_CNTL_PLL_I2C_PU_W {
        RTC_CNTL_PLL_I2C_PU_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_ckgen_i2c_pu(&mut self) -> RTC_CNTL_CKGEN_I2C_PU_W {
        RTC_CNTL_CKGEN_I2C_PU_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_rfrx_pbus_pu(&mut self) -> RTC_CNTL_RFRX_PBUS_PU_W {
        RTC_CNTL_RFRX_PBUS_PU_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_txrf_i2c_pu(&mut self) -> RTC_CNTL_TXRF_I2C_PU_W {
        RTC_CNTL_TXRF_I2C_PU_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_pvtmon_pu(&mut self) -> RTC_CNTL_PVTMON_PU_W {
        RTC_CNTL_PVTMON_PU_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_cal_slp_start(&mut self) -> RTC_CNTL_BBPLL_CAL_SLP_START_W {
        RTC_CNTL_BBPLL_CAL_SLP_START_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_plla_force_pu(&mut self) -> RTC_CNTL_PLLA_FORCE_PU_W {
        RTC_CNTL_PLLA_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_plla_force_pd(&mut self) -> RTC_CNTL_PLLA_FORCE_PD_W {
        RTC_CNTL_PLLA_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_sar_i2c_pu(&mut self) -> RTC_CNTL_SAR_I2C_PU_W {
        RTC_CNTL_SAR_I2C_PU_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_glitch_rst_en(&mut self) -> RTC_CNTL_GLITCH_RST_EN_W {
        RTC_CNTL_GLITCH_RST_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_i2c_reset_por_force_pu(&mut self) -> RTC_CNTL_I2C_RESET_POR_FORCE_PU_W {
        RTC_CNTL_I2C_RESET_POR_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_i2c_reset_por_force_pd(&mut self) -> RTC_CNTL_I2C_RESET_POR_FORCE_PD_W {
        RTC_CNTL_I2C_RESET_POR_FORCE_PD_W { w: self }
    }
}
