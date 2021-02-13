#[doc = "Reader of register SPI_MEM_CMD"]
pub type R = crate::R<u32, super::SPI_MEM_CMD>;
#[doc = "Writer for register SPI_MEM_CMD"]
pub type W = crate::W<u32, super::SPI_MEM_CMD>;
#[doc = "Register SPI_MEM_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_READ`"]
pub type SPI_MEM_FLASH_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_READ`"]
pub struct SPI_MEM_FLASH_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_READ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_WREN`"]
pub type SPI_MEM_FLASH_WREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_WREN`"]
pub struct SPI_MEM_FLASH_WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_WREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_WRDI`"]
pub type SPI_MEM_FLASH_WRDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_WRDI`"]
pub struct SPI_MEM_FLASH_WRDI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_WRDI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_RDID`"]
pub type SPI_MEM_FLASH_RDID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_RDID`"]
pub struct SPI_MEM_FLASH_RDID_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_RDID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_RDSR`"]
pub type SPI_MEM_FLASH_RDSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_RDSR`"]
pub struct SPI_MEM_FLASH_RDSR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_RDSR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_WRSR`"]
pub type SPI_MEM_FLASH_WRSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_WRSR`"]
pub struct SPI_MEM_FLASH_WRSR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_WRSR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_PP`"]
pub type SPI_MEM_FLASH_PP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PP`"]
pub struct SPI_MEM_FLASH_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_SE`"]
pub type SPI_MEM_FLASH_SE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_SE`"]
pub struct SPI_MEM_FLASH_SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_SE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_BE`"]
pub type SPI_MEM_FLASH_BE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_BE`"]
pub struct SPI_MEM_FLASH_BE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_BE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_CE`"]
pub type SPI_MEM_FLASH_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_CE`"]
pub struct SPI_MEM_FLASH_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_DP`"]
pub type SPI_MEM_FLASH_DP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_DP`"]
pub struct SPI_MEM_FLASH_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_DP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_RES`"]
pub type SPI_MEM_FLASH_RES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_RES`"]
pub struct SPI_MEM_FLASH_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_HPM`"]
pub type SPI_MEM_FLASH_HPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_HPM`"]
pub struct SPI_MEM_FLASH_HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_HPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_USR`"]
pub type SPI_MEM_USR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_USR`"]
pub struct SPI_MEM_USR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_USR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_PE`"]
pub type SPI_MEM_FLASH_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PE`"]
pub struct SPI_MEM_FLASH_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_SLV_ST`"]
pub type SPI_MEM_SLV_ST_R = crate::R<u8, u8>;
#[doc = "Reader of field `SPI_MEM_MST_ST`"]
pub type SPI_MEM_MST_ST_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_mem_flash_read(&self) -> SPI_MEM_FLASH_READ_R {
        SPI_MEM_FLASH_READ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_mem_flash_wren(&self) -> SPI_MEM_FLASH_WREN_R {
        SPI_MEM_FLASH_WREN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_mem_flash_wrdi(&self) -> SPI_MEM_FLASH_WRDI_R {
        SPI_MEM_FLASH_WRDI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_mem_flash_rdid(&self) -> SPI_MEM_FLASH_RDID_R {
        SPI_MEM_FLASH_RDID_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_mem_flash_rdsr(&self) -> SPI_MEM_FLASH_RDSR_R {
        SPI_MEM_FLASH_RDSR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_mem_flash_wrsr(&self) -> SPI_MEM_FLASH_WRSR_R {
        SPI_MEM_FLASH_WRSR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn spi_mem_flash_pp(&self) -> SPI_MEM_FLASH_PP_R {
        SPI_MEM_FLASH_PP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_mem_flash_se(&self) -> SPI_MEM_FLASH_SE_R {
        SPI_MEM_FLASH_SE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_mem_flash_be(&self) -> SPI_MEM_FLASH_BE_R {
        SPI_MEM_FLASH_BE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_mem_flash_ce(&self) -> SPI_MEM_FLASH_CE_R {
        SPI_MEM_FLASH_CE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_mem_flash_dp(&self) -> SPI_MEM_FLASH_DP_R {
        SPI_MEM_FLASH_DP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_mem_flash_res(&self) -> SPI_MEM_FLASH_RES_R {
        SPI_MEM_FLASH_RES_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_mem_flash_hpm(&self) -> SPI_MEM_FLASH_HPM_R {
        SPI_MEM_FLASH_HPM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_mem_usr(&self) -> SPI_MEM_USR_R {
        SPI_MEM_USR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_mem_flash_pe(&self) -> SPI_MEM_FLASH_PE_R {
        SPI_MEM_FLASH_PE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn spi_mem_slv_st(&self) -> SPI_MEM_SLV_ST_R {
        SPI_MEM_SLV_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn spi_mem_mst_st(&self) -> SPI_MEM_MST_ST_R {
        SPI_MEM_MST_ST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_mem_flash_read(&mut self) -> SPI_MEM_FLASH_READ_W {
        SPI_MEM_FLASH_READ_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_mem_flash_wren(&mut self) -> SPI_MEM_FLASH_WREN_W {
        SPI_MEM_FLASH_WREN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_mem_flash_wrdi(&mut self) -> SPI_MEM_FLASH_WRDI_W {
        SPI_MEM_FLASH_WRDI_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_mem_flash_rdid(&mut self) -> SPI_MEM_FLASH_RDID_W {
        SPI_MEM_FLASH_RDID_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_mem_flash_rdsr(&mut self) -> SPI_MEM_FLASH_RDSR_W {
        SPI_MEM_FLASH_RDSR_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_mem_flash_wrsr(&mut self) -> SPI_MEM_FLASH_WRSR_W {
        SPI_MEM_FLASH_WRSR_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn spi_mem_flash_pp(&mut self) -> SPI_MEM_FLASH_PP_W {
        SPI_MEM_FLASH_PP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_mem_flash_se(&mut self) -> SPI_MEM_FLASH_SE_W {
        SPI_MEM_FLASH_SE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_mem_flash_be(&mut self) -> SPI_MEM_FLASH_BE_W {
        SPI_MEM_FLASH_BE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_mem_flash_ce(&mut self) -> SPI_MEM_FLASH_CE_W {
        SPI_MEM_FLASH_CE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_mem_flash_dp(&mut self) -> SPI_MEM_FLASH_DP_W {
        SPI_MEM_FLASH_DP_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_mem_flash_res(&mut self) -> SPI_MEM_FLASH_RES_W {
        SPI_MEM_FLASH_RES_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_mem_flash_hpm(&mut self) -> SPI_MEM_FLASH_HPM_W {
        SPI_MEM_FLASH_HPM_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_mem_usr(&mut self) -> SPI_MEM_USR_W {
        SPI_MEM_USR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_mem_flash_pe(&mut self) -> SPI_MEM_FLASH_PE_W {
        SPI_MEM_FLASH_PE_W { w: self }
    }
}
