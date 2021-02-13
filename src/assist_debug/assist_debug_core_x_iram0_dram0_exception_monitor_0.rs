#[doc = "Reader of register ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0>;
#[doc = "Writer for register ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0>;
#[doc = "Register ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0`"]
pub type ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0`"]
pub struct ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn assist_debug_core_x_iram0_dram0_limit_cycle_0(
        &self,
    ) -> ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R {
        ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn assist_debug_core_x_iram0_dram0_limit_cycle_0(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W {
        ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W { w: self }
    }
}
