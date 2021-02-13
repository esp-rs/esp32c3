#[doc = "Reader of register ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_C0RE_0_DEBUG_MODE>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DEBUG_MODULE_ACTIVE`"]
pub type ASSIST_DEBUG_CORE_0_DEBUG_MODULE_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DEBUG_MODE`"]
pub type ASSIST_DEBUG_CORE_0_DEBUG_MODE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn assist_debug_core_0_debug_module_active(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_DEBUG_MODULE_ACTIVE_R {
        ASSIST_DEBUG_CORE_0_DEBUG_MODULE_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn assist_debug_core_0_debug_mode(&self) -> ASSIST_DEBUG_CORE_0_DEBUG_MODE_R {
        ASSIST_DEBUG_CORE_0_DEBUG_MODE_R::new((self.bits & 0x01) != 0)
    }
}
