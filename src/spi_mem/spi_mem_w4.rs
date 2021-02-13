#[doc = "Reader of register SPI_MEM_W4"]
pub type R = crate::R<u32, super::SPI_MEM_W4>;
#[doc = "Writer for register SPI_MEM_W4"]
pub type W = crate::W<u32, super::SPI_MEM_W4>;
#[doc = "Register SPI_MEM_W4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_W4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_BUF4`"]
pub type SPI_MEM_BUF4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_MEM_BUF4`"]
pub struct SPI_MEM_BUF4_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_BUF4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_mem_buf4(&self) -> SPI_MEM_BUF4_R {
        SPI_MEM_BUF4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_mem_buf4(&mut self) -> SPI_MEM_BUF4_W {
        SPI_MEM_BUF4_W { w: self }
    }
}
