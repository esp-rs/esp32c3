#[doc = "Reader of register SPI_MEM_FLASH_SUS_CMD"]
pub type R = crate::R<u32, super::SPI_MEM_FLASH_SUS_CMD>;
#[doc = "Writer for register SPI_MEM_FLASH_SUS_CMD"]
pub type W = crate::W<u32, super::SPI_MEM_FLASH_SUS_CMD>;
#[doc = "Register SPI_MEM_FLASH_SUS_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_FLASH_SUS_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_WAIT_PESR_COMMAND`"]
pub type SPI_MEM_WAIT_PESR_COMMAND_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_MEM_WAIT_PESR_COMMAND`"]
pub struct SPI_MEM_WAIT_PESR_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WAIT_PESR_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_PES_COMMAND`"]
pub type SPI_MEM_FLASH_PES_COMMAND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PES_COMMAND`"]
pub struct SPI_MEM_FLASH_PES_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PES_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_PER_COMMAND`"]
pub type SPI_MEM_FLASH_PER_COMMAND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PER_COMMAND`"]
pub struct SPI_MEM_FLASH_PER_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PER_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn spi_mem_wait_pesr_command(&self) -> SPI_MEM_WAIT_PESR_COMMAND_R {
        SPI_MEM_WAIT_PESR_COMMAND_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn spi_mem_flash_pes_command(&self) -> SPI_MEM_FLASH_PES_COMMAND_R {
        SPI_MEM_FLASH_PES_COMMAND_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn spi_mem_flash_per_command(&self) -> SPI_MEM_FLASH_PER_COMMAND_R {
        SPI_MEM_FLASH_PER_COMMAND_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn spi_mem_wait_pesr_command(&mut self) -> SPI_MEM_WAIT_PESR_COMMAND_W {
        SPI_MEM_WAIT_PESR_COMMAND_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn spi_mem_flash_pes_command(&mut self) -> SPI_MEM_FLASH_PES_COMMAND_W {
        SPI_MEM_FLASH_PES_COMMAND_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn spi_mem_flash_per_command(&mut self) -> SPI_MEM_FLASH_PER_COMMAND_W {
        SPI_MEM_FLASH_PER_COMMAND_W { w: self }
    }
}
