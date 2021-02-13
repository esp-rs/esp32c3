#[doc = "Reader of register APB_SARADC_SAR_PATT_TAB2"]
pub type R = crate::R<u32, super::APB_SARADC_SAR_PATT_TAB2>;
#[doc = "Writer for register APB_SARADC_SAR_PATT_TAB2"]
pub type W = crate::W<u32, super::APB_SARADC_SAR_PATT_TAB2>;
#[doc = "Register APB_SARADC_SAR_PATT_TAB2 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_SAR_PATT_TAB2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_SAR_PATT_TAB2`"]
pub type APB_SARADC_SAR_PATT_TAB2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_SARADC_SAR_PATT_TAB2`"]
pub struct APB_SARADC_SAR_PATT_TAB2_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_SAR_PATT_TAB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn apb_saradc_sar_patt_tab2(&self) -> APB_SARADC_SAR_PATT_TAB2_R {
        APB_SARADC_SAR_PATT_TAB2_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn apb_saradc_sar_patt_tab2(&mut self) -> APB_SARADC_SAR_PATT_TAB2_W {
        APB_SARADC_SAR_PATT_TAB2_W { w: self }
    }
}
