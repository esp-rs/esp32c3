#[doc = "Reader of register ASSIST_DEBUG_LOG_DATA_MASK"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_LOG_DATA_MASK>;
#[doc = "Writer for register ASSIST_DEBUG_LOG_DATA_MASK"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_LOG_DATA_MASK>;
#[doc = "Register ASSIST_DEBUG_LOG_DATA_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_LOG_DATA_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_LOG_DATA_SIZE`"]
pub type ASSIST_DEBUG_LOG_DATA_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ASSIST_DEBUG_LOG_DATA_SIZE`"]
pub struct ASSIST_DEBUG_LOG_DATA_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_LOG_DATA_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn assist_debug_log_data_size(&self) -> ASSIST_DEBUG_LOG_DATA_SIZE_R {
        ASSIST_DEBUG_LOG_DATA_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn assist_debug_log_data_size(&mut self) -> ASSIST_DEBUG_LOG_DATA_SIZE_W {
        ASSIST_DEBUG_LOG_DATA_SIZE_W { w: self }
    }
}
