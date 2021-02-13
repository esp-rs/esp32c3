#[doc = "Writer for register TIMG_INT_CLR_TIMERS"]
pub type W = crate::W<u32, super::TIMG_INT_CLR_TIMERS>;
#[doc = "Register TIMG_INT_CLR_TIMERS `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_INT_CLR_TIMERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TIMG_WDT_INT_CLR`"]
pub struct TIMG_WDT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `TIMG_T0_INT_CLR`"]
pub struct TIMG_T0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timg_wdt_int_clr(&mut self) -> TIMG_WDT_INT_CLR_W {
        TIMG_WDT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timg_t0_int_clr(&mut self) -> TIMG_T0_INT_CLR_W {
        TIMG_T0_INT_CLR_W { w: self }
    }
}
