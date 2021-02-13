#[doc = "Reader of register UART_ID"]
pub type R = crate::R<u32, super::UART_ID>;
#[doc = "Writer for register UART_ID"]
pub type W = crate::W<u32, super::UART_ID>;
#[doc = "Register UART_ID `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_ID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_UPDATE`"]
pub type UART_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_UPDATE`"]
pub struct UART_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_UPDATE_W<'a> {
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
#[doc = "Reader of field `UART_HIGH_SPEED`"]
pub type UART_HIGH_SPEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_HIGH_SPEED`"]
pub struct UART_HIGH_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_HIGH_SPEED_W<'a> {
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
#[doc = "Reader of field `UART_ID`"]
pub type UART_ID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UART_ID`"]
pub struct UART_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn uart_update(&self) -> UART_UPDATE_R {
        UART_UPDATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn uart_high_speed(&self) -> UART_HIGH_SPEED_R {
        UART_HIGH_SPEED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn uart_id(&self) -> UART_ID_R {
        UART_ID_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn uart_update(&mut self) -> UART_UPDATE_W {
        UART_UPDATE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn uart_high_speed(&mut self) -> UART_HIGH_SPEED_W {
        UART_HIGH_SPEED_W { w: self }
    }
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn uart_id(&mut self) -> UART_ID_W {
        UART_ID_W { w: self }
    }
}
