#[doc = "Reader of register APB_SARADC_CALI"]
pub type R = crate::R<u32, super::APB_SARADC_CALI>;
#[doc = "Writer for register APB_SARADC_CALI"]
pub type W = crate::W<u32, super::APB_SARADC_CALI>;
#[doc = "Register APB_SARADC_CALI `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_CALI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_CALI_CFG`"]
pub type APB_SARADC_CALI_CFG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_SARADC_CALI_CFG`"]
pub struct APB_SARADC_CALI_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_CALI_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn apb_saradc_cali_cfg(&self) -> APB_SARADC_CALI_CFG_R {
        APB_SARADC_CALI_CFG_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn apb_saradc_cali_cfg(&mut self) -> APB_SARADC_CALI_CFG_W {
        APB_SARADC_CALI_CFG_W { w: self }
    }
}
