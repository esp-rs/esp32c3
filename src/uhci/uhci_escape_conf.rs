#[doc = "Reader of register UHCI_ESCAPE_CONF"]
pub type R = crate::R<u32, super::UHCI_ESCAPE_CONF>;
#[doc = "Writer for register UHCI_ESCAPE_CONF"]
pub type W = crate::W<u32, super::UHCI_ESCAPE_CONF>;
#[doc = "Register UHCI_ESCAPE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_ESCAPE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_RX_13_ESC_EN`"]
pub type UHCI_RX_13_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_RX_13_ESC_EN`"]
pub struct UHCI_RX_13_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_13_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `UHCI_RX_11_ESC_EN`"]
pub type UHCI_RX_11_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_RX_11_ESC_EN`"]
pub struct UHCI_RX_11_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_11_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `UHCI_RX_DB_ESC_EN`"]
pub type UHCI_RX_DB_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_RX_DB_ESC_EN`"]
pub struct UHCI_RX_DB_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_DB_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UHCI_RX_C0_ESC_EN`"]
pub type UHCI_RX_C0_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_RX_C0_ESC_EN`"]
pub struct UHCI_RX_C0_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_C0_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `UHCI_TX_13_ESC_EN`"]
pub type UHCI_TX_13_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_13_ESC_EN`"]
pub struct UHCI_TX_13_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_13_ESC_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_TX_11_ESC_EN`"]
pub type UHCI_TX_11_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_11_ESC_EN`"]
pub struct UHCI_TX_11_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_11_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UHCI_TX_DB_ESC_EN`"]
pub type UHCI_TX_DB_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_DB_ESC_EN`"]
pub struct UHCI_TX_DB_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_DB_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `UHCI_TX_C0_ESC_EN`"]
pub type UHCI_TX_C0_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_C0_ESC_EN`"]
pub struct UHCI_TX_C0_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_C0_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_rx_13_esc_en(&self) -> UHCI_RX_13_ESC_EN_R {
        UHCI_RX_13_ESC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhci_rx_11_esc_en(&self) -> UHCI_RX_11_ESC_EN_R {
        UHCI_RX_11_ESC_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_rx_db_esc_en(&self) -> UHCI_RX_DB_ESC_EN_R {
        UHCI_RX_DB_ESC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhci_rx_c0_esc_en(&self) -> UHCI_RX_C0_ESC_EN_R {
        UHCI_RX_C0_ESC_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_tx_13_esc_en(&self) -> UHCI_TX_13_ESC_EN_R {
        UHCI_TX_13_ESC_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_tx_11_esc_en(&self) -> UHCI_TX_11_ESC_EN_R {
        UHCI_TX_11_ESC_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_tx_db_esc_en(&self) -> UHCI_TX_DB_ESC_EN_R {
        UHCI_TX_DB_ESC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_tx_c0_esc_en(&self) -> UHCI_TX_C0_ESC_EN_R {
        UHCI_TX_C0_ESC_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_rx_13_esc_en(&mut self) -> UHCI_RX_13_ESC_EN_W {
        UHCI_RX_13_ESC_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhci_rx_11_esc_en(&mut self) -> UHCI_RX_11_ESC_EN_W {
        UHCI_RX_11_ESC_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_rx_db_esc_en(&mut self) -> UHCI_RX_DB_ESC_EN_W {
        UHCI_RX_DB_ESC_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhci_rx_c0_esc_en(&mut self) -> UHCI_RX_C0_ESC_EN_W {
        UHCI_RX_C0_ESC_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_tx_13_esc_en(&mut self) -> UHCI_TX_13_ESC_EN_W {
        UHCI_TX_13_ESC_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_tx_11_esc_en(&mut self) -> UHCI_TX_11_ESC_EN_W {
        UHCI_TX_11_ESC_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_tx_db_esc_en(&mut self) -> UHCI_TX_DB_ESC_EN_W {
        UHCI_TX_DB_ESC_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_tx_c0_esc_en(&mut self) -> UHCI_TX_C0_ESC_EN_W {
        UHCI_TX_C0_ESC_EN_W { w: self }
    }
}
