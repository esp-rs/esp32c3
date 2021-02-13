#[doc = "Reader of register APB_SARADC_2_DATA_STATUS"]
pub type R = crate::R<u32, super::APB_SARADC_2_DATA_STATUS>;
#[doc = "Reader of field `APB_SARADC_ADC2_DATA`"]
pub type APB_SARADC_ADC2_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn apb_saradc_adc2_data(&self) -> APB_SARADC_ADC2_DATA_R {
        APB_SARADC_ADC2_DATA_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
