#[doc = "Reader of register SPI_SLAVE1"]
pub type R = crate::R<u32, super::SPI_SLAVE1>;
#[doc = "Writer for register SPI_SLAVE1"]
pub type W = crate::W<u32, super::SPI_SLAVE1>;
#[doc = "Register SPI_SLAVE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_SLAVE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_SLV_LAST_ADDR`"]
pub type SPI_SLV_LAST_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_LAST_ADDR`"]
pub struct SPI_SLV_LAST_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_LAST_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_LAST_COMMAND`"]
pub type SPI_SLV_LAST_COMMAND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_LAST_COMMAND`"]
pub struct SPI_SLV_LAST_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_LAST_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 18)) | (((value as u32) & 0xff) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_DATA_BITLEN`"]
pub type SPI_SLV_DATA_BITLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_SLV_DATA_BITLEN`"]
pub struct SPI_SLV_DATA_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_DATA_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn spi_slv_last_addr(&self) -> SPI_SLV_LAST_ADDR_R {
        SPI_SLV_LAST_ADDR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 18:25"]
    #[inline(always)]
    pub fn spi_slv_last_command(&self) -> SPI_SLV_LAST_COMMAND_R {
        SPI_SLV_LAST_COMMAND_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn spi_slv_data_bitlen(&self) -> SPI_SLV_DATA_BITLEN_R {
        SPI_SLV_DATA_BITLEN_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn spi_slv_last_addr(&mut self) -> SPI_SLV_LAST_ADDR_W {
        SPI_SLV_LAST_ADDR_W { w: self }
    }
    #[doc = "Bits 18:25"]
    #[inline(always)]
    pub fn spi_slv_last_command(&mut self) -> SPI_SLV_LAST_COMMAND_W {
        SPI_SLV_LAST_COMMAND_W { w: self }
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn spi_slv_data_bitlen(&mut self) -> SPI_SLV_DATA_BITLEN_W {
        SPI_SLV_DATA_BITLEN_W { w: self }
    }
}
