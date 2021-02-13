#[doc = "Reader of register APB_SARADC_APB_TSENS_CTRL2"]
pub type R = crate::R<u32, super::APB_SARADC_APB_TSENS_CTRL2>;
#[doc = "Writer for register APB_SARADC_APB_TSENS_CTRL2"]
pub type W = crate::W<u32, super::APB_SARADC_APB_TSENS_CTRL2>;
#[doc = "Register APB_SARADC_APB_TSENS_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_APB_TSENS_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_TSENS_CLK_SEL`"]
pub type APB_SARADC_TSENS_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_TSENS_CLK_SEL`"]
pub struct APB_SARADC_TSENS_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TSENS_CLK_SEL_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_TSENS_CLK_INV`"]
pub type APB_SARADC_TSENS_CLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_TSENS_CLK_INV`"]
pub struct APB_SARADC_TSENS_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TSENS_CLK_INV_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_TSENS_XPD_FORCE`"]
pub type APB_SARADC_TSENS_XPD_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_TSENS_XPD_FORCE`"]
pub struct APB_SARADC_TSENS_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TSENS_XPD_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_TSENS_XPD_WAIT`"]
pub type APB_SARADC_TSENS_XPD_WAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APB_SARADC_TSENS_XPD_WAIT`"]
pub struct APB_SARADC_TSENS_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TSENS_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn apb_saradc_tsens_clk_sel(&self) -> APB_SARADC_TSENS_CLK_SEL_R {
        APB_SARADC_TSENS_CLK_SEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn apb_saradc_tsens_clk_inv(&self) -> APB_SARADC_TSENS_CLK_INV_R {
        APB_SARADC_TSENS_CLK_INV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn apb_saradc_tsens_xpd_force(&self) -> APB_SARADC_TSENS_XPD_FORCE_R {
        APB_SARADC_TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn apb_saradc_tsens_xpd_wait(&self) -> APB_SARADC_TSENS_XPD_WAIT_R {
        APB_SARADC_TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn apb_saradc_tsens_clk_sel(&mut self) -> APB_SARADC_TSENS_CLK_SEL_W {
        APB_SARADC_TSENS_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn apb_saradc_tsens_clk_inv(&mut self) -> APB_SARADC_TSENS_CLK_INV_W {
        APB_SARADC_TSENS_CLK_INV_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn apb_saradc_tsens_xpd_force(&mut self) -> APB_SARADC_TSENS_XPD_FORCE_W {
        APB_SARADC_TSENS_XPD_FORCE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn apb_saradc_tsens_xpd_wait(&mut self) -> APB_SARADC_TSENS_XPD_WAIT_W {
        APB_SARADC_TSENS_XPD_WAIT_W { w: self }
    }
}
