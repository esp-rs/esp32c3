#[doc = "Reader of register UART_INT_ST"]
pub type R = crate::R<u32, super::UART_INT_ST>;
#[doc = "Reader of field `UART_WAKEUP_INT_ST`"]
pub type UART_WAKEUP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_AT_CMD_CHAR_DET_INT_ST`"]
pub type UART_AT_CMD_CHAR_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_RS485_CLASH_INT_ST`"]
pub type UART_RS485_CLASH_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_RS485_FRM_ERR_INT_ST`"]
pub type UART_RS485_FRM_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_RS485_PARITY_ERR_INT_ST`"]
pub type UART_RS485_PARITY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_TX_DONE_INT_ST`"]
pub type UART_TX_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_TX_BRK_IDLE_DONE_INT_ST`"]
pub type UART_TX_BRK_IDLE_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_TX_BRK_DONE_INT_ST`"]
pub type UART_TX_BRK_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_GLITCH_DET_INT_ST`"]
pub type UART_GLITCH_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_SW_XOFF_INT_ST`"]
pub type UART_SW_XOFF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_SW_XON_INT_ST`"]
pub type UART_SW_XON_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_RXFIFO_TOUT_INT_ST`"]
pub type UART_RXFIFO_TOUT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_BRK_DET_INT_ST`"]
pub type UART_BRK_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_CTS_CHG_INT_ST`"]
pub type UART_CTS_CHG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_DSR_CHG_INT_ST`"]
pub type UART_DSR_CHG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_RXFIFO_OVF_INT_ST`"]
pub type UART_RXFIFO_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_FRM_ERR_INT_ST`"]
pub type UART_FRM_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_PARITY_ERR_INT_ST`"]
pub type UART_PARITY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_TXFIFO_EMPTY_INT_ST`"]
pub type UART_TXFIFO_EMPTY_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_RXFIFO_FULL_INT_ST`"]
pub type UART_RXFIFO_FULL_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uart_wakeup_int_st(&self) -> UART_WAKEUP_INT_ST_R {
        UART_WAKEUP_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart_at_cmd_char_det_int_st(&self) -> UART_AT_CMD_CHAR_DET_INT_ST_R {
        UART_AT_CMD_CHAR_DET_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart_rs485_clash_int_st(&self) -> UART_RS485_CLASH_INT_ST_R {
        UART_RS485_CLASH_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart_rs485_frm_err_int_st(&self) -> UART_RS485_FRM_ERR_INT_ST_R {
        UART_RS485_FRM_ERR_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uart_rs485_parity_err_int_st(&self) -> UART_RS485_PARITY_ERR_INT_ST_R {
        UART_RS485_PARITY_ERR_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn uart_tx_done_int_st(&self) -> UART_TX_DONE_INT_ST_R {
        UART_TX_DONE_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn uart_tx_brk_idle_done_int_st(&self) -> UART_TX_BRK_IDLE_DONE_INT_ST_R {
        UART_TX_BRK_IDLE_DONE_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uart_tx_brk_done_int_st(&self) -> UART_TX_BRK_DONE_INT_ST_R {
        UART_TX_BRK_DONE_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uart_glitch_det_int_st(&self) -> UART_GLITCH_DET_INT_ST_R {
        UART_GLITCH_DET_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uart_sw_xoff_int_st(&self) -> UART_SW_XOFF_INT_ST_R {
        UART_SW_XOFF_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uart_sw_xon_int_st(&self) -> UART_SW_XON_INT_ST_R {
        UART_SW_XON_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart_rxfifo_tout_int_st(&self) -> UART_RXFIFO_TOUT_INT_ST_R {
        UART_RXFIFO_TOUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uart_brk_det_int_st(&self) -> UART_BRK_DET_INT_ST_R {
        UART_BRK_DET_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uart_cts_chg_int_st(&self) -> UART_CTS_CHG_INT_ST_R {
        UART_CTS_CHG_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart_dsr_chg_int_st(&self) -> UART_DSR_CHG_INT_ST_R {
        UART_DSR_CHG_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_rxfifo_ovf_int_st(&self) -> UART_RXFIFO_OVF_INT_ST_R {
        UART_RXFIFO_OVF_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart_frm_err_int_st(&self) -> UART_FRM_ERR_INT_ST_R {
        UART_FRM_ERR_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_parity_err_int_st(&self) -> UART_PARITY_ERR_INT_ST_R {
        UART_PARITY_ERR_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uart_txfifo_empty_int_st(&self) -> UART_TXFIFO_EMPTY_INT_ST_R {
        UART_TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_rxfifo_full_int_st(&self) -> UART_RXFIFO_FULL_INT_ST_R {
        UART_RXFIFO_FULL_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
