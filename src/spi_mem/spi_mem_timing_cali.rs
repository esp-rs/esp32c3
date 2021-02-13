#[doc = "Reader of register SPI_MEM_TIMING_CALI"]
pub type R = crate::R<u32, super::SPI_MEM_TIMING_CALI>;
#[doc = "Writer for register SPI_MEM_TIMING_CALI"]
pub type W = crate::W<u32, super::SPI_MEM_TIMING_CALI>;
#[doc = "Register SPI_MEM_TIMING_CALI `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_TIMING_CALI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_EXTRA_DUMMY_CYCLELEN`"]
pub type SPI_MEM_EXTRA_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_EXTRA_DUMMY_CYCLELEN`"]
pub struct SPI_MEM_EXTRA_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_EXTRA_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_TIMING_CALI`"]
pub type SPI_MEM_TIMING_CALI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_TIMING_CALI`"]
pub struct SPI_MEM_TIMING_CALI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_TIMING_CALI_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_TIMING_CLK_ENA`"]
pub type SPI_MEM_TIMING_CLK_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_TIMING_CLK_ENA`"]
pub struct SPI_MEM_TIMING_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_TIMING_CLK_ENA_W<'a> {
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
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn spi_mem_extra_dummy_cyclelen(&self) -> SPI_MEM_EXTRA_DUMMY_CYCLELEN_R {
        SPI_MEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_timing_cali(&self) -> SPI_MEM_TIMING_CALI_R {
        SPI_MEM_TIMING_CALI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_timing_clk_ena(&self) -> SPI_MEM_TIMING_CLK_ENA_R {
        SPI_MEM_TIMING_CLK_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn spi_mem_extra_dummy_cyclelen(&mut self) -> SPI_MEM_EXTRA_DUMMY_CYCLELEN_W {
        SPI_MEM_EXTRA_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_timing_cali(&mut self) -> SPI_MEM_TIMING_CALI_W {
        SPI_MEM_TIMING_CALI_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_timing_clk_ena(&mut self) -> SPI_MEM_TIMING_CLK_ENA_W {
        SPI_MEM_TIMING_CLK_ENA_W { w: self }
    }
}
