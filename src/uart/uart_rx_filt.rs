#[doc = "Reader of register UART_RX_FILT"]
pub type R = crate::R<u32, super::UART_RX_FILT>;
#[doc = "Writer for register UART_RX_FILT"]
pub type W = crate::W<u32, super::UART_RX_FILT>;
#[doc = "Register UART_RX_FILT `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_RX_FILT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_GLITCH_FILT_EN`"]
pub type UART_GLITCH_FILT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_GLITCH_FILT_EN`"]
pub struct UART_GLITCH_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_GLITCH_FILT_EN_W<'a> {
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
#[doc = "Reader of field `UART_GLITCH_FILT`"]
pub type UART_GLITCH_FILT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_GLITCH_FILT`"]
pub struct UART_GLITCH_FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_GLITCH_FILT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart_glitch_filt_en(&self) -> UART_GLITCH_FILT_EN_R {
        UART_GLITCH_FILT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_glitch_filt(&self) -> UART_GLITCH_FILT_R {
        UART_GLITCH_FILT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart_glitch_filt_en(&mut self) -> UART_GLITCH_FILT_EN_W {
        UART_GLITCH_FILT_EN_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_glitch_filt(&mut self) -> UART_GLITCH_FILT_W {
        UART_GLITCH_FILT_W { w: self }
    }
}
