#[doc = "Reader of register APB_SARADC_SAR_PATT_TAB1"]
pub type R = crate::R<u32, super::APB_SARADC_SAR_PATT_TAB1>;
#[doc = "Writer for register APB_SARADC_SAR_PATT_TAB1"]
pub type W = crate::W<u32, super::APB_SARADC_SAR_PATT_TAB1>;
#[doc = "Register APB_SARADC_SAR_PATT_TAB1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_SAR_PATT_TAB1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_SAR_PATT_TAB1`"]
pub type APB_SARADC_SAR_PATT_TAB1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_SARADC_SAR_PATT_TAB1`"]
pub struct APB_SARADC_SAR_PATT_TAB1_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_SAR_PATT_TAB1_W<'a> {
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
    pub fn apb_saradc_sar_patt_tab1(&self) -> APB_SARADC_SAR_PATT_TAB1_R {
        APB_SARADC_SAR_PATT_TAB1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn apb_saradc_sar_patt_tab1(&mut self) -> APB_SARADC_SAR_PATT_TAB1_W {
        APB_SARADC_SAR_PATT_TAB1_W { w: self }
    }
}
