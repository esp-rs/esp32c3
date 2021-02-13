#[doc = "Reader of register UART_STATUS"]
pub type R = crate::R<u32, super::UART_STATUS>;
#[doc = "Reader of field `UART_TXD`"]
pub type UART_TXD_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_RTSN`"]
pub type UART_RTSN_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_DTRN`"]
pub type UART_DTRN_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_TXFIFO_CNT`"]
pub type UART_TXFIFO_CNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `UART_RXD`"]
pub type UART_RXD_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_CTSN`"]
pub type UART_CTSN_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_DSRN`"]
pub type UART_DSRN_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_RXFIFO_CNT`"]
pub type UART_RXFIFO_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn uart_txd(&self) -> UART_TXD_R {
        UART_TXD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn uart_rtsn(&self) -> UART_RTSN_R {
        UART_RTSN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn uart_dtrn(&self) -> UART_DTRN_R {
        UART_DTRN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn uart_txfifo_cnt(&self) -> UART_TXFIFO_CNT_R {
        UART_TXFIFO_CNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uart_rxd(&self) -> UART_RXD_R {
        UART_RXD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn uart_ctsn(&self) -> UART_CTSN_R {
        UART_CTSN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn uart_dsrn(&self) -> UART_DSRN_R {
        UART_DSRN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn uart_rxfifo_cnt(&self) -> UART_RXFIFO_CNT_R {
        UART_RXFIFO_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
