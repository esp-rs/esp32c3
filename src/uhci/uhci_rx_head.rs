#[doc = "Reader of register UHCI_RX_HEAD"]
pub type R = crate::R<u32, super::UHCI_RX_HEAD>;
#[doc = "Reader of field `UHCI_RX_HEAD`"]
pub type UHCI_RX_HEAD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uhci_rx_head(&self) -> UHCI_RX_HEAD_R {
        UHCI_RX_HEAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
