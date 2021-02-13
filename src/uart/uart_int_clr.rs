#[doc = "Writer for register UART_INT_CLR"]
pub type W = crate::W<u32, super::UART_INT_CLR>;
#[doc = "Register UART_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `UART_WAKEUP_INT_CLR`"]
pub struct UART_WAKEUP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_WAKEUP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `UART_AT_CMD_CHAR_DET_INT_CLR`"]
pub struct UART_AT_CMD_CHAR_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_AT_CMD_CHAR_DET_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `UART_RS485_CLASH_INT_CLR`"]
pub struct UART_RS485_CLASH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_CLASH_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `UART_RS485_FRM_ERR_INT_CLR`"]
pub struct UART_RS485_FRM_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_FRM_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `UART_RS485_PARITY_ERR_INT_CLR`"]
pub struct UART_RS485_PARITY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RS485_PARITY_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `UART_TX_DONE_INT_CLR`"]
pub struct UART_TX_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `UART_TX_BRK_IDLE_DONE_INT_CLR`"]
pub struct UART_TX_BRK_IDLE_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_BRK_IDLE_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `UART_TX_BRK_DONE_INT_CLR`"]
pub struct UART_TX_BRK_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_BRK_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_GLITCH_DET_INT_CLR`"]
pub struct UART_GLITCH_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_GLITCH_DET_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_SW_XOFF_INT_CLR`"]
pub struct UART_SW_XOFF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_XOFF_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_SW_XON_INT_CLR`"]
pub struct UART_SW_XON_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_XON_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_RXFIFO_TOUT_INT_CLR`"]
pub struct UART_RXFIFO_TOUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_TOUT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_BRK_DET_INT_CLR`"]
pub struct UART_BRK_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_BRK_DET_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_CTS_CHG_INT_CLR`"]
pub struct UART_CTS_CHG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTS_CHG_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_DSR_CHG_INT_CLR`"]
pub struct UART_DSR_CHG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DSR_CHG_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_RXFIFO_OVF_INT_CLR`"]
pub struct UART_RXFIFO_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_OVF_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_FRM_ERR_INT_CLR`"]
pub struct UART_FRM_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FRM_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_PARITY_ERR_INT_CLR`"]
pub struct UART_PARITY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PARITY_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_TXFIFO_EMPTY_INT_CLR`"]
pub struct UART_TXFIFO_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXFIFO_EMPTY_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `UART_RXFIFO_FULL_INT_CLR`"]
pub struct UART_RXFIFO_FULL_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_FULL_INT_CLR_W<'a> {
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
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uart_wakeup_int_clr(&mut self) -> UART_WAKEUP_INT_CLR_W {
        UART_WAKEUP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart_at_cmd_char_det_int_clr(&mut self) -> UART_AT_CMD_CHAR_DET_INT_CLR_W {
        UART_AT_CMD_CHAR_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart_rs485_clash_int_clr(&mut self) -> UART_RS485_CLASH_INT_CLR_W {
        UART_RS485_CLASH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart_rs485_frm_err_int_clr(&mut self) -> UART_RS485_FRM_ERR_INT_CLR_W {
        UART_RS485_FRM_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uart_rs485_parity_err_int_clr(&mut self) -> UART_RS485_PARITY_ERR_INT_CLR_W {
        UART_RS485_PARITY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn uart_tx_done_int_clr(&mut self) -> UART_TX_DONE_INT_CLR_W {
        UART_TX_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn uart_tx_brk_idle_done_int_clr(&mut self) -> UART_TX_BRK_IDLE_DONE_INT_CLR_W {
        UART_TX_BRK_IDLE_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uart_tx_brk_done_int_clr(&mut self) -> UART_TX_BRK_DONE_INT_CLR_W {
        UART_TX_BRK_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uart_glitch_det_int_clr(&mut self) -> UART_GLITCH_DET_INT_CLR_W {
        UART_GLITCH_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uart_sw_xoff_int_clr(&mut self) -> UART_SW_XOFF_INT_CLR_W {
        UART_SW_XOFF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uart_sw_xon_int_clr(&mut self) -> UART_SW_XON_INT_CLR_W {
        UART_SW_XON_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart_rxfifo_tout_int_clr(&mut self) -> UART_RXFIFO_TOUT_INT_CLR_W {
        UART_RXFIFO_TOUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uart_brk_det_int_clr(&mut self) -> UART_BRK_DET_INT_CLR_W {
        UART_BRK_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uart_cts_chg_int_clr(&mut self) -> UART_CTS_CHG_INT_CLR_W {
        UART_CTS_CHG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart_dsr_chg_int_clr(&mut self) -> UART_DSR_CHG_INT_CLR_W {
        UART_DSR_CHG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_rxfifo_ovf_int_clr(&mut self) -> UART_RXFIFO_OVF_INT_CLR_W {
        UART_RXFIFO_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart_frm_err_int_clr(&mut self) -> UART_FRM_ERR_INT_CLR_W {
        UART_FRM_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_parity_err_int_clr(&mut self) -> UART_PARITY_ERR_INT_CLR_W {
        UART_PARITY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uart_txfifo_empty_int_clr(&mut self) -> UART_TXFIFO_EMPTY_INT_CLR_W {
        UART_TXFIFO_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_rxfifo_full_int_clr(&mut self) -> UART_RXFIFO_FULL_INT_CLR_W {
        UART_RXFIFO_FULL_INT_CLR_W { w: self }
    }
}
