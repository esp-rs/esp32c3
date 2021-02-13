#[doc = "Reader of register EFUSE_RD_REPEAT_ERR3"]
pub type R = crate::R<u32, super::EFUSE_RD_REPEAT_ERR3>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED1_ERR`"]
pub type EFUSE_RPT4_RESERVED1_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SECURE_VERSION_ERR`"]
pub type EFUSE_SECURE_VERSION_ERR_R = crate::R<u16, u16>;
#[doc = "Reader of field `EFUSE_FORCE_SEND_RESUME_ERR`"]
pub type EFUSE_FORCE_SEND_RESUME_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_FLASH_ECC_EN`"]
pub type EFUSE_FLASH_ECC_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_FLASH_PAGE_SIZE`"]
pub type EFUSE_FLASH_PAGE_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_FLASH_TYPE_ERR`"]
pub type EFUSE_FLASH_TYPE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_PIN_POWER_SELECTION_ERR`"]
pub type EFUSE_PIN_POWER_SELECTION_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_UART_PRINT_CONTROL_ERR`"]
pub type EFUSE_UART_PRINT_CONTROL_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_ENABLE_SECURITY_DOWNLOAD_ERR`"]
pub type EFUSE_ENABLE_SECURITY_DOWNLOAD_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_USB_DOWNLOAD_MODE_ERR`"]
pub type EFUSE_DIS_USB_DOWNLOAD_MODE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_FLASH_ECC_MODE_ERR`"]
pub type EFUSE_FLASH_ECC_MODE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_UART_PRINT_CHANNEL_ERR`"]
pub type EFUSE_UART_PRINT_CHANNEL_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_LEGACY_SPI_BOOT_ERR`"]
pub type EFUSE_DIS_LEGACY_SPI_BOOT_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_DOWNLOAD_MODE_ERR`"]
pub type EFUSE_DIS_DOWNLOAD_MODE_ERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved1_err(&self) -> EFUSE_RPT4_RESERVED1_ERR_R {
        EFUSE_RPT4_RESERVED1_ERR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn efuse_secure_version_err(&self) -> EFUSE_SECURE_VERSION_ERR_R {
        EFUSE_SECURE_VERSION_ERR_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn efuse_force_send_resume_err(&self) -> EFUSE_FORCE_SEND_RESUME_ERR_R {
        EFUSE_FORCE_SEND_RESUME_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
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
    pub fn efuse_flash_type_err(&self) -> EFUSE_FLASH_TYPE_ERR_R {
        EFUSE_FLASH_TYPE_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_pin_power_selection_err(&self) -> EFUSE_PIN_POWER_SELECTION_ERR_R {
        EFUSE_PIN_POWER_SELECTION_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn efuse_uart_print_control_err(&self) -> EFUSE_UART_PRINT_CONTROL_ERR_R {
        EFUSE_UART_PRINT_CONTROL_ERR_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn efuse_enable_security_download_err(&self) -> EFUSE_ENABLE_SECURITY_DOWNLOAD_ERR_R {
        EFUSE_ENABLE_SECURITY_DOWNLOAD_ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn efuse_dis_usb_download_mode_err(&self) -> EFUSE_DIS_USB_DOWNLOAD_MODE_ERR_R {
        EFUSE_DIS_USB_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_flash_ecc_mode_err(&self) -> EFUSE_FLASH_ECC_MODE_ERR_R {
        EFUSE_FLASH_ECC_MODE_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_uart_print_channel_err(&self) -> EFUSE_UART_PRINT_CHANNEL_ERR_R {
        EFUSE_UART_PRINT_CHANNEL_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_dis_legacy_spi_boot_err(&self) -> EFUSE_DIS_LEGACY_SPI_BOOT_ERR_R {
        EFUSE_DIS_LEGACY_SPI_BOOT_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_dis_download_mode_err(&self) -> EFUSE_DIS_DOWNLOAD_MODE_ERR_R {
        EFUSE_DIS_DOWNLOAD_MODE_ERR_R::new((self.bits & 0x01) != 0)
    }
}
