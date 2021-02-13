#[doc = "Reader of register UART_MEM_CONF"]
pub type R = crate::R<u32, super::UART_MEM_CONF>;
#[doc = "Writer for register UART_MEM_CONF"]
pub type W = crate::W<u32, super::UART_MEM_CONF>;
#[doc = "Register UART_MEM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_MEM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_MEM_FORCE_PU`"]
pub type UART_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MEM_FORCE_PU`"]
pub struct UART_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `UART_MEM_FORCE_PD`"]
pub type UART_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MEM_FORCE_PD`"]
pub struct UART_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `UART_RX_TOUT_THRHD`"]
pub type UART_RX_TOUT_THRHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_RX_TOUT_THRHD`"]
pub struct UART_RX_TOUT_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_TOUT_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `UART_RX_FLOW_THRHD`"]
pub type UART_RX_FLOW_THRHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_RX_FLOW_THRHD`"]
pub struct UART_RX_FLOW_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_FLOW_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | (((value as u32) & 0x01ff) << 7);
        self.w
    }
}
#[doc = "Reader of field `UART_TX_SIZE`"]
pub type UART_TX_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_TX_SIZE`"]
pub struct UART_TX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `UART_RX_SIZE`"]
pub type UART_RX_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_RX_SIZE`"]
pub struct UART_RX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn uart_mem_force_pu(&self) -> UART_MEM_FORCE_PU_R {
        UART_MEM_FORCE_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn uart_mem_force_pd(&self) -> UART_MEM_FORCE_PD_R {
        UART_MEM_FORCE_PD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn uart_rx_tout_thrhd(&self) -> UART_RX_TOUT_THRHD_R {
        UART_RX_TOUT_THRHD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn uart_rx_flow_thrhd(&self) -> UART_RX_FLOW_THRHD_R {
        UART_RX_FLOW_THRHD_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn uart_tx_size(&self) -> UART_TX_SIZE_R {
        UART_TX_SIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn uart_rx_size(&self) -> UART_RX_SIZE_R {
        UART_RX_SIZE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn uart_mem_force_pu(&mut self) -> UART_MEM_FORCE_PU_W {
        UART_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn uart_mem_force_pd(&mut self) -> UART_MEM_FORCE_PD_W {
        UART_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn uart_rx_tout_thrhd(&mut self) -> UART_RX_TOUT_THRHD_W {
        UART_RX_TOUT_THRHD_W { w: self }
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn uart_rx_flow_thrhd(&mut self) -> UART_RX_FLOW_THRHD_W {
        UART_RX_FLOW_THRHD_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn uart_tx_size(&mut self) -> UART_TX_SIZE_W {
        UART_TX_SIZE_W { w: self }
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn uart_rx_size(&mut self) -> UART_RX_SIZE_W {
        UART_RX_SIZE_W { w: self }
    }
}
