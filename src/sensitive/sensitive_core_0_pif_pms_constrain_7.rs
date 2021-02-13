#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7>;
#[doc = "Writer for register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7>;
#[doc = "Register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_pwr(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_wifimac(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_rwbt(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_i2s1(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_can(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_apb_ctrl(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_spi_2(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_pwr(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_wifimac(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_rwbt(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_i2s1(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_can(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_apb_ctrl(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_spi_2(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_W { w: self }
    }
}
