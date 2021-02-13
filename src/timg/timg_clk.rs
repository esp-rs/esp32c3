#[doc = "Reader of register TIMG_CLK"]
pub type R = crate::R<u32, super::TIMG_CLK>;
#[doc = "Writer for register TIMG_CLK"]
pub type W = crate::W<u32, super::TIMG_CLK>;
#[doc = "Register TIMG_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_CLK_EN`"]
pub type TIMG_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_CLK_EN`"]
pub struct TIMG_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `TIMG_TIMER_CLK_IS_ACTIVE`"]
pub type TIMG_TIMER_CLK_IS_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_TIMER_CLK_IS_ACTIVE`"]
pub struct TIMG_TIMER_CLK_IS_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_TIMER_CLK_IS_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_CLK_IS_ACTIVE`"]
pub type TIMG_WDT_CLK_IS_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_WDT_CLK_IS_ACTIVE`"]
pub struct TIMG_WDT_CLK_IS_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_CLK_IS_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_clk_en(&self) -> TIMG_CLK_EN_R {
        TIMG_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn timg_timer_clk_is_active(&self) -> TIMG_TIMER_CLK_IS_ACTIVE_R {
        TIMG_TIMER_CLK_IS_ACTIVE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timg_wdt_clk_is_active(&self) -> TIMG_WDT_CLK_IS_ACTIVE_R {
        TIMG_WDT_CLK_IS_ACTIVE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_clk_en(&mut self) -> TIMG_CLK_EN_W {
        TIMG_CLK_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn timg_timer_clk_is_active(&mut self) -> TIMG_TIMER_CLK_IS_ACTIVE_W {
        TIMG_TIMER_CLK_IS_ACTIVE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timg_wdt_clk_is_active(&mut self) -> TIMG_WDT_CLK_IS_ACTIVE_W {
        TIMG_WDT_CLK_IS_ACTIVE_W { w: self }
    }
}
