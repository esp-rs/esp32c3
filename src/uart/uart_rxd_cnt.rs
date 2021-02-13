#[doc = "Reader of register UART_RXD_CNT"]
pub type R = crate::R<u32, super::UART_RXD_CNT>;
#[doc = "Reader of field `UART_RXD_EDGE_CNT`"]
pub type UART_RXD_EDGE_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn uart_rxd_edge_cnt(&self) -> UART_RXD_EDGE_CNT_R {
        UART_RXD_EDGE_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
