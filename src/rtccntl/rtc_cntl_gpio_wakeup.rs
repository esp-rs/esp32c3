#[doc = "Reader of register RTC_CNTL_GPIO_WAKEUP"]
pub type R = crate::R<u32, super::RTC_CNTL_GPIO_WAKEUP>;
#[doc = "Writer for register RTC_CNTL_GPIO_WAKEUP"]
pub type W = crate::W<u32, super::RTC_CNTL_GPIO_WAKEUP>;
#[doc = "Register RTC_CNTL_GPIO_WAKEUP `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_GPIO_WAKEUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE`"]
pub type RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE`"]
pub struct RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE`"]
pub type RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE`"]
pub struct RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE`"]
pub type RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE`"]
pub struct RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE`"]
pub type RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE`"]
pub struct RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE`"]
pub type RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE`"]
pub struct RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE`"]
pub type RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE`"]
pub struct RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN0_INT_TYPE`"]
pub type RTC_CNTL_GPIO_PIN0_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN0_INT_TYPE`"]
pub struct RTC_CNTL_GPIO_PIN0_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN0_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN1_INT_TYPE`"]
pub type RTC_CNTL_GPIO_PIN1_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN1_INT_TYPE`"]
pub struct RTC_CNTL_GPIO_PIN1_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN1_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN2_INT_TYPE`"]
pub type RTC_CNTL_GPIO_PIN2_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN2_INT_TYPE`"]
pub struct RTC_CNTL_GPIO_PIN2_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN2_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN3_INT_TYPE`"]
pub type RTC_CNTL_GPIO_PIN3_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN3_INT_TYPE`"]
pub struct RTC_CNTL_GPIO_PIN3_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN3_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN4_INT_TYPE`"]
pub type RTC_CNTL_GPIO_PIN4_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN4_INT_TYPE`"]
pub struct RTC_CNTL_GPIO_PIN4_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN4_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN5_INT_TYPE`"]
pub type RTC_CNTL_GPIO_PIN5_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN5_INT_TYPE`"]
pub struct RTC_CNTL_GPIO_PIN5_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN5_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN_CLK_GATE`"]
pub type RTC_CNTL_GPIO_PIN_CLK_GATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN_CLK_GATE`"]
pub struct RTC_CNTL_GPIO_PIN_CLK_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN_CLK_GATE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_WAKEUP_STATUS_CLR`"]
pub type RTC_CNTL_GPIO_WAKEUP_STATUS_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_WAKEUP_STATUS_CLR`"]
pub struct RTC_CNTL_GPIO_WAKEUP_STATUS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_WAKEUP_STATUS_CLR_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_WAKEUP_STATUS`"]
pub type RTC_CNTL_GPIO_WAKEUP_STATUS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_wakeup_enable(&self) -> RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE_R {
        RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_wakeup_enable(&self) -> RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE_R {
        RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_wakeup_enable(&self) -> RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE_R {
        RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_wakeup_enable(&self) -> RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE_R {
        RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_wakeup_enable(&self) -> RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE_R {
        RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_wakeup_enable(&self) -> RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE_R {
        RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_int_type(&self) -> RTC_CNTL_GPIO_PIN0_INT_TYPE_R {
        RTC_CNTL_GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_int_type(&self) -> RTC_CNTL_GPIO_PIN1_INT_TYPE_R {
        RTC_CNTL_GPIO_PIN1_INT_TYPE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_int_type(&self) -> RTC_CNTL_GPIO_PIN2_INT_TYPE_R {
        RTC_CNTL_GPIO_PIN2_INT_TYPE_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_int_type(&self) -> RTC_CNTL_GPIO_PIN3_INT_TYPE_R {
        RTC_CNTL_GPIO_PIN3_INT_TYPE_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_int_type(&self) -> RTC_CNTL_GPIO_PIN4_INT_TYPE_R {
        RTC_CNTL_GPIO_PIN4_INT_TYPE_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_int_type(&self) -> RTC_CNTL_GPIO_PIN5_INT_TYPE_R {
        RTC_CNTL_GPIO_PIN5_INT_TYPE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin_clk_gate(&self) -> RTC_CNTL_GPIO_PIN_CLK_GATE_R {
        RTC_CNTL_GPIO_PIN_CLK_GATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_wakeup_status_clr(&self) -> RTC_CNTL_GPIO_WAKEUP_STATUS_CLR_R {
        RTC_CNTL_GPIO_WAKEUP_STATUS_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_wakeup_status(&self) -> RTC_CNTL_GPIO_WAKEUP_STATUS_R {
        RTC_CNTL_GPIO_WAKEUP_STATUS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_wakeup_enable(&mut self) -> RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE_W {
        RTC_CNTL_GPIO_PIN0_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_wakeup_enable(&mut self) -> RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE_W {
        RTC_CNTL_GPIO_PIN1_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_wakeup_enable(&mut self) -> RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE_W {
        RTC_CNTL_GPIO_PIN2_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_wakeup_enable(&mut self) -> RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE_W {
        RTC_CNTL_GPIO_PIN3_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_wakeup_enable(&mut self) -> RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE_W {
        RTC_CNTL_GPIO_PIN4_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_wakeup_enable(&mut self) -> RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE_W {
        RTC_CNTL_GPIO_PIN5_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_int_type(&mut self) -> RTC_CNTL_GPIO_PIN0_INT_TYPE_W {
        RTC_CNTL_GPIO_PIN0_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_int_type(&mut self) -> RTC_CNTL_GPIO_PIN1_INT_TYPE_W {
        RTC_CNTL_GPIO_PIN1_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_int_type(&mut self) -> RTC_CNTL_GPIO_PIN2_INT_TYPE_W {
        RTC_CNTL_GPIO_PIN2_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_int_type(&mut self) -> RTC_CNTL_GPIO_PIN3_INT_TYPE_W {
        RTC_CNTL_GPIO_PIN3_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_int_type(&mut self) -> RTC_CNTL_GPIO_PIN4_INT_TYPE_W {
        RTC_CNTL_GPIO_PIN4_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_int_type(&mut self) -> RTC_CNTL_GPIO_PIN5_INT_TYPE_W {
        RTC_CNTL_GPIO_PIN5_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin_clk_gate(&mut self) -> RTC_CNTL_GPIO_PIN_CLK_GATE_W {
        RTC_CNTL_GPIO_PIN_CLK_GATE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_wakeup_status_clr(&mut self) -> RTC_CNTL_GPIO_WAKEUP_STATUS_CLR_W {
        RTC_CNTL_GPIO_WAKEUP_STATUS_CLR_W { w: self }
    }
}
