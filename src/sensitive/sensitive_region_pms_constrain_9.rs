#[doc = "Reader of register SENSITIVE_REGION_PMS_CONSTRAIN_9"]
pub type R = crate::R<u32, super::SENSITIVE_REGION_PMS_CONSTRAIN_9>;
#[doc = "Writer for register SENSITIVE_REGION_PMS_CONSTRAIN_9"]
pub type W = crate::W<u32, super::SENSITIVE_REGION_PMS_CONSTRAIN_9>;
#[doc = "Register SENSITIVE_REGION_PMS_CONSTRAIN_9 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_REGION_PMS_CONSTRAIN_9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6`"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6`"]
pub struct SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_addr_6(&self) -> SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6_R {
        SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn sensitive_region_pms_constrain_addr_6(
        &mut self,
    ) -> SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6_W {
        SENSITIVE_REGION_PMS_CONSTRAIN_ADDR_6_W { w: self }
    }
}
