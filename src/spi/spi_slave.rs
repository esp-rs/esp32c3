#[doc = "Reader of register SPI_SLAVE"]
pub type R = crate::R<u32, super::SPI_SLAVE>;
#[doc = "Writer for register SPI_SLAVE"]
pub type W = crate::W<u32, super::SPI_SLAVE>;
#[doc = "Register SPI_SLAVE `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_SLAVE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_USR_CONF`"]
pub type SPI_USR_CONF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_CONF`"]
pub struct SPI_USR_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_CONF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `SPI_SOFT_RESET`"]
pub struct SPI_SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SOFT_RESET_W<'a> {
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
#[doc = "Reader of field `SPI_SLAVE_MODE`"]
pub type SPI_SLAVE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLAVE_MODE`"]
pub struct SPI_SLAVE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLAVE_MODE_W<'a> {
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
#[doc = "Reader of field `SPI_DMA_SEG_MAGIC_VALUE`"]
pub type SPI_DMA_SEG_MAGIC_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_DMA_SEG_MAGIC_VALUE`"]
pub struct SPI_DMA_SEG_MAGIC_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_SEG_MAGIC_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_WRBUF_BITLEN_EN`"]
pub type SPI_SLV_WRBUF_BITLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_WRBUF_BITLEN_EN`"]
pub struct SPI_SLV_WRBUF_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WRBUF_BITLEN_EN_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_RDBUF_BITLEN_EN`"]
pub type SPI_SLV_RDBUF_BITLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_RDBUF_BITLEN_EN`"]
pub struct SPI_SLV_RDBUF_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDBUF_BITLEN_EN_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_WRDMA_BITLEN_EN`"]
pub type SPI_SLV_WRDMA_BITLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_WRDMA_BITLEN_EN`"]
pub struct SPI_SLV_WRDMA_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WRDMA_BITLEN_EN_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_RDDMA_BITLEN_EN`"]
pub type SPI_SLV_RDDMA_BITLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_RDDMA_BITLEN_EN`"]
pub struct SPI_SLV_RDDMA_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDDMA_BITLEN_EN_W<'a> {
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
#[doc = "Reader of field `SPI_RSCK_DATA_OUT`"]
pub type SPI_RSCK_DATA_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RSCK_DATA_OUT`"]
pub struct SPI_RSCK_DATA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RSCK_DATA_OUT_W<'a> {
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
#[doc = "Reader of field `SPI_CLK_MODE_13`"]
pub type SPI_CLK_MODE_13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CLK_MODE_13`"]
pub struct SPI_CLK_MODE_13_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_MODE_13_W<'a> {
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
#[doc = "Reader of field `SPI_CLK_MODE`"]
pub type SPI_CLK_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CLK_MODE`"]
pub struct SPI_CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_usr_conf(&self) -> SPI_USR_CONF_R {
        SPI_USR_CONF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_slave_mode(&self) -> SPI_SLAVE_MODE_R {
        SPI_SLAVE_MODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn spi_dma_seg_magic_value(&self) -> SPI_DMA_SEG_MAGIC_VALUE_R {
        SPI_DMA_SEG_MAGIC_VALUE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spi_slv_wrbuf_bitlen_en(&self) -> SPI_SLV_WRBUF_BITLEN_EN_R {
        SPI_SLV_WRBUF_BITLEN_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_slv_rdbuf_bitlen_en(&self) -> SPI_SLV_RDBUF_BITLEN_EN_R {
        SPI_SLV_RDBUF_BITLEN_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_slv_wrdma_bitlen_en(&self) -> SPI_SLV_WRDMA_BITLEN_EN_R {
        SPI_SLV_WRDMA_BITLEN_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_slv_rddma_bitlen_en(&self) -> SPI_SLV_RDDMA_BITLEN_EN_R {
        SPI_SLV_RDDMA_BITLEN_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_rsck_data_out(&self) -> SPI_RSCK_DATA_OUT_R {
        SPI_RSCK_DATA_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_clk_mode_13(&self) -> SPI_CLK_MODE_13_R {
        SPI_CLK_MODE_13_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_clk_mode(&self) -> SPI_CLK_MODE_R {
        SPI_CLK_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_usr_conf(&mut self) -> SPI_USR_CONF_W {
        SPI_USR_CONF_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_soft_reset(&mut self) -> SPI_SOFT_RESET_W {
        SPI_SOFT_RESET_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_slave_mode(&mut self) -> SPI_SLAVE_MODE_W {
        SPI_SLAVE_MODE_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn spi_dma_seg_magic_value(&mut self) -> SPI_DMA_SEG_MAGIC_VALUE_W {
        SPI_DMA_SEG_MAGIC_VALUE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spi_slv_wrbuf_bitlen_en(&mut self) -> SPI_SLV_WRBUF_BITLEN_EN_W {
        SPI_SLV_WRBUF_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_slv_rdbuf_bitlen_en(&mut self) -> SPI_SLV_RDBUF_BITLEN_EN_W {
        SPI_SLV_RDBUF_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_slv_wrdma_bitlen_en(&mut self) -> SPI_SLV_WRDMA_BITLEN_EN_W {
        SPI_SLV_WRDMA_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_slv_rddma_bitlen_en(&mut self) -> SPI_SLV_RDDMA_BITLEN_EN_W {
        SPI_SLV_RDDMA_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_rsck_data_out(&mut self) -> SPI_RSCK_DATA_OUT_W {
        SPI_RSCK_DATA_OUT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_clk_mode_13(&mut self) -> SPI_CLK_MODE_13_W {
        SPI_CLK_MODE_13_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_clk_mode(&mut self) -> SPI_CLK_MODE_W {
        SPI_CLK_MODE_W { w: self }
    }
}
