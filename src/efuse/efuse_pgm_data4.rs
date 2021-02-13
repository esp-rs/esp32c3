#[doc = "Reader of register EFUSE_PGM_DATA4"]
pub type R = crate::R<u32, super::EFUSE_PGM_DATA4>;
#[doc = "Writer for register EFUSE_PGM_DATA4"]
pub type W = crate::W<u32, super::EFUSE_PGM_DATA4>;
#[doc = "Register EFUSE_PGM_DATA4 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_PGM_DATA4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_RPT4_RESERVED1`"]
pub type EFUSE_RPT4_RESERVED1_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SECURE_VERSION`"]
pub type EFUSE_SECURE_VERSION_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFUSE_SECURE_VERSION`"]
pub struct EFUSE_SECURE_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SECURE_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 14)) | (((value as u32) & 0xffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_FORCE_SEND_RESUME`"]
pub type EFUSE_FORCE_SEND_RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_FORCE_SEND_RESUME`"]
pub struct EFUSE_FORCE_SEND_RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_FORCE_SEND_RESUME_W<'a> {
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
#[doc = "Reader of field `EFUSE_FLASH_ECC_EN`"]
pub type EFUSE_FLASH_ECC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_FLASH_ECC_EN`"]
pub struct EFUSE_FLASH_ECC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_FLASH_ECC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_FLASH_PAGE_SIZE`"]
pub type EFUSE_FLASH_PAGE_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_FLASH_PAGE_SIZE`"]
pub struct EFUSE_FLASH_PAGE_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_FLASH_PAGE_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_FLASH_TYPE`"]
pub type EFUSE_FLASH_TYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_FLASH_TYPE`"]
pub struct EFUSE_FLASH_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_FLASH_TYPE_W<'a> {
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
#[doc = "Reader of field `EFUSE_PIN_POWER_SELECTION`"]
pub type EFUSE_PIN_POWER_SELECTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_PIN_POWER_SELECTION`"]
pub struct EFUSE_PIN_POWER_SELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PIN_POWER_SELECTION_W<'a> {
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
#[doc = "Reader of field `EFUSE_UART_PRINT_CONTROL`"]
pub type EFUSE_UART_PRINT_CONTROL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_UART_PRINT_CONTROL`"]
pub struct EFUSE_UART_PRINT_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_UART_PRINT_CONTROL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_ENABLE_SECURITY_DOWNLOAD`"]
pub type EFUSE_ENABLE_SECURITY_DOWNLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_ENABLE_SECURITY_DOWNLOAD`"]
pub struct EFUSE_ENABLE_SECURITY_DOWNLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_ENABLE_SECURITY_DOWNLOAD_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_USB_DOWNLOAD_MODE`"]
pub type EFUSE_DIS_USB_DOWNLOAD_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_USB_DOWNLOAD_MODE`"]
pub struct EFUSE_DIS_USB_DOWNLOAD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_USB_DOWNLOAD_MODE_W<'a> {
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
#[doc = "Reader of field `EFUSE_FLASH_ECC_MODE`"]
pub type EFUSE_FLASH_ECC_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_FLASH_ECC_MODE`"]
pub struct EFUSE_FLASH_ECC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_FLASH_ECC_MODE_W<'a> {
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
#[doc = "Reader of field `EFUSE_UART_PRINT_CHANNEL`"]
pub type EFUSE_UART_PRINT_CHANNEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_UART_PRINT_CHANNEL`"]
pub struct EFUSE_UART_PRINT_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_UART_PRINT_CHANNEL_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_LEGACY_SPI_BOOT`"]
pub type EFUSE_DIS_LEGACY_SPI_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_LEGACY_SPI_BOOT`"]
pub struct EFUSE_DIS_LEGACY_SPI_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_LEGACY_SPI_BOOT_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_DOWNLOAD_MODE`"]
pub type EFUSE_DIS_DOWNLOAD_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_DOWNLOAD_MODE`"]
pub struct EFUSE_DIS_DOWNLOAD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_DOWNLOAD_MODE_W<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved1(&self) -> EFUSE_RPT4_RESERVED1_R {
        EFUSE_RPT4_RESERVED1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn efuse_secure_version(&self) -> EFUSE_SECURE_VERSION_R {
        EFUSE_SECURE_VERSION_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn efuse_force_send_resume(&self) -> EFUSE_FORCE_SEND_RESUME_R {
        EFUSE_FORCE_SEND_RESUME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn efuse_flash_ecc_en(&self) -> EFUSE_FLASH_ECC_EN_R {
        EFUSE_FLASH_ECC_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn efuse_flash_page_size(&self) -> EFUSE_FLASH_PAGE_SIZE_R {
        EFUSE_FLASH_PAGE_SIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn efuse_flash_type(&self) -> EFUSE_FLASH_TYPE_R {
        EFUSE_FLASH_TYPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_pin_power_selection(&self) -> EFUSE_PIN_POWER_SELECTION_R {
        EFUSE_PIN_POWER_SELECTION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn efuse_uart_print_control(&self) -> EFUSE_UART_PRINT_CONTROL_R {
        EFUSE_UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn efuse_enable_security_download(&self) -> EFUSE_ENABLE_SECURITY_DOWNLOAD_R {
        EFUSE_ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn efuse_dis_usb_download_mode(&self) -> EFUSE_DIS_USB_DOWNLOAD_MODE_R {
        EFUSE_DIS_USB_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_flash_ecc_mode(&self) -> EFUSE_FLASH_ECC_MODE_R {
        EFUSE_FLASH_ECC_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_uart_print_channel(&self) -> EFUSE_UART_PRINT_CHANNEL_R {
        EFUSE_UART_PRINT_CHANNEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_dis_legacy_spi_boot(&self) -> EFUSE_DIS_LEGACY_SPI_BOOT_R {
        EFUSE_DIS_LEGACY_SPI_BOOT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_dis_download_mode(&self) -> EFUSE_DIS_DOWNLOAD_MODE_R {
        EFUSE_DIS_DOWNLOAD_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn efuse_secure_version(&mut self) -> EFUSE_SECURE_VERSION_W {
        EFUSE_SECURE_VERSION_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn efuse_force_send_resume(&mut self) -> EFUSE_FORCE_SEND_RESUME_W {
        EFUSE_FORCE_SEND_RESUME_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn efuse_flash_ecc_en(&mut self) -> EFUSE_FLASH_ECC_EN_W {
        EFUSE_FLASH_ECC_EN_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn efuse_flash_page_size(&mut self) -> EFUSE_FLASH_PAGE_SIZE_W {
        EFUSE_FLASH_PAGE_SIZE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn efuse_flash_type(&mut self) -> EFUSE_FLASH_TYPE_W {
        EFUSE_FLASH_TYPE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_pin_power_selection(&mut self) -> EFUSE_PIN_POWER_SELECTION_W {
        EFUSE_PIN_POWER_SELECTION_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn efuse_uart_print_control(&mut self) -> EFUSE_UART_PRINT_CONTROL_W {
        EFUSE_UART_PRINT_CONTROL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn efuse_enable_security_download(&mut self) -> EFUSE_ENABLE_SECURITY_DOWNLOAD_W {
        EFUSE_ENABLE_SECURITY_DOWNLOAD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn efuse_dis_usb_download_mode(&mut self) -> EFUSE_DIS_USB_DOWNLOAD_MODE_W {
        EFUSE_DIS_USB_DOWNLOAD_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_flash_ecc_mode(&mut self) -> EFUSE_FLASH_ECC_MODE_W {
        EFUSE_FLASH_ECC_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_uart_print_channel(&mut self) -> EFUSE_UART_PRINT_CHANNEL_W {
        EFUSE_UART_PRINT_CHANNEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_dis_legacy_spi_boot(&mut self) -> EFUSE_DIS_LEGACY_SPI_BOOT_W {
        EFUSE_DIS_LEGACY_SPI_BOOT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_dis_download_mode(&mut self) -> EFUSE_DIS_DOWNLOAD_MODE_W {
        EFUSE_DIS_DOWNLOAD_MODE_W { w: self }
    }
}
