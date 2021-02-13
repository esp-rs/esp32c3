#[doc = "Reader of register RTC_CNTL_USB_CONF"]
pub type R = crate::R<u32, super::RTC_CNTL_USB_CONF>;
#[doc = "Writer for register RTC_CNTL_USB_CONF"]
pub type W = crate::W<u32, super::RTC_CNTL_USB_CONF>;
#[doc = "Register RTC_CNTL_USB_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_USB_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_IO_MUX_RESET_DISABLE`"]
pub type RTC_CNTL_IO_MUX_RESET_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_IO_MUX_RESET_DISABLE`"]
pub struct RTC_CNTL_IO_MUX_RESET_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_IO_MUX_RESET_DISABLE_W<'a> {
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
impl R {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_io_mux_reset_disable(&self) -> RTC_CNTL_IO_MUX_RESET_DISABLE_R {
        RTC_CNTL_IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtc_cntl_io_mux_reset_disable(&mut self) -> RTC_CNTL_IO_MUX_RESET_DISABLE_W {
        RTC_CNTL_IO_MUX_RESET_DISABLE_W { w: self }
    }
}
