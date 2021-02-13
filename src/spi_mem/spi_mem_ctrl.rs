#[doc = "Reader of register SPI_MEM_CTRL"]
pub type R = crate::R<u32, super::SPI_MEM_CTRL>;
#[doc = "Writer for register SPI_MEM_CTRL"]
pub type W = crate::W<u32, super::SPI_MEM_CTRL>;
#[doc = "Register SPI_MEM_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_FREAD_QIO`"]
pub type SPI_MEM_FREAD_QIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FREAD_QIO`"]
pub struct SPI_MEM_FREAD_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FREAD_QIO_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FREAD_DIO`"]
pub type SPI_MEM_FREAD_DIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FREAD_DIO`"]
pub struct SPI_MEM_FREAD_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FREAD_DIO_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_WRSR_2B`"]
pub type SPI_MEM_WRSR_2B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_WRSR_2B`"]
pub struct SPI_MEM_WRSR_2B_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WRSR_2B_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_WP_REG`"]
pub type SPI_MEM_WP_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_WP_REG`"]
pub struct SPI_MEM_WP_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_WP_REG_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FREAD_QUAD`"]
pub type SPI_MEM_FREAD_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FREAD_QUAD`"]
pub struct SPI_MEM_FREAD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FREAD_QUAD_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_D_POL`"]
pub type SPI_MEM_D_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_D_POL`"]
pub struct SPI_MEM_D_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_D_POL_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_Q_POL`"]
pub type SPI_MEM_Q_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_Q_POL`"]
pub struct SPI_MEM_Q_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_Q_POL_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_RESANDRES`"]
pub type SPI_MEM_RESANDRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_RESANDRES`"]
pub struct SPI_MEM_RESANDRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_RESANDRES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FREAD_DUAL`"]
pub type SPI_MEM_FREAD_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FREAD_DUAL`"]
pub struct SPI_MEM_FREAD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FREAD_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FASTRD_MODE`"]
pub type SPI_MEM_FASTRD_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FASTRD_MODE`"]
pub struct SPI_MEM_FASTRD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FASTRD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_TX_CRC_EN`"]
pub type SPI_MEM_TX_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_TX_CRC_EN`"]
pub struct SPI_MEM_TX_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_TX_CRC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_FCS_CRC_EN`"]
pub type SPI_MEM_FCS_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FCS_CRC_EN`"]
pub struct SPI_MEM_FCS_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FCS_CRC_EN_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FCMD_QUAD`"]
pub type SPI_MEM_FCMD_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FCMD_QUAD`"]
pub struct SPI_MEM_FCMD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FCMD_QUAD_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FCMD_DUAL`"]
pub type SPI_MEM_FCMD_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FCMD_DUAL`"]
pub struct SPI_MEM_FCMD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FCMD_DUAL_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_FDUMMY_OUT`"]
pub type SPI_MEM_FDUMMY_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MEM_FDUMMY_OUT`"]
pub struct SPI_MEM_FDUMMY_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_FDUMMY_OUT_W<'a> {
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
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_mem_fread_qio(&self) -> SPI_MEM_FREAD_QIO_R {
        SPI_MEM_FREAD_QIO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_mem_fread_dio(&self) -> SPI_MEM_FREAD_DIO_R {
        SPI_MEM_FREAD_DIO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_mem_wrsr_2b(&self) -> SPI_MEM_WRSR_2B_R {
        SPI_MEM_WRSR_2B_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_mem_wp_reg(&self) -> SPI_MEM_WP_REG_R {
        SPI_MEM_WP_REG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_mem_fread_quad(&self) -> SPI_MEM_FREAD_QUAD_R {
        SPI_MEM_FREAD_QUAD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_mem_d_pol(&self) -> SPI_MEM_D_POL_R {
        SPI_MEM_D_POL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_mem_q_pol(&self) -> SPI_MEM_Q_POL_R {
        SPI_MEM_Q_POL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_mem_resandres(&self) -> SPI_MEM_RESANDRES_R {
        SPI_MEM_RESANDRES_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_mem_fread_dual(&self) -> SPI_MEM_FREAD_DUAL_R {
        SPI_MEM_FREAD_DUAL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_mem_fastrd_mode(&self) -> SPI_MEM_FASTRD_MODE_R {
        SPI_MEM_FASTRD_MODE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spi_mem_tx_crc_en(&self) -> SPI_MEM_TX_CRC_EN_R {
        SPI_MEM_TX_CRC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_mem_fcs_crc_en(&self) -> SPI_MEM_FCS_CRC_EN_R {
        SPI_MEM_FCS_CRC_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_mem_fcmd_quad(&self) -> SPI_MEM_FCMD_QUAD_R {
        SPI_MEM_FCMD_QUAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_mem_fcmd_dual(&self) -> SPI_MEM_FCMD_DUAL_R {
        SPI_MEM_FCMD_DUAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_fdummy_out(&self) -> SPI_MEM_FDUMMY_OUT_R {
        SPI_MEM_FDUMMY_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_mem_fread_qio(&mut self) -> SPI_MEM_FREAD_QIO_W {
        SPI_MEM_FREAD_QIO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_mem_fread_dio(&mut self) -> SPI_MEM_FREAD_DIO_W {
        SPI_MEM_FREAD_DIO_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_mem_wrsr_2b(&mut self) -> SPI_MEM_WRSR_2B_W {
        SPI_MEM_WRSR_2B_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_mem_wp_reg(&mut self) -> SPI_MEM_WP_REG_W {
        SPI_MEM_WP_REG_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_mem_fread_quad(&mut self) -> SPI_MEM_FREAD_QUAD_W {
        SPI_MEM_FREAD_QUAD_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_mem_d_pol(&mut self) -> SPI_MEM_D_POL_W {
        SPI_MEM_D_POL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_mem_q_pol(&mut self) -> SPI_MEM_Q_POL_W {
        SPI_MEM_Q_POL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_mem_resandres(&mut self) -> SPI_MEM_RESANDRES_W {
        SPI_MEM_RESANDRES_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_mem_fread_dual(&mut self) -> SPI_MEM_FREAD_DUAL_W {
        SPI_MEM_FREAD_DUAL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_mem_fastrd_mode(&mut self) -> SPI_MEM_FASTRD_MODE_W {
        SPI_MEM_FASTRD_MODE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spi_mem_tx_crc_en(&mut self) -> SPI_MEM_TX_CRC_EN_W {
        SPI_MEM_TX_CRC_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_mem_fcs_crc_en(&mut self) -> SPI_MEM_FCS_CRC_EN_W {
        SPI_MEM_FCS_CRC_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_mem_fcmd_quad(&mut self) -> SPI_MEM_FCMD_QUAD_W {
        SPI_MEM_FCMD_QUAD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_mem_fcmd_dual(&mut self) -> SPI_MEM_FCMD_DUAL_W {
        SPI_MEM_FCMD_DUAL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_mem_fdummy_out(&mut self) -> SPI_MEM_FDUMMY_OUT_W {
        SPI_MEM_FDUMMY_OUT_W { w: self }
    }
}
