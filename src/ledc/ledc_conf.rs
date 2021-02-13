#[doc = "Reader of register LEDC_CONF"]
pub type R = crate::R<u32, super::LEDC_CONF>;
#[doc = "Writer for register LEDC_CONF"]
pub type W = crate::W<u32, super::LEDC_CONF>;
#[doc = "Register LEDC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_CLK_EN`"]
pub type LEDC_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_CLK_EN`"]
pub struct LEDC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_CLK_EN_W<'a> {
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
#[doc = "Reader of field `LEDC_APB_CLK_SEL`"]
pub type LEDC_APB_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEDC_APB_CLK_SEL`"]
pub struct LEDC_APB_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_APB_CLK_SEL_W<'a> {
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
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ledc_apb_clk_sel(&self) -> LEDC_APB_CLK_SEL_R {
        LEDC_APB_CLK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W {
        LEDC_CLK_EN_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ledc_apb_clk_sel(&mut self) -> LEDC_APB_CLK_SEL_W {
        LEDC_APB_CLK_SEL_W { w: self }
    }
}
