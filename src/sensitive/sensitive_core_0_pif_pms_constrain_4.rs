#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4>;
#[doc = "Writer for register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4>;
#[doc = "Register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_world_controller(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R::new(
            ((self.bits >> 30) & 0x03) as u8,
        )
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_dio(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_ad(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_cache_config(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R::new(
            ((self.bits >> 24) & 0x03) as u8,
        )
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_dma_copy(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_interrupt(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R::new(
            ((self.bits >> 20) & 0x03) as u8,
        )
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_sensitive(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R::new(
            ((self.bits >> 18) & 0x03) as u8,
        )
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_system(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_usb_device(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R::new(
            ((self.bits >> 14) & 0x03) as u8,
        )
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_bt_pwr(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_apb_adc(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_crypto_dma(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R::new(
            ((self.bits >> 6) & 0x03) as u8,
        )
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_crypto_peri(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R::new(
            ((self.bits >> 4) & 0x03) as u8,
        )
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_usb_wrap(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_world_controller(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_dio(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_ad(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_cache_config(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_dma_copy(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_interrupt(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_sensitive(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_system(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_usb_device(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_bt_pwr(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_apb_adc(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_crypto_dma(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_crypto_peri(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_0_usb_wrap(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W { w: self }
    }
}
