#[doc = "Reader of register APB_CTRL_FRONT_END_MEM_PD"]
pub type R = crate::R<u32, super::APB_CTRL_FRONT_END_MEM_PD>;
#[doc = "Writer for register APB_CTRL_FRONT_END_MEM_PD"]
pub type W = crate::W<u32, super::APB_CTRL_FRONT_END_MEM_PD>;
#[doc = "Register APB_CTRL_FRONT_END_MEM_PD `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_FRONT_END_MEM_PD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_DC_MEM_FORCE_PD`"]
pub type APB_CTRL_DC_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_DC_MEM_FORCE_PD`"]
pub struct APB_CTRL_DC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_DC_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_DC_MEM_FORCE_PU`"]
pub type APB_CTRL_DC_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_DC_MEM_FORCE_PU`"]
pub struct APB_CTRL_DC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_DC_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_PBUS_MEM_FORCE_PD`"]
pub type APB_CTRL_PBUS_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_PBUS_MEM_FORCE_PD`"]
pub struct APB_CTRL_PBUS_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_PBUS_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_PBUS_MEM_FORCE_PU`"]
pub type APB_CTRL_PBUS_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_PBUS_MEM_FORCE_PU`"]
pub struct APB_CTRL_PBUS_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_PBUS_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_AGC_MEM_FORCE_PD`"]
pub type APB_CTRL_AGC_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_AGC_MEM_FORCE_PD`"]
pub struct APB_CTRL_AGC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_AGC_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_AGC_MEM_FORCE_PU`"]
pub type APB_CTRL_AGC_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_AGC_MEM_FORCE_PU`"]
pub struct APB_CTRL_AGC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_AGC_MEM_FORCE_PU_W<'a> {
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
    pub fn apb_ctrl_dc_mem_force_pd(&self) -> APB_CTRL_DC_MEM_FORCE_PD_R {
        APB_CTRL_DC_MEM_FORCE_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb_ctrl_dc_mem_force_pu(&self) -> APB_CTRL_DC_MEM_FORCE_PU_R {
        APB_CTRL_DC_MEM_FORCE_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb_ctrl_pbus_mem_force_pd(&self) -> APB_CTRL_PBUS_MEM_FORCE_PD_R {
        APB_CTRL_PBUS_MEM_FORCE_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_ctrl_pbus_mem_force_pu(&self) -> APB_CTRL_PBUS_MEM_FORCE_PU_R {
        APB_CTRL_PBUS_MEM_FORCE_PU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_ctrl_agc_mem_force_pd(&self) -> APB_CTRL_AGC_MEM_FORCE_PD_R {
        APB_CTRL_AGC_MEM_FORCE_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_agc_mem_force_pu(&self) -> APB_CTRL_AGC_MEM_FORCE_PU_R {
        APB_CTRL_AGC_MEM_FORCE_PU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn apb_ctrl_dc_mem_force_pd(&mut self) -> APB_CTRL_DC_MEM_FORCE_PD_W {
        APB_CTRL_DC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb_ctrl_dc_mem_force_pu(&mut self) -> APB_CTRL_DC_MEM_FORCE_PU_W {
        APB_CTRL_DC_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb_ctrl_pbus_mem_force_pd(&mut self) -> APB_CTRL_PBUS_MEM_FORCE_PD_W {
        APB_CTRL_PBUS_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_ctrl_pbus_mem_force_pu(&mut self) -> APB_CTRL_PBUS_MEM_FORCE_PU_W {
        APB_CTRL_PBUS_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_ctrl_agc_mem_force_pd(&mut self) -> APB_CTRL_AGC_MEM_FORCE_PD_W {
        APB_CTRL_AGC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_agc_mem_force_pu(&mut self) -> APB_CTRL_AGC_MEM_FORCE_PU_W {
        APB_CTRL_AGC_MEM_FORCE_PU_W { w: self }
    }
}
