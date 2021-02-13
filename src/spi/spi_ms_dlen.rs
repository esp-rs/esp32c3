#[doc = "Reader of register SPI_MS_DLEN"]
pub type R = crate::R<u32, super::SPI_MS_DLEN>;
#[doc = "Writer for register SPI_MS_DLEN"]
pub type W = crate::W<u32, super::SPI_MS_DLEN>;
#[doc = "Register SPI_MS_DLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MS_DLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MS_DATA_BITLEN`"]
pub type SPI_MS_DATA_BITLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_MS_DATA_BITLEN`"]
pub struct SPI_MS_DATA_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MS_DATA_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn spi_ms_data_bitlen(&self) -> SPI_MS_DATA_BITLEN_R {
        SPI_MS_DATA_BITLEN_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn spi_ms_data_bitlen(&mut self) -> SPI_MS_DATA_BITLEN_W {
        SPI_MS_DATA_BITLEN_W { w: self }
    }
}
