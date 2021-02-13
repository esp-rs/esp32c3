#[doc = "Reader of register SPI_MEM_CTRL1"]
pub type R = crate::R<u32, super::SPI_MEM_CTRL1>;
#[doc = "Writer for register SPI_MEM_CTRL1"]
pub type W = crate::W<u32, super::SPI_MEM_CTRL1>;
#[doc = "Register SPI_MEM_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_RXFIFO_WFULL_ERR`"]
pub type SPI_MEM_RXFIFO_WFULL_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_RXFIFO_RST`"]
pub struct SPI_MEM_RXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_RXFIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_CS_HOLD_DLY_RES`"]
pub type SPI_MEM_CS_HOLD_DLY_RES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_MEM_CS_HOLD_DLY_RES`"]
pub struct SPI_MEM_CS_HOLD_DLY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CS_HOLD_DLY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | (((value as u32) & 0x03ff) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_CLK_MODE`"]
pub type SPI_MEM_CLK_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_CLK_MODE`"]
pub struct SPI_MEM_CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CLK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_mem_rxfifo_wfull_err(&self) -> SPI_MEM_RXFIFO_WFULL_ERR_R {
        SPI_MEM_RXFIFO_WFULL_ERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn spi_mem_cs_hold_dly_res(&self) -> SPI_MEM_CS_HOLD_DLY_RES_R {
        SPI_MEM_CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_mem_clk_mode(&self) -> SPI_MEM_CLK_MODE_R {
        SPI_MEM_CLK_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_mem_rxfifo_rst(&mut self) -> SPI_MEM_RXFIFO_RST_W {
        SPI_MEM_RXFIFO_RST_W { w: self }
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn spi_mem_cs_hold_dly_res(&mut self) -> SPI_MEM_CS_HOLD_DLY_RES_W {
        SPI_MEM_CS_HOLD_DLY_RES_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_mem_clk_mode(&mut self) -> SPI_MEM_CLK_MODE_W {
        SPI_MEM_CLK_MODE_W { w: self }
    }
}
