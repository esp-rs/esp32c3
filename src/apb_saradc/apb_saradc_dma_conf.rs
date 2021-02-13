#[doc = "Reader of register APB_SARADC_DMA_CONF"]
pub type R = crate::R<u32, super::APB_SARADC_DMA_CONF>;
#[doc = "Writer for register APB_SARADC_DMA_CONF"]
pub type W = crate::W<u32, super::APB_SARADC_DMA_CONF>;
#[doc = "Register APB_SARADC_DMA_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_DMA_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_APB_ADC_TRANS`"]
pub type APB_SARADC_APB_ADC_TRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_APB_ADC_TRANS`"]
pub struct APB_SARADC_APB_ADC_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_APB_ADC_TRANS_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_APB_ADC_RESET_FSM`"]
pub type APB_SARADC_APB_ADC_RESET_FSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_SARADC_APB_ADC_RESET_FSM`"]
pub struct APB_SARADC_APB_ADC_RESET_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_APB_ADC_RESET_FSM_W<'a> {
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
#[doc = "Reader of field `APB_SARADC_APB_ADC_EOF_NUM`"]
pub type APB_SARADC_APB_ADC_EOF_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APB_SARADC_APB_ADC_EOF_NUM`"]
pub struct APB_SARADC_APB_ADC_EOF_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_APB_ADC_EOF_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_apb_adc_trans(&self) -> APB_SARADC_APB_ADC_TRANS_R {
        APB_SARADC_APB_ADC_TRANS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc_apb_adc_reset_fsm(&self) -> APB_SARADC_APB_ADC_RESET_FSM_R {
        APB_SARADC_APB_ADC_RESET_FSM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn apb_saradc_apb_adc_eof_num(&self) -> APB_SARADC_APB_ADC_EOF_NUM_R {
        APB_SARADC_APB_ADC_EOF_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc_apb_adc_trans(&mut self) -> APB_SARADC_APB_ADC_TRANS_W {
        APB_SARADC_APB_ADC_TRANS_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc_apb_adc_reset_fsm(&mut self) -> APB_SARADC_APB_ADC_RESET_FSM_W {
        APB_SARADC_APB_ADC_RESET_FSM_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn apb_saradc_apb_adc_eof_num(&mut self) -> APB_SARADC_APB_ADC_EOF_NUM_W {
        APB_SARADC_APB_ADC_EOF_NUM_W { w: self }
    }
}
