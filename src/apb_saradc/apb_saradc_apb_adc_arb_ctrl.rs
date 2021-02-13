#[doc = "Reader of register APB_SARADC_APB_ADC_ARB_CTRL"]
pub type R = crate::R<u32, super::APB_SARADC_APB_ADC_ARB_CTRL>;
#[doc = "Writer for register APB_SARADC_APB_ADC_ARB_CTRL"]
pub type W = crate::W<u32, super::APB_SARADC_APB_ADC_ARB_CTRL>;
#[doc = "Register APB_SARADC_APB_ADC_ARB_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_APB_ADC_ARB_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_ADC_ARB_FIX_PRIORITY`"]
pub type APB_SARADC_ADC_ARB_FIX_PRIORITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_ADC_ARB_FIX_PRIORITY`"]
pub struct APB_SARADC_ADC_ARB_FIX_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC_ARB_FIX_PRIORITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_ADC_ARB_WIFI_PRIORITY`"]
pub type APB_SARADC_ADC_ARB_WIFI_PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_ADC_ARB_WIFI_PRIORITY`"]
pub struct APB_SARADC_ADC_ARB_WIFI_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC_ARB_WIFI_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_ADC_ARB_RTC_PRIORITY`"]
pub type APB_SARADC_ADC_ARB_RTC_PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_ADC_ARB_RTC_PRIORITY`"]
pub struct APB_SARADC_ADC_ARB_RTC_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC_ARB_RTC_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_ADC_ARB_APB_PRIORITY`"]
pub type APB_SARADC_ADC_ARB_APB_PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_ADC_ARB_APB_PRIORITY`"]
pub struct APB_SARADC_ADC_ARB_APB_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC_ARB_APB_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_ADC_ARB_GRANT_FORCE`"]
pub type APB_SARADC_ADC_ARB_GRANT_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_ADC_ARB_GRANT_FORCE`"]
pub struct APB_SARADC_ADC_ARB_GRANT_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC_ARB_GRANT_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_ADC_ARB_WIFI_FORCE`"]
pub type APB_SARADC_ADC_ARB_WIFI_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_ADC_ARB_WIFI_FORCE`"]
pub struct APB_SARADC_ADC_ARB_WIFI_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC_ARB_WIFI_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_ADC_ARB_RTC_FORCE`"]
pub type APB_SARADC_ADC_ARB_RTC_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_ADC_ARB_RTC_FORCE`"]
pub struct APB_SARADC_ADC_ARB_RTC_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC_ARB_RTC_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_ADC_ARB_APB_FORCE`"]
pub type APB_SARADC_ADC_ARB_APB_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_ADC_ARB_APB_FORCE`"]
pub struct APB_SARADC_ADC_ARB_APB_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ADC_ARB_APB_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_fix_priority(&self) -> APB_SARADC_ADC_ARB_FIX_PRIORITY_R {
        APB_SARADC_ADC_ARB_FIX_PRIORITY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_wifi_priority(&self) -> APB_SARADC_ADC_ARB_WIFI_PRIORITY_R {
        APB_SARADC_ADC_ARB_WIFI_PRIORITY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_rtc_priority(&self) -> APB_SARADC_ADC_ARB_RTC_PRIORITY_R {
        APB_SARADC_ADC_ARB_RTC_PRIORITY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_apb_priority(&self) -> APB_SARADC_ADC_ARB_APB_PRIORITY_R {
        APB_SARADC_ADC_ARB_APB_PRIORITY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_grant_force(&self) -> APB_SARADC_ADC_ARB_GRANT_FORCE_R {
        APB_SARADC_ADC_ARB_GRANT_FORCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_wifi_force(&self) -> APB_SARADC_ADC_ARB_WIFI_FORCE_R {
        APB_SARADC_ADC_ARB_WIFI_FORCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_rtc_force(&self) -> APB_SARADC_ADC_ARB_RTC_FORCE_R {
        APB_SARADC_ADC_ARB_RTC_FORCE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_apb_force(&self) -> APB_SARADC_ADC_ARB_APB_FORCE_R {
        APB_SARADC_ADC_ARB_APB_FORCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_fix_priority(&mut self) -> APB_SARADC_ADC_ARB_FIX_PRIORITY_W {
        APB_SARADC_ADC_ARB_FIX_PRIORITY_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_wifi_priority(&mut self) -> APB_SARADC_ADC_ARB_WIFI_PRIORITY_W {
        APB_SARADC_ADC_ARB_WIFI_PRIORITY_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_rtc_priority(&mut self) -> APB_SARADC_ADC_ARB_RTC_PRIORITY_W {
        APB_SARADC_ADC_ARB_RTC_PRIORITY_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_apb_priority(&mut self) -> APB_SARADC_ADC_ARB_APB_PRIORITY_W {
        APB_SARADC_ADC_ARB_APB_PRIORITY_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_grant_force(&mut self) -> APB_SARADC_ADC_ARB_GRANT_FORCE_W {
        APB_SARADC_ADC_ARB_GRANT_FORCE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_wifi_force(&mut self) -> APB_SARADC_ADC_ARB_WIFI_FORCE_W {
        APB_SARADC_ADC_ARB_WIFI_FORCE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_rtc_force(&mut self) -> APB_SARADC_ADC_ARB_RTC_FORCE_W {
        APB_SARADC_ADC_ARB_RTC_FORCE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_saradc_adc_arb_apb_force(&mut self) -> APB_SARADC_ADC_ARB_APB_FORCE_W {
        APB_SARADC_ADC_ARB_APB_FORCE_W { w: self }
    }
}
