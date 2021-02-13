#[doc = "Reader of register APB_SARADC_ONETIME_SAMPLE"]
pub type R = crate::R<u32, super::APB_SARADC_ONETIME_SAMPLE>;
#[doc = "Writer for register APB_SARADC_ONETIME_SAMPLE"]
pub type W = crate::W<u32, super::APB_SARADC_ONETIME_SAMPLE>;
#[doc = "Register APB_SARADC_ONETIME_SAMPLE `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_ONETIME_SAMPLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC1_ONETIME_SAMPLE`"]
pub type APB_SARADC1_ONETIME_SAMPLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC1_ONETIME_SAMPLE`"]
pub struct APB_SARADC1_ONETIME_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC1_ONETIME_SAMPLE_W<'a> {
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
#[doc = "Reader of field `APB_SARADC2_ONETIME_SAMPLE`"]
pub type APB_SARADC2_ONETIME_SAMPLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC2_ONETIME_SAMPLE`"]
pub struct APB_SARADC2_ONETIME_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC2_ONETIME_SAMPLE_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_ONETIME_START`"]
pub type APB_SARADC_ONETIME_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_ONETIME_START`"]
pub struct APB_SARADC_ONETIME_START_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ONETIME_START_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_ONETIME_CHANNEL`"]
pub type APB_SARADC_ONETIME_CHANNEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_ONETIME_CHANNEL`"]
pub struct APB_SARADC_ONETIME_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ONETIME_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_ONETIME_ATTEN`"]
pub type APB_SARADC_ONETIME_ATTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_ONETIME_ATTEN`"]
pub struct APB_SARADC_ONETIME_ATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_ONETIME_ATTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc1_onetime_sample(&self) -> APB_SARADC1_ONETIME_SAMPLE_R {
        APB_SARADC1_ONETIME_SAMPLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc2_onetime_sample(&self) -> APB_SARADC2_ONETIME_SAMPLE_R {
        APB_SARADC2_ONETIME_SAMPLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn apb_saradc_onetime_start(&self) -> APB_SARADC_ONETIME_START_R {
        APB_SARADC_ONETIME_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn apb_saradc_onetime_channel(&self) -> APB_SARADC_ONETIME_CHANNEL_R {
        APB_SARADC_ONETIME_CHANNEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn apb_saradc_onetime_atten(&self) -> APB_SARADC_ONETIME_ATTEN_R {
        APB_SARADC_ONETIME_ATTEN_R::new(((self.bits >> 23) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc1_onetime_sample(&mut self) -> APB_SARADC1_ONETIME_SAMPLE_W {
        APB_SARADC1_ONETIME_SAMPLE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc2_onetime_sample(&mut self) -> APB_SARADC2_ONETIME_SAMPLE_W {
        APB_SARADC2_ONETIME_SAMPLE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn apb_saradc_onetime_start(&mut self) -> APB_SARADC_ONETIME_START_W {
        APB_SARADC_ONETIME_START_W { w: self }
    }
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn apb_saradc_onetime_channel(&mut self) -> APB_SARADC_ONETIME_CHANNEL_W {
        APB_SARADC_ONETIME_CHANNEL_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn apb_saradc_onetime_atten(&mut self) -> APB_SARADC_ONETIME_ATTEN_W {
        APB_SARADC_ONETIME_ATTEN_W { w: self }
    }
}
