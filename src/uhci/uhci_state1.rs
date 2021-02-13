#[doc = "Reader of register UHCI_STATE1"]
pub type R = crate::R<u32, super::UHCI_STATE1>;
#[doc = "Reader of field `UHCI_ENCODE_STATE`"]
pub type UHCI_ENCODE_STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhci_encode_state(&self) -> UHCI_ENCODE_STATE_R {
        UHCI_ENCODE_STATE_R::new((self.bits & 0x07) as u8)
    }
}
