#[doc = "Reader of register APB_SARADC_APB_TSENS_CTRL"]
pub type R = crate::R<u32, super::APB_SARADC_APB_TSENS_CTRL>;
#[doc = "Writer for register APB_SARADC_APB_TSENS_CTRL"]
pub type W = crate::W<u32, super::APB_SARADC_APB_TSENS_CTRL>;
#[doc = "Register APB_SARADC_APB_TSENS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_APB_TSENS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_TSENS_PU`"]
pub type APB_SARADC_TSENS_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_TSENS_PU`"]
pub struct APB_SARADC_TSENS_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TSENS_PU_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_TSENS_CLK_DIV`"]
pub type APB_SARADC_TSENS_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_TSENS_CLK_DIV`"]
pub struct APB_SARADC_TSENS_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TSENS_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | (((value as u32) & 0xff) << 14);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_TSENS_IN_INV`"]
pub type APB_SARADC_TSENS_IN_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_TSENS_IN_INV`"]
pub struct APB_SARADC_TSENS_IN_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TSENS_IN_INV_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_TSENS_OUT`"]
pub type APB_SARADC_TSENS_OUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn apb_saradc_tsens_pu(&self) -> APB_SARADC_TSENS_PU_R {
        APB_SARADC_TSENS_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 14:21"]
    #[inline(always)]
    pub fn apb_saradc_tsens_clk_div(&self) -> APB_SARADC_TSENS_CLK_DIV_R {
        APB_SARADC_TSENS_CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn apb_saradc_tsens_in_inv(&self) -> APB_SARADC_TSENS_IN_INV_R {
        APB_SARADC_TSENS_IN_INV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_saradc_tsens_out(&self) -> APB_SARADC_TSENS_OUT_R {
        APB_SARADC_TSENS_OUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn apb_saradc_tsens_pu(&mut self) -> APB_SARADC_TSENS_PU_W {
        APB_SARADC_TSENS_PU_W { w: self }
    }
    #[doc = "Bits 14:21"]
    #[inline(always)]
    pub fn apb_saradc_tsens_clk_div(&mut self) -> APB_SARADC_TSENS_CLK_DIV_W {
        APB_SARADC_TSENS_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn apb_saradc_tsens_in_inv(&mut self) -> APB_SARADC_TSENS_IN_INV_W {
        APB_SARADC_TSENS_IN_INV_W { w: self }
    }
}
