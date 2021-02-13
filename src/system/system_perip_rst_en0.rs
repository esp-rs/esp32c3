#[doc = "Reader of register SYSTEM_PERIP_RST_EN0"]
pub type R = crate::R<u32, super::SYSTEM_PERIP_RST_EN0>;
#[doc = "Writer for register SYSTEM_PERIP_RST_EN0"]
pub type W = crate::W<u32, super::SYSTEM_PERIP_RST_EN0>;
#[doc = "Register SYSTEM_PERIP_RST_EN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_PERIP_RST_EN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_SPI4_RST`"]
pub type SYSTEM_SPI4_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_SPI4_RST`"]
pub struct SYSTEM_SPI4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SPI4_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_ADC2_ARB_RST`"]
pub type SYSTEM_ADC2_ARB_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_ADC2_ARB_RST`"]
pub struct SYSTEM_ADC2_ARB_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_ADC2_ARB_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_SYSTIMER_RST`"]
pub type SYSTEM_SYSTIMER_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_SYSTIMER_RST`"]
pub struct SYSTEM_SYSTIMER_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SYSTIMER_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_APB_SARADC_RST`"]
pub type SYSTEM_APB_SARADC_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_APB_SARADC_RST`"]
pub struct SYSTEM_APB_SARADC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_APB_SARADC_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_SPI3_DMA_RST`"]
pub type SYSTEM_SPI3_DMA_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_SPI3_DMA_RST`"]
pub struct SYSTEM_SPI3_DMA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SPI3_DMA_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_PWM3_RST`"]
pub type SYSTEM_PWM3_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_PWM3_RST`"]
pub struct SYSTEM_PWM3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_PWM3_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_PWM2_RST`"]
pub type SYSTEM_PWM2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_PWM2_RST`"]
pub struct SYSTEM_PWM2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_PWM2_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_UART_MEM_RST`"]
pub type SYSTEM_UART_MEM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_UART_MEM_RST`"]
pub struct SYSTEM_UART_MEM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_UART_MEM_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_USB_DEVICE_RST`"]
pub type SYSTEM_USB_DEVICE_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_USB_DEVICE_RST`"]
pub struct SYSTEM_USB_DEVICE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_USB_DEVICE_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_SPI2_DMA_RST`"]
pub type SYSTEM_SPI2_DMA_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_SPI2_DMA_RST`"]
pub struct SYSTEM_SPI2_DMA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SPI2_DMA_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_I2S1_RST`"]
pub type SYSTEM_I2S1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_I2S1_RST`"]
pub struct SYSTEM_I2S1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_I2S1_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_PWM1_RST`"]
pub type SYSTEM_PWM1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_PWM1_RST`"]
pub struct SYSTEM_PWM1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_PWM1_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_TWAI_RST`"]
pub type SYSTEM_TWAI_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_TWAI_RST`"]
pub struct SYSTEM_TWAI_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_TWAI_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_I2C_EXT1_RST`"]
pub type SYSTEM_I2C_EXT1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_I2C_EXT1_RST`"]
pub struct SYSTEM_I2C_EXT1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_I2C_EXT1_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_PWM0_RST`"]
pub type SYSTEM_PWM0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_PWM0_RST`"]
pub struct SYSTEM_PWM0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_PWM0_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_SPI3_RST`"]
pub type SYSTEM_SPI3_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_SPI3_RST`"]
pub struct SYSTEM_SPI3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SPI3_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_TIMERGROUP1_RST`"]
pub type SYSTEM_TIMERGROUP1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_TIMERGROUP1_RST`"]
pub struct SYSTEM_TIMERGROUP1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_TIMERGROUP1_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_EFUSE_RST`"]
pub type SYSTEM_EFUSE_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_EFUSE_RST`"]
pub struct SYSTEM_EFUSE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_EFUSE_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_TIMERGROUP_RST`"]
pub type SYSTEM_TIMERGROUP_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_TIMERGROUP_RST`"]
pub struct SYSTEM_TIMERGROUP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_TIMERGROUP_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_UHCI1_RST`"]
pub type SYSTEM_UHCI1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_UHCI1_RST`"]
pub struct SYSTEM_UHCI1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_UHCI1_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_LEDC_RST`"]
pub type SYSTEM_LEDC_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_LEDC_RST`"]
pub struct SYSTEM_LEDC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_LEDC_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_PCNT_RST`"]
pub type SYSTEM_PCNT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_PCNT_RST`"]
pub struct SYSTEM_PCNT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_PCNT_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_RMT_RST`"]
pub type SYSTEM_RMT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_RMT_RST`"]
pub struct SYSTEM_RMT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_RMT_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_UHCI0_RST`"]
pub type SYSTEM_UHCI0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_UHCI0_RST`"]
pub struct SYSTEM_UHCI0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_UHCI0_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_I2C_EXT0_RST`"]
pub type SYSTEM_I2C_EXT0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_I2C_EXT0_RST`"]
pub struct SYSTEM_I2C_EXT0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_I2C_EXT0_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_SPI2_RST`"]
pub type SYSTEM_SPI2_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_SPI2_RST`"]
pub struct SYSTEM_SPI2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SPI2_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_UART1_RST`"]
pub type SYSTEM_UART1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_UART1_RST`"]
pub struct SYSTEM_UART1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_UART1_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_I2S0_RST`"]
pub type SYSTEM_I2S0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_I2S0_RST`"]
pub struct SYSTEM_I2S0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_I2S0_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_WDG_RST`"]
pub type SYSTEM_WDG_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_WDG_RST`"]
pub struct SYSTEM_WDG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_WDG_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_UART_RST`"]
pub type SYSTEM_UART_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_UART_RST`"]
pub struct SYSTEM_UART_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_UART_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_SPI01_RST`"]
pub type SYSTEM_SPI01_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_SPI01_RST`"]
pub struct SYSTEM_SPI01_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SPI01_RST_W<'a> {
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
#[doc = "Reader of field `SYSTEM_TIMERS_RST`"]
pub type SYSTEM_TIMERS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_TIMERS_RST`"]
pub struct SYSTEM_TIMERS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_TIMERS_RST_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn system_spi4_rst(&self) -> SYSTEM_SPI4_RST_R {
        SYSTEM_SPI4_RST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn system_adc2_arb_rst(&self) -> SYSTEM_ADC2_ARB_RST_R {
        SYSTEM_ADC2_ARB_RST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn system_systimer_rst(&self) -> SYSTEM_SYSTIMER_RST_R {
        SYSTEM_SYSTIMER_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn system_apb_saradc_rst(&self) -> SYSTEM_APB_SARADC_RST_R {
        SYSTEM_APB_SARADC_RST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn system_spi3_dma_rst(&self) -> SYSTEM_SPI3_DMA_RST_R {
        SYSTEM_SPI3_DMA_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn system_pwm3_rst(&self) -> SYSTEM_PWM3_RST_R {
        SYSTEM_PWM3_RST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn system_pwm2_rst(&self) -> SYSTEM_PWM2_RST_R {
        SYSTEM_PWM2_RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn system_uart_mem_rst(&self) -> SYSTEM_UART_MEM_RST_R {
        SYSTEM_UART_MEM_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn system_usb_device_rst(&self) -> SYSTEM_USB_DEVICE_RST_R {
        SYSTEM_USB_DEVICE_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn system_spi2_dma_rst(&self) -> SYSTEM_SPI2_DMA_RST_R {
        SYSTEM_SPI2_DMA_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn system_i2s1_rst(&self) -> SYSTEM_I2S1_RST_R {
        SYSTEM_I2S1_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn system_pwm1_rst(&self) -> SYSTEM_PWM1_RST_R {
        SYSTEM_PWM1_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn system_twai_rst(&self) -> SYSTEM_TWAI_RST_R {
        SYSTEM_TWAI_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn system_i2c_ext1_rst(&self) -> SYSTEM_I2C_EXT1_RST_R {
        SYSTEM_I2C_EXT1_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn system_pwm0_rst(&self) -> SYSTEM_PWM0_RST_R {
        SYSTEM_PWM0_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn system_spi3_rst(&self) -> SYSTEM_SPI3_RST_R {
        SYSTEM_SPI3_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn system_timergroup1_rst(&self) -> SYSTEM_TIMERGROUP1_RST_R {
        SYSTEM_TIMERGROUP1_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn system_efuse_rst(&self) -> SYSTEM_EFUSE_RST_R {
        SYSTEM_EFUSE_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn system_timergroup_rst(&self) -> SYSTEM_TIMERGROUP_RST_R {
        SYSTEM_TIMERGROUP_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn system_uhci1_rst(&self) -> SYSTEM_UHCI1_RST_R {
        SYSTEM_UHCI1_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn system_ledc_rst(&self) -> SYSTEM_LEDC_RST_R {
        SYSTEM_LEDC_RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn system_pcnt_rst(&self) -> SYSTEM_PCNT_RST_R {
        SYSTEM_PCNT_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn system_rmt_rst(&self) -> SYSTEM_RMT_RST_R {
        SYSTEM_RMT_RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn system_uhci0_rst(&self) -> SYSTEM_UHCI0_RST_R {
        SYSTEM_UHCI0_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn system_i2c_ext0_rst(&self) -> SYSTEM_I2C_EXT0_RST_R {
        SYSTEM_I2C_EXT0_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn system_spi2_rst(&self) -> SYSTEM_SPI2_RST_R {
        SYSTEM_SPI2_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn system_uart1_rst(&self) -> SYSTEM_UART1_RST_R {
        SYSTEM_UART1_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn system_i2s0_rst(&self) -> SYSTEM_I2S0_RST_R {
        SYSTEM_I2S0_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn system_wdg_rst(&self) -> SYSTEM_WDG_RST_R {
        SYSTEM_WDG_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_uart_rst(&self) -> SYSTEM_UART_RST_R {
        SYSTEM_UART_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_spi01_rst(&self) -> SYSTEM_SPI01_RST_R {
        SYSTEM_SPI01_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_timers_rst(&self) -> SYSTEM_TIMERS_RST_R {
        SYSTEM_TIMERS_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn system_spi4_rst(&mut self) -> SYSTEM_SPI4_RST_W {
        SYSTEM_SPI4_RST_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn system_adc2_arb_rst(&mut self) -> SYSTEM_ADC2_ARB_RST_W {
        SYSTEM_ADC2_ARB_RST_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn system_systimer_rst(&mut self) -> SYSTEM_SYSTIMER_RST_W {
        SYSTEM_SYSTIMER_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn system_apb_saradc_rst(&mut self) -> SYSTEM_APB_SARADC_RST_W {
        SYSTEM_APB_SARADC_RST_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn system_spi3_dma_rst(&mut self) -> SYSTEM_SPI3_DMA_RST_W {
        SYSTEM_SPI3_DMA_RST_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn system_pwm3_rst(&mut self) -> SYSTEM_PWM3_RST_W {
        SYSTEM_PWM3_RST_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn system_pwm2_rst(&mut self) -> SYSTEM_PWM2_RST_W {
        SYSTEM_PWM2_RST_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn system_uart_mem_rst(&mut self) -> SYSTEM_UART_MEM_RST_W {
        SYSTEM_UART_MEM_RST_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn system_usb_device_rst(&mut self) -> SYSTEM_USB_DEVICE_RST_W {
        SYSTEM_USB_DEVICE_RST_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn system_spi2_dma_rst(&mut self) -> SYSTEM_SPI2_DMA_RST_W {
        SYSTEM_SPI2_DMA_RST_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn system_i2s1_rst(&mut self) -> SYSTEM_I2S1_RST_W {
        SYSTEM_I2S1_RST_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn system_pwm1_rst(&mut self) -> SYSTEM_PWM1_RST_W {
        SYSTEM_PWM1_RST_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn system_twai_rst(&mut self) -> SYSTEM_TWAI_RST_W {
        SYSTEM_TWAI_RST_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn system_i2c_ext1_rst(&mut self) -> SYSTEM_I2C_EXT1_RST_W {
        SYSTEM_I2C_EXT1_RST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn system_pwm0_rst(&mut self) -> SYSTEM_PWM0_RST_W {
        SYSTEM_PWM0_RST_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn system_spi3_rst(&mut self) -> SYSTEM_SPI3_RST_W {
        SYSTEM_SPI3_RST_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn system_timergroup1_rst(&mut self) -> SYSTEM_TIMERGROUP1_RST_W {
        SYSTEM_TIMERGROUP1_RST_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn system_efuse_rst(&mut self) -> SYSTEM_EFUSE_RST_W {
        SYSTEM_EFUSE_RST_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn system_timergroup_rst(&mut self) -> SYSTEM_TIMERGROUP_RST_W {
        SYSTEM_TIMERGROUP_RST_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn system_uhci1_rst(&mut self) -> SYSTEM_UHCI1_RST_W {
        SYSTEM_UHCI1_RST_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn system_ledc_rst(&mut self) -> SYSTEM_LEDC_RST_W {
        SYSTEM_LEDC_RST_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn system_pcnt_rst(&mut self) -> SYSTEM_PCNT_RST_W {
        SYSTEM_PCNT_RST_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn system_rmt_rst(&mut self) -> SYSTEM_RMT_RST_W {
        SYSTEM_RMT_RST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn system_uhci0_rst(&mut self) -> SYSTEM_UHCI0_RST_W {
        SYSTEM_UHCI0_RST_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn system_i2c_ext0_rst(&mut self) -> SYSTEM_I2C_EXT0_RST_W {
        SYSTEM_I2C_EXT0_RST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn system_spi2_rst(&mut self) -> SYSTEM_SPI2_RST_W {
        SYSTEM_SPI2_RST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn system_uart1_rst(&mut self) -> SYSTEM_UART1_RST_W {
        SYSTEM_UART1_RST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn system_i2s0_rst(&mut self) -> SYSTEM_I2S0_RST_W {
        SYSTEM_I2S0_RST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn system_wdg_rst(&mut self) -> SYSTEM_WDG_RST_W {
        SYSTEM_WDG_RST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_uart_rst(&mut self) -> SYSTEM_UART_RST_W {
        SYSTEM_UART_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_spi01_rst(&mut self) -> SYSTEM_SPI01_RST_W {
        SYSTEM_SPI01_RST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_timers_rst(&mut self) -> SYSTEM_TIMERS_RST_W {
        SYSTEM_TIMERS_RST_W { w: self }
    }
}
