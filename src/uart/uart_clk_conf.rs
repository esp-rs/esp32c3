#[doc = "Reader of register UART_CLK_CONF"]
pub type R = crate::R<u32, super::UART_CLK_CONF>;
#[doc = "Writer for register UART_CLK_CONF"]
pub type W = crate::W<u32, super::UART_CLK_CONF>;
#[doc = "Register UART_CLK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_CLK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_RX_RST_CORE`"]
pub type UART_RX_RST_CORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RX_RST_CORE`"]
pub struct UART_RX_RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_RST_CORE_W<'a> {
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
#[doc = "Reader of field `UART_TX_RST_CORE`"]
pub type UART_TX_RST_CORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_RST_CORE`"]
pub struct UART_TX_RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_RST_CORE_W<'a> {
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
#[doc = "Reader of field `UART_RX_SCLK_EN`"]
pub type UART_RX_SCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RX_SCLK_EN`"]
pub struct UART_RX_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_SCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `UART_TX_SCLK_EN`"]
pub type UART_TX_SCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_SCLK_EN`"]
pub struct UART_TX_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_SCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `UART_RST_CORE`"]
pub type UART_RST_CORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RST_CORE`"]
pub struct UART_RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RST_CORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `UART_SCLK_EN`"]
pub type UART_SCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SCLK_EN`"]
pub struct UART_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `UART_SCLK_SEL`"]
pub type UART_SCLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_SCLK_SEL`"]
pub struct UART_SCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SCLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `UART_SCLK_DIV_NUM`"]
pub type UART_SCLK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_SCLK_DIV_NUM`"]
pub struct UART_SCLK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SCLK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `UART_SCLK_DIV_A`"]
pub type UART_SCLK_DIV_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_SCLK_DIV_A`"]
pub struct UART_SCLK_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SCLK_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `UART_SCLK_DIV_B`"]
pub type UART_SCLK_DIV_B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_SCLK_DIV_B`"]
pub struct UART_SCLK_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SCLK_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn uart_rx_rst_core(&self) -> UART_RX_RST_CORE_R {
        UART_RX_RST_CORE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn uart_tx_rst_core(&self) -> UART_TX_RST_CORE_R {
        UART_TX_RST_CORE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn uart_rx_sclk_en(&self) -> UART_RX_SCLK_EN_R {
        UART_RX_SCLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_tx_sclk_en(&self) -> UART_TX_SCLK_EN_R {
        UART_TX_SCLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart_rst_core(&self) -> UART_RST_CORE_R {
        UART_RST_CORE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn uart_sclk_en(&self) -> UART_SCLK_EN_R {
        UART_SCLK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn uart_sclk_sel(&self) -> UART_SCLK_SEL_R {
        UART_SCLK_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn uart_sclk_div_num(&self) -> UART_SCLK_DIV_NUM_R {
        UART_SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn uart_sclk_div_a(&self) -> UART_SCLK_DIV_A_R {
        UART_SCLK_DIV_A_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn uart_sclk_div_b(&self) -> UART_SCLK_DIV_B_R {
        UART_SCLK_DIV_B_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn uart_rx_rst_core(&mut self) -> UART_RX_RST_CORE_W {
        UART_RX_RST_CORE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn uart_tx_rst_core(&mut self) -> UART_TX_RST_CORE_W {
        UART_TX_RST_CORE_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn uart_rx_sclk_en(&mut self) -> UART_RX_SCLK_EN_W {
        UART_RX_SCLK_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_tx_sclk_en(&mut self) -> UART_TX_SCLK_EN_W {
        UART_TX_SCLK_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart_rst_core(&mut self) -> UART_RST_CORE_W {
        UART_RST_CORE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn uart_sclk_en(&mut self) -> UART_SCLK_EN_W {
        UART_SCLK_EN_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn uart_sclk_sel(&mut self) -> UART_SCLK_SEL_W {
        UART_SCLK_SEL_W { w: self }
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn uart_sclk_div_num(&mut self) -> UART_SCLK_DIV_NUM_W {
        UART_SCLK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn uart_sclk_div_a(&mut self) -> UART_SCLK_DIV_A_W {
        UART_SCLK_DIV_A_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn uart_sclk_div_b(&mut self) -> UART_SCLK_DIV_B_W {
        UART_SCLK_DIV_B_W { w: self }
    }
}
