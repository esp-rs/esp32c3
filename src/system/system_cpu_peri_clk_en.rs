#[doc = "Reader of register SYSTEM_CPU_PERI_CLK_EN"]
pub type R = crate::R<u32, super::SYSTEM_CPU_PERI_CLK_EN>;
#[doc = "Writer for register SYSTEM_CPU_PERI_CLK_EN"]
pub type W = crate::W<u32, super::SYSTEM_CPU_PERI_CLK_EN>;
#[doc = "Register SYSTEM_CPU_PERI_CLK_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_CPU_PERI_CLK_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_CLK_EN_DEDICATED_GPIO`"]
pub type SYSTEM_CLK_EN_DEDICATED_GPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CLK_EN_DEDICATED_GPIO`"]
pub struct SYSTEM_CLK_EN_DEDICATED_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CLK_EN_DEDICATED_GPIO_W<'a> {
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
#[doc = "Reader of field `SYSTEM_CLK_EN_ASSIST_DEBUG`"]
pub type SYSTEM_CLK_EN_ASSIST_DEBUG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CLK_EN_ASSIST_DEBUG`"]
pub struct SYSTEM_CLK_EN_ASSIST_DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CLK_EN_ASSIST_DEBUG_W<'a> {
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
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn system_clk_en_dedicated_gpio(&self) -> SYSTEM_CLK_EN_DEDICATED_GPIO_R {
        SYSTEM_CLK_EN_DEDICATED_GPIO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn system_clk_en_assist_debug(&self) -> SYSTEM_CLK_EN_ASSIST_DEBUG_R {
        SYSTEM_CLK_EN_ASSIST_DEBUG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn system_clk_en_dedicated_gpio(&mut self) -> SYSTEM_CLK_EN_DEDICATED_GPIO_W {
        SYSTEM_CLK_EN_DEDICATED_GPIO_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn system_clk_en_assist_debug(&mut self) -> SYSTEM_CLK_EN_ASSIST_DEBUG_W {
        SYSTEM_CLK_EN_ASSIST_DEBUG_W { w: self }
    }
}
