#[doc = "Reader of register UART_CONF1"]
pub type R = crate::R<u32, super::UART_CONF1>;
#[doc = "Writer for register UART_CONF1"]
pub type W = crate::W<u32, super::UART_CONF1>;
#[doc = "Register UART_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_RX_TOUT_EN`"]
pub type UART_RX_TOUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RX_TOUT_EN`"]
pub struct UART_RX_TOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_TOUT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `UART_RX_FLOW_EN`"]
pub type UART_RX_FLOW_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RX_FLOW_EN`"]
pub struct UART_RX_FLOW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_FLOW_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `UART_RX_TOUT_FLOW_DIS`"]
pub type UART_RX_TOUT_FLOW_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RX_TOUT_FLOW_DIS`"]
pub struct UART_RX_TOUT_FLOW_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_TOUT_FLOW_DIS_W<'a> {
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
#[doc = "Reader of field `UART_DIS_RX_DAT_OVF`"]
pub type UART_DIS_RX_DAT_OVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DIS_RX_DAT_OVF`"]
pub struct UART_DIS_RX_DAT_OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DIS_RX_DAT_OVF_W<'a> {
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
#[doc = "Reader of field `UART_TXFIFO_EMPTY_THRHD`"]
pub type UART_TXFIFO_EMPTY_THRHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_TXFIFO_EMPTY_THRHD`"]
pub struct UART_TXFIFO_EMPTY_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXFIFO_EMPTY_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | (((value as u32) & 0x01ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `UART_RXFIFO_FULL_THRHD`"]
pub type UART_RXFIFO_FULL_THRHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_RXFIFO_FULL_THRHD`"]
pub struct UART_RXFIFO_FULL_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_FULL_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn uart_rx_tout_en(&self) -> UART_RX_TOUT_EN_R {
        UART_RX_TOUT_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uart_rx_flow_en(&self) -> UART_RX_FLOW_EN_R {
        UART_RX_FLOW_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uart_rx_tout_flow_dis(&self) -> UART_RX_TOUT_FLOW_DIS_R {
        UART_RX_TOUT_FLOW_DIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart_dis_rx_dat_ovf(&self) -> UART_DIS_RX_DAT_OVF_R {
        UART_DIS_RX_DAT_OVF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn uart_txfifo_empty_thrhd(&self) -> UART_TXFIFO_EMPTY_THRHD_R {
        UART_TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn uart_rxfifo_full_thrhd(&self) -> UART_RXFIFO_FULL_THRHD_R {
        UART_RXFIFO_FULL_THRHD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn uart_rx_tout_en(&mut self) -> UART_RX_TOUT_EN_W {
        UART_RX_TOUT_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uart_rx_flow_en(&mut self) -> UART_RX_FLOW_EN_W {
        UART_RX_FLOW_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uart_rx_tout_flow_dis(&mut self) -> UART_RX_TOUT_FLOW_DIS_W {
        UART_RX_TOUT_FLOW_DIS_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart_dis_rx_dat_ovf(&mut self) -> UART_DIS_RX_DAT_OVF_W {
        UART_DIS_RX_DAT_OVF_W { w: self }
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn uart_txfifo_empty_thrhd(&mut self) -> UART_TXFIFO_EMPTY_THRHD_W {
        UART_TXFIFO_EMPTY_THRHD_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn uart_rxfifo_full_thrhd(&mut self) -> UART_RXFIFO_FULL_THRHD_W {
        UART_RXFIFO_FULL_THRHD_W { w: self }
    }
}
