#[doc = "Reader of register APB_CTRL_CLK_OUT_EN"]
pub type R = crate::R<u32, super::APB_CTRL_CLK_OUT_EN>;
#[doc = "Writer for register APB_CTRL_CLK_OUT_EN"]
pub type W = crate::W<u32, super::APB_CTRL_CLK_OUT_EN>;
#[doc = "Register APB_CTRL_CLK_OUT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_CLK_OUT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_CLK_XTAL_OEN`"]
pub type APB_CTRL_CLK_XTAL_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK_XTAL_OEN`"]
pub struct APB_CTRL_CLK_XTAL_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK_XTAL_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK40X_BB_OEN`"]
pub type APB_CTRL_CLK40X_BB_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK40X_BB_OEN`"]
pub struct APB_CTRL_CLK40X_BB_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK40X_BB_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK_DAC_CPU_OEN`"]
pub type APB_CTRL_CLK_DAC_CPU_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK_DAC_CPU_OEN`"]
pub struct APB_CTRL_CLK_DAC_CPU_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK_DAC_CPU_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK_ADC_INF_OEN`"]
pub type APB_CTRL_CLK_ADC_INF_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK_ADC_INF_OEN`"]
pub struct APB_CTRL_CLK_ADC_INF_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK_ADC_INF_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK_320M_OEN`"]
pub type APB_CTRL_CLK_320M_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK_320M_OEN`"]
pub struct APB_CTRL_CLK_320M_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK_320M_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK160_OEN`"]
pub type APB_CTRL_CLK160_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK160_OEN`"]
pub struct APB_CTRL_CLK160_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK160_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK80_OEN`"]
pub type APB_CTRL_CLK80_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK80_OEN`"]
pub struct APB_CTRL_CLK80_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK80_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK_BB_OEN`"]
pub type APB_CTRL_CLK_BB_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK_BB_OEN`"]
pub struct APB_CTRL_CLK_BB_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK_BB_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK44_OEN`"]
pub type APB_CTRL_CLK44_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK44_OEN`"]
pub struct APB_CTRL_CLK44_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK44_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK22_OEN`"]
pub type APB_CTRL_CLK22_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK22_OEN`"]
pub struct APB_CTRL_CLK22_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK22_OEN_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_CLK20_OEN`"]
pub type APB_CTRL_CLK20_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK20_OEN`"]
pub struct APB_CTRL_CLK20_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK20_OEN_W<'a> {
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
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn apb_ctrl_clk_xtal_oen(&self) -> APB_CTRL_CLK_XTAL_OEN_R {
        APB_CTRL_CLK_XTAL_OEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn apb_ctrl_clk40x_bb_oen(&self) -> APB_CTRL_CLK40X_BB_OEN_R {
        APB_CTRL_CLK40X_BB_OEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn apb_ctrl_clk_dac_cpu_oen(&self) -> APB_CTRL_CLK_DAC_CPU_OEN_R {
        APB_CTRL_CLK_DAC_CPU_OEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn apb_ctrl_clk_adc_inf_oen(&self) -> APB_CTRL_CLK_ADC_INF_OEN_R {
        APB_CTRL_CLK_ADC_INF_OEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn apb_ctrl_clk_320m_oen(&self) -> APB_CTRL_CLK_320M_OEN_R {
        APB_CTRL_CLK_320M_OEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn apb_ctrl_clk160_oen(&self) -> APB_CTRL_CLK160_OEN_R {
        APB_CTRL_CLK160_OEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb_ctrl_clk80_oen(&self) -> APB_CTRL_CLK80_OEN_R {
        APB_CTRL_CLK80_OEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb_ctrl_clk_bb_oen(&self) -> APB_CTRL_CLK_BB_OEN_R {
        APB_CTRL_CLK_BB_OEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_ctrl_clk44_oen(&self) -> APB_CTRL_CLK44_OEN_R {
        APB_CTRL_CLK44_OEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_ctrl_clk22_oen(&self) -> APB_CTRL_CLK22_OEN_R {
        APB_CTRL_CLK22_OEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_clk20_oen(&self) -> APB_CTRL_CLK20_OEN_R {
        APB_CTRL_CLK20_OEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn apb_ctrl_clk_xtal_oen(&mut self) -> APB_CTRL_CLK_XTAL_OEN_W {
        APB_CTRL_CLK_XTAL_OEN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn apb_ctrl_clk40x_bb_oen(&mut self) -> APB_CTRL_CLK40X_BB_OEN_W {
        APB_CTRL_CLK40X_BB_OEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn apb_ctrl_clk_dac_cpu_oen(&mut self) -> APB_CTRL_CLK_DAC_CPU_OEN_W {
        APB_CTRL_CLK_DAC_CPU_OEN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn apb_ctrl_clk_adc_inf_oen(&mut self) -> APB_CTRL_CLK_ADC_INF_OEN_W {
        APB_CTRL_CLK_ADC_INF_OEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn apb_ctrl_clk_320m_oen(&mut self) -> APB_CTRL_CLK_320M_OEN_W {
        APB_CTRL_CLK_320M_OEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn apb_ctrl_clk160_oen(&mut self) -> APB_CTRL_CLK160_OEN_W {
        APB_CTRL_CLK160_OEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb_ctrl_clk80_oen(&mut self) -> APB_CTRL_CLK80_OEN_W {
        APB_CTRL_CLK80_OEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb_ctrl_clk_bb_oen(&mut self) -> APB_CTRL_CLK_BB_OEN_W {
        APB_CTRL_CLK_BB_OEN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_ctrl_clk44_oen(&mut self) -> APB_CTRL_CLK44_OEN_W {
        APB_CTRL_CLK44_OEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_ctrl_clk22_oen(&mut self) -> APB_CTRL_CLK22_OEN_W {
        APB_CTRL_CLK22_OEN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_clk20_oen(&mut self) -> APB_CTRL_CLK20_OEN_W {
        APB_CTRL_CLK20_OEN_W { w: self }
    }
}
