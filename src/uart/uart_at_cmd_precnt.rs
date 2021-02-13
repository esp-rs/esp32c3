#[doc = "Reader of register UART_AT_CMD_PRECNT"]
pub type R = crate::R<u32, super::UART_AT_CMD_PRECNT>;
#[doc = "Writer for register UART_AT_CMD_PRECNT"]
pub type W = crate::W<u32, super::UART_AT_CMD_PRECNT>;
#[doc = "Register UART_AT_CMD_PRECNT `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_AT_CMD_PRECNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_PRE_IDLE_NUM`"]
pub type UART_PRE_IDLE_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_PRE_IDLE_NUM`"]
pub struct UART_PRE_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PRE_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn uart_pre_idle_num(&self) -> UART_PRE_IDLE_NUM_R {
        UART_PRE_IDLE_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn uart_pre_idle_num(&mut self) -> UART_PRE_IDLE_NUM_W {
        UART_PRE_IDLE_NUM_W { w: self }
    }
}
