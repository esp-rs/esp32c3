#[doc = "Reader of register SPI_MEM_DATE"]
pub type R = crate::R<u32, super::SPI_MEM_DATE>;
#[doc = "Writer for register SPI_MEM_DATE"]
pub type W = crate::W<u32, super::SPI_MEM_DATE>;
#[doc = "Register SPI_MEM_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_DATE`"]
pub type SPI_MEM_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_MEM_DATE`"]
pub struct SPI_MEM_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn spi_mem_date(&self) -> SPI_MEM_DATE_R {
        SPI_MEM_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn spi_mem_date(&mut self) -> SPI_MEM_DATE_W {
        SPI_MEM_DATE_W { w: self }
    }
}
