#[doc = "Reader of register UART_TXBRK_CONF"]
pub type R = crate::R<u32, super::UART_TXBRK_CONF>;
#[doc = "Writer for register UART_TXBRK_CONF"]
pub type W = crate::W<u32, super::UART_TXBRK_CONF>;
#[doc = "Register UART_TXBRK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_TXBRK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_TX_BRK_NUM`"]
pub type UART_TX_BRK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_TX_BRK_NUM`"]
pub struct UART_TX_BRK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_BRK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_tx_brk_num(&self) -> UART_TX_BRK_NUM_R {
        UART_TX_BRK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_tx_brk_num(&mut self) -> UART_TX_BRK_NUM_W {
        UART_TX_BRK_NUM_W { w: self }
    }
}
