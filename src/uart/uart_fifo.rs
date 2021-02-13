#[doc = "Reader of register UART_FIFO"]
pub type R = crate::R<u32, super::UART_FIFO>;
#[doc = "Reader of field `UART_RXFIFO_RD_BYTE`"]
pub type UART_RXFIFO_RD_BYTE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_rxfifo_rd_byte(&self) -> UART_RXFIFO_RD_BYTE_R {
        UART_RXFIFO_RD_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
