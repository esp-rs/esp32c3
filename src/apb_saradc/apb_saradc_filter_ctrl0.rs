#[doc = "Reader of register APB_SARADC_FILTER_CTRL0"]
pub type R = crate::R<u32, super::APB_SARADC_FILTER_CTRL0>;
#[doc = "Writer for register APB_SARADC_FILTER_CTRL0"]
pub type W = crate::W<u32, super::APB_SARADC_FILTER_CTRL0>;
#[doc = "Register APB_SARADC_FILTER_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_FILTER_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_FILTER_RESET`"]
pub type APB_SARADC_FILTER_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_FILTER_RESET`"]
pub struct APB_SARADC_FILTER_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_FILTER_RESET_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_FILTER_CHANNEL0`"]
pub type APB_SARADC_FILTER_CHANNEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_FILTER_CHANNEL0`"]
pub struct APB_SARADC_FILTER_CHANNEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_FILTER_CHANNEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_FILTER_CHANNEL1`"]
pub type APB_SARADC_FILTER_CHANNEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_FILTER_CHANNEL1`"]
pub struct APB_SARADC_FILTER_CHANNEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_FILTER_CHANNEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_filter_reset(&self) -> APB_SARADC_FILTER_RESET_R {
        APB_SARADC_FILTER_RESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn apb_saradc_filter_channel0(&self) -> APB_SARADC_FILTER_CHANNEL0_R {
        APB_SARADC_FILTER_CHANNEL0_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn apb_saradc_filter_channel1(&self) -> APB_SARADC_FILTER_CHANNEL1_R {
        APB_SARADC_FILTER_CHANNEL1_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_filter_reset(&mut self) -> APB_SARADC_FILTER_RESET_W {
        APB_SARADC_FILTER_RESET_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn apb_saradc_filter_channel0(&mut self) -> APB_SARADC_FILTER_CHANNEL0_W {
        APB_SARADC_FILTER_CHANNEL0_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn apb_saradc_filter_channel1(&mut self) -> APB_SARADC_FILTER_CHANNEL1_W {
        APB_SARADC_FILTER_CHANNEL1_W { w: self }
    }
}
