#[doc = "Reader of register SPI_MEM_INT_ST"]
pub type R = crate::R<u32, super::SPI_MEM_INT_ST>;
#[doc = "Reader of field `SPI_MEM_MST_ST_END_INT_ST`"]
pub type SPI_MEM_MST_ST_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_MEM_SLV_ST_END_INT_ST`"]
pub type SPI_MEM_SLV_ST_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_MEM_WPE_END_INT_ST`"]
pub type SPI_MEM_WPE_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_MEM_PES_END_INT_ST`"]
pub type SPI_MEM_PES_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_MEM_PER_END_INT_ST`"]
pub type SPI_MEM_PER_END_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_st(&self) -> SPI_MEM_MST_ST_END_INT_ST_R {
        SPI_MEM_MST_ST_END_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_st(&self) -> SPI_MEM_SLV_ST_END_INT_ST_R {
        SPI_MEM_SLV_ST_END_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_st(&self) -> SPI_MEM_WPE_END_INT_ST_R {
        SPI_MEM_WPE_END_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_st(&self) -> SPI_MEM_PES_END_INT_ST_R {
        SPI_MEM_PES_END_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_per_end_int_st(&self) -> SPI_MEM_PER_END_INT_ST_R {
        SPI_MEM_PER_END_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
