#[doc = "Reader of register EFUSE_RD_REPEAT_ERR0"]
pub type R = crate::R<u32, super::EFUSE_RD_REPEAT_ERR0>;
#[doc = "Reader of field `EFUSE_POWER_GLITCH_DSENSE_ERR`"]
pub type EFUSE_POWER_GLITCH_DSENSE_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_POWERGLITCH_EN_ERR`"]
pub type EFUSE_POWERGLITCH_EN_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_BTLC_GPIO_ENABLE_ERR`"]
pub type EFUSE_BTLC_GPIO_ENABLE_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_VDD_SPI_AS_GPIO_ERR`"]
pub type EFUSE_VDD_SPI_AS_GPIO_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_USB_EXCHG_PINS_ERR`"]
pub type EFUSE_USB_EXCHG_PINS_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_USB_DREFL_ERR`"]
pub type EFUSE_USB_DREFL_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_USB_DREFH_ERR`"]
pub type EFUSE_USB_DREFH_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR`"]
pub type EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_PAD_JTAG_ERR`"]
pub type EFUSE_DIS_PAD_JTAG_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_SOFT_DIS_JTAG_ERR`"]
pub type EFUSE_SOFT_DIS_JTAG_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_JTAG_SEL_ENABLE_ERR`"]
pub type EFUSE_JTAG_SEL_ENABLE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_TWAI_ERR`"]
pub type EFUSE_DIS_TWAI_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED6_ERR`"]
pub type EFUSE_RPT4_RESERVED6_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_FORCE_DOWNLOAD_ERR`"]
pub type EFUSE_DIS_FORCE_DOWNLOAD_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_USB_DEVICE_ERR`"]
pub type EFUSE_DIS_USB_DEVICE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_DOWNLOAD_ICACHE`"]
pub type EFUSE_DIS_DOWNLOAD_ICACHE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_USB_JTAG_ERR`"]
pub type EFUSE_DIS_USB_JTAG_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_ICACHE_ERR`"]
pub type EFUSE_DIS_ICACHE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_RTC_RAM_BOOT_ERR`"]
pub type EFUSE_DIS_RTC_RAM_BOOT_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_RD_DIS_ERR`"]
pub type EFUSE_RD_DIS_ERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn efuse_power_glitch_dsense_err(&self) -> EFUSE_POWER_GLITCH_DSENSE_ERR_R {
        EFUSE_POWER_GLITCH_DSENSE_ERR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn efuse_powerglitch_en_err(&self) -> EFUSE_POWERGLITCH_EN_ERR_R {
        EFUSE_POWERGLITCH_EN_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn efuse_btlc_gpio_enable_err(&self) -> EFUSE_BTLC_GPIO_ENABLE_ERR_R {
        EFUSE_BTLC_GPIO_ENABLE_ERR_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn efuse_vdd_spi_as_gpio_err(&self) -> EFUSE_VDD_SPI_AS_GPIO_ERR_R {
        EFUSE_VDD_SPI_AS_GPIO_ERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn efuse_usb_exchg_pins_err(&self) -> EFUSE_USB_EXCHG_PINS_ERR_R {
        EFUSE_USB_EXCHG_PINS_ERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn efuse_usb_drefl_err(&self) -> EFUSE_USB_DREFL_ERR_R {
        EFUSE_USB_DREFL_ERR_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn efuse_usb_drefh_err(&self) -> EFUSE_USB_DREFH_ERR_R {
        EFUSE_USB_DREFH_ERR_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn efuse_dis_download_manual_encrypt_err(&self) -> EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
        EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn efuse_dis_pad_jtag_err(&self) -> EFUSE_DIS_PAD_JTAG_ERR_R {
        EFUSE_DIS_PAD_JTAG_ERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn efuse_soft_dis_jtag_err(&self) -> EFUSE_SOFT_DIS_JTAG_ERR_R {
        EFUSE_SOFT_DIS_JTAG_ERR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn efuse_jtag_sel_enable_err(&self) -> EFUSE_JTAG_SEL_ENABLE_ERR_R {
        EFUSE_JTAG_SEL_ENABLE_ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_dis_twai_err(&self) -> EFUSE_DIS_TWAI_ERR_R {
        EFUSE_DIS_TWAI_ERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved6_err(&self) -> EFUSE_RPT4_RESERVED6_ERR_R {
        EFUSE_RPT4_RESERVED6_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn efuse_dis_force_download_err(&self) -> EFUSE_DIS_FORCE_DOWNLOAD_ERR_R {
        EFUSE_DIS_FORCE_DOWNLOAD_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn efuse_dis_usb_device_err(&self) -> EFUSE_DIS_USB_DEVICE_ERR_R {
        EFUSE_DIS_USB_DEVICE_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn efuse_dis_download_icache(&self) -> EFUSE_DIS_DOWNLOAD_ICACHE_R {
        EFUSE_DIS_DOWNLOAD_ICACHE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn efuse_dis_usb_jtag_err(&self) -> EFUSE_DIS_USB_JTAG_ERR_R {
        EFUSE_DIS_USB_JTAG_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_dis_icache_err(&self) -> EFUSE_DIS_ICACHE_ERR_R {
        EFUSE_DIS_ICACHE_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn efuse_dis_rtc_ram_boot_err(&self) -> EFUSE_DIS_RTC_RAM_BOOT_ERR_R {
        EFUSE_DIS_RTC_RAM_BOOT_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn efuse_rd_dis_err(&self) -> EFUSE_RD_DIS_ERR_R {
        EFUSE_RD_DIS_ERR_R::new((self.bits & 0x7f) as u8)
    }
}
