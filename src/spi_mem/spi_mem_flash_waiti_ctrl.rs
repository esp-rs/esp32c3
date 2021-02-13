#[doc = "Reader of register SPI_MEM_FLASH_WAITI_CTRL"]
pub type R = crate::R<u32, super::SPI_MEM_FLASH_WAITI_CTRL>;
#[doc = "Writer for register SPI_MEM_FLASH_WAITI_CTRL"]
pub type W = crate::W<u32, super::SPI_MEM_FLASH_WAITI_CTRL>;
#[doc = "Register SPI_MEM_FLASH_WAITI_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_FLASH_WAITI_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_WAITI_DUMMY_CYCLELEN`"]
pub type SPI_MEM_WAITI_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_WAITI_DUMMY_CYCLELEN`"]
pub struct SPI_MEM_WAITI_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WAITI_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_WAITI_CMD`"]
pub type SPI_MEM_WAITI_CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_WAITI_CMD`"]
pub struct SPI_MEM_WAITI_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WAITI_CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | (((value as u32) & 0xff) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_WAITI_DUMMY`"]
pub type SPI_MEM_WAITI_DUMMY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_WAITI_DUMMY`"]
pub struct SPI_MEM_WAITI_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WAITI_DUMMY_W<'a> {
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
impl R {
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn spi_mem_waiti_dummy_cyclelen(&self) -> SPI_MEM_WAITI_DUMMY_CYCLELEN_R {
        SPI_MEM_WAITI_DUMMY_CYCLELEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 2:9"]
    #[inline(always)]
    pub fn spi_mem_waiti_cmd(&self) -> SPI_MEM_WAITI_CMD_R {
        SPI_MEM_WAITI_CMD_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_waiti_dummy(&self) -> SPI_MEM_WAITI_DUMMY_R {
        SPI_MEM_WAITI_DUMMY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn spi_mem_waiti_dummy_cyclelen(&mut self) -> SPI_MEM_WAITI_DUMMY_CYCLELEN_W {
        SPI_MEM_WAITI_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 2:9"]
    #[inline(always)]
    pub fn spi_mem_waiti_cmd(&mut self) -> SPI_MEM_WAITI_CMD_W {
        SPI_MEM_WAITI_CMD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_waiti_dummy(&mut self) -> SPI_MEM_WAITI_DUMMY_W {
        SPI_MEM_WAITI_DUMMY_W { w: self }
    }
}
