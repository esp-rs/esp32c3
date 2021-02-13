#[doc = "Reader of register SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4"]
pub type R = crate::R<u32, super::SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4>;
#[doc = "Writer for register SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4"]
pub type W = crate::W<u32, super::SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4>;
#[doc = "Register SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE`"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE`"]
pub struct SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR`"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR`"]
pub struct SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC`"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC`"]
pub struct SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA`"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA`"]
pub struct SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI`"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI`"]
pub struct SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP`"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP`"]
pub struct SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_usb_device(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_bt_pwr(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_apb_adc(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_crypto_dma(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_crypto_peri(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_usb_wrap(
        &self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_usb_device(
        &mut self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_bt_pwr(
        &mut self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_apb_adc(
        &mut self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_crypto_dma(
        &mut self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_crypto_peri(
        &mut self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_backup_bus_pms_constrain_usb_wrap(
        &mut self,
    ) -> SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W {
        SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W { w: self }
    }
}
