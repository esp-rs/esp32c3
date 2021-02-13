#[doc = "Reader of register RTC_CNTL_PAD_HOLD"]
pub type R = crate::R<u32, super::RTC_CNTL_PAD_HOLD>;
#[doc = "Writer for register RTC_CNTL_PAD_HOLD"]
pub type W = crate::W<u32, super::RTC_CNTL_PAD_HOLD>;
#[doc = "Register RTC_CNTL_PAD_HOLD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_PAD_HOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN5_HOLD`"]
pub type RTC_CNTL_GPIO_PIN5_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN5_HOLD`"]
pub struct RTC_CNTL_GPIO_PIN5_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN5_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN4_HOLD`"]
pub type RTC_CNTL_GPIO_PIN4_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN4_HOLD`"]
pub struct RTC_CNTL_GPIO_PIN4_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN4_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN3_HOLD`"]
pub type RTC_CNTL_GPIO_PIN3_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN3_HOLD`"]
pub struct RTC_CNTL_GPIO_PIN3_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN3_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN2_HOLD`"]
pub type RTC_CNTL_GPIO_PIN2_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN2_HOLD`"]
pub struct RTC_CNTL_GPIO_PIN2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN2_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN1_HOLD`"]
pub type RTC_CNTL_GPIO_PIN1_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN1_HOLD`"]
pub struct RTC_CNTL_GPIO_PIN1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN1_HOLD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_GPIO_PIN0_HOLD`"]
pub type RTC_CNTL_GPIO_PIN0_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_GPIO_PIN0_HOLD`"]
pub struct RTC_CNTL_GPIO_PIN0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_GPIO_PIN0_HOLD_W<'a> {
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_hold(&self) -> RTC_CNTL_GPIO_PIN5_HOLD_R {
        RTC_CNTL_GPIO_PIN5_HOLD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_hold(&self) -> RTC_CNTL_GPIO_PIN4_HOLD_R {
        RTC_CNTL_GPIO_PIN4_HOLD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_hold(&self) -> RTC_CNTL_GPIO_PIN3_HOLD_R {
        RTC_CNTL_GPIO_PIN3_HOLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_hold(&self) -> RTC_CNTL_GPIO_PIN2_HOLD_R {
        RTC_CNTL_GPIO_PIN2_HOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_hold(&self) -> RTC_CNTL_GPIO_PIN1_HOLD_R {
        RTC_CNTL_GPIO_PIN1_HOLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_hold(&self) -> RTC_CNTL_GPIO_PIN0_HOLD_R {
        RTC_CNTL_GPIO_PIN0_HOLD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin5_hold(&mut self) -> RTC_CNTL_GPIO_PIN5_HOLD_W {
        RTC_CNTL_GPIO_PIN5_HOLD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin4_hold(&mut self) -> RTC_CNTL_GPIO_PIN4_HOLD_W {
        RTC_CNTL_GPIO_PIN4_HOLD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin3_hold(&mut self) -> RTC_CNTL_GPIO_PIN3_HOLD_W {
        RTC_CNTL_GPIO_PIN3_HOLD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin2_hold(&mut self) -> RTC_CNTL_GPIO_PIN2_HOLD_W {
        RTC_CNTL_GPIO_PIN2_HOLD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin1_hold(&mut self) -> RTC_CNTL_GPIO_PIN1_HOLD_W {
        RTC_CNTL_GPIO_PIN1_HOLD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_cntl_gpio_pin0_hold(&mut self) -> RTC_CNTL_GPIO_PIN0_HOLD_W {
        RTC_CNTL_GPIO_PIN0_HOLD_W { w: self }
    }
}
