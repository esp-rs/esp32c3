#[doc = "Reader of register SYSCON_FRONT_END_MEM_PD"]
pub type R = crate::R<u32, super::SYSCON_FRONT_END_MEM_PD>;
#[doc = "Writer for register SYSCON_FRONT_END_MEM_PD"]
pub type W = crate::W<u32, super::SYSCON_FRONT_END_MEM_PD>;
#[doc = "Register SYSCON_FRONT_END_MEM_PD `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_FRONT_END_MEM_PD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_DC_MEM_FORCE_PD`"]
pub type SYSCON_DC_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_DC_MEM_FORCE_PD`"]
pub struct SYSCON_DC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_DC_MEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_DC_MEM_FORCE_PU`"]
pub type SYSCON_DC_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_DC_MEM_FORCE_PU`"]
pub struct SYSCON_DC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_DC_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `SYSCON_PBUS_MEM_FORCE_PD`"]
pub type SYSCON_PBUS_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_PBUS_MEM_FORCE_PD`"]
pub struct SYSCON_PBUS_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PBUS_MEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_PBUS_MEM_FORCE_PU`"]
pub type SYSCON_PBUS_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_PBUS_MEM_FORCE_PU`"]
pub struct SYSCON_PBUS_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PBUS_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `SYSCON_AGC_MEM_FORCE_PD`"]
pub type SYSCON_AGC_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_AGC_MEM_FORCE_PD`"]
pub struct SYSCON_AGC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_AGC_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `SYSCON_AGC_MEM_FORCE_PU`"]
pub type SYSCON_AGC_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_AGC_MEM_FORCE_PU`"]
pub struct SYSCON_AGC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_AGC_MEM_FORCE_PU_W<'a> {
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn syscon_dc_mem_force_pd(&self) -> SYSCON_DC_MEM_FORCE_PD_R {
        SYSCON_DC_MEM_FORCE_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn syscon_dc_mem_force_pu(&self) -> SYSCON_DC_MEM_FORCE_PU_R {
        SYSCON_DC_MEM_FORCE_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn syscon_pbus_mem_force_pd(&self) -> SYSCON_PBUS_MEM_FORCE_PD_R {
        SYSCON_PBUS_MEM_FORCE_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syscon_pbus_mem_force_pu(&self) -> SYSCON_PBUS_MEM_FORCE_PU_R {
        SYSCON_PBUS_MEM_FORCE_PU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn syscon_agc_mem_force_pd(&self) -> SYSCON_AGC_MEM_FORCE_PD_R {
        SYSCON_AGC_MEM_FORCE_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscon_agc_mem_force_pu(&self) -> SYSCON_AGC_MEM_FORCE_PU_R {
        SYSCON_AGC_MEM_FORCE_PU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn syscon_dc_mem_force_pd(&mut self) -> SYSCON_DC_MEM_FORCE_PD_W {
        SYSCON_DC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn syscon_dc_mem_force_pu(&mut self) -> SYSCON_DC_MEM_FORCE_PU_W {
        SYSCON_DC_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn syscon_pbus_mem_force_pd(&mut self) -> SYSCON_PBUS_MEM_FORCE_PD_W {
        SYSCON_PBUS_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syscon_pbus_mem_force_pu(&mut self) -> SYSCON_PBUS_MEM_FORCE_PU_W {
        SYSCON_PBUS_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn syscon_agc_mem_force_pd(&mut self) -> SYSCON_AGC_MEM_FORCE_PD_W {
        SYSCON_AGC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscon_agc_mem_force_pu(&mut self) -> SYSCON_AGC_MEM_FORCE_PU_W {
        SYSCON_AGC_MEM_FORCE_PU_W { w: self }
    }
}
