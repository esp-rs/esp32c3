#[doc = "Reader of register TIMG_INT_RAW_TIMERS"]
pub type R = crate::R<u32, super::TIMG_INT_RAW_TIMERS>;
#[doc = "Writer for register TIMG_INT_RAW_TIMERS"]
pub type W = crate::W<u32, super::TIMG_INT_RAW_TIMERS>;
#[doc = "Register TIMG_INT_RAW_TIMERS `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_INT_RAW_TIMERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_WDT_INT_RAW`"]
pub type TIMG_WDT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_WDT_INT_RAW`"]
pub struct TIMG_WDT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TIMG_T0_INT_RAW`"]
pub type TIMG_T0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_INT_RAW`"]
pub struct TIMG_T0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_INT_RAW_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timg_wdt_int_raw(&self) -> TIMG_WDT_INT_RAW_R {
        TIMG_WDT_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timg_t0_int_raw(&self) -> TIMG_T0_INT_RAW_R {
        TIMG_T0_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timg_wdt_int_raw(&mut self) -> TIMG_WDT_INT_RAW_W {
        TIMG_WDT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timg_t0_int_raw(&mut self) -> TIMG_T0_INT_RAW_W {
        TIMG_T0_INT_RAW_W { w: self }
    }
}
