#[doc = "Reader of register APB_SARADC_INT_ENA"]
pub type R = crate::R<u32, super::APB_SARADC_INT_ENA>;
#[doc = "Writer for register APB_SARADC_INT_ENA"]
pub type W = crate::W<u32, super::APB_SARADC_INT_ENA>;
#[doc = "Register APB_SARADC_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_ADC1_DONE_INT_ENA`"]
pub type APB_SARADC_ADC1_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_ADC1_DONE_INT_ENA`"]
pub struct APB_SARADC_ADC1_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC1_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_ADC2_DONE_INT_ENA`"]
pub type APB_SARADC_ADC2_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_ADC2_DONE_INT_ENA`"]
pub struct APB_SARADC_ADC2_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC2_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_THRES0_HIGH_INT_ENA`"]
pub type APB_SARADC_THRES0_HIGH_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_THRES0_HIGH_INT_ENA`"]
pub struct APB_SARADC_THRES0_HIGH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES0_HIGH_INT_ENA_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_THRES1_HIGH_INT_ENA`"]
pub type APB_SARADC_THRES1_HIGH_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_THRES1_HIGH_INT_ENA`"]
pub struct APB_SARADC_THRES1_HIGH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_HIGH_INT_ENA_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_THRES0_LOW_INT_ENA`"]
pub type APB_SARADC_THRES0_LOW_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_THRES0_LOW_INT_ENA`"]
pub struct APB_SARADC_THRES0_LOW_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES0_LOW_INT_ENA_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_THRES1_LOW_INT_ENA`"]
pub type APB_SARADC_THRES1_LOW_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_THRES1_LOW_INT_ENA`"]
pub struct APB_SARADC_THRES1_LOW_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_LOW_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_adc1_done_int_ena(&self) -> APB_SARADC_ADC1_DONE_INT_ENA_R {
        APB_SARADC_ADC1_DONE_INT_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc_adc2_done_int_ena(&self) -> APB_SARADC_ADC2_DONE_INT_ENA_R {
        APB_SARADC_ADC2_DONE_INT_ENA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_ena(&self) -> APB_SARADC_THRES0_HIGH_INT_ENA_R {
        APB_SARADC_THRES0_HIGH_INT_ENA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_ena(&self) -> APB_SARADC_THRES1_HIGH_INT_ENA_R {
        APB_SARADC_THRES1_HIGH_INT_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_ena(&self) -> APB_SARADC_THRES0_LOW_INT_ENA_R {
        APB_SARADC_THRES0_LOW_INT_ENA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_ena(&self) -> APB_SARADC_THRES1_LOW_INT_ENA_R {
        APB_SARADC_THRES1_LOW_INT_ENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_adc1_done_int_ena(&mut self) -> APB_SARADC_ADC1_DONE_INT_ENA_W {
        APB_SARADC_ADC1_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc_adc2_done_int_ena(&mut self) -> APB_SARADC_ADC2_DONE_INT_ENA_W {
        APB_SARADC_ADC2_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_ena(&mut self) -> APB_SARADC_THRES0_HIGH_INT_ENA_W {
        APB_SARADC_THRES0_HIGH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_ena(&mut self) -> APB_SARADC_THRES1_HIGH_INT_ENA_W {
        APB_SARADC_THRES1_HIGH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_ena(&mut self) -> APB_SARADC_THRES0_LOW_INT_ENA_W {
        APB_SARADC_THRES0_LOW_INT_ENA_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_ena(&mut self) -> APB_SARADC_THRES1_LOW_INT_ENA_W {
        APB_SARADC_THRES1_LOW_INT_ENA_W { w: self }
    }
}
