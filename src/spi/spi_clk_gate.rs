#[doc = "Reader of register SPI_CLK_GATE"]
pub type R = crate::R<u32, super::SPI_CLK_GATE>;
#[doc = "Writer for register SPI_CLK_GATE"]
pub type W = crate::W<u32, super::SPI_CLK_GATE>;
#[doc = "Register SPI_CLK_GATE `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CLK_GATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MST_CLK_SEL`"]
pub type SPI_MST_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MST_CLK_SEL`"]
pub struct SPI_MST_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MST_CLK_SEL_W<'a> {
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
#[doc = "Reader of field `SPI_MST_CLK_ACTIVE`"]
pub type SPI_MST_CLK_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MST_CLK_ACTIVE`"]
pub struct SPI_MST_CLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MST_CLK_ACTIVE_W<'a> {
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
#[doc = "Reader of field `SPI_CLK_EN`"]
pub type SPI_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CLK_EN`"]
pub struct SPI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_EN_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mst_clk_sel(&self) -> SPI_MST_CLK_SEL_R {
        SPI_MST_CLK_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mst_clk_active(&self) -> SPI_MST_CLK_ACTIVE_R {
        SPI_MST_CLK_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mst_clk_sel(&mut self) -> SPI_MST_CLK_SEL_W {
        SPI_MST_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mst_clk_active(&mut self) -> SPI_MST_CLK_ACTIVE_W {
        SPI_MST_CLK_ACTIVE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W {
        SPI_CLK_EN_W { w: self }
    }
}
