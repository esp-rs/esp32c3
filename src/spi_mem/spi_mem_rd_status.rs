#[doc = "Reader of register SPI_MEM_RD_STATUS"]
pub type R = crate::R<u32, super::SPI_MEM_RD_STATUS>;
#[doc = "Writer for register SPI_MEM_RD_STATUS"]
pub type W = crate::W<u32, super::SPI_MEM_RD_STATUS>;
#[doc = "Register SPI_MEM_RD_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_RD_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_WB_MODE`"]
pub type SPI_MEM_WB_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_WB_MODE`"]
pub struct SPI_MEM_WB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WB_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_STATUS`"]
pub type SPI_MEM_STATUS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_MEM_STATUS`"]
pub struct SPI_MEM_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn spi_mem_wb_mode(&self) -> SPI_MEM_WB_MODE_R {
        SPI_MEM_WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn spi_mem_status(&self) -> SPI_MEM_STATUS_R {
        SPI_MEM_STATUS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn spi_mem_wb_mode(&mut self) -> SPI_MEM_WB_MODE_W {
        SPI_MEM_WB_MODE_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn spi_mem_status(&mut self) -> SPI_MEM_STATUS_W {
        SPI_MEM_STATUS_W { w: self }
    }
}
