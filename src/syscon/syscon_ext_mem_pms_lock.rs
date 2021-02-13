#[doc = "Reader of register SYSCON_EXT_MEM_PMS_LOCK"]
pub type R = crate::R<u32, super::SYSCON_EXT_MEM_PMS_LOCK>;
#[doc = "Writer for register SYSCON_EXT_MEM_PMS_LOCK"]
pub type W = crate::W<u32, super::SYSCON_EXT_MEM_PMS_LOCK>;
#[doc = "Register SYSCON_EXT_MEM_PMS_LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_EXT_MEM_PMS_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_EXT_MEM_PMS_LOCK`"]
pub type SYSCON_EXT_MEM_PMS_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_EXT_MEM_PMS_LOCK`"]
pub struct SYSCON_EXT_MEM_PMS_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_EXT_MEM_PMS_LOCK_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscon_ext_mem_pms_lock(&self) -> SYSCON_EXT_MEM_PMS_LOCK_R {
        SYSCON_EXT_MEM_PMS_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscon_ext_mem_pms_lock(&mut self) -> SYSCON_EXT_MEM_PMS_LOCK_W {
        SYSCON_EXT_MEM_PMS_LOCK_W { w: self }
    }
}
