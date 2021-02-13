#[doc = "Reader of register UHCI_ACK_NUM"]
pub type R = crate::R<u32, super::UHCI_ACK_NUM>;
#[doc = "Writer for register UHCI_ACK_NUM"]
pub type W = crate::W<u32, super::UHCI_ACK_NUM>;
#[doc = "Register UHCI_ACK_NUM `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_ACK_NUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `UHCI_ACK_NUM_LOAD`"]
pub struct UHCI_ACK_NUM_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ACK_NUM_LOAD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UHCI_ACK_NUM`"]
pub type UHCI_ACK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_ACK_NUM`"]
pub struct UHCI_ACK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ACK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhci_ack_num(&self) -> UHCI_ACK_NUM_R {
        UHCI_ACK_NUM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_ack_num_load(&mut self) -> UHCI_ACK_NUM_LOAD_W {
        UHCI_ACK_NUM_LOAD_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhci_ack_num(&mut self) -> UHCI_ACK_NUM_W {
        UHCI_ACK_NUM_W { w: self }
    }
}
