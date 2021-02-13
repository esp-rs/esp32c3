#[doc = "Reader of register UHCI_CONF0"]
pub type R = crate::R<u32, super::UHCI_CONF0>;
#[doc = "Writer for register UHCI_CONF0"]
pub type W = crate::W<u32, super::UHCI_CONF0>;
#[doc = "Register UHCI_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_UART_RX_BRK_EOF_EN`"]
pub type UHCI_UART_RX_BRK_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART_RX_BRK_EOF_EN`"]
pub struct UHCI_UART_RX_BRK_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART_RX_BRK_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `UHCI_CLK_EN`"]
pub type UHCI_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_CLK_EN`"]
pub struct UHCI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `UHCI_ENCODE_CRC_EN`"]
pub type UHCI_ENCODE_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_ENCODE_CRC_EN`"]
pub struct UHCI_ENCODE_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ENCODE_CRC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `UHCI_LEN_EOF_EN`"]
pub type UHCI_LEN_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_LEN_EOF_EN`"]
pub struct UHCI_LEN_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_LEN_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `UHCI_UART_IDLE_EOF_EN`"]
pub type UHCI_UART_IDLE_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART_IDLE_EOF_EN`"]
pub struct UHCI_UART_IDLE_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART_IDLE_EOF_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_CRC_REC_EN`"]
pub type UHCI_CRC_REC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_CRC_REC_EN`"]
pub struct UHCI_CRC_REC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_CRC_REC_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_HEAD_EN`"]
pub type UHCI_HEAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_HEAD_EN`"]
pub struct UHCI_HEAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_HEAD_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_SEPER_EN`"]
pub type UHCI_SEPER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_SEPER_EN`"]
pub struct UHCI_SEPER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEPER_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_UART1_CE`"]
pub type UHCI_UART1_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART1_CE`"]
pub struct UHCI_UART1_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART1_CE_W<'a> {
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
#[doc = "Reader of field `UHCI_UART0_CE`"]
pub type UHCI_UART0_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART0_CE`"]
pub struct UHCI_UART0_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART0_CE_W<'a> {
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
#[doc = "Reader of field `UHCI_RX_RST`"]
pub type UHCI_RX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_RX_RST`"]
pub struct UHCI_RX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_RX_RST_W<'a> {
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
#[doc = "Reader of field `UHCI_TX_RST`"]
pub type UHCI_TX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_RST`"]
pub struct UHCI_TX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_RST_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci_uart_rx_brk_eof_en(&self) -> UHCI_UART_RX_BRK_EOF_EN_R {
        UHCI_UART_RX_BRK_EOF_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uhci_clk_en(&self) -> UHCI_CLK_EN_R {
        UHCI_CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhci_encode_crc_en(&self) -> UHCI_ENCODE_CRC_EN_R {
        UHCI_ENCODE_CRC_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uhci_len_eof_en(&self) -> UHCI_LEN_EOF_EN_R {
        UHCI_LEN_EOF_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci_uart_idle_eof_en(&self) -> UHCI_UART_IDLE_EOF_EN_R {
        UHCI_UART_IDLE_EOF_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_crc_rec_en(&self) -> UHCI_CRC_REC_EN_R {
        UHCI_CRC_REC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhci_head_en(&self) -> UHCI_HEAD_EN_R {
        UHCI_HEAD_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_seper_en(&self) -> UHCI_SEPER_EN_R {
        UHCI_SEPER_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_uart1_ce(&self) -> UHCI_UART1_CE_R {
        UHCI_UART1_CE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_uart0_ce(&self) -> UHCI_UART0_CE_R {
        UHCI_UART0_CE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_rx_rst(&self) -> UHCI_RX_RST_R {
        UHCI_RX_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_tx_rst(&self) -> UHCI_TX_RST_R {
        UHCI_TX_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci_uart_rx_brk_eof_en(&mut self) -> UHCI_UART_RX_BRK_EOF_EN_W {
        UHCI_UART_RX_BRK_EOF_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uhci_clk_en(&mut self) -> UHCI_CLK_EN_W {
        UHCI_CLK_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhci_encode_crc_en(&mut self) -> UHCI_ENCODE_CRC_EN_W {
        UHCI_ENCODE_CRC_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uhci_len_eof_en(&mut self) -> UHCI_LEN_EOF_EN_W {
        UHCI_LEN_EOF_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci_uart_idle_eof_en(&mut self) -> UHCI_UART_IDLE_EOF_EN_W {
        UHCI_UART_IDLE_EOF_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_crc_rec_en(&mut self) -> UHCI_CRC_REC_EN_W {
        UHCI_CRC_REC_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhci_head_en(&mut self) -> UHCI_HEAD_EN_W {
        UHCI_HEAD_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_seper_en(&mut self) -> UHCI_SEPER_EN_W {
        UHCI_SEPER_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_uart1_ce(&mut self) -> UHCI_UART1_CE_W {
        UHCI_UART1_CE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_uart0_ce(&mut self) -> UHCI_UART0_CE_W {
        UHCI_UART0_CE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_rx_rst(&mut self) -> UHCI_RX_RST_W {
        UHCI_RX_RST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_tx_rst(&mut self) -> UHCI_TX_RST_W {
        UHCI_TX_RST_W { w: self }
    }
}
