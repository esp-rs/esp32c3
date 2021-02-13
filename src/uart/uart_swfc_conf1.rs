#[doc = "Reader of register UART_SWFC_CONF1"]
pub type R = crate::R<u32, super::UART_SWFC_CONF1>;
#[doc = "Writer for register UART_SWFC_CONF1"]
pub type W = crate::W<u32, super::UART_SWFC_CONF1>;
#[doc = "Register UART_SWFC_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_SWFC_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_XON_CHAR`"]
pub type UART_XON_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_XON_CHAR`"]
pub struct UART_XON_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_XON_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | (((value as u32) & 0xff) << 9);
        self.w
    }
}
#[doc = "Reader of field `UART_XON_THRESHOLD`"]
pub type UART_XON_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_XON_THRESHOLD`"]
pub struct UART_XON_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_XON_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn uart_xon_char(&self) -> UART_XON_CHAR_R {
        UART_XON_CHAR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn uart_xon_threshold(&self) -> UART_XON_THRESHOLD_R {
        UART_XON_THRESHOLD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn uart_xon_char(&mut self) -> UART_XON_CHAR_W {
        UART_XON_CHAR_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn uart_xon_threshold(&mut self) -> UART_XON_THRESHOLD_W {
        UART_XON_THRESHOLD_W { w: self }
    }
}
