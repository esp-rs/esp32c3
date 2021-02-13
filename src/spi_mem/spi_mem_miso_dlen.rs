#[doc = "Reader of register SPI_MEM_MISO_DLEN"]
pub type R = crate::R<u32, super::SPI_MEM_MISO_DLEN>;
#[doc = "Writer for register SPI_MEM_MISO_DLEN"]
pub type W = crate::W<u32, super::SPI_MEM_MISO_DLEN>;
#[doc = "Register SPI_MEM_MISO_DLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_MISO_DLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_USR_MISO_DBITLEN`"]
pub type SPI_MEM_USR_MISO_DBITLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_MEM_USR_MISO_DBITLEN`"]
pub struct SPI_MEM_USR_MISO_DBITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_USR_MISO_DBITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn spi_mem_usr_miso_dbitlen(&self) -> SPI_MEM_USR_MISO_DBITLEN_R {
        SPI_MEM_USR_MISO_DBITLEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn spi_mem_usr_miso_dbitlen(&mut self) -> SPI_MEM_USR_MISO_DBITLEN_W {
        SPI_MEM_USR_MISO_DBITLEN_W { w: self }
    }
}
