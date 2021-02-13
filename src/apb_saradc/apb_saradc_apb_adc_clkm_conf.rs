#[doc = "Reader of register APB_SARADC_APB_ADC_CLKM_CONF"]
pub type R = crate::R<u32, super::APB_SARADC_APB_ADC_CLKM_CONF>;
#[doc = "Writer for register APB_SARADC_APB_ADC_CLKM_CONF"]
pub type W = crate::W<u32, super::APB_SARADC_APB_ADC_CLKM_CONF>;
#[doc = "Register APB_SARADC_APB_ADC_CLKM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_APB_ADC_CLKM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_CLK_SEL`"]
pub type APB_SARADC_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_CLK_SEL`"]
pub struct APB_SARADC_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_CLK_EN`"]
pub type APB_SARADC_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_CLK_EN`"]
pub struct APB_SARADC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_CLK_EN_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_CLKM_DIV_A`"]
pub type APB_SARADC_CLKM_DIV_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_CLKM_DIV_A`"]
pub struct APB_SARADC_CLKM_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_CLKM_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_CLKM_DIV_B`"]
pub type APB_SARADC_CLKM_DIV_B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_CLKM_DIV_B`"]
pub struct APB_SARADC_CLKM_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_CLKM_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_CLKM_DIV_NUM`"]
pub type APB_SARADC_CLKM_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_CLKM_DIV_NUM`"]
pub struct APB_SARADC_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn apb_saradc_clk_sel(&self) -> APB_SARADC_CLK_SEL_R {
        APB_SARADC_CLK_SEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&self) -> APB_SARADC_CLK_EN_R {
        APB_SARADC_CLK_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn apb_saradc_clkm_div_a(&self) -> APB_SARADC_CLKM_DIV_A_R {
        APB_SARADC_CLKM_DIV_A_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn apb_saradc_clkm_div_b(&self) -> APB_SARADC_CLKM_DIV_B_R {
        APB_SARADC_CLKM_DIV_B_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_saradc_clkm_div_num(&self) -> APB_SARADC_CLKM_DIV_NUM_R {
        APB_SARADC_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn apb_saradc_clk_sel(&mut self) -> APB_SARADC_CLK_SEL_W {
        APB_SARADC_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&mut self) -> APB_SARADC_CLK_EN_W {
        APB_SARADC_CLK_EN_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn apb_saradc_clkm_div_a(&mut self) -> APB_SARADC_CLKM_DIV_A_W {
        APB_SARADC_CLKM_DIV_A_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn apb_saradc_clkm_div_b(&mut self) -> APB_SARADC_CLKM_DIV_B_W {
        APB_SARADC_CLKM_DIV_B_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_saradc_clkm_div_num(&mut self) -> APB_SARADC_CLKM_DIV_NUM_W {
        APB_SARADC_CLKM_DIV_NUM_W { w: self }
    }
}
