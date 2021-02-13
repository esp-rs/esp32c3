#[doc = "Writer for register UHCI_INT_CLR"]
pub type W = crate::W<u32, super::UHCI_INT_CLR>;
#[doc = "Register UHCI_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `UHCI_APP_CTRL1_INT_CLR`"]
pub struct UHCI_APP_CTRL1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_APP_CTRL1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `UHCI_APP_CTRL0_INT_CLR`"]
pub struct UHCI_APP_CTRL0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_APP_CTRL0_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UHCI_OUTLINK_EOF_ERR_INT_CLR`"]
pub struct UHCI_OUTLINK_EOF_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTLINK_EOF_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UHCI_SEND_A_Q_INT_CLR`"]
pub struct UHCI_SEND_A_Q_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEND_A_Q_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UHCI_SEND_S_Q_INT_CLR`"]
pub struct UHCI_SEND_S_Q_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEND_S_Q_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UHCI_TX_HUNG_INT_CLR`"]
pub struct UHCI_TX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_HUNG_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UHCI_RX_HUNG_INT_CLR`"]
pub struct UHCI_RX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_HUNG_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UHCI_TX_START_INT_CLR`"]
pub struct UHCI_TX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_START_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UHCI_RX_START_INT_CLR`"]
pub struct UHCI_RX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_START_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci_app_ctrl1_int_clr(&mut self) -> UHCI_APP_CTRL1_INT_CLR_W {
        UHCI_APP_CTRL1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_app_ctrl0_int_clr(&mut self) -> UHCI_APP_CTRL0_INT_CLR_W {
        UHCI_APP_CTRL0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhci_outlink_eof_err_int_clr(&mut self) -> UHCI_OUTLINK_EOF_ERR_INT_CLR_W {
        UHCI_OUTLINK_EOF_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_send_a_q_int_clr(&mut self) -> UHCI_SEND_A_Q_INT_CLR_W {
        UHCI_SEND_A_Q_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhci_send_s_q_int_clr(&mut self) -> UHCI_SEND_S_Q_INT_CLR_W {
        UHCI_SEND_S_Q_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_tx_hung_int_clr(&mut self) -> UHCI_TX_HUNG_INT_CLR_W {
        UHCI_TX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_rx_hung_int_clr(&mut self) -> UHCI_RX_HUNG_INT_CLR_W {
        UHCI_RX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_tx_start_int_clr(&mut self) -> UHCI_TX_START_INT_CLR_W {
        UHCI_TX_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_rx_start_int_clr(&mut self) -> UHCI_RX_START_INT_CLR_W {
        UHCI_RX_START_INT_CLR_W { w: self }
    }
}
