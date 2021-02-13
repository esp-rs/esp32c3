#[doc = "Reader of register SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL"]
pub type R = crate::R<u32, super::SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL>;
#[doc = "Writer for register SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL"]
pub type W = crate::W<u32, super::SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL>;
#[doc = "Register SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT`"]
pub type SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT`"]
pub struct SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
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
#[doc = "Reader of field `SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT`"]
pub type SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT`"]
pub struct SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT_W<'a> {
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
#[doc = "Reader of field `SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT`"]
pub type SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT`"]
pub struct SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT_W<'a> {
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
#[doc = "Reader of field `SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT`"]
pub type SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT`"]
pub struct SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn system_enable_download_manual_encrypt(&self) -> SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R {
        SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_enable_download_g0cb_decrypt(&self) -> SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT_R {
        SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_enable_download_db_encrypt(&self) -> SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT_R {
        SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_enable_spi_manual_encrypt(&self) -> SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT_R {
        SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn system_enable_download_manual_encrypt(
        &mut self,
    ) -> SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W {
        SYSTEM_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_enable_download_g0cb_decrypt(&mut self) -> SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT_W {
        SYSTEM_ENABLE_DOWNLOAD_G0CB_DECRYPT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_enable_download_db_encrypt(&mut self) -> SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT_W {
        SYSTEM_ENABLE_DOWNLOAD_DB_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_enable_spi_manual_encrypt(&mut self) -> SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT_W {
        SYSTEM_ENABLE_SPI_MANUAL_ENCRYPT_W { w: self }
    }
}
