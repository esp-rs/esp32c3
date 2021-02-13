#[doc = "Reader of register ASSIST_DEBUG_CORE_0_INTR_ENA"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_INTR_ENA>;
#[doc = "Writer for register ASSIST_DEBUG_CORE_0_INTR_ENA"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_CORE_0_INTR_ENA>;
#[doc = "Register ASSIST_DEBUG_CORE_0_INTR_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_CORE_0_INTR_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA`"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA`"]
pub type ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA`"]
pub type ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA`"]
pub type ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA`"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA_W<'a> {
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
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA`"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA`"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA_W<'a> {
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
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA`"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA_W<'a> {
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
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA_W<'a> {
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
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA_W<'a> {
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
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA_W<'a> {
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
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA`"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA`"]
pub struct ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA_W<'a> {
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
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn assist_debug_core_0_dram0_exception_monitor_ena(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R {
        ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn assist_debug_core_0_iram0_exception_monitor_ena(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R {
        ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn assist_debug_core_0_sp_spill_max_ena(&self) -> ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA_R {
        ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn assist_debug_core_0_sp_spill_min_ena(&self) -> ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA_R {
        ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_1_wr_ena(&self) -> ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA_R {
        ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_1_rd_ena(&self) -> ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA_R {
        ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_0_wr_ena(&self) -> ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA_R {
        ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_0_rd_ena(&self) -> ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA_R {
        ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_1_wr_ena(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_1_rd_ena(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_0_wr_ena(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_0_rd_ena(
        &self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA_R {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn assist_debug_core_0_dram0_exception_monitor_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W {
        ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn assist_debug_core_0_iram0_exception_monitor_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W {
        ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn assist_debug_core_0_sp_spill_max_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA_W {
        ASSIST_DEBUG_CORE_0_SP_SPILL_MAX_ENA_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn assist_debug_core_0_sp_spill_min_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA_W {
        ASSIST_DEBUG_CORE_0_SP_SPILL_MIN_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_1_wr_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA_W {
        ASSIST_DEBUG_CORE_0_AREA_PIF_1_WR_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_1_rd_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA_W {
        ASSIST_DEBUG_CORE_0_AREA_PIF_1_RD_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_0_wr_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA_W {
        ASSIST_DEBUG_CORE_0_AREA_PIF_0_WR_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_pif_0_rd_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA_W {
        ASSIST_DEBUG_CORE_0_AREA_PIF_0_RD_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_1_wr_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA_W {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_WR_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_1_rd_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA_W {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_RD_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_0_wr_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA_W {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_WR_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_dram0_0_rd_ena(
        &mut self,
    ) -> ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA_W {
        ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_RD_ENA_W { w: self }
    }
}
