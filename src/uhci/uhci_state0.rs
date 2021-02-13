#[doc = "Reader of register UHCI_STATE0"]
pub type R = crate::R<u32, super::UHCI_STATE0>;
#[doc = "Reader of field `UHCI_DECODE_STATE`"]
pub type UHCI_DECODE_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `UHCI_RX_ERR_CAUSE`"]
pub type UHCI_RX_ERR_CAUSE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn uhci_decode_state(&self) -> UHCI_DECODE_STATE_R {
        UHCI_DECODE_STATE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhci_rx_err_cause(&self) -> UHCI_RX_ERR_CAUSE_R {
        UHCI_RX_ERR_CAUSE_R::new((self.bits & 0x07) as u8)
    }
}
