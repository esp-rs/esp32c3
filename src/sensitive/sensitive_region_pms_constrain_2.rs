#[doc = "Reader of register SENSITIVE_REGION_PMS_CONSTRAIN_2"]
pub type R = crate::R<u32, super::SENSITIVE_REGION_PMS_CONSTRAIN_2>;
#[doc = "Writer for register SENSITIVE_REGION_PMS_CONSTRAIN_2"]
pub type W = crate::W<u32, super::SENSITIVE_REGION_PMS_CONSTRAIN_2>;
#[doc = "Register SENSITIVE_REGION_PMS_CONSTRAIN_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_REGION_PMS_CONSTRAIN_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6`"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6`"]
pub struct SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5`"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5`"]
pub struct SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4`"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4`"]
pub struct SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3`"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3`"]
pub struct SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2`"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2`"]
pub struct SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1`"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1`"]
pub struct SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0`"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0`"]
pub struct SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_6(
        &self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_5(
        &self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_4(
        &self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_3(
        &self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_2(
        &self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_1(
        &self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_0(
        &self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_6(
        &mut self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_5(
        &mut self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_4(
        &mut self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_3(
        &mut self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_2(
        &mut self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_1(
        &mut self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_world_1_area_0(
        &mut self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W {
        SENSITIVE_REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W { w: self }
    }
}
