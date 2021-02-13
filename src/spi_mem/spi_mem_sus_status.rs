#[doc = "Reader of register SPI_MEM_SUS_STATUS"]
pub type R = crate::R<u32, super::SPI_MEM_SUS_STATUS>;
#[doc = "Writer for register SPI_MEM_SUS_STATUS"]
pub type W = crate::W<u32, super::SPI_MEM_SUS_STATUS>;
#[doc = "Register SPI_MEM_SUS_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_SUS_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_SPI0_LOCK_EN`"]
pub type SPI_MEM_SPI0_LOCK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_SPI0_LOCK_EN`"]
pub struct SPI_MEM_SPI0_LOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_SPI0_LOCK_EN_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_PES_DLY_128`"]
pub type SPI_MEM_FLASH_PES_DLY_128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PES_DLY_128`"]
pub struct SPI_MEM_FLASH_PES_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PES_DLY_128_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_PER_DLY_128`"]
pub type SPI_MEM_FLASH_PER_DLY_128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_PER_DLY_128`"]
pub struct SPI_MEM_FLASH_PER_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_PER_DLY_128_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_DP_DLY_128`"]
pub type SPI_MEM_FLASH_DP_DLY_128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_DP_DLY_128`"]
pub struct SPI_MEM_FLASH_DP_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_DP_DLY_128_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_RES_DLY_128`"]
pub type SPI_MEM_FLASH_RES_DLY_128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_RES_DLY_128`"]
pub struct SPI_MEM_FLASH_RES_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_RES_DLY_128_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_HPM_DLY_128`"]
pub type SPI_MEM_FLASH_HPM_DLY_128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_HPM_DLY_128`"]
pub struct SPI_MEM_FLASH_HPM_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_HPM_DLY_128_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_WAIT_PESR_CMD_2B`"]
pub type SPI_MEM_WAIT_PESR_CMD_2B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_WAIT_PESR_CMD_2B`"]
pub struct SPI_MEM_WAIT_PESR_CMD_2B_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WAIT_PESR_CMD_2B_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FLASH_SUS`"]
pub type SPI_MEM_FLASH_SUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FLASH_SUS`"]
pub struct SPI_MEM_FLASH_SUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FLASH_SUS_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_mem_spi0_lock_en(&self) -> SPI_MEM_SPI0_LOCK_EN_R {
        SPI_MEM_SPI0_LOCK_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_mem_flash_pes_dly_128(&self) -> SPI_MEM_FLASH_PES_DLY_128_R {
        SPI_MEM_FLASH_PES_DLY_128_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_mem_flash_per_dly_128(&self) -> SPI_MEM_FLASH_PER_DLY_128_R {
        SPI_MEM_FLASH_PER_DLY_128_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_flash_dp_dly_128(&self) -> SPI_MEM_FLASH_DP_DLY_128_R {
        SPI_MEM_FLASH_DP_DLY_128_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_flash_res_dly_128(&self) -> SPI_MEM_FLASH_RES_DLY_128_R {
        SPI_MEM_FLASH_RES_DLY_128_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_flash_hpm_dly_128(&self) -> SPI_MEM_FLASH_HPM_DLY_128_R {
        SPI_MEM_FLASH_HPM_DLY_128_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_wait_pesr_cmd_2b(&self) -> SPI_MEM_WAIT_PESR_CMD_2B_R {
        SPI_MEM_WAIT_PESR_CMD_2B_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_flash_sus(&self) -> SPI_MEM_FLASH_SUS_R {
        SPI_MEM_FLASH_SUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_mem_spi0_lock_en(&mut self) -> SPI_MEM_SPI0_LOCK_EN_W {
        SPI_MEM_SPI0_LOCK_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_mem_flash_pes_dly_128(&mut self) -> SPI_MEM_FLASH_PES_DLY_128_W {
        SPI_MEM_FLASH_PES_DLY_128_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_mem_flash_per_dly_128(&mut self) -> SPI_MEM_FLASH_PER_DLY_128_W {
        SPI_MEM_FLASH_PER_DLY_128_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_flash_dp_dly_128(&mut self) -> SPI_MEM_FLASH_DP_DLY_128_W {
        SPI_MEM_FLASH_DP_DLY_128_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_flash_res_dly_128(&mut self) -> SPI_MEM_FLASH_RES_DLY_128_W {
        SPI_MEM_FLASH_RES_DLY_128_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_flash_hpm_dly_128(&mut self) -> SPI_MEM_FLASH_HPM_DLY_128_W {
        SPI_MEM_FLASH_HPM_DLY_128_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_wait_pesr_cmd_2b(&mut self) -> SPI_MEM_WAIT_PESR_CMD_2B_W {
        SPI_MEM_WAIT_PESR_CMD_2B_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_flash_sus(&mut self) -> SPI_MEM_FLASH_SUS_W {
        SPI_MEM_FLASH_SUS_W { w: self }
    }
}
