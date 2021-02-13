#[doc = "Reader of register APB_SARADC_THRES1_CTRL"]
pub type R = crate::R<u32, super::APB_SARADC_THRES1_CTRL>;
#[doc = "Writer for register APB_SARADC_THRES1_CTRL"]
pub type W = crate::W<u32, super::APB_SARADC_THRES1_CTRL>;
#[doc = "Register APB_SARADC_THRES1_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_THRES1_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_THRES1_LOW`"]
pub type APB_SARADC_THRES1_LOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APB_SARADC_THRES1_LOW`"]
pub struct APB_SARADC_THRES1_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 18)) | (((value as u32) & 0x1fff) << 18);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_THRES1_HIGH`"]
pub type APB_SARADC_THRES1_HIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APB_SARADC_THRES1_HIGH`"]
pub struct APB_SARADC_THRES1_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 5)) | (((value as u32) & 0x1fff) << 5);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_THRES1_CHANNEL`"]
pub type APB_SARADC_THRES1_CHANNEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_THRES1_CHANNEL`"]
pub struct APB_SARADC_THRES1_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:30"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low(&self) -> APB_SARADC_THRES1_LOW_R {
        APB_SARADC_THRES1_LOW_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
    #[doc = "Bits 5:17"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high(&self) -> APB_SARADC_THRES1_HIGH_R {
        APB_SARADC_THRES1_HIGH_R::new(((self.bits >> 5) & 0x1fff) as u16)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn apb_saradc_thres1_channel(&self) -> APB_SARADC_THRES1_CHANNEL_R {
        APB_SARADC_THRES1_CHANNEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 18:30"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low(&mut self) -> APB_SARADC_THRES1_LOW_W {
        APB_SARADC_THRES1_LOW_W { w: self }
    }
    #[doc = "Bits 5:17"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high(&mut self) -> APB_SARADC_THRES1_HIGH_W {
        APB_SARADC_THRES1_HIGH_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn apb_saradc_thres1_channel(&mut self) -> APB_SARADC_THRES1_CHANNEL_W {
        APB_SARADC_THRES1_CHANNEL_W { w: self }
    }
}
