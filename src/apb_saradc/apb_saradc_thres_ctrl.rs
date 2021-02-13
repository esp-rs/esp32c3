#[doc = "Reader of register APB_SARADC_THRES_CTRL"]
pub type R = crate::R<u32, super::APB_SARADC_THRES_CTRL>;
#[doc = "Writer for register APB_SARADC_THRES_CTRL"]
pub type W = crate::W<u32, super::APB_SARADC_THRES_CTRL>;
#[doc = "Register APB_SARADC_THRES_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_THRES_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_THRES0_EN`"]
pub type APB_SARADC_THRES0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_THRES0_EN`"]
pub struct APB_SARADC_THRES0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES0_EN_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_THRES1_EN`"]
pub type APB_SARADC_THRES1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_THRES1_EN`"]
pub struct APB_SARADC_THRES1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_EN_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_thres0_en(&self) -> APB_SARADC_THRES0_EN_R {
        APB_SARADC_THRES0_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc_thres1_en(&self) -> APB_SARADC_THRES1_EN_R {
        APB_SARADC_THRES1_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_thres0_en(&mut self) -> APB_SARADC_THRES0_EN_W {
        APB_SARADC_THRES0_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc_thres1_en(&mut self) -> APB_SARADC_THRES1_EN_W {
        APB_SARADC_THRES1_EN_W { w: self }
    }
}
