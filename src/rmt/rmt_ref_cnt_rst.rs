#[doc = "Writer for register RMT_REF_CNT_RST"]
pub type W = crate::W<u32, super::RMT_REF_CNT_RST>;
#[doc = "Register RMT_REF_CNT_RST `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_REF_CNT_RST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RMT_REF_CNT_RST_CH3`"]
pub struct RMT_REF_CNT_RST_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_REF_CNT_RST_CH3_W<'a> {
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
#[doc = "Write proxy for field `RMT_REF_CNT_RST_CH2`"]
pub struct RMT_REF_CNT_RST_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_REF_CNT_RST_CH2_W<'a> {
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
#[doc = "Write proxy for field `RMT_REF_CNT_RST_CH1`"]
pub struct RMT_REF_CNT_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_REF_CNT_RST_CH1_W<'a> {
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
#[doc = "Write proxy for field `RMT_REF_CNT_RST_CH0`"]
pub struct RMT_REF_CNT_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_REF_CNT_RST_CH0_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_ref_cnt_rst_ch3(&mut self) -> RMT_REF_CNT_RST_CH3_W {
        RMT_REF_CNT_RST_CH3_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rmt_ref_cnt_rst_ch2(&mut self) -> RMT_REF_CNT_RST_CH2_W {
        RMT_REF_CNT_RST_CH2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rmt_ref_cnt_rst_ch1(&mut self) -> RMT_REF_CNT_RST_CH1_W {
        RMT_REF_CNT_RST_CH1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_ref_cnt_rst_ch0(&mut self) -> RMT_REF_CNT_RST_CH0_W {
        RMT_REF_CNT_RST_CH0_W { w: self }
    }
}
