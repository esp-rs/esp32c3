#[doc = "Reader of register RTC_CNTL_PWC"]
pub type R = crate::R<u32, super::RTC_CNTL_PWC>;
#[doc = "Writer for register RTC_CNTL_PWC"]
pub type W = crate::W<u32, super::RTC_CNTL_PWC>;
#[doc = "Register RTC_CNTL_PWC `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_PWC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_PAD_FORCE_HOLD`"]
pub type RTC_CNTL_PAD_FORCE_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_PAD_FORCE_HOLD`"]
pub struct RTC_CNTL_PAD_FORCE_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PAD_FORCE_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtc_cntl_pad_force_hold(&self) -> RTC_CNTL_PAD_FORCE_HOLD_R {
        RTC_CNTL_PAD_FORCE_HOLD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtc_cntl_pad_force_hold(&mut self) -> RTC_CNTL_PAD_FORCE_HOLD_W {
        RTC_CNTL_PAD_FORCE_HOLD_W { w: self }
    }
}
