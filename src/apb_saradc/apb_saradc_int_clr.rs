#[doc = "Writer for register APB_SARADC_INT_CLR"]
pub type W = crate::W<u32, super::APB_SARADC_INT_CLR>;
#[doc = "Register APB_SARADC_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `APB_SARADC_ADC1_DONE_INT_CLR`"]
pub struct APB_SARADC_ADC1_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC1_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `APB_SARADC_ADC2_DONE_INT_CLR`"]
pub struct APB_SARADC_ADC2_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC2_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `APB_SARADC_THRES0_HIGH_INT_CLR`"]
pub struct APB_SARADC_THRES0_HIGH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES0_HIGH_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `APB_SARADC_THRES1_HIGH_INT_CLR`"]
pub struct APB_SARADC_THRES1_HIGH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_HIGH_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `APB_SARADC_THRES0_LOW_INT_CLR`"]
pub struct APB_SARADC_THRES0_LOW_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES0_LOW_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `APB_SARADC_THRES1_LOW_INT_CLR`"]
pub struct APB_SARADC_THRES1_LOW_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_LOW_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_adc1_done_int_clr(&mut self) -> APB_SARADC_ADC1_DONE_INT_CLR_W {
        APB_SARADC_ADC1_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc_adc2_done_int_clr(&mut self) -> APB_SARADC_ADC2_DONE_INT_CLR_W {
        APB_SARADC_ADC2_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_clr(&mut self) -> APB_SARADC_THRES0_HIGH_INT_CLR_W {
        APB_SARADC_THRES0_HIGH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_clr(&mut self) -> APB_SARADC_THRES1_HIGH_INT_CLR_W {
        APB_SARADC_THRES1_HIGH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_clr(&mut self) -> APB_SARADC_THRES0_LOW_INT_CLR_W {
        APB_SARADC_THRES0_LOW_INT_CLR_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_clr(&mut self) -> APB_SARADC_THRES1_LOW_INT_CLR_W {
        APB_SARADC_THRES1_LOW_INT_CLR_W { w: self }
    }
}
