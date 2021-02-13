#[doc = "Reader of register SPI_CMD"]
pub type R = crate::R<u32, super::SPI_CMD>;
#[doc = "Writer for register SPI_CMD"]
pub type W = crate::W<u32, super::SPI_CMD>;
#[doc = "Register SPI_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_USR`"]
pub type SPI_USR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR`"]
pub struct SPI_USR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_W<'a> {
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
#[doc = "Write proxy for field `SPI_UPDATE`"]
pub struct SPI_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_UPDATE_W<'a> {
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
#[doc = "Reader of field `SPI_CONF_BITLEN`"]
pub type SPI_CONF_BITLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_CONF_BITLEN`"]
pub struct SPI_CONF_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CONF_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_usr(&self) -> SPI_USR_R {
        SPI_USR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn spi_conf_bitlen(&self) -> SPI_CONF_BITLEN_R {
        SPI_CONF_BITLEN_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_usr(&mut self) -> SPI_USR_W {
        SPI_USR_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_update(&mut self) -> SPI_UPDATE_W {
        SPI_UPDATE_W { w: self }
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn spi_conf_bitlen(&mut self) -> SPI_CONF_BITLEN_W {
        SPI_CONF_BITLEN_W { w: self }
    }
}
