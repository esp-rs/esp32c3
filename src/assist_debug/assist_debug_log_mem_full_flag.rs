#[doc = "Reader of register ASSIST_DEBUG_LOG_MEM_FULL_FLAG"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_LOG_MEM_FULL_FLAG>;
#[doc = "Writer for register ASSIST_DEBUG_LOG_MEM_FULL_FLAG"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_LOG_MEM_FULL_FLAG>;
#[doc = "Register ASSIST_DEBUG_LOG_MEM_FULL_FLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_LOG_MEM_FULL_FLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG`"]
pub type ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG`"]
pub struct ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG_W<'a> {
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
#[doc = "Reader of field `ASSIST_DEBUG_LOG_MEM_FULL_FLAG`"]
pub type ASSIST_DEBUG_LOG_MEM_FULL_FLAG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn assist_debug_clr_log_mem_full_flag(&self) -> ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG_R {
        ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn assist_debug_log_mem_full_flag(&self) -> ASSIST_DEBUG_LOG_MEM_FULL_FLAG_R {
        ASSIST_DEBUG_LOG_MEM_FULL_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn assist_debug_clr_log_mem_full_flag(&mut self) -> ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG_W {
        ASSIST_DEBUG_CLR_LOG_MEM_FULL_FLAG_W { w: self }
    }
}
