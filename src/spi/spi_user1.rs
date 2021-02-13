#[doc = "Reader of register SPI_USER1"]
pub type R = crate::R<u32, super::SPI_USER1>;
#[doc = "Writer for register SPI_USER1"]
pub type W = crate::W<u32, super::SPI_USER1>;
#[doc = "Register SPI_USER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_USER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_USR_ADDR_BITLEN`"]
pub type SPI_USR_ADDR_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_USR_ADDR_BITLEN`"]
pub struct SPI_USR_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `SPI_CS_HOLD_TIME`"]
pub type SPI_CS_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CS_HOLD_TIME`"]
pub struct SPI_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | (((value as u32) & 0x1f) << 22);
        self.w
    }
}
#[doc = "Reader of field `SPI_CS_SETUP_TIME`"]
pub type SPI_CS_SETUP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CS_SETUP_TIME`"]
pub struct SPI_CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Reader of field `SPI_MST_WFULL_ERR_END_EN`"]
pub type SPI_MST_WFULL_ERR_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_MST_WFULL_ERR_END_EN`"]
pub struct SPI_MST_WFULL_ERR_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MST_WFULL_ERR_END_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_DUMMY_CYCLELEN`"]
pub type SPI_USR_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_USR_DUMMY_CYCLELEN`"]
pub struct SPI_USR_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn spi_usr_addr_bitlen(&self) -> SPI_USR_ADDR_BITLEN_R {
        SPI_USR_ADDR_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn spi_cs_hold_time(&self) -> SPI_CS_HOLD_TIME_R {
        SPI_CS_HOLD_TIME_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn spi_cs_setup_time(&self) -> SPI_CS_SETUP_TIME_R {
        SPI_CS_SETUP_TIME_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_mst_wfull_err_end_en(&self) -> SPI_MST_WFULL_ERR_END_EN_R {
        SPI_MST_WFULL_ERR_END_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn spi_usr_dummy_cyclelen(&self) -> SPI_USR_DUMMY_CYCLELEN_R {
        SPI_USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn spi_usr_addr_bitlen(&mut self) -> SPI_USR_ADDR_BITLEN_W {
        SPI_USR_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn spi_cs_hold_time(&mut self) -> SPI_CS_HOLD_TIME_W {
        SPI_CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn spi_cs_setup_time(&mut self) -> SPI_CS_SETUP_TIME_W {
        SPI_CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_mst_wfull_err_end_en(&mut self) -> SPI_MST_WFULL_ERR_END_EN_W {
        SPI_MST_WFULL_ERR_END_EN_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn spi_usr_dummy_cyclelen(&mut self) -> SPI_USR_DUMMY_CYCLELEN_W {
        SPI_USR_DUMMY_CYCLELEN_W { w: self }
    }
}
