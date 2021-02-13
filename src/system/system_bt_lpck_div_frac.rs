#[doc = "Reader of register SYSTEM_BT_LPCK_DIV_FRAC"]
pub type R = crate::R<u32, super::SYSTEM_BT_LPCK_DIV_FRAC>;
#[doc = "Writer for register SYSTEM_BT_LPCK_DIV_FRAC"]
pub type W = crate::W<u32, super::SYSTEM_BT_LPCK_DIV_FRAC>;
#[doc = "Register SYSTEM_BT_LPCK_DIV_FRAC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_BT_LPCK_DIV_FRAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_LPCLK_RTC_EN`"]
pub type SYSTEM_LPCLK_RTC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_LPCLK_RTC_EN`"]
pub struct SYSTEM_LPCLK_RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_LPCLK_RTC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_LPCLK_SEL_XTAL32K`"]
pub type SYSTEM_LPCLK_SEL_XTAL32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_LPCLK_SEL_XTAL32K`"]
pub struct SYSTEM_LPCLK_SEL_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_LPCLK_SEL_XTAL32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_LPCLK_SEL_XTAL`"]
pub type SYSTEM_LPCLK_SEL_XTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_LPCLK_SEL_XTAL`"]
pub struct SYSTEM_LPCLK_SEL_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_LPCLK_SEL_XTAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_LPCLK_SEL_8M`"]
pub type SYSTEM_LPCLK_SEL_8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_LPCLK_SEL_8M`"]
pub struct SYSTEM_LPCLK_SEL_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_LPCLK_SEL_8M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_LPCLK_SEL_RTC_SLOW`"]
pub type SYSTEM_LPCLK_SEL_RTC_SLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_LPCLK_SEL_RTC_SLOW`"]
pub struct SYSTEM_LPCLK_SEL_RTC_SLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_LPCLK_SEL_RTC_SLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_BT_LPCK_DIV_A`"]
pub type SYSTEM_BT_LPCK_DIV_A_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSTEM_BT_LPCK_DIV_A`"]
pub struct SYSTEM_BT_LPCK_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_BT_LPCK_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | (((value as u32) & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_BT_LPCK_DIV_B`"]
pub type SYSTEM_BT_LPCK_DIV_B_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSTEM_BT_LPCK_DIV_B`"]
pub struct SYSTEM_BT_LPCK_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_BT_LPCK_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn system_lpclk_rtc_en(&self) -> SYSTEM_LPCLK_RTC_EN_R {
        SYSTEM_LPCLK_RTC_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn system_lpclk_sel_xtal32k(&self) -> SYSTEM_LPCLK_SEL_XTAL32K_R {
        SYSTEM_LPCLK_SEL_XTAL32K_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn system_lpclk_sel_xtal(&self) -> SYSTEM_LPCLK_SEL_XTAL_R {
        SYSTEM_LPCLK_SEL_XTAL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn system_lpclk_sel_8m(&self) -> SYSTEM_LPCLK_SEL_8M_R {
        SYSTEM_LPCLK_SEL_8M_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn system_lpclk_sel_rtc_slow(&self) -> SYSTEM_LPCLK_SEL_RTC_SLOW_R {
        SYSTEM_LPCLK_SEL_RTC_SLOW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 12:23"]
    #[inline(always)]
    pub fn system_bt_lpck_div_a(&self) -> SYSTEM_BT_LPCK_DIV_A_R {
        SYSTEM_BT_LPCK_DIV_A_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn system_bt_lpck_div_b(&self) -> SYSTEM_BT_LPCK_DIV_B_R {
        SYSTEM_BT_LPCK_DIV_B_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn system_lpclk_rtc_en(&mut self) -> SYSTEM_LPCLK_RTC_EN_W {
        SYSTEM_LPCLK_RTC_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn system_lpclk_sel_xtal32k(&mut self) -> SYSTEM_LPCLK_SEL_XTAL32K_W {
        SYSTEM_LPCLK_SEL_XTAL32K_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn system_lpclk_sel_xtal(&mut self) -> SYSTEM_LPCLK_SEL_XTAL_W {
        SYSTEM_LPCLK_SEL_XTAL_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn system_lpclk_sel_8m(&mut self) -> SYSTEM_LPCLK_SEL_8M_W {
        SYSTEM_LPCLK_SEL_8M_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn system_lpclk_sel_rtc_slow(&mut self) -> SYSTEM_LPCLK_SEL_RTC_SLOW_W {
        SYSTEM_LPCLK_SEL_RTC_SLOW_W { w: self }
    }
    #[doc = "Bits 12:23"]
    #[inline(always)]
    pub fn system_bt_lpck_div_a(&mut self) -> SYSTEM_BT_LPCK_DIV_A_W {
        SYSTEM_BT_LPCK_DIV_A_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn system_bt_lpck_div_b(&mut self) -> SYSTEM_BT_LPCK_DIV_B_W {
        SYSTEM_BT_LPCK_DIV_B_W { w: self }
    }
}
