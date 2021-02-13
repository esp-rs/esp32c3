#[doc = "Writer for register LEDC_INT_CLR"]
pub type W = crate::W<u32, super::LEDC_INT_CLR>;
#[doc = "Register LEDC_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH5_INT_CLR`"]
pub struct LEDC_OVF_CNT_LSCH5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH5_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH4_INT_CLR`"]
pub struct LEDC_OVF_CNT_LSCH4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH4_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH3_INT_CLR`"]
pub struct LEDC_OVF_CNT_LSCH3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH3_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH2_INT_CLR`"]
pub struct LEDC_OVF_CNT_LSCH2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH2_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH1_INT_CLR`"]
pub struct LEDC_OVF_CNT_LSCH1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH0_INT_CLR`"]
pub struct LEDC_OVF_CNT_LSCH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH0_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH5_INT_CLR`"]
pub struct LEDC_DUTY_CHNG_END_LSCH5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH5_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH4_INT_CLR`"]
pub struct LEDC_DUTY_CHNG_END_LSCH4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH4_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH3_INT_CLR`"]
pub struct LEDC_DUTY_CHNG_END_LSCH3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH3_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH2_INT_CLR`"]
pub struct LEDC_DUTY_CHNG_END_LSCH2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH2_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH1_INT_CLR`"]
pub struct LEDC_DUTY_CHNG_END_LSCH1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH1_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH0_INT_CLR`"]
pub struct LEDC_DUTY_CHNG_END_LSCH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH0_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LEDC_LSTIMER3_OVF_INT_CLR`"]
pub struct LEDC_LSTIMER3_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER3_OVF_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LEDC_LSTIMER2_OVF_INT_CLR`"]
pub struct LEDC_LSTIMER2_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER2_OVF_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LEDC_LSTIMER1_OVF_INT_CLR`"]
pub struct LEDC_LSTIMER1_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER1_OVF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `LEDC_LSTIMER0_OVF_INT_CLR`"]
pub struct LEDC_LSTIMER0_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER0_OVF_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch5_int_clr(&mut self) -> LEDC_OVF_CNT_LSCH5_INT_CLR_W {
        LEDC_OVF_CNT_LSCH5_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch4_int_clr(&mut self) -> LEDC_OVF_CNT_LSCH4_INT_CLR_W {
        LEDC_OVF_CNT_LSCH4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch3_int_clr(&mut self) -> LEDC_OVF_CNT_LSCH3_INT_CLR_W {
        LEDC_OVF_CNT_LSCH3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch2_int_clr(&mut self) -> LEDC_OVF_CNT_LSCH2_INT_CLR_W {
        LEDC_OVF_CNT_LSCH2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch1_int_clr(&mut self) -> LEDC_OVF_CNT_LSCH1_INT_CLR_W {
        LEDC_OVF_CNT_LSCH1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch0_int_clr(&mut self) -> LEDC_OVF_CNT_LSCH0_INT_CLR_W {
        LEDC_OVF_CNT_LSCH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch5_int_clr(&mut self) -> LEDC_DUTY_CHNG_END_LSCH5_INT_CLR_W {
        LEDC_DUTY_CHNG_END_LSCH5_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch4_int_clr(&mut self) -> LEDC_DUTY_CHNG_END_LSCH4_INT_CLR_W {
        LEDC_DUTY_CHNG_END_LSCH4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch3_int_clr(&mut self) -> LEDC_DUTY_CHNG_END_LSCH3_INT_CLR_W {
        LEDC_DUTY_CHNG_END_LSCH3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch2_int_clr(&mut self) -> LEDC_DUTY_CHNG_END_LSCH2_INT_CLR_W {
        LEDC_DUTY_CHNG_END_LSCH2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch1_int_clr(&mut self) -> LEDC_DUTY_CHNG_END_LSCH1_INT_CLR_W {
        LEDC_DUTY_CHNG_END_LSCH1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch0_int_clr(&mut self) -> LEDC_DUTY_CHNG_END_LSCH0_INT_CLR_W {
        LEDC_DUTY_CHNG_END_LSCH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ledc_lstimer3_ovf_int_clr(&mut self) -> LEDC_LSTIMER3_OVF_INT_CLR_W {
        LEDC_LSTIMER3_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ledc_lstimer2_ovf_int_clr(&mut self) -> LEDC_LSTIMER2_OVF_INT_CLR_W {
        LEDC_LSTIMER2_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ledc_lstimer1_ovf_int_clr(&mut self) -> LEDC_LSTIMER1_OVF_INT_CLR_W {
        LEDC_LSTIMER1_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ledc_lstimer0_ovf_int_clr(&mut self) -> LEDC_LSTIMER0_OVF_INT_CLR_W {
        LEDC_LSTIMER0_OVF_INT_CLR_W { w: self }
    }
}
