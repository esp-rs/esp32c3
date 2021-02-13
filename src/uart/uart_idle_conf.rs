#[doc = "Reader of register UART_IDLE_CONF"]
pub type R = crate::R<u32, super::UART_IDLE_CONF>;
#[doc = "Writer for register UART_IDLE_CONF"]
pub type W = crate::W<u32, super::UART_IDLE_CONF>;
#[doc = "Register UART_IDLE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_IDLE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_TX_IDLE_NUM`"]
pub type UART_TX_IDLE_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_TX_IDLE_NUM`"]
pub struct UART_TX_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `UART_RX_IDLE_THRHD`"]
pub type UART_RX_IDLE_THRHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_RX_IDLE_THRHD`"]
pub struct UART_RX_IDLE_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_IDLE_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn uart_tx_idle_num(&self) -> UART_TX_IDLE_NUM_R {
        UART_TX_IDLE_NUM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn uart_rx_idle_thrhd(&self) -> UART_RX_IDLE_THRHD_R {
        UART_RX_IDLE_THRHD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn uart_tx_idle_num(&mut self) -> UART_TX_IDLE_NUM_W {
        UART_TX_IDLE_NUM_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn uart_rx_idle_thrhd(&mut self) -> UART_RX_IDLE_THRHD_W {
        UART_RX_IDLE_THRHD_W { w: self }
    }
}
