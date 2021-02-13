#[doc = "Reader of register RTC_CNTL_DBG_MAP"]
pub type R = crate::R<u32, super::RTC_CNTL_DBG_MAP>;
#[doc = "Writer for register RTC_CNTL_DBG_MAP"]
pub type W = crate::W<u32, super::RTC_CNTL_DBG_MAP>;
#[doc = "Register RTC_CNTL_DBG_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_DBG_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN0_FUN_SEL`"]
pub type RTC_CNTL_GPIO_PIN0_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN0_FUN_SEL`"]
pub struct RTC_CNTL_GPIO_PIN0_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN0_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN1_FUN_SEL`"]
pub type RTC_CNTL_GPIO_PIN1_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN1_FUN_SEL`"]
pub struct RTC_CNTL_GPIO_PIN1_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN1_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN2_FUN_SEL`"]
pub type RTC_CNTL_GPIO_PIN2_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN2_FUN_SEL`"]
pub struct RTC_CNTL_GPIO_PIN2_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN2_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN3_FUN_SEL`"]
pub type RTC_CNTL_GPIO_PIN3_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN3_FUN_SEL`"]
pub struct RTC_CNTL_GPIO_PIN3_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN3_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN4_FUN_SEL`"]
pub type RTC_CNTL_GPIO_PIN4_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN4_FUN_SEL`"]
pub struct RTC_CNTL_GPIO_PIN4_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN4_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN5_FUN_SEL`"]
pub type RTC_CNTL_GPIO_PIN5_FUN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN5_FUN_SEL`"]
pub struct RTC_CNTL_GPIO_PIN5_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN5_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN0_MUX_SEL`"]
pub type RTC_CNTL_GPIO_PIN0_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN0_MUX_SEL`"]
pub struct RTC_CNTL_GPIO_PIN0_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN0_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN1_MUX_SEL`"]
pub type RTC_CNTL_GPIO_PIN1_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN1_MUX_SEL`"]
pub struct RTC_CNTL_GPIO_PIN1_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN1_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN2_MUX_SEL`"]
pub type RTC_CNTL_GPIO_PIN2_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN2_MUX_SEL`"]
pub struct RTC_CNTL_GPIO_PIN2_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN2_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN3_MUX_SEL`"]
pub type RTC_CNTL_GPIO_PIN3_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN3_MUX_SEL`"]
pub struct RTC_CNTL_GPIO_PIN3_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN3_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN4_MUX_SEL`"]
pub type RTC_CNTL_GPIO_PIN4_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN4_MUX_SEL`"]
pub struct RTC_CNTL_GPIO_PIN4_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN4_MUX_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN5_MUX_SEL`"]
pub type RTC_CNTL_GPIO_PIN5_MUX_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN5_MUX_SEL`"]
pub struct RTC_CNTL_GPIO_PIN5_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN5_MUX_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_fun_sel(&self) -> RTC_CNTL_GPIO_PIN0_FUN_SEL_R {
        RTC_CNTL_GPIO_PIN0_FUN_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_fun_sel(&self) -> RTC_CNTL_GPIO_PIN1_FUN_SEL_R {
        RTC_CNTL_GPIO_PIN1_FUN_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_fun_sel(&self) -> RTC_CNTL_GPIO_PIN2_FUN_SEL_R {
        RTC_CNTL_GPIO_PIN2_FUN_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_fun_sel(&self) -> RTC_CNTL_GPIO_PIN3_FUN_SEL_R {
        RTC_CNTL_GPIO_PIN3_FUN_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_fun_sel(&self) -> RTC_CNTL_GPIO_PIN4_FUN_SEL_R {
        RTC_CNTL_GPIO_PIN4_FUN_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_fun_sel(&self) -> RTC_CNTL_GPIO_PIN5_FUN_SEL_R {
        RTC_CNTL_GPIO_PIN5_FUN_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_mux_sel(&self) -> RTC_CNTL_GPIO_PIN0_MUX_SEL_R {
        RTC_CNTL_GPIO_PIN0_MUX_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_mux_sel(&self) -> RTC_CNTL_GPIO_PIN1_MUX_SEL_R {
        RTC_CNTL_GPIO_PIN1_MUX_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_mux_sel(&self) -> RTC_CNTL_GPIO_PIN2_MUX_SEL_R {
        RTC_CNTL_GPIO_PIN2_MUX_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_mux_sel(&self) -> RTC_CNTL_GPIO_PIN3_MUX_SEL_R {
        RTC_CNTL_GPIO_PIN3_MUX_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_mux_sel(&self) -> RTC_CNTL_GPIO_PIN4_MUX_SEL_R {
        RTC_CNTL_GPIO_PIN4_MUX_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_mux_sel(&self) -> RTC_CNTL_GPIO_PIN5_MUX_SEL_R {
        RTC_CNTL_GPIO_PIN5_MUX_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_fun_sel(&mut self) -> RTC_CNTL_GPIO_PIN0_FUN_SEL_W {
        RTC_CNTL_GPIO_PIN0_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_fun_sel(&mut self) -> RTC_CNTL_GPIO_PIN1_FUN_SEL_W {
        RTC_CNTL_GPIO_PIN1_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_fun_sel(&mut self) -> RTC_CNTL_GPIO_PIN2_FUN_SEL_W {
        RTC_CNTL_GPIO_PIN2_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_fun_sel(&mut self) -> RTC_CNTL_GPIO_PIN3_FUN_SEL_W {
        RTC_CNTL_GPIO_PIN3_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_fun_sel(&mut self) -> RTC_CNTL_GPIO_PIN4_FUN_SEL_W {
        RTC_CNTL_GPIO_PIN4_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_fun_sel(&mut self) -> RTC_CNTL_GPIO_PIN5_FUN_SEL_W {
        RTC_CNTL_GPIO_PIN5_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_mux_sel(&mut self) -> RTC_CNTL_GPIO_PIN0_MUX_SEL_W {
        RTC_CNTL_GPIO_PIN0_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_mux_sel(&mut self) -> RTC_CNTL_GPIO_PIN1_MUX_SEL_W {
        RTC_CNTL_GPIO_PIN1_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_mux_sel(&mut self) -> RTC_CNTL_GPIO_PIN2_MUX_SEL_W {
        RTC_CNTL_GPIO_PIN2_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_mux_sel(&mut self) -> RTC_CNTL_GPIO_PIN3_MUX_SEL_W {
        RTC_CNTL_GPIO_PIN3_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_mux_sel(&mut self) -> RTC_CNTL_GPIO_PIN4_MUX_SEL_W {
        RTC_CNTL_GPIO_PIN4_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_mux_sel(&mut self) -> RTC_CNTL_GPIO_PIN5_MUX_SEL_W {
        RTC_CNTL_GPIO_PIN5_MUX_SEL_W { w: self }
    }
}
