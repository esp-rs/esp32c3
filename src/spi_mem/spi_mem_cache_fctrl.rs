#[doc = "Reader of register SPI_MEM_CACHE_FCTRL"]
pub type R = crate::R<u32, super::SPI_MEM_CACHE_FCTRL>;
#[doc = "Writer for register SPI_MEM_CACHE_FCTRL"]
pub type W = crate::W<u32, super::SPI_MEM_CACHE_FCTRL>;
#[doc = "Register SPI_MEM_CACHE_FCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_CACHE_FCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_FADDR_QUAD`"]
pub type SPI_MEM_FADDR_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FADDR_QUAD`"]
pub struct SPI_MEM_FADDR_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FADDR_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FDOUT_QUAD`"]
pub type SPI_MEM_FDOUT_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FDOUT_QUAD`"]
pub struct SPI_MEM_FDOUT_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FDOUT_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FDIN_QUAD`"]
pub type SPI_MEM_FDIN_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FDIN_QUAD`"]
pub struct SPI_MEM_FDIN_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FDIN_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FADDR_DUAL`"]
pub type SPI_MEM_FADDR_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FADDR_DUAL`"]
pub struct SPI_MEM_FADDR_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FADDR_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FDOUT_DUAL`"]
pub type SPI_MEM_FDOUT_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FDOUT_DUAL`"]
pub struct SPI_MEM_FDOUT_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FDOUT_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FDIN_DUAL`"]
pub type SPI_MEM_FDIN_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FDIN_DUAL`"]
pub struct SPI_MEM_FDIN_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FDIN_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_CACHE_FLASH_USR_CMD`"]
pub type SPI_MEM_CACHE_FLASH_USR_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CACHE_FLASH_USR_CMD`"]
pub struct SPI_MEM_CACHE_FLASH_USR_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CACHE_FLASH_USR_CMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_CACHE_USR_ADDR_4BYTE`"]
pub type SPI_MEM_CACHE_USR_ADDR_4BYTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CACHE_USR_ADDR_4BYTE`"]
pub struct SPI_MEM_CACHE_USR_ADDR_4BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CACHE_USR_ADDR_4BYTE_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_CACHE_REQ_EN`"]
pub type SPI_MEM_CACHE_REQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CACHE_REQ_EN`"]
pub struct SPI_MEM_CACHE_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CACHE_REQ_EN_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_mem_faddr_quad(&self) -> SPI_MEM_FADDR_QUAD_R {
        SPI_MEM_FADDR_QUAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_mem_fdout_quad(&self) -> SPI_MEM_FDOUT_QUAD_R {
        SPI_MEM_FDOUT_QUAD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_mem_fdin_quad(&self) -> SPI_MEM_FDIN_QUAD_R {
        SPI_MEM_FDIN_QUAD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_mem_faddr_dual(&self) -> SPI_MEM_FADDR_DUAL_R {
        SPI_MEM_FADDR_DUAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_fdout_dual(&self) -> SPI_MEM_FDOUT_DUAL_R {
        SPI_MEM_FDOUT_DUAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_fdin_dual(&self) -> SPI_MEM_FDIN_DUAL_R {
        SPI_MEM_FDIN_DUAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_cache_flash_usr_cmd(&self) -> SPI_MEM_CACHE_FLASH_USR_CMD_R {
        SPI_MEM_CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_cache_usr_addr_4byte(&self) -> SPI_MEM_CACHE_USR_ADDR_4BYTE_R {
        SPI_MEM_CACHE_USR_ADDR_4BYTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_cache_req_en(&self) -> SPI_MEM_CACHE_REQ_EN_R {
        SPI_MEM_CACHE_REQ_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_mem_faddr_quad(&mut self) -> SPI_MEM_FADDR_QUAD_W {
        SPI_MEM_FADDR_QUAD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_mem_fdout_quad(&mut self) -> SPI_MEM_FDOUT_QUAD_W {
        SPI_MEM_FDOUT_QUAD_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_mem_fdin_quad(&mut self) -> SPI_MEM_FDIN_QUAD_W {
        SPI_MEM_FDIN_QUAD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_mem_faddr_dual(&mut self) -> SPI_MEM_FADDR_DUAL_W {
        SPI_MEM_FADDR_DUAL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_fdout_dual(&mut self) -> SPI_MEM_FDOUT_DUAL_W {
        SPI_MEM_FDOUT_DUAL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_fdin_dual(&mut self) -> SPI_MEM_FDIN_DUAL_W {
        SPI_MEM_FDIN_DUAL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_cache_flash_usr_cmd(&mut self) -> SPI_MEM_CACHE_FLASH_USR_CMD_W {
        SPI_MEM_CACHE_FLASH_USR_CMD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_cache_usr_addr_4byte(&mut self) -> SPI_MEM_CACHE_USR_ADDR_4BYTE_W {
        SPI_MEM_CACHE_USR_ADDR_4BYTE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_cache_req_en(&mut self) -> SPI_MEM_CACHE_REQ_EN_W {
        SPI_MEM_CACHE_REQ_EN_W { w: self }
    }
}
