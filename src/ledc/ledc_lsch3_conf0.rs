#[doc = "Reader of register LEDC_LSCH3_CONF0"]
pub type R = crate::R<u32, super::LEDC_LSCH3_CONF0>;
#[doc = "Writer for register LEDC_LSCH3_CONF0"]
pub type W = crate::W<u32, super::LEDC_LSCH3_CONF0>;
#[doc = "Register LEDC_LSCH3_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_LSCH3_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LEDC_OVF_CNT_RESET_LSCH3`"]
pub struct LEDC_OVF_CNT_RESET_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_RESET_LSCH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `LEDC_OVF_CNT_EN_LSCH3`"]
pub type LEDC_OVF_CNT_EN_LSCH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_OVF_CNT_EN_LSCH3`"]
pub struct LEDC_OVF_CNT_EN_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_EN_LSCH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `LEDC_OVF_NUM_LSCH3`"]
pub type LEDC_OVF_NUM_LSCH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEDC_OVF_NUM_LSCH3`"]
pub struct LEDC_OVF_NUM_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_NUM_LSCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 5)) | (((value as u32) & 0x03ff) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `LEDC_PARA_UP_LSCH3`"]
pub struct LEDC_PARA_UP_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_PARA_UP_LSCH3_W<'a> {
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
#[doc = "Reader of field `LEDC_IDLE_LV_LSCH3`"]
pub type LEDC_IDLE_LV_LSCH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_IDLE_LV_LSCH3`"]
pub struct LEDC_IDLE_LV_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_IDLE_LV_LSCH3_W<'a> {
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
#[doc = "Reader of field `LEDC_SIG_OUT_EN_LSCH3`"]
pub type LEDC_SIG_OUT_EN_LSCH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_SIG_OUT_EN_LSCH3`"]
pub struct LEDC_SIG_OUT_EN_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_SIG_OUT_EN_LSCH3_W<'a> {
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
#[doc = "Reader of field `LEDC_TIMER_SEL_LSCH3`"]
pub type LEDC_TIMER_SEL_LSCH3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEDC_TIMER_SEL_LSCH3`"]
pub struct LEDC_TIMER_SEL_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_TIMER_SEL_LSCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_en_lsch3(&self) -> LEDC_OVF_CNT_EN_LSCH3_R {
        LEDC_OVF_CNT_EN_LSCH3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 5:14"]
    #[inline(always)]
    pub fn ledc_ovf_num_lsch3(&self) -> LEDC_OVF_NUM_LSCH3_R {
        LEDC_OVF_NUM_LSCH3_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ledc_idle_lv_lsch3(&self) -> LEDC_IDLE_LV_LSCH3_R {
        LEDC_IDLE_LV_LSCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ledc_sig_out_en_lsch3(&self) -> LEDC_SIG_OUT_EN_LSCH3_R {
        LEDC_SIG_OUT_EN_LSCH3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ledc_timer_sel_lsch3(&self) -> LEDC_TIMER_SEL_LSCH3_R {
        LEDC_TIMER_SEL_LSCH3_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_reset_lsch3(&mut self) -> LEDC_OVF_CNT_RESET_LSCH3_W {
        LEDC_OVF_CNT_RESET_LSCH3_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_en_lsch3(&mut self) -> LEDC_OVF_CNT_EN_LSCH3_W {
        LEDC_OVF_CNT_EN_LSCH3_W { w: self }
    }
    #[doc = "Bits 5:14"]
    #[inline(always)]
    pub fn ledc_ovf_num_lsch3(&mut self) -> LEDC_OVF_NUM_LSCH3_W {
        LEDC_OVF_NUM_LSCH3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ledc_para_up_lsch3(&mut self) -> LEDC_PARA_UP_LSCH3_W {
        LEDC_PARA_UP_LSCH3_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ledc_idle_lv_lsch3(&mut self) -> LEDC_IDLE_LV_LSCH3_W {
        LEDC_IDLE_LV_LSCH3_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ledc_sig_out_en_lsch3(&mut self) -> LEDC_SIG_OUT_EN_LSCH3_W {
        LEDC_SIG_OUT_EN_LSCH3_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ledc_timer_sel_lsch3(&mut self) -> LEDC_TIMER_SEL_LSCH3_W {
        LEDC_TIMER_SEL_LSCH3_W { w: self }
    }
}
