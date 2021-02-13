#[doc = "Reader of register RTC_CNTL_OPTIONS0"]
pub type R = crate::R<u32, super::RTC_CNTL_OPTIONS0>;
#[doc = "Writer for register RTC_CNTL_OPTIONS0"]
pub type W = crate::W<u32, super::RTC_CNTL_OPTIONS0>;
#[doc = "Register RTC_CNTL_OPTIONS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_OPTIONS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RTC_CNTL_SW_SYS_RST`"]
pub struct RTC_CNTL_SW_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_SYS_RST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_NORST`"]
pub type RTC_CNTL_DG_WRAP_FORCE_NORST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_NORST`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_NORST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_NORST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DG_WRAP_FORCE_RST`"]
pub type RTC_CNTL_DG_WRAP_FORCE_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DG_WRAP_FORCE_RST`"]
pub struct RTC_CNTL_DG_WRAP_FORCE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_WRAP_FORCE_RST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ANALOG_FORCE_NOISO`"]
pub type RTC_CNTL_ANALOG_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ANALOG_FORCE_NOISO`"]
pub struct RTC_CNTL_ANALOG_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ANALOG_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_PLL_FORCE_NOISO`"]
pub type RTC_CNTL_PLL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PLL_FORCE_NOISO`"]
pub struct RTC_CNTL_PLL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PLL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_FORCE_NOISO`"]
pub type RTC_CNTL_XTL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_FORCE_NOISO`"]
pub struct RTC_CNTL_XTL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ANALOG_FORCE_ISO`"]
pub type RTC_CNTL_ANALOG_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ANALOG_FORCE_ISO`"]
pub struct RTC_CNTL_ANALOG_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ANALOG_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_PLL_FORCE_ISO`"]
pub type RTC_CNTL_PLL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PLL_FORCE_ISO`"]
pub struct RTC_CNTL_PLL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PLL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_FORCE_ISO`"]
pub type RTC_CNTL_XTL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_FORCE_ISO`"]
pub struct RTC_CNTL_XTL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_EXT_CTR_SEL`"]
pub type RTC_CNTL_XTL_EXT_CTR_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_EXT_CTR_SEL`"]
pub struct RTC_CNTL_XTL_EXT_CTR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_EXT_CTR_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XTL_EN_WAIT`"]
pub type RTC_CNTL_XTL_EN_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_EN_WAIT`"]
pub struct RTC_CNTL_XTL_EN_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_EN_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XTL_FORCE_PU`"]
pub type RTC_CNTL_XTL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_FORCE_PU`"]
pub struct RTC_CNTL_XTL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_FORCE_PD`"]
pub type RTC_CNTL_XTL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_FORCE_PD`"]
pub struct RTC_CNTL_XTL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_FORCE_PU`"]
pub type RTC_CNTL_BBPLL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_FORCE_PU`"]
pub struct RTC_CNTL_BBPLL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_FORCE_PD`"]
pub type RTC_CNTL_BBPLL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_FORCE_PD`"]
pub struct RTC_CNTL_BBPLL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_I2C_FORCE_PU`"]
pub type RTC_CNTL_BBPLL_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_I2C_FORCE_PU`"]
pub struct RTC_CNTL_BBPLL_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BBPLL_I2C_FORCE_PD`"]
pub type RTC_CNTL_BBPLL_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BBPLL_I2C_FORCE_PD`"]
pub struct RTC_CNTL_BBPLL_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BBPLL_I2C_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BB_I2C_FORCE_PU`"]
pub type RTC_CNTL_BB_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BB_I2C_FORCE_PU`"]
pub struct RTC_CNTL_BB_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BB_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_BB_I2C_FORCE_PD`"]
pub type RTC_CNTL_BB_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_BB_I2C_FORCE_PD`"]
pub struct RTC_CNTL_BB_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_BB_I2C_FORCE_PD_W<'a> {
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
#[doc = "Write proxy for field `RTC_CNTL_SW_PROCPU_RST`"]
pub struct RTC_CNTL_SW_PROCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_PROCPU_RST_W<'a> {
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
#[doc = "Write proxy for field `RTC_CNTL_SW_APPCPU_RST`"]
pub struct RTC_CNTL_SW_APPCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_APPCPU_RST_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SW_STALL_PROCPU_C0`"]
pub type RTC_CNTL_SW_STALL_PROCPU_C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SW_STALL_PROCPU_C0`"]
pub struct RTC_CNTL_SW_STALL_PROCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_STALL_PROCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_SW_STALL_APPCPU_C0`"]
pub type RTC_CNTL_SW_STALL_APPCPU_C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SW_STALL_APPCPU_C0`"]
pub struct RTC_CNTL_SW_STALL_APPCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SW_STALL_APPCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_norst(&self) -> RTC_CNTL_DG_WRAP_FORCE_NORST_R {
        RTC_CNTL_DG_WRAP_FORCE_NORST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_rst(&self) -> RTC_CNTL_DG_WRAP_FORCE_RST_R {
        RTC_CNTL_DG_WRAP_FORCE_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_analog_force_noiso(&self) -> RTC_CNTL_ANALOG_FORCE_NOISO_R {
        RTC_CNTL_ANALOG_FORCE_NOISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_pll_force_noiso(&self) -> RTC_CNTL_PLL_FORCE_NOISO_R {
        RTC_CNTL_PLL_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_noiso(&self) -> RTC_CNTL_XTL_FORCE_NOISO_R {
        RTC_CNTL_XTL_FORCE_NOISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_analog_force_iso(&self) -> RTC_CNTL_ANALOG_FORCE_ISO_R {
        RTC_CNTL_ANALOG_FORCE_ISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_pll_force_iso(&self) -> RTC_CNTL_PLL_FORCE_ISO_R {
        RTC_CNTL_PLL_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_iso(&self) -> RTC_CNTL_XTL_FORCE_ISO_R {
        RTC_CNTL_XTL_FORCE_ISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_sel(&self) -> RTC_CNTL_XTL_EXT_CTR_SEL_R {
        RTC_CNTL_XTL_EXT_CTR_SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_en_wait(&self) -> RTC_CNTL_XTL_EN_WAIT_R {
        RTC_CNTL_XTL_EN_WAIT_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_pu(&self) -> RTC_CNTL_XTL_FORCE_PU_R {
        RTC_CNTL_XTL_FORCE_PU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_pd(&self) -> RTC_CNTL_XTL_FORCE_PD_R {
        RTC_CNTL_XTL_FORCE_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_force_pu(&self) -> RTC_CNTL_BBPLL_FORCE_PU_R {
        RTC_CNTL_BBPLL_FORCE_PU_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_force_pd(&self) -> RTC_CNTL_BBPLL_FORCE_PD_R {
        RTC_CNTL_BBPLL_FORCE_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_i2c_force_pu(&self) -> RTC_CNTL_BBPLL_I2C_FORCE_PU_R {
        RTC_CNTL_BBPLL_I2C_FORCE_PU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_i2c_force_pd(&self) -> RTC_CNTL_BBPLL_I2C_FORCE_PD_R {
        RTC_CNTL_BBPLL_I2C_FORCE_PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_bb_i2c_force_pu(&self) -> RTC_CNTL_BB_I2C_FORCE_PU_R {
        RTC_CNTL_BB_I2C_FORCE_PU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_bb_i2c_force_pd(&self) -> RTC_CNTL_BB_I2C_FORCE_PD_R {
        RTC_CNTL_BB_I2C_FORCE_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rtc_cntl_sw_stall_procpu_c0(&self) -> RTC_CNTL_SW_STALL_PROCPU_C0_R {
        RTC_CNTL_SW_STALL_PROCPU_C0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc_cntl_sw_stall_appcpu_c0(&self) -> RTC_CNTL_SW_STALL_APPCPU_C0_R {
        RTC_CNTL_SW_STALL_APPCPU_C0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_sw_sys_rst(&mut self) -> RTC_CNTL_SW_SYS_RST_W {
        RTC_CNTL_SW_SYS_RST_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_norst(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_NORST_W {
        RTC_CNTL_DG_WRAP_FORCE_NORST_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_dg_wrap_force_rst(&mut self) -> RTC_CNTL_DG_WRAP_FORCE_RST_W {
        RTC_CNTL_DG_WRAP_FORCE_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_analog_force_noiso(&mut self) -> RTC_CNTL_ANALOG_FORCE_NOISO_W {
        RTC_CNTL_ANALOG_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_pll_force_noiso(&mut self) -> RTC_CNTL_PLL_FORCE_NOISO_W {
        RTC_CNTL_PLL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_noiso(&mut self) -> RTC_CNTL_XTL_FORCE_NOISO_W {
        RTC_CNTL_XTL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_analog_force_iso(&mut self) -> RTC_CNTL_ANALOG_FORCE_ISO_W {
        RTC_CNTL_ANALOG_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_pll_force_iso(&mut self) -> RTC_CNTL_PLL_FORCE_ISO_W {
        RTC_CNTL_PLL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_iso(&mut self) -> RTC_CNTL_XTL_FORCE_ISO_W {
        RTC_CNTL_XTL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_sel(&mut self) -> RTC_CNTL_XTL_EXT_CTR_SEL_W {
        RTC_CNTL_XTL_EXT_CTR_SEL_W { w: self }
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_en_wait(&mut self) -> RTC_CNTL_XTL_EN_WAIT_W {
        RTC_CNTL_XTL_EN_WAIT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_pu(&mut self) -> RTC_CNTL_XTL_FORCE_PU_W {
        RTC_CNTL_XTL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_force_pd(&mut self) -> RTC_CNTL_XTL_FORCE_PD_W {
        RTC_CNTL_XTL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_force_pu(&mut self) -> RTC_CNTL_BBPLL_FORCE_PU_W {
        RTC_CNTL_BBPLL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_force_pd(&mut self) -> RTC_CNTL_BBPLL_FORCE_PD_W {
        RTC_CNTL_BBPLL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_i2c_force_pu(&mut self) -> RTC_CNTL_BBPLL_I2C_FORCE_PU_W {
        RTC_CNTL_BBPLL_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_cntl_bbpll_i2c_force_pd(&mut self) -> RTC_CNTL_BBPLL_I2C_FORCE_PD_W {
        RTC_CNTL_BBPLL_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_bb_i2c_force_pu(&mut self) -> RTC_CNTL_BB_I2C_FORCE_PU_W {
        RTC_CNTL_BB_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_bb_i2c_force_pd(&mut self) -> RTC_CNTL_BB_I2C_FORCE_PD_W {
        RTC_CNTL_BB_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_cntl_sw_procpu_rst(&mut self) -> RTC_CNTL_SW_PROCPU_RST_W {
        RTC_CNTL_SW_PROCPU_RST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_sw_appcpu_rst(&mut self) -> RTC_CNTL_SW_APPCPU_RST_W {
        RTC_CNTL_SW_APPCPU_RST_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rtc_cntl_sw_stall_procpu_c0(&mut self) -> RTC_CNTL_SW_STALL_PROCPU_C0_W {
        RTC_CNTL_SW_STALL_PROCPU_C0_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc_cntl_sw_stall_appcpu_c0(&mut self) -> RTC_CNTL_SW_STALL_APPCPU_C0_W {
        RTC_CNTL_SW_STALL_APPCPU_C0_W { w: self }
    }
}
