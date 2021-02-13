#[doc = "Reader of register APB_SARADC_INT_RAW"]
pub type R = crate::R<u32, super::APB_SARADC_INT_RAW>;
#[doc = "Reader of field `APB_SARADC_ADC1_DONE_INT_RAW`"]
pub type APB_SARADC_ADC1_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `APB_SARADC_ADC2_DONE_INT_RAW`"]
pub type APB_SARADC_ADC2_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `APB_SARADC_THRES0_HIGH_INT_RAW`"]
pub type APB_SARADC_THRES0_HIGH_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `APB_SARADC_THRES1_HIGH_INT_RAW`"]
pub type APB_SARADC_THRES1_HIGH_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `APB_SARADC_THRES0_LOW_INT_RAW`"]
pub type APB_SARADC_THRES0_LOW_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `APB_SARADC_THRES1_LOW_INT_RAW`"]
pub type APB_SARADC_THRES1_LOW_INT_RAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_adc1_done_int_raw(&self) -> APB_SARADC_ADC1_DONE_INT_RAW_R {
        APB_SARADC_ADC1_DONE_INT_RAW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc_adc2_done_int_raw(&self) -> APB_SARADC_ADC2_DONE_INT_RAW_R {
        APB_SARADC_ADC2_DONE_INT_RAW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_raw(&self) -> APB_SARADC_THRES0_HIGH_INT_RAW_R {
        APB_SARADC_THRES0_HIGH_INT_RAW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_raw(&self) -> APB_SARADC_THRES1_HIGH_INT_RAW_R {
        APB_SARADC_THRES1_HIGH_INT_RAW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_raw(&self) -> APB_SARADC_THRES0_LOW_INT_RAW_R {
        APB_SARADC_THRES0_LOW_INT_RAW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_raw(&self) -> APB_SARADC_THRES1_LOW_INT_RAW_R {
        APB_SARADC_THRES1_LOW_INT_RAW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
