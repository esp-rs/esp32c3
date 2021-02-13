#[doc = "Reader of register SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2>;
#[doc = "Writer for register SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2>;
#[doc = "Register SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS`"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS`"]
pub struct SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0`"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R =
    crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0`"]
pub struct SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3`"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3`"]
pub struct SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2`"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2`"]
pub struct SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1`"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1`"]
pub struct SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0`"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0`"]
pub struct SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_rom_world_0_pms(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R::new(
            ((self.bits >> 18) & 0x07) as u8,
        )
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_cachedataarray_pms_0(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R::new(
            ((self.bits >> 12) & 0x07) as u8,
        )
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_pms_3(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R::new(
            ((self.bits >> 9) & 0x07) as u8,
        )
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_pms_2(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R::new(
            ((self.bits >> 6) & 0x07) as u8,
        )
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_pms_1(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R::new(
            ((self.bits >> 3) & 0x07) as u8,
        )
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_pms_0(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_rom_world_0_pms(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_cachedataarray_pms_0(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_W {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_pms_3(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_pms_2(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_pms_1(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_pms_constrain_sram_world_0_pms_0(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W {
        SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W { w: self }
    }
}
