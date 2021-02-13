#[doc = "Reader of register SYSTEM_CPU_PER_CONF"]
pub type R = crate::R<u32, super::SYSTEM_CPU_PER_CONF>;
#[doc = "Writer for register SYSTEM_CPU_PER_CONF"]
pub type W = crate::W<u32, super::SYSTEM_CPU_PER_CONF>;
#[doc = "Register SYSTEM_CPU_PER_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_CPU_PER_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_CPU_WAITI_DELAY_NUM`"]
pub type SYSTEM_CPU_WAITI_DELAY_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSTEM_CPU_WAITI_DELAY_NUM`"]
pub struct SYSTEM_CPU_WAITI_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CPU_WAITI_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_CPU_WAIT_MODE_FORCE_ON`"]
pub type SYSTEM_CPU_WAIT_MODE_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CPU_WAIT_MODE_FORCE_ON`"]
pub struct SYSTEM_CPU_WAIT_MODE_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CPU_WAIT_MODE_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `SYSTEM_PLL_FREQ_SEL`"]
pub type SYSTEM_PLL_FREQ_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_PLL_FREQ_SEL`"]
pub struct SYSTEM_PLL_FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_PLL_FREQ_SEL_W<'a> {
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
#[doc = "Reader of field `SYSTEM_CPUPERIOD_SEL`"]
pub type SYSTEM_CPUPERIOD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSTEM_CPUPERIOD_SEL`"]
pub struct SYSTEM_CPUPERIOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CPUPERIOD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn system_cpu_waiti_delay_num(&self) -> SYSTEM_CPU_WAITI_DELAY_NUM_R {
        SYSTEM_CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn system_cpu_wait_mode_force_on(&self) -> SYSTEM_CPU_WAIT_MODE_FORCE_ON_R {
        SYSTEM_CPU_WAIT_MODE_FORCE_ON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_pll_freq_sel(&self) -> SYSTEM_PLL_FREQ_SEL_R {
        SYSTEM_PLL_FREQ_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn system_cpuperiod_sel(&self) -> SYSTEM_CPUPERIOD_SEL_R {
        SYSTEM_CPUPERIOD_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn system_cpu_waiti_delay_num(&mut self) -> SYSTEM_CPU_WAITI_DELAY_NUM_W {
        SYSTEM_CPU_WAITI_DELAY_NUM_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn system_cpu_wait_mode_force_on(&mut self) -> SYSTEM_CPU_WAIT_MODE_FORCE_ON_W {
        SYSTEM_CPU_WAIT_MODE_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_pll_freq_sel(&mut self) -> SYSTEM_PLL_FREQ_SEL_W {
        SYSTEM_PLL_FREQ_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn system_cpuperiod_sel(&mut self) -> SYSTEM_CPUPERIOD_SEL_W {
        SYSTEM_CPUPERIOD_SEL_W { w: self }
    }
}
