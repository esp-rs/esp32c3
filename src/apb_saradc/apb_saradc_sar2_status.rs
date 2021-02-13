#[doc = "Reader of register APB_SARADC_SAR2_STATUS"]
pub type R = crate::R<u32, super::APB_SARADC_SAR2_STATUS>;
#[doc = "Reader of field `APB_SARADC_SAR2_STATUS`"]
pub type APB_SARADC_SAR2_STATUS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_saradc_sar2_status(&self) -> APB_SARADC_SAR2_STATUS_R {
        APB_SARADC_SAR2_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
