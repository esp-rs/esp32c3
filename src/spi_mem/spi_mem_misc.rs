#[doc = "Reader of register SPI_MEM_MISC"]
pub type R = crate::R<u32, super::SPI_MEM_MISC>;
#[doc = "Writer for register SPI_MEM_MISC"]
pub type W = crate::W<u32, super::SPI_MEM_MISC>;
#[doc = "Register SPI_MEM_MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_CS_KEEP_ACTIVE`"]
pub type SPI_MEM_CS_KEEP_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CS_KEEP_ACTIVE`"]
pub struct SPI_MEM_CS_KEEP_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CS_KEEP_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_CK_IDLE_EDGE`"]
pub type SPI_MEM_CK_IDLE_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CK_IDLE_EDGE`"]
pub struct SPI_MEM_CK_IDLE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CK_IDLE_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_SLV_ST_TRANS_END_INT_ENA`"]
pub type SPI_MEM_SLV_ST_TRANS_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_SLV_ST_TRANS_END_INT_ENA`"]
pub struct SPI_MEM_SLV_ST_TRANS_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_SLV_ST_TRANS_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_SLV_ST_TRANS_END`"]
pub type SPI_MEM_SLV_ST_TRANS_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_SLV_ST_TRANS_END`"]
pub struct SPI_MEM_SLV_ST_TRANS_END_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_SLV_ST_TRANS_END_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_CSPI_ST_TRANS_END_INT_ENA`"]
pub type SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CSPI_ST_TRANS_END_INT_ENA`"]
pub struct SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_CSPI_ST_TRANS_END`"]
pub type SPI_MEM_CSPI_ST_TRANS_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CSPI_ST_TRANS_END`"]
pub struct SPI_MEM_CSPI_ST_TRANS_END_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CSPI_ST_TRANS_END_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_TRANS_END_INT_ENA`"]
pub type SPI_MEM_TRANS_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_TRANS_END_INT_ENA`"]
pub struct SPI_MEM_TRANS_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_TRANS_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_CS1_DIS`"]
pub type SPI_MEM_CS1_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CS1_DIS`"]
pub struct SPI_MEM_CS1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CS1_DIS_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_CS0_DIS`"]
pub type SPI_MEM_CS0_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_CS0_DIS`"]
pub struct SPI_MEM_CS0_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CS0_DIS_W<'a> {
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
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&self) -> SPI_MEM_CS_KEEP_ACTIVE_R {
        SPI_MEM_CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&self) -> SPI_MEM_CK_IDLE_EDGE_R {
        SPI_MEM_CK_IDLE_EDGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_mem_slv_st_trans_end_int_ena(&self) -> SPI_MEM_SLV_ST_TRANS_END_INT_ENA_R {
        SPI_MEM_SLV_ST_TRANS_END_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_mem_slv_st_trans_end(&self) -> SPI_MEM_SLV_ST_TRANS_END_R {
        SPI_MEM_SLV_ST_TRANS_END_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_cspi_st_trans_end_int_ena(&self) -> SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_R {
        SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_cspi_st_trans_end(&self) -> SPI_MEM_CSPI_ST_TRANS_END_R {
        SPI_MEM_CSPI_ST_TRANS_END_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_trans_end_int_ena(&self) -> SPI_MEM_TRANS_END_INT_ENA_R {
        SPI_MEM_TRANS_END_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_cs1_dis(&self) -> SPI_MEM_CS1_DIS_R {
        SPI_MEM_CS1_DIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_cs0_dis(&self) -> SPI_MEM_CS0_DIS_R {
        SPI_MEM_CS0_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&mut self) -> SPI_MEM_CS_KEEP_ACTIVE_W {
        SPI_MEM_CS_KEEP_ACTIVE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&mut self) -> SPI_MEM_CK_IDLE_EDGE_W {
        SPI_MEM_CK_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_mem_slv_st_trans_end_int_ena(&mut self) -> SPI_MEM_SLV_ST_TRANS_END_INT_ENA_W {
        SPI_MEM_SLV_ST_TRANS_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_mem_slv_st_trans_end(&mut self) -> SPI_MEM_SLV_ST_TRANS_END_W {
        SPI_MEM_SLV_ST_TRANS_END_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_mem_cspi_st_trans_end_int_ena(&mut self) -> SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_W {
        SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_cspi_st_trans_end(&mut self) -> SPI_MEM_CSPI_ST_TRANS_END_W {
        SPI_MEM_CSPI_ST_TRANS_END_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_mem_trans_end_int_ena(&mut self) -> SPI_MEM_TRANS_END_INT_ENA_W {
        SPI_MEM_TRANS_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_mem_cs1_dis(&mut self) -> SPI_MEM_CS1_DIS_W {
        SPI_MEM_CS1_DIS_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mem_cs0_dis(&mut self) -> SPI_MEM_CS0_DIS_W {
        SPI_MEM_CS0_DIS_W { w: self }
    }
}
