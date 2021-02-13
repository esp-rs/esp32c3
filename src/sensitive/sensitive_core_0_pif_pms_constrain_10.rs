#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10>;
#[doc = "Writer for register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10>;
#[doc = "Register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_world_1_h(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_world_1_l(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_world_0_h(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_world_0_l(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_world_1_h(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_world_1_l(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_world_0_h(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_world_0_l(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_W { w: self }
    }
}
