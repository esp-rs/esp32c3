#[doc = "Reader of register SYSCON_CLK_OUT_EN"]
pub type R = crate::R<u32, super::SYSCON_CLK_OUT_EN>;
#[doc = "Writer for register SYSCON_CLK_OUT_EN"]
pub type W = crate::W<u32, super::SYSCON_CLK_OUT_EN>;
#[doc = "Register SYSCON_CLK_OUT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_CLK_OUT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_CLK_XTAL_OEN`"]
pub type SYSCON_CLK_XTAL_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK_XTAL_OEN`"]
pub struct SYSCON_CLK_XTAL_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK_XTAL_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK40X_BB_OEN`"]
pub type SYSCON_CLK40X_BB_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK40X_BB_OEN`"]
pub struct SYSCON_CLK40X_BB_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK40X_BB_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK_DAC_CPU_OEN`"]
pub type SYSCON_CLK_DAC_CPU_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK_DAC_CPU_OEN`"]
pub struct SYSCON_CLK_DAC_CPU_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK_DAC_CPU_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK_ADC_INF_OEN`"]
pub type SYSCON_CLK_ADC_INF_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK_ADC_INF_OEN`"]
pub struct SYSCON_CLK_ADC_INF_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK_ADC_INF_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK_320M_OEN`"]
pub type SYSCON_CLK_320M_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK_320M_OEN`"]
pub struct SYSCON_CLK_320M_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK_320M_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK160_OEN`"]
pub type SYSCON_CLK160_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK160_OEN`"]
pub struct SYSCON_CLK160_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK160_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK80_OEN`"]
pub type SYSCON_CLK80_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK80_OEN`"]
pub struct SYSCON_CLK80_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK80_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK_BB_OEN`"]
pub type SYSCON_CLK_BB_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK_BB_OEN`"]
pub struct SYSCON_CLK_BB_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK_BB_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK44_OEN`"]
pub type SYSCON_CLK44_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK44_OEN`"]
pub struct SYSCON_CLK44_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK44_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK22_OEN`"]
pub type SYSCON_CLK22_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK22_OEN`"]
pub struct SYSCON_CLK22_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK22_OEN_W<'a> {
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
#[doc = "Reader of field `SYSCON_CLK20_OEN`"]
pub type SYSCON_CLK20_OEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_CLK20_OEN`"]
pub struct SYSCON_CLK20_OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CLK20_OEN_W<'a> {
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
    pub fn syscon_clk_xtal_oen(&self) -> SYSCON_CLK_XTAL_OEN_R {
        SYSCON_CLK_XTAL_OEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn syscon_clk40x_bb_oen(&self) -> SYSCON_CLK40X_BB_OEN_R {
        SYSCON_CLK40X_BB_OEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn syscon_clk_dac_cpu_oen(&self) -> SYSCON_CLK_DAC_CPU_OEN_R {
        SYSCON_CLK_DAC_CPU_OEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn syscon_clk_adc_inf_oen(&self) -> SYSCON_CLK_ADC_INF_OEN_R {
        SYSCON_CLK_ADC_INF_OEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn syscon_clk_320m_oen(&self) -> SYSCON_CLK_320M_OEN_R {
        SYSCON_CLK_320M_OEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn syscon_clk160_oen(&self) -> SYSCON_CLK160_OEN_R {
        SYSCON_CLK160_OEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn syscon_clk80_oen(&self) -> SYSCON_CLK80_OEN_R {
        SYSCON_CLK80_OEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn syscon_clk_bb_oen(&self) -> SYSCON_CLK_BB_OEN_R {
        SYSCON_CLK_BB_OEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syscon_clk44_oen(&self) -> SYSCON_CLK44_OEN_R {
        SYSCON_CLK44_OEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn syscon_clk22_oen(&self) -> SYSCON_CLK22_OEN_R {
        SYSCON_CLK22_OEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscon_clk20_oen(&self) -> SYSCON_CLK20_OEN_R {
        SYSCON_CLK20_OEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn syscon_clk_xtal_oen(&mut self) -> SYSCON_CLK_XTAL_OEN_W {
        SYSCON_CLK_XTAL_OEN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn syscon_clk40x_bb_oen(&mut self) -> SYSCON_CLK40X_BB_OEN_W {
        SYSCON_CLK40X_BB_OEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn syscon_clk_dac_cpu_oen(&mut self) -> SYSCON_CLK_DAC_CPU_OEN_W {
        SYSCON_CLK_DAC_CPU_OEN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn syscon_clk_adc_inf_oen(&mut self) -> SYSCON_CLK_ADC_INF_OEN_W {
        SYSCON_CLK_ADC_INF_OEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn syscon_clk_320m_oen(&mut self) -> SYSCON_CLK_320M_OEN_W {
        SYSCON_CLK_320M_OEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn syscon_clk160_oen(&mut self) -> SYSCON_CLK160_OEN_W {
        SYSCON_CLK160_OEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn syscon_clk80_oen(&mut self) -> SYSCON_CLK80_OEN_W {
        SYSCON_CLK80_OEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn syscon_clk_bb_oen(&mut self) -> SYSCON_CLK_BB_OEN_W {
        SYSCON_CLK_BB_OEN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syscon_clk44_oen(&mut self) -> SYSCON_CLK44_OEN_W {
        SYSCON_CLK44_OEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn syscon_clk22_oen(&mut self) -> SYSCON_CLK22_OEN_W {
        SYSCON_CLK22_OEN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscon_clk20_oen(&mut self) -> SYSCON_CLK20_OEN_W {
        SYSCON_CLK20_OEN_W { w: self }
    }
}
