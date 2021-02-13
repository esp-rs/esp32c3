#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9>;
#[doc = "Writer for register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9>;
#[doc = "Register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_spltaddr_world_1(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R::new(
            ((self.bits >> 11) & 0x07ff) as u16,
        )
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_spltaddr_world_0(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R::new(
            (self.bits & 0x07ff) as u16,
        )
    }
}
impl W {
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_spltaddr_world_1(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_rtcfast_spltaddr_world_0(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W { w: self }
    }
}
