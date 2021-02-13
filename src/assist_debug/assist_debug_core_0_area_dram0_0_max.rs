#[doc = "Reader of register ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX>;
#[doc = "Writer for register ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX>;
#[doc = "Register ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_0_max(&self) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_0_max(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_W {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_W { w: self }
    }
}
