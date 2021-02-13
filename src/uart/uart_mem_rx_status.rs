#[doc = "Reader of register UART_MEM_RX_STATUS"]
pub type R = crate::R<u32, super::UART_MEM_RX_STATUS>;
#[doc = "Reader of field `UART_RX_WADDR`"]
pub type UART_RX_WADDR_R = crate::R<u16, u16>;
#[doc = "Reader of field `UART_APB_RX_RADDR`"]
pub type UART_APB_RX_RADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn uart_rx_waddr(&self) -> UART_RX_WADDR_R {
        UART_RX_WADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn uart_apb_rx_raddr(&self) -> UART_APB_RX_RADDR_R {
        UART_APB_RX_RADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
