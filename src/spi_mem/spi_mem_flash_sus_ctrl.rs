#[doc = "Reader of register SPI_MEM_FLASH_SUS_CTRL"]
pub type R = crate::R<u32, super::SPI_MEM_FLASH_SUS_CTRL>;
#[doc = "Writer for register SPI_MEM_FLASH_SUS_CTRL"]
pub type W = crate::W<u32, super::SPI_MEM_FLASH_SUS_CTRL>;
#[doc = "Register SPI_MEM_FLASH_SUS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_FLASH_SUS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_SUS_TIMEOUT_CNT`"]
pub type SPI_MEM_SUS_TIMEOUT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_SUS_TIMEOUT_CNT`"]
pub struct SPI_MEM_SUS_TIMEOUT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_SUS_TIMEOUT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_PES_END_EN`"]
pub type SPI_MEM_PES_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_PES_END_EN`"]
pub struct SPI_MEM_PES_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_PES_END_EN_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_PER_END_EN`"]
pub type SPI_MEM_PER_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_PER_END_EN`"]
pub struct SPI_MEM_PER_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_PER_END_EN_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FMEM_RD_SUS_2B`"]
pub type SPI_MEM_FMEM_RD_SUS_2B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FMEM_RD_SUS_2B`"]
pub struct SPI_MEM_FMEM_RD_SUS_2B_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FMEM_RD_SUS_2B_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_PESR_END_MSK`"]
pub type SPI_MEM_PESR_END_MSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_MEM_PESR_END_MSK`"]
pub struct SPI_MEM_PESR_END_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_PESR_END_MSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 6)) | (((value as u32) & 0xffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FLASH_PES_EN`"]
pub type SPI_MEM_FLASH_PES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PES_EN`"]
pub struct SPI_MEM_FLASH_PES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PES_EN_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_PES_PER_EN`"]
pub type SPI_MEM_PES_PER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_PES_PER_EN`"]
pub struct SPI_MEM_PES_PER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_PES_PER_EN_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_PES_WAIT_EN`"]
pub type SPI_MEM_FLASH_PES_WAIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PES_WAIT_EN`"]
pub struct SPI_MEM_FLASH_PES_WAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PES_WAIT_EN_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_PER_WAIT_EN`"]
pub type SPI_MEM_FLASH_PER_WAIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PER_WAIT_EN`"]
pub struct SPI_MEM_FLASH_PER_WAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PER_WAIT_EN_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_PES`"]
pub type SPI_MEM_FLASH_PES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PES`"]
pub struct SPI_MEM_FLASH_PES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PES_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_PER`"]
pub type SPI_MEM_FLASH_PER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PER`"]
pub struct SPI_MEM_FLASH_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PER_W<'a> {
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
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn spi_mem_sus_timeout_cnt(&self) -> SPI_MEM_SUS_TIMEOUT_CNT_R {
        SPI_MEM_SUS_TIMEOUT_CNT_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_mem_pes_end_en(&self) -> SPI_MEM_PES_END_EN_R {
        SPI_MEM_PES_END_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_mem_per_end_en(&self) -> SPI_MEM_PER_END_EN_R {
        SPI_MEM_PER_END_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_mem_fmem_rd_sus_2b(&self) -> SPI_MEM_FMEM_RD_SUS_2B_R {
        SPI_MEM_FMEM_RD_SUS_2B_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 6:21"]
    #[inline(always)]
    pub fn spi_mem_pesr_end_msk(&self) -> SPI_MEM_PESR_END_MSK_R {
        SPI_MEM_PESR_END_MSK_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_mem_flash_pes_en(&self) -> SPI_MEM_FLASH_PES_EN_R {
        SPI_MEM_FLASH_PES_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_pes_per_en(&self) -> SPI_MEM_PES_PER_EN_R {
        SPI_MEM_PES_PER_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_flash_pes_wait_en(&self) -> SPI_MEM_FLASH_PES_WAIT_EN_R {
        SPI_MEM_FLASH_PES_WAIT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_flash_per_wait_en(&self) -> SPI_MEM_FLASH_PER_WAIT_EN_R {
        SPI_MEM_FLASH_PER_WAIT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_flash_pes(&self) -> SPI_MEM_FLASH_PES_R {
        SPI_MEM_FLASH_PES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_flash_per(&self) -> SPI_MEM_FLASH_PER_R {
        SPI_MEM_FLASH_PER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn spi_mem_sus_timeout_cnt(&mut self) -> SPI_MEM_SUS_TIMEOUT_CNT_W {
        SPI_MEM_SUS_TIMEOUT_CNT_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_mem_pes_end_en(&mut self) -> SPI_MEM_PES_END_EN_W {
        SPI_MEM_PES_END_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_mem_per_end_en(&mut self) -> SPI_MEM_PER_END_EN_W {
        SPI_MEM_PER_END_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_mem_fmem_rd_sus_2b(&mut self) -> SPI_MEM_FMEM_RD_SUS_2B_W {
        SPI_MEM_FMEM_RD_SUS_2B_W { w: self }
    }
    #[doc = "Bits 6:21"]
    #[inline(always)]
    pub fn spi_mem_pesr_end_msk(&mut self) -> SPI_MEM_PESR_END_MSK_W {
        SPI_MEM_PESR_END_MSK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_mem_flash_pes_en(&mut self) -> SPI_MEM_FLASH_PES_EN_W {
        SPI_MEM_FLASH_PES_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_pes_per_en(&mut self) -> SPI_MEM_PES_PER_EN_W {
        SPI_MEM_PES_PER_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_flash_pes_wait_en(&mut self) -> SPI_MEM_FLASH_PES_WAIT_EN_W {
        SPI_MEM_FLASH_PES_WAIT_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_flash_per_wait_en(&mut self) -> SPI_MEM_FLASH_PER_WAIT_EN_W {
        SPI_MEM_FLASH_PER_WAIT_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_flash_pes(&mut self) -> SPI_MEM_FLASH_PES_W {
        SPI_MEM_FLASH_PES_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_flash_per(&mut self) -> SPI_MEM_FLASH_PER_W {
        SPI_MEM_FLASH_PER_W { w: self }
    }
}
