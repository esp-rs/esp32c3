#[doc = "Reader of register EXTMEM_ICACHE_AUTOLOAD_CTRL"]
pub type R = crate::R<u32, super::EXTMEM_ICACHE_AUTOLOAD_CTRL>;
#[doc = "Writer for register EXTMEM_ICACHE_AUTOLOAD_CTRL"]
pub type W = crate::W<u32, super::EXTMEM_ICACHE_AUTOLOAD_CTRL>;
#[doc = "Register EXTMEM_ICACHE_AUTOLOAD_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_ICACHE_AUTOLOAD_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_AUTOLOAD_RQST`"]
pub type EXTMEM_ICACHE_AUTOLOAD_RQST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_AUTOLOAD_RQST`"]
pub struct EXTMEM_ICACHE_AUTOLOAD_RQST_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_AUTOLOAD_RQST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_AUTOLOAD_ORDER`"]
pub type EXTMEM_ICACHE_AUTOLOAD_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_AUTOLOAD_ORDER`"]
pub struct EXTMEM_ICACHE_AUTOLOAD_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_AUTOLOAD_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_AUTOLOAD_DONE`"]
pub type EXTMEM_ICACHE_AUTOLOAD_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_ICACHE_AUTOLOAD_ENA`"]
pub type EXTMEM_ICACHE_AUTOLOAD_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_AUTOLOAD_ENA`"]
pub struct EXTMEM_ICACHE_AUTOLOAD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_AUTOLOAD_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA`"]
pub type EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA`"]
pub struct EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA`"]
pub type EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA`"]
pub struct EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA_W<'a> {
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
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn extmem_icache_autoload_rqst(&self) -> EXTMEM_ICACHE_AUTOLOAD_RQST_R {
        EXTMEM_ICACHE_AUTOLOAD_RQST_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn extmem_icache_autoload_order(&self) -> EXTMEM_ICACHE_AUTOLOAD_ORDER_R {
        EXTMEM_ICACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn extmem_icache_autoload_done(&self) -> EXTMEM_ICACHE_AUTOLOAD_DONE_R {
        EXTMEM_ICACHE_AUTOLOAD_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_icache_autoload_ena(&self) -> EXTMEM_ICACHE_AUTOLOAD_ENA_R {
        EXTMEM_ICACHE_AUTOLOAD_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_autoload_sct1_ena(&self) -> EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA_R {
        EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_autoload_sct0_ena(&self) -> EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA_R {
        EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn extmem_icache_autoload_rqst(&mut self) -> EXTMEM_ICACHE_AUTOLOAD_RQST_W {
        EXTMEM_ICACHE_AUTOLOAD_RQST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn extmem_icache_autoload_order(&mut self) -> EXTMEM_ICACHE_AUTOLOAD_ORDER_W {
        EXTMEM_ICACHE_AUTOLOAD_ORDER_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_icache_autoload_ena(&mut self) -> EXTMEM_ICACHE_AUTOLOAD_ENA_W {
        EXTMEM_ICACHE_AUTOLOAD_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_autoload_sct1_ena(&mut self) -> EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA_W {
        EXTMEM_ICACHE_AUTOLOAD_SCT1_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_autoload_sct0_ena(&mut self) -> EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA_W {
        EXTMEM_ICACHE_AUTOLOAD_SCT0_ENA_W { w: self }
    }
}
