#[doc = "Reader of register UHCI_INT_ST"]
pub type R = crate::R<u32, super::UHCI_INT_ST>;
#[doc = "Reader of field `UHCI_APP_CTRL1_INT_ST`"]
pub type UHCI_APP_CTRL1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHCI_APP_CTRL0_INT_ST`"]
pub type UHCI_APP_CTRL0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHCI_OUTLINK_EOF_ERR_INT_ST`"]
pub type UHCI_OUTLINK_EOF_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHCI_SEND_A_Q_INT_ST`"]
pub type UHCI_SEND_A_Q_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHCI_SEND_S_Q_INT_ST`"]
pub type UHCI_SEND_S_Q_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHCI_TX_HUNG_INT_ST`"]
pub type UHCI_TX_HUNG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHCI_RX_HUNG_INT_ST`"]
pub type UHCI_RX_HUNG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHCI_TX_START_INT_ST`"]
pub type UHCI_TX_START_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHCI_RX_START_INT_ST`"]
pub type UHCI_RX_START_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci_app_ctrl1_int_st(&self) -> UHCI_APP_CTRL1_INT_ST_R {
        UHCI_APP_CTRL1_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_app_ctrl0_int_st(&self) -> UHCI_APP_CTRL0_INT_ST_R {
        UHCI_APP_CTRL0_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhci_outlink_eof_err_int_st(&self) -> UHCI_OUTLINK_EOF_ERR_INT_ST_R {
        UHCI_OUTLINK_EOF_ERR_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_send_a_q_int_st(&self) -> UHCI_SEND_A_Q_INT_ST_R {
        UHCI_SEND_A_Q_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhci_send_s_q_int_st(&self) -> UHCI_SEND_S_Q_INT_ST_R {
        UHCI_SEND_S_Q_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_tx_hung_int_st(&self) -> UHCI_TX_HUNG_INT_ST_R {
        UHCI_TX_HUNG_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_rx_hung_int_st(&self) -> UHCI_RX_HUNG_INT_ST_R {
        UHCI_RX_HUNG_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_tx_start_int_st(&self) -> UHCI_TX_START_INT_ST_R {
        UHCI_TX_START_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_rx_start_int_st(&self) -> UHCI_RX_START_INT_ST_R {
        UHCI_RX_START_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
