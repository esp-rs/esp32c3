#[doc = "Reader of register SYSTEM_SYSCLK_CONF"]
pub type R = crate::R<u32, super::SYSTEM_SYSCLK_CONF>;
#[doc = "Writer for register SYSTEM_SYSCLK_CONF"]
pub type W = crate::W<u32, super::SYSTEM_SYSCLK_CONF>;
#[doc = "Register SYSTEM_SYSCLK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_SYSCLK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_CLK_DIV_EN`"]
pub type SYSTEM_CLK_DIV_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSTEM_CLK_XTAL_FREQ`"]
pub type SYSTEM_CLK_XTAL_FREQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `SYSTEM_SOC_CLK_SEL`"]
pub type SYSTEM_SOC_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSTEM_SOC_CLK_SEL`"]
pub struct SYSTEM_SOC_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SOC_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_PRE_DIV_CNT`"]
pub type SYSTEM_PRE_DIV_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSTEM_PRE_DIV_CNT`"]
pub struct SYSTEM_PRE_DIV_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_PRE_DIV_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn system_clk_div_en(&self) -> SYSTEM_CLK_DIV_EN_R {
        SYSTEM_CLK_DIV_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 12:18"]
    #[inline(always)]
    pub fn system_clk_xtal_freq(&self) -> SYSTEM_CLK_XTAL_FREQ_R {
        SYSTEM_CLK_XTAL_FREQ_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn system_soc_clk_sel(&self) -> SYSTEM_SOC_CLK_SEL_R {
        SYSTEM_SOC_CLK_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn system_pre_div_cnt(&self) -> SYSTEM_PRE_DIV_CNT_R {
        SYSTEM_PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn system_soc_clk_sel(&mut self) -> SYSTEM_SOC_CLK_SEL_W {
        SYSTEM_SOC_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn system_pre_div_cnt(&mut self) -> SYSTEM_PRE_DIV_CNT_W {
        SYSTEM_PRE_DIV_CNT_W { w: self }
    }
}
