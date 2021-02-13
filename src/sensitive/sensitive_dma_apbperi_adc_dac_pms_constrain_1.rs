#[doc = "Reader of register SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1"]
pub type R = crate::R<u32, super::SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1>;
#[doc = "Writer for register SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1"]
pub type W = crate::W<u32, super::SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1>;
#[doc = "Register SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3`"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3`"]
pub struct SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2`"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2`"]
pub struct SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1`"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1`"]
pub struct SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0`"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0`"]
pub struct SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3`"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3`"]
pub struct SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2`"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2`"]
pub struct SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1`"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1`"]
pub struct SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0`"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0`"]
pub struct SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R::new(
            ((self.bits >> 18) & 0x03) as u8,
        )
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R::new(
            ((self.bits >> 16) & 0x03) as u8,
        )
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R::new(
            ((self.bits >> 14) & 0x03) as u8,
        )
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R::new(
            ((self.bits >> 12) & 0x03) as u8,
        )
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R::new(
            ((self.bits >> 6) & 0x03) as u8,
        )
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R::new(
            ((self.bits >> 4) & 0x03) as u8,
        )
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R::new(
            ((self.bits >> 2) & 0x03) as u8,
        )
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0(
        &self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R::new(
            (self.bits & 0x03) as u8,
        )
    }
}
impl W {
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3(
        &mut self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2(
        &mut self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1(
        &mut self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0(
        &mut self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3(
        &mut self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2(
        &mut self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1(
        &mut self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0(
        &mut self,
    ) -> SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W {
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W { w: self }
    }
}
