#[doc = "Reader of register RTC_CNTL_RESET_STATE"]
pub type R = crate::R<u32, super::RTC_CNTL_RESET_STATE>;
#[doc = "Writer for register RTC_CNTL_RESET_STATE"]
pub type W = crate::W<u32, super::RTC_CNTL_RESET_STATE>;
#[doc = "Register RTC_CNTL_RESET_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_RESET_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DRESET_MASK_PROCPU`"]
pub type RTC_CNTL_DRESET_MASK_PROCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DRESET_MASK_PROCPU`"]
pub struct RTC_CNTL_DRESET_MASK_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DRESET_MASK_PROCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DRESET_MASK_APPCPU`"]
pub type RTC_CNTL_DRESET_MASK_APPCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DRESET_MASK_APPCPU`"]
pub struct RTC_CNTL_DRESET_MASK_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DRESET_MASK_APPCPU_W<'a> {
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
#[doc = "Write proxy for field `RTC_CNTL_JTAG_RESET_FLAG_CLR_APPCPU`"]
pub struct RTC_CNTL_JTAG_RESET_FLAG_CLR_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_JTAG_RESET_FLAG_CLR_APPCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_JTAG_RESET_FLAG_CLR_PROCPU`"]
pub struct RTC_CNTL_JTAG_RESET_FLAG_CLR_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_JTAG_RESET_FLAG_CLR_PROCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_JTAG_RESET_FLAG_APPCPU`"]
pub type RTC_CNTL_JTAG_RESET_FLAG_APPCPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_JTAG_RESET_FLAG_PROCPU`"]
pub type RTC_CNTL_JTAG_RESET_FLAG_PROCPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_OCD_HALT_ON_RESET_PROCPU`"]
pub type RTC_CNTL_OCD_HALT_ON_RESET_PROCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_OCD_HALT_ON_RESET_PROCPU`"]
pub struct RTC_CNTL_OCD_HALT_ON_RESET_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_OCD_HALT_ON_RESET_PROCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_OCD_HALT_ON_RESET_APPCPU`"]
pub type RTC_CNTL_OCD_HALT_ON_RESET_APPCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_OCD_HALT_ON_RESET_APPCPU`"]
pub struct RTC_CNTL_OCD_HALT_ON_RESET_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_OCD_HALT_ON_RESET_APPCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_ALL_RESET_FLAG_CLR_APPCPU`"]
pub struct RTC_CNTL_ALL_RESET_FLAG_CLR_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ALL_RESET_FLAG_CLR_APPCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `RTC_CNTL_ALL_RESET_FLAG_CLR_PROCPU`"]
pub struct RTC_CNTL_ALL_RESET_FLAG_CLR_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ALL_RESET_FLAG_CLR_PROCPU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_ALL_RESET_FLAG_APPCPU`"]
pub type RTC_CNTL_ALL_RESET_FLAG_APPCPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_ALL_RESET_FLAG_PROCPU`"]
pub type RTC_CNTL_ALL_RESET_FLAG_PROCPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CNTL_STAT_VECTOR_SEL_PROCPU`"]
pub type RTC_CNTL_STAT_VECTOR_SEL_PROCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_STAT_VECTOR_SEL_PROCPU`"]
pub struct RTC_CNTL_STAT_VECTOR_SEL_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_STAT_VECTOR_SEL_PROCPU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_STAT_VECTOR_SEL_APPCPU`"]
pub type RTC_CNTL_STAT_VECTOR_SEL_APPCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_STAT_VECTOR_SEL_APPCPU`"]
pub struct RTC_CNTL_STAT_VECTOR_SEL_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_STAT_VECTOR_SEL_APPCPU_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_RESET_CAUSE_APPCPU`"]
pub type RTC_CNTL_RESET_CAUSE_APPCPU_R = crate::R<u8, u8>;
#[doc = "Reader of field `RTC_CNTL_RESET_CAUSE_PROCPU`"]
pub type RTC_CNTL_RESET_CAUSE_PROCPU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_dreset_mask_procpu(&self) -> RTC_CNTL_DRESET_MASK_PROCPU_R {
        RTC_CNTL_DRESET_MASK_PROCPU_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_dreset_mask_appcpu(&self) -> RTC_CNTL_DRESET_MASK_APPCPU_R {
        RTC_CNTL_DRESET_MASK_APPCPU_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtc_cntl_jtag_reset_flag_appcpu(&self) -> RTC_CNTL_JTAG_RESET_FLAG_APPCPU_R {
        RTC_CNTL_JTAG_RESET_FLAG_APPCPU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtc_cntl_jtag_reset_flag_procpu(&self) -> RTC_CNTL_JTAG_RESET_FLAG_PROCPU_R {
        RTC_CNTL_JTAG_RESET_FLAG_PROCPU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_ocd_halt_on_reset_procpu(&self) -> RTC_CNTL_OCD_HALT_ON_RESET_PROCPU_R {
        RTC_CNTL_OCD_HALT_ON_RESET_PROCPU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_ocd_halt_on_reset_appcpu(&self) -> RTC_CNTL_OCD_HALT_ON_RESET_APPCPU_R {
        RTC_CNTL_OCD_HALT_ON_RESET_APPCPU_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cntl_all_reset_flag_appcpu(&self) -> RTC_CNTL_ALL_RESET_FLAG_APPCPU_R {
        RTC_CNTL_ALL_RESET_FLAG_APPCPU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtc_cntl_all_reset_flag_procpu(&self) -> RTC_CNTL_ALL_RESET_FLAG_PROCPU_R {
        RTC_CNTL_ALL_RESET_FLAG_PROCPU_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_stat_vector_sel_procpu(&self) -> RTC_CNTL_STAT_VECTOR_SEL_PROCPU_R {
        RTC_CNTL_STAT_VECTOR_SEL_PROCPU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_stat_vector_sel_appcpu(&self) -> RTC_CNTL_STAT_VECTOR_SEL_APPCPU_R {
        RTC_CNTL_STAT_VECTOR_SEL_APPCPU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn rtc_cntl_reset_cause_appcpu(&self) -> RTC_CNTL_RESET_CAUSE_APPCPU_R {
        RTC_CNTL_RESET_CAUSE_APPCPU_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rtc_cntl_reset_cause_procpu(&self) -> RTC_CNTL_RESET_CAUSE_PROCPU_R {
        RTC_CNTL_RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_cntl_dreset_mask_procpu(&mut self) -> RTC_CNTL_DRESET_MASK_PROCPU_W {
        RTC_CNTL_DRESET_MASK_PROCPU_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_cntl_dreset_mask_appcpu(&mut self) -> RTC_CNTL_DRESET_MASK_APPCPU_W {
        RTC_CNTL_DRESET_MASK_APPCPU_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rtc_cntl_jtag_reset_flag_clr_appcpu(&mut self) -> RTC_CNTL_JTAG_RESET_FLAG_CLR_APPCPU_W {
        RTC_CNTL_JTAG_RESET_FLAG_CLR_APPCPU_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_cntl_jtag_reset_flag_clr_procpu(&mut self) -> RTC_CNTL_JTAG_RESET_FLAG_CLR_PROCPU_W {
        RTC_CNTL_JTAG_RESET_FLAG_CLR_PROCPU_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtc_cntl_ocd_halt_on_reset_procpu(&mut self) -> RTC_CNTL_OCD_HALT_ON_RESET_PROCPU_W {
        RTC_CNTL_OCD_HALT_ON_RESET_PROCPU_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_ocd_halt_on_reset_appcpu(&mut self) -> RTC_CNTL_OCD_HALT_ON_RESET_APPCPU_W {
        RTC_CNTL_OCD_HALT_ON_RESET_APPCPU_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtc_cntl_all_reset_flag_clr_appcpu(&mut self) -> RTC_CNTL_ALL_RESET_FLAG_CLR_APPCPU_W {
        RTC_CNTL_ALL_RESET_FLAG_CLR_APPCPU_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtc_cntl_all_reset_flag_clr_procpu(&mut self) -> RTC_CNTL_ALL_RESET_FLAG_CLR_PROCPU_W {
        RTC_CNTL_ALL_RESET_FLAG_CLR_PROCPU_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_cntl_stat_vector_sel_procpu(&mut self) -> RTC_CNTL_STAT_VECTOR_SEL_PROCPU_W {
        RTC_CNTL_STAT_VECTOR_SEL_PROCPU_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cntl_stat_vector_sel_appcpu(&mut self) -> RTC_CNTL_STAT_VECTOR_SEL_APPCPU_W {
        RTC_CNTL_STAT_VECTOR_SEL_APPCPU_W { w: self }
    }
}
