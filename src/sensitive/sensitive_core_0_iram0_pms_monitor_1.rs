#[doc = "Reader of register SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1>;
#[doc = "Writer for register SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1>;
#[doc = "Register SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN`"]
pub type SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN`"]
pub struct SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN_W<'a> {
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
#[doc = "Reader of field `SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR`"]
pub type SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR`"]
pub struct SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sensitive_core_0_iram0_pms_monitor_violate_en(
        &self,
    ) -> SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN_R {
        SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_core_0_iram0_pms_monitor_violate_clr(
        &self,
    ) -> SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR_R {
        SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sensitive_core_0_iram0_pms_monitor_violate_en(
        &mut self,
    ) -> SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN_W {
        SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sensitive_core_0_iram0_pms_monitor_violate_clr(
        &mut self,
    ) -> SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR_W {
        SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR_W { w: self }
    }
}
