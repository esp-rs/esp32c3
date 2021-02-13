#[doc = "Reader of register GPIO_SIGMADELTA_MISC"]
pub type R = crate::R<u32, super::GPIO_SIGMADELTA_MISC>;
#[doc = "Writer for register GPIO_SIGMADELTA_MISC"]
pub type W = crate::W<u32, super::GPIO_SIGMADELTA_MISC>;
#[doc = "Register GPIO_SIGMADELTA_MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_SIGMADELTA_MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_SPI_SWAP`"]
pub type GPIO_SPI_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_SPI_SWAP`"]
pub struct GPIO_SPI_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SPI_SWAP_W<'a> {
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
#[doc = "Reader of field `GPIO_FUNCTION_CLK_EN`"]
pub type GPIO_FUNCTION_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_FUNCTION_CLK_EN`"]
pub struct GPIO_FUNCTION_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNCTION_CLK_EN_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_spi_swap(&self) -> GPIO_SPI_SWAP_R {
        GPIO_SPI_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio_function_clk_en(&self) -> GPIO_FUNCTION_CLK_EN_R {
        GPIO_FUNCTION_CLK_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_spi_swap(&mut self) -> GPIO_SPI_SWAP_W {
        GPIO_SPI_SWAP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio_function_clk_en(&mut self) -> GPIO_FUNCTION_CLK_EN_W {
        GPIO_FUNCTION_CLK_EN_W { w: self }
    }
}
