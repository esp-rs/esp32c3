#[doc = "Reader of register ASSIST_DEBUG_LOG_SETTING"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_LOG_SETTING>;
#[doc = "Writer for register ASSIST_DEBUG_LOG_SETTING"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_LOG_SETTING>;
#[doc = "Register ASSIST_DEBUG_LOG_SETTING `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_LOG_SETTING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE`"]
pub type ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE`"]
pub struct ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_LOG_MODE`"]
pub type ASSIST_DEBUG_LOG_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASSIST_DEBUG_LOG_MODE`"]
pub struct ASSIST_DEBUG_LOG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_LOG_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_LOG_ENA`"]
pub type ASSIST_DEBUG_LOG_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASSIST_DEBUG_LOG_ENA`"]
pub struct ASSIST_DEBUG_LOG_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_LOG_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn assist_debug_log_mem_loop_enable(&self) -> ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE_R {
        ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn assist_debug_log_mode(&self) -> ASSIST_DEBUG_LOG_MODE_R {
        ASSIST_DEBUG_LOG_MODE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn assist_debug_log_ena(&self) -> ASSIST_DEBUG_LOG_ENA_R {
        ASSIST_DEBUG_LOG_ENA_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn assist_debug_log_mem_loop_enable(&mut self) -> ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE_W {
        ASSIST_DEBUG_LOG_MEM_LOOP_ENABLE_W { w: self }
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn assist_debug_log_mode(&mut self) -> ASSIST_DEBUG_LOG_MODE_W {
        ASSIST_DEBUG_LOG_MODE_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn assist_debug_log_ena(&mut self) -> ASSIST_DEBUG_LOG_ENA_W {
        ASSIST_DEBUG_LOG_ENA_W { w: self }
    }
}
