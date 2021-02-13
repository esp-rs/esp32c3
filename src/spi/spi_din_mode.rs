#[doc = "Reader of register SPI_DIN_MODE"]
pub type R = crate::R<u32, super::SPI_DIN_MODE>;
#[doc = "Writer for register SPI_DIN_MODE"]
pub type W = crate::W<u32, super::SPI_DIN_MODE>;
#[doc = "Register SPI_DIN_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DIN_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_TIMING_HCLK_ACTIVE`"]
pub type SPI_TIMING_HCLK_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_TIMING_HCLK_ACTIVE`"]
pub struct SPI_TIMING_HCLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TIMING_HCLK_ACTIVE_W<'a> {
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
#[doc = "Reader of field `SPI_DIN3_MODE`"]
pub type SPI_DIN3_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_DIN3_MODE`"]
pub struct SPI_DIN3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DIN3_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_DIN2_MODE`"]
pub type SPI_DIN2_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_DIN2_MODE`"]
pub struct SPI_DIN2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DIN2_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI_DIN1_MODE`"]
pub type SPI_DIN1_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_DIN1_MODE`"]
pub struct SPI_DIN1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DIN1_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI_DIN0_MODE`"]
pub type SPI_DIN0_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_DIN0_MODE`"]
pub struct SPI_DIN0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DIN0_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_timing_hclk_active(&self) -> SPI_TIMING_HCLK_ACTIVE_R {
        SPI_TIMING_HCLK_ACTIVE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn spi_din3_mode(&self) -> SPI_DIN3_MODE_R {
        SPI_DIN3_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi_din2_mode(&self) -> SPI_DIN2_MODE_R {
        SPI_DIN2_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi_din1_mode(&self) -> SPI_DIN1_MODE_R {
        SPI_DIN1_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_din0_mode(&self) -> SPI_DIN0_MODE_R {
        SPI_DIN0_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_timing_hclk_active(&mut self) -> SPI_TIMING_HCLK_ACTIVE_W {
        SPI_TIMING_HCLK_ACTIVE_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn spi_din3_mode(&mut self) -> SPI_DIN3_MODE_W {
        SPI_DIN3_MODE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi_din2_mode(&mut self) -> SPI_DIN2_MODE_W {
        SPI_DIN2_MODE_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi_din1_mode(&mut self) -> SPI_DIN1_MODE_W {
        SPI_DIN1_MODE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_din0_mode(&mut self) -> SPI_DIN0_MODE_W {
        SPI_DIN0_MODE_W { w: self }
    }
}
