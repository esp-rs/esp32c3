#[doc = "Writer for register SPI_MEM_INT_CLR"]
pub type W = crate::W<u32, super::SPI_MEM_INT_CLR>;
#[doc = "Register SPI_MEM_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SPI_MEM_MST_ST_END_INT_CLR`"]
pub struct SPI_MEM_MST_ST_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_MST_ST_END_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_MEM_SLV_ST_END_INT_CLR`"]
pub struct SPI_MEM_SLV_ST_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_SLV_ST_END_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_MEM_WPE_END_INT_CLR`"]
pub struct SPI_MEM_WPE_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WPE_END_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_MEM_PES_END_INT_CLR`"]
pub struct SPI_MEM_PES_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_PES_END_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SPI_MEM_PER_END_INT_CLR`"]
pub struct SPI_MEM_PER_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_PER_END_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_clr(&mut self) -> SPI_MEM_MST_ST_END_INT_CLR_W {
        SPI_MEM_MST_ST_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_clr(&mut self) -> SPI_MEM_SLV_ST_END_INT_CLR_W {
        SPI_MEM_SLV_ST_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_clr(&mut self) -> SPI_MEM_WPE_END_INT_CLR_W {
        SPI_MEM_WPE_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_clr(&mut self) -> SPI_MEM_PES_END_INT_CLR_W {
        SPI_MEM_PES_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_per_end_int_clr(&mut self) -> SPI_MEM_PER_END_INT_CLR_W {
        SPI_MEM_PER_END_INT_CLR_W { w: self }
    }
}
