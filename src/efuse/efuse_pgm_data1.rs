#[doc = "Reader of register EFUSE_PGM_DATA1"]
pub type R = crate::R<u32, super::EFUSE_PGM_DATA1>;
#[doc = "Writer for register EFUSE_PGM_DATA1"]
pub type W = crate::W<u32, super::EFUSE_PGM_DATA1>;
#[doc = "Register EFUSE_PGM_DATA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_PGM_DATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_POWER_GLITCH_DSENSE`"]
pub type EFUSE_POWER_GLITCH_DSENSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_POWER_GLITCH_DSENSE`"]
pub struct EFUSE_POWER_GLITCH_DSENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_POWER_GLITCH_DSENSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_POWERGLITCH_EN`"]
pub type EFUSE_POWERGLITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_POWERGLITCH_EN`"]
pub struct EFUSE_POWERGLITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_POWERGLITCH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_BTLC_GPIO_ENABLE`"]
pub type EFUSE_BTLC_GPIO_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_BTLC_GPIO_ENABLE`"]
pub struct EFUSE_BTLC_GPIO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_BTLC_GPIO_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_VDD_SPI_AS_GPIO`"]
pub type EFUSE_VDD_SPI_AS_GPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_VDD_SPI_AS_GPIO`"]
pub struct EFUSE_VDD_SPI_AS_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_VDD_SPI_AS_GPIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_USB_EXCHG_PINS`"]
pub type EFUSE_USB_EXCHG_PINS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_USB_EXCHG_PINS`"]
pub struct EFUSE_USB_EXCHG_PINS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_USB_EXCHG_PINS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_USB_DREFL`"]
pub type EFUSE_USB_DREFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_USB_DREFL`"]
pub struct EFUSE_USB_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_USB_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_USB_DREFH`"]
pub type EFUSE_USB_DREFH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_USB_DREFH`"]
pub struct EFUSE_USB_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_USB_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT`"]
pub type EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT`"]
pub struct EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_PAD_JTAG`"]
pub type EFUSE_DIS_PAD_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_PAD_JTAG`"]
pub struct EFUSE_DIS_PAD_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_PAD_JTAG_W<'a> {
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
#[doc = "Reader of field `EFUSE_SOFT_DIS_JTAG`"]
pub type EFUSE_SOFT_DIS_JTAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_SOFT_DIS_JTAG`"]
pub struct EFUSE_SOFT_DIS_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SOFT_DIS_JTAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_JTAG_SEL_ENABLE`"]
pub type EFUSE_JTAG_SEL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_JTAG_SEL_ENABLE`"]
pub struct EFUSE_JTAG_SEL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_JTAG_SEL_ENABLE_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_TWAI`"]
pub type EFUSE_DIS_TWAI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_TWAI`"]
pub struct EFUSE_DIS_TWAI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_TWAI_W<'a> {
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
#[doc = "Reader of field `EFUSE_RPT4_RESERVED6_ERR`"]
pub type EFUSE_RPT4_RESERVED6_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_FORCE_DOWNLOAD`"]
pub type EFUSE_DIS_FORCE_DOWNLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_FORCE_DOWNLOAD`"]
pub struct EFUSE_DIS_FORCE_DOWNLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_FORCE_DOWNLOAD_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_USB_DEVICE`"]
pub type EFUSE_DIS_USB_DEVICE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_USB_DEVICE`"]
pub struct EFUSE_DIS_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_USB_DEVICE_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_DOWNLOAD_ICACHE`"]
pub type EFUSE_DIS_DOWNLOAD_ICACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_DOWNLOAD_ICACHE`"]
pub struct EFUSE_DIS_DOWNLOAD_ICACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_DOWNLOAD_ICACHE_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_USB_JTAG`"]
pub type EFUSE_DIS_USB_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_USB_JTAG`"]
pub struct EFUSE_DIS_USB_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_USB_JTAG_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_ICACHE`"]
pub type EFUSE_DIS_ICACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_ICACHE`"]
pub struct EFUSE_DIS_ICACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_ICACHE_W<'a> {
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
#[doc = "Reader of field `EFUSE_DIS_RTC_RAM_BOOT`"]
pub type EFUSE_DIS_RTC_RAM_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DIS_RTC_RAM_BOOT`"]
pub struct EFUSE_DIS_RTC_RAM_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DIS_RTC_RAM_BOOT_W<'a> {
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
#[doc = "Reader of field `EFUSE_RD_DIS`"]
pub type EFUSE_RD_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_DIS`"]
pub struct EFUSE_RD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn efuse_power_glitch_dsense(&self) -> EFUSE_POWER_GLITCH_DSENSE_R {
        EFUSE_POWER_GLITCH_DSENSE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn efuse_powerglitch_en(&self) -> EFUSE_POWERGLITCH_EN_R {
        EFUSE_POWERGLITCH_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn efuse_btlc_gpio_enable(&self) -> EFUSE_BTLC_GPIO_ENABLE_R {
        EFUSE_BTLC_GPIO_ENABLE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn efuse_vdd_spi_as_gpio(&self) -> EFUSE_VDD_SPI_AS_GPIO_R {
        EFUSE_VDD_SPI_AS_GPIO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn efuse_usb_exchg_pins(&self) -> EFUSE_USB_EXCHG_PINS_R {
        EFUSE_USB_EXCHG_PINS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn efuse_usb_drefl(&self) -> EFUSE_USB_DREFL_R {
        EFUSE_USB_DREFL_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn efuse_usb_drefh(&self) -> EFUSE_USB_DREFH_R {
        EFUSE_USB_DREFH_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn efuse_dis_download_manual_encrypt(&self) -> EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn efuse_dis_pad_jtag(&self) -> EFUSE_DIS_PAD_JTAG_R {
        EFUSE_DIS_PAD_JTAG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn efuse_soft_dis_jtag(&self) -> EFUSE_SOFT_DIS_JTAG_R {
        EFUSE_SOFT_DIS_JTAG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn efuse_jtag_sel_enable(&self) -> EFUSE_JTAG_SEL_ENABLE_R {
        EFUSE_JTAG_SEL_ENABLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_dis_twai(&self) -> EFUSE_DIS_TWAI_R {
        EFUSE_DIS_TWAI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved6_err(&self) -> EFUSE_RPT4_RESERVED6_ERR_R {
        EFUSE_RPT4_RESERVED6_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn efuse_dis_force_download(&self) -> EFUSE_DIS_FORCE_DOWNLOAD_R {
        EFUSE_DIS_FORCE_DOWNLOAD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn efuse_dis_usb_device(&self) -> EFUSE_DIS_USB_DEVICE_R {
        EFUSE_DIS_USB_DEVICE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn efuse_dis_download_icache(&self) -> EFUSE_DIS_DOWNLOAD_ICACHE_R {
        EFUSE_DIS_DOWNLOAD_ICACHE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn efuse_dis_usb_jtag(&self) -> EFUSE_DIS_USB_JTAG_R {
        EFUSE_DIS_USB_JTAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_dis_icache(&self) -> EFUSE_DIS_ICACHE_R {
        EFUSE_DIS_ICACHE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn efuse_dis_rtc_ram_boot(&self) -> EFUSE_DIS_RTC_RAM_BOOT_R {
        EFUSE_DIS_RTC_RAM_BOOT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn efuse_rd_dis(&self) -> EFUSE_RD_DIS_R {
        EFUSE_RD_DIS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn efuse_power_glitch_dsense(&mut self) -> EFUSE_POWER_GLITCH_DSENSE_W {
        EFUSE_POWER_GLITCH_DSENSE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn efuse_powerglitch_en(&mut self) -> EFUSE_POWERGLITCH_EN_W {
        EFUSE_POWERGLITCH_EN_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn efuse_btlc_gpio_enable(&mut self) -> EFUSE_BTLC_GPIO_ENABLE_W {
        EFUSE_BTLC_GPIO_ENABLE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn efuse_vdd_spi_as_gpio(&mut self) -> EFUSE_VDD_SPI_AS_GPIO_W {
        EFUSE_VDD_SPI_AS_GPIO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn efuse_usb_exchg_pins(&mut self) -> EFUSE_USB_EXCHG_PINS_W {
        EFUSE_USB_EXCHG_PINS_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn efuse_usb_drefl(&mut self) -> EFUSE_USB_DREFL_W {
        EFUSE_USB_DREFL_W { w: self }
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn efuse_usb_drefh(&mut self) -> EFUSE_USB_DREFH_W {
        EFUSE_USB_DREFH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn efuse_dis_download_manual_encrypt(&mut self) -> EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_W {
        EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn efuse_dis_pad_jtag(&mut self) -> EFUSE_DIS_PAD_JTAG_W {
        EFUSE_DIS_PAD_JTAG_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn efuse_soft_dis_jtag(&mut self) -> EFUSE_SOFT_DIS_JTAG_W {
        EFUSE_SOFT_DIS_JTAG_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn efuse_jtag_sel_enable(&mut self) -> EFUSE_JTAG_SEL_ENABLE_W {
        EFUSE_JTAG_SEL_ENABLE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_dis_twai(&mut self) -> EFUSE_DIS_TWAI_W {
        EFUSE_DIS_TWAI_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn efuse_dis_force_download(&mut self) -> EFUSE_DIS_FORCE_DOWNLOAD_W {
        EFUSE_DIS_FORCE_DOWNLOAD_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn efuse_dis_usb_device(&mut self) -> EFUSE_DIS_USB_DEVICE_W {
        EFUSE_DIS_USB_DEVICE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn efuse_dis_download_icache(&mut self) -> EFUSE_DIS_DOWNLOAD_ICACHE_W {
        EFUSE_DIS_DOWNLOAD_ICACHE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn efuse_dis_usb_jtag(&mut self) -> EFUSE_DIS_USB_JTAG_W {
        EFUSE_DIS_USB_JTAG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_dis_icache(&mut self) -> EFUSE_DIS_ICACHE_W {
        EFUSE_DIS_ICACHE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn efuse_dis_rtc_ram_boot(&mut self) -> EFUSE_DIS_RTC_RAM_BOOT_W {
        EFUSE_DIS_RTC_RAM_BOOT_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn efuse_rd_dis(&mut self) -> EFUSE_RD_DIS_W {
        EFUSE_RD_DIS_W { w: self }
    }
}
