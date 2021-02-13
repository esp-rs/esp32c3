#[doc = "Reader of register APB_SARADC_FILTER_CTRL1"]
pub type R = crate::R<u32, super::APB_SARADC_FILTER_CTRL1>;
#[doc = "Writer for register APB_SARADC_FILTER_CTRL1"]
pub type W = crate::W<u32, super::APB_SARADC_FILTER_CTRL1>;
#[doc = "Register APB_SARADC_FILTER_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_FILTER_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_FILTER_FACTOR0`"]
pub type APB_SARADC_FILTER_FACTOR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_FILTER_FACTOR0`"]
pub struct APB_SARADC_FILTER_FACTOR0_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_FILTER_FACTOR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_FILTER_FACTOR1`"]
pub type APB_SARADC_FILTER_FACTOR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_FILTER_FACTOR1`"]
pub struct APB_SARADC_FILTER_FACTOR1_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_FILTER_FACTOR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn apb_saradc_filter_factor0(&self) -> APB_SARADC_FILTER_FACTOR0_R {
        APB_SARADC_FILTER_FACTOR0_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn apb_saradc_filter_factor1(&self) -> APB_SARADC_FILTER_FACTOR1_R {
        APB_SARADC_FILTER_FACTOR1_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn apb_saradc_filter_factor0(&mut self) -> APB_SARADC_FILTER_FACTOR0_W {
        APB_SARADC_FILTER_FACTOR0_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn apb_saradc_filter_factor1(&mut self) -> APB_SARADC_FILTER_FACTOR1_W {
        APB_SARADC_FILTER_FACTOR1_W { w: self }
    }
}
