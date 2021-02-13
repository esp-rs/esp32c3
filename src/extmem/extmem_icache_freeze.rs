#[doc = "Reader of register EXTMEM_ICACHE_FREEZE"]
pub type R = crate::R<u32, super::EXTMEM_ICACHE_FREEZE>;
#[doc = "Writer for register EXTMEM_ICACHE_FREEZE"]
pub type W = crate::W<u32, super::EXTMEM_ICACHE_FREEZE>;
#[doc = "Register EXTMEM_ICACHE_FREEZE `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_ICACHE_FREEZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_FREEZE_DONE`"]
pub type EXTMEM_ICACHE_FREEZE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_ICACHE_FREEZE_MODE`"]
pub type EXTMEM_ICACHE_FREEZE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_FREEZE_MODE`"]
pub struct EXTMEM_ICACHE_FREEZE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_FREEZE_MODE_W<'a> {
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
#[doc = "Reader of field `EXTMEM_ICACHE_FREEZE_ENA`"]
pub type EXTMEM_ICACHE_FREEZE_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_FREEZE_ENA`"]
pub struct EXTMEM_ICACHE_FREEZE_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_FREEZE_ENA_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_icache_freeze_done(&self) -> EXTMEM_ICACHE_FREEZE_DONE_R {
        EXTMEM_ICACHE_FREEZE_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_freeze_mode(&self) -> EXTMEM_ICACHE_FREEZE_MODE_R {
        EXTMEM_ICACHE_FREEZE_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_freeze_ena(&self) -> EXTMEM_ICACHE_FREEZE_ENA_R {
        EXTMEM_ICACHE_FREEZE_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_freeze_mode(&mut self) -> EXTMEM_ICACHE_FREEZE_MODE_W {
        EXTMEM_ICACHE_FREEZE_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_freeze_ena(&mut self) -> EXTMEM_ICACHE_FREEZE_ENA_W {
        EXTMEM_ICACHE_FREEZE_ENA_W { w: self }
    }
}
