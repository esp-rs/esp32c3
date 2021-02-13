#[doc = "Reader of register APB_SARADC_CTRL2"]
pub type R = crate::R<u32, super::APB_SARADC_CTRL2>;
#[doc = "Writer for register APB_SARADC_CTRL2"]
pub type W = crate::W<u32, super::APB_SARADC_CTRL2>;
#[doc = "Register APB_SARADC_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_TIMER_EN`"]
pub type APB_SARADC_TIMER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_TIMER_EN`"]
pub struct APB_SARADC_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TIMER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_TIMER_TARGET`"]
pub type APB_SARADC_TIMER_TARGET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APB_SARADC_TIMER_TARGET`"]
pub struct APB_SARADC_TIMER_TARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_TIMER_TARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | (((value as u32) & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_SAR2_INV`"]
pub type APB_SARADC_SAR2_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_SAR2_INV`"]
pub struct APB_SARADC_SAR2_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_SAR2_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_SAR1_INV`"]
pub type APB_SARADC_SAR1_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_SAR1_INV`"]
pub struct APB_SARADC_SAR1_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_SAR1_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_MAX_MEAS_NUM`"]
pub type APB_SARADC_MAX_MEAS_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_MAX_MEAS_NUM`"]
pub struct APB_SARADC_MAX_MEAS_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_MAX_MEAS_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 1)) | (((value as u32) & 0xff) << 1);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_MEAS_NUM_LIMIT`"]
pub type APB_SARADC_MEAS_NUM_LIMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_MEAS_NUM_LIMIT`"]
pub struct APB_SARADC_MEAS_NUM_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_MEAS_NUM_LIMIT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn apb_saradc_timer_en(&self) -> APB_SARADC_TIMER_EN_R {
        APB_SARADC_TIMER_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 12:23"]
    #[inline(always)]
    pub fn apb_saradc_timer_target(&self) -> APB_SARADC_TIMER_TARGET_R {
        APB_SARADC_TIMER_TARGET_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn apb_saradc_sar2_inv(&self) -> APB_SARADC_SAR2_INV_R {
        APB_SARADC_SAR2_INV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn apb_saradc_sar1_inv(&self) -> APB_SARADC_SAR1_INV_R {
        APB_SARADC_SAR1_INV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 1:8"]
    #[inline(always)]
    pub fn apb_saradc_max_meas_num(&self) -> APB_SARADC_MAX_MEAS_NUM_R {
        APB_SARADC_MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_saradc_meas_num_limit(&self) -> APB_SARADC_MEAS_NUM_LIMIT_R {
        APB_SARADC_MEAS_NUM_LIMIT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn apb_saradc_timer_en(&mut self) -> APB_SARADC_TIMER_EN_W {
        APB_SARADC_TIMER_EN_W { w: self }
    }
    #[doc = "Bits 12:23"]
    #[inline(always)]
    pub fn apb_saradc_timer_target(&mut self) -> APB_SARADC_TIMER_TARGET_W {
        APB_SARADC_TIMER_TARGET_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn apb_saradc_sar2_inv(&mut self) -> APB_SARADC_SAR2_INV_W {
        APB_SARADC_SAR2_INV_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn apb_saradc_sar1_inv(&mut self) -> APB_SARADC_SAR1_INV_W {
        APB_SARADC_SAR1_INV_W { w: self }
    }
    #[doc = "Bits 1:8"]
    #[inline(always)]
    pub fn apb_saradc_max_meas_num(&mut self) -> APB_SARADC_MAX_MEAS_NUM_W {
        APB_SARADC_MAX_MEAS_NUM_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_saradc_meas_num_limit(&mut self) -> APB_SARADC_MEAS_NUM_LIMIT_W {
        APB_SARADC_MEAS_NUM_LIMIT_W { w: self }
    }
}
