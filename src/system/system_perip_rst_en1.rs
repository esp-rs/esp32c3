#[doc = "Reader of register SYSTEM_PERIP_RST_EN1"]
pub type R = crate::R<u32, super::SYSTEM_PERIP_RST_EN1>;
#[doc = "Writer for register SYSTEM_PERIP_RST_EN1"]
pub type W = crate::W<u32, super::SYSTEM_PERIP_RST_EN1>;
#[doc = "Register SYSTEM_PERIP_RST_EN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_PERIP_RST_EN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_TSENS_RST`"]
pub type SYSTEM_TSENS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_TSENS_RST`"]
pub struct SYSTEM_TSENS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_TSENS_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_UART2_RST`"]
pub type SYSTEM_UART2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_UART2_RST`"]
pub struct SYSTEM_UART2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_UART2_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_LCD_CAM_RST`"]
pub type SYSTEM_LCD_CAM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_LCD_CAM_RST`"]
pub struct SYSTEM_LCD_CAM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_LCD_CAM_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_SDIO_HOST_RST`"]
pub type SYSTEM_SDIO_HOST_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_SDIO_HOST_RST`"]
pub struct SYSTEM_SDIO_HOST_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SDIO_HOST_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_DMA_RST`"]
pub type SYSTEM_DMA_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_DMA_RST`"]
pub struct SYSTEM_DMA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_DMA_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_CRYPTO_HMAC_RST`"]
pub type SYSTEM_CRYPTO_HMAC_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CRYPTO_HMAC_RST`"]
pub struct SYSTEM_CRYPTO_HMAC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CRYPTO_HMAC_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_CRYPTO_DS_RST`"]
pub type SYSTEM_CRYPTO_DS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CRYPTO_DS_RST`"]
pub struct SYSTEM_CRYPTO_DS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CRYPTO_DS_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_CRYPTO_RSA_RST`"]
pub type SYSTEM_CRYPTO_RSA_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CRYPTO_RSA_RST`"]
pub struct SYSTEM_CRYPTO_RSA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CRYPTO_RSA_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_CRYPTO_SHA_RST`"]
pub type SYSTEM_CRYPTO_SHA_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CRYPTO_SHA_RST`"]
pub struct SYSTEM_CRYPTO_SHA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CRYPTO_SHA_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_CRYPTO_AES_RST`"]
pub type SYSTEM_CRYPTO_AES_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CRYPTO_AES_RST`"]
pub struct SYSTEM_CRYPTO_AES_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CRYPTO_AES_RST_W<'a> {
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
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn system_tsens_rst(&self) -> SYSTEM_TSENS_RST_R {
        SYSTEM_TSENS_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn system_uart2_rst(&self) -> SYSTEM_UART2_RST_R {
        SYSTEM_UART2_RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn system_lcd_cam_rst(&self) -> SYSTEM_LCD_CAM_RST_R {
        SYSTEM_LCD_CAM_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn system_sdio_host_rst(&self) -> SYSTEM_SDIO_HOST_RST_R {
        SYSTEM_SDIO_HOST_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn system_dma_rst(&self) -> SYSTEM_DMA_RST_R {
        SYSTEM_DMA_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn system_crypto_hmac_rst(&self) -> SYSTEM_CRYPTO_HMAC_RST_R {
        SYSTEM_CRYPTO_HMAC_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn system_crypto_ds_rst(&self) -> SYSTEM_CRYPTO_DS_RST_R {
        SYSTEM_CRYPTO_DS_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn system_crypto_rsa_rst(&self) -> SYSTEM_CRYPTO_RSA_RST_R {
        SYSTEM_CRYPTO_RSA_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_crypto_sha_rst(&self) -> SYSTEM_CRYPTO_SHA_RST_R {
        SYSTEM_CRYPTO_SHA_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_crypto_aes_rst(&self) -> SYSTEM_CRYPTO_AES_RST_R {
        SYSTEM_CRYPTO_AES_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn system_tsens_rst(&mut self) -> SYSTEM_TSENS_RST_W {
        SYSTEM_TSENS_RST_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn system_uart2_rst(&mut self) -> SYSTEM_UART2_RST_W {
        SYSTEM_UART2_RST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn system_lcd_cam_rst(&mut self) -> SYSTEM_LCD_CAM_RST_W {
        SYSTEM_LCD_CAM_RST_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn system_sdio_host_rst(&mut self) -> SYSTEM_SDIO_HOST_RST_W {
        SYSTEM_SDIO_HOST_RST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn system_dma_rst(&mut self) -> SYSTEM_DMA_RST_W {
        SYSTEM_DMA_RST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn system_crypto_hmac_rst(&mut self) -> SYSTEM_CRYPTO_HMAC_RST_W {
        SYSTEM_CRYPTO_HMAC_RST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn system_crypto_ds_rst(&mut self) -> SYSTEM_CRYPTO_DS_RST_W {
        SYSTEM_CRYPTO_DS_RST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn system_crypto_rsa_rst(&mut self) -> SYSTEM_CRYPTO_RSA_RST_W {
        SYSTEM_CRYPTO_RSA_RST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_crypto_sha_rst(&mut self) -> SYSTEM_CRYPTO_SHA_RST_W {
        SYSTEM_CRYPTO_SHA_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_crypto_aes_rst(&mut self) -> SYSTEM_CRYPTO_AES_RST_W {
        SYSTEM_CRYPTO_AES_RST_W { w: self }
    }
}
