#[doc = "Reader of register ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_LASTPC_BEFORE_EXC`"]
pub type ASSIST_DEBUG_CORE_0_LASTPC_BEFORE_EXC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn assist_debug_core_0_lastpc_before_exc(&self) -> ASSIST_DEBUG_CORE_0_LASTPC_BEFORE_EXC_R {
        ASSIST_DEBUG_CORE_0_LASTPC_BEFORE_EXC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
