#[doc = "Reader of register APB_SARADC_CTRL"]
pub type R = crate::R<u32, super::APB_SARADC_CTRL>;
#[doc = "Writer for register APB_SARADC_CTRL"]
pub type W = crate::W<u32, super::APB_SARADC_CTRL>;
#[doc = "Register APB_SARADC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_WAIT_ARB_CYCLE`"]
pub type APB_SARADC_WAIT_ARB_CYCLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_WAIT_ARB_CYCLE`"]
pub struct APB_SARADC_WAIT_ARB_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_WAIT_ARB_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_XPD_SAR_FORCE`"]
pub type APB_SARADC_XPD_SAR_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_XPD_SAR_FORCE`"]
pub struct APB_SARADC_XPD_SAR_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_XPD_SAR_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_SAR_PATT_P_CLEAR`"]
pub type APB_SARADC_SAR_PATT_P_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_SAR_PATT_P_CLEAR`"]
pub struct APB_SARADC_SAR_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_SAR_PATT_P_CLEAR_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_SAR_PATT_LEN`"]
pub type APB_SARADC_SAR_PATT_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_SAR_PATT_LEN`"]
pub struct APB_SARADC_SAR_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_SAR_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_SAR_CLK_DIV`"]
pub type APB_SARADC_SAR_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_SAR_CLK_DIV`"]
pub struct APB_SARADC_SAR_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_SAR_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 7)) | (((value as u32) & 0xff) << 7);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_SAR_CLK_GATED`"]
pub type APB_SARADC_SAR_CLK_GATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_SAR_CLK_GATED`"]
pub struct APB_SARADC_SAR_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_SAR_CLK_GATED_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_START`"]
pub type APB_SARADC_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_START`"]
pub struct APB_SARADC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_START_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_START_FORCE`"]
pub type APB_SARADC_START_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_START_FORCE`"]
pub struct APB_SARADC_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_START_FORCE_W<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn apb_saradc_wait_arb_cycle(&self) -> APB_SARADC_WAIT_ARB_CYCLE_R {
        APB_SARADC_WAIT_ARB_CYCLE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn apb_saradc_xpd_sar_force(&self) -> APB_SARADC_XPD_SAR_FORCE_R {
        APB_SARADC_XPD_SAR_FORCE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn apb_saradc_sar_patt_p_clear(&self) -> APB_SARADC_SAR_PATT_P_CLEAR_R {
        APB_SARADC_SAR_PATT_P_CLEAR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn apb_saradc_sar_patt_len(&self) -> APB_SARADC_SAR_PATT_LEN_R {
        APB_SARADC_SAR_PATT_LEN_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 7:14"]
    #[inline(always)]
    pub fn apb_saradc_sar_clk_div(&self) -> APB_SARADC_SAR_CLK_DIV_R {
        APB_SARADC_SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn apb_saradc_sar_clk_gated(&self) -> APB_SARADC_SAR_CLK_GATED_R {
        APB_SARADC_SAR_CLK_GATED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_saradc_start(&self) -> APB_SARADC_START_R {
        APB_SARADC_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_saradc_start_force(&self) -> APB_SARADC_START_FORCE_R {
        APB_SARADC_START_FORCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn apb_saradc_wait_arb_cycle(&mut self) -> APB_SARADC_WAIT_ARB_CYCLE_W {
        APB_SARADC_WAIT_ARB_CYCLE_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn apb_saradc_xpd_sar_force(&mut self) -> APB_SARADC_XPD_SAR_FORCE_W {
        APB_SARADC_XPD_SAR_FORCE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn apb_saradc_sar_patt_p_clear(&mut self) -> APB_SARADC_SAR_PATT_P_CLEAR_W {
        APB_SARADC_SAR_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn apb_saradc_sar_patt_len(&mut self) -> APB_SARADC_SAR_PATT_LEN_W {
        APB_SARADC_SAR_PATT_LEN_W { w: self }
    }
    #[doc = "Bits 7:14"]
    #[inline(always)]
    pub fn apb_saradc_sar_clk_div(&mut self) -> APB_SARADC_SAR_CLK_DIV_W {
        APB_SARADC_SAR_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn apb_saradc_sar_clk_gated(&mut self) -> APB_SARADC_SAR_CLK_GATED_W {
        APB_SARADC_SAR_CLK_GATED_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_saradc_start(&mut self) -> APB_SARADC_START_W {
        APB_SARADC_START_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_saradc_start_force(&mut self) -> APB_SARADC_START_FORCE_W {
        APB_SARADC_START_FORCE_W { w: self }
    }
}
