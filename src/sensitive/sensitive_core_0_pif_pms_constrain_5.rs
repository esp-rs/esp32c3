#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5>;
#[doc = "Writer for register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5>;
#[doc = "Register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_uart1(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_i2c(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_misc(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_wdg(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_io_mux(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_rtc(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_timer(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_fe(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_fe2(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_gpio(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_g0spi_0(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_g0spi_1(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_uart(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_uart1(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_i2c(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_misc(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_wdg(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_io_mux(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_rtc(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_timer(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_fe(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_fe2(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_gpio(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_g0spi_0(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_g0spi_1(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_uart(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_W { w: self }
    }
}
