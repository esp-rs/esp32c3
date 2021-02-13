#[doc = "Reader of register I2S_TX_CLKM_CONF"]
pub type R = crate::R<u32, super::I2S_TX_CLKM_CONF>;
#[doc = "Writer for register I2S_TX_CLKM_CONF"]
pub type W = crate::W<u32, super::I2S_TX_CLKM_CONF>;
#[doc = "Register I2S_TX_CLKM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_TX_CLKM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_CLK_EN`"]
pub type I2S_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CLK_EN`"]
pub struct I2S_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CLK_EN_W<'a> {
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
#[doc = "Reader of field `I2S_TX_CLK_SEL`"]
pub type I2S_TX_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_CLK_SEL`"]
pub struct I2S_TX_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_CLK_ACTIVE`"]
pub type I2S_TX_CLK_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_CLK_ACTIVE`"]
pub struct I2S_TX_CLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CLK_ACTIVE_W<'a> {
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
#[doc = "Reader of field `I2S_TX_CLKM_DIV_NUM`"]
pub type I2S_TX_CLKM_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_CLKM_DIV_NUM`"]
pub struct I2S_TX_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn i2s_clk_en(&self) -> I2S_CLK_EN_R {
        I2S_CLK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn i2s_tx_clk_sel(&self) -> I2S_TX_CLK_SEL_R {
        I2S_TX_CLK_SEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn i2s_tx_clk_active(&self) -> I2S_TX_CLK_ACTIVE_R {
        I2S_TX_CLK_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s_tx_clkm_div_num(&self) -> I2S_TX_CLKM_DIV_NUM_R {
        I2S_TX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn i2s_clk_en(&mut self) -> I2S_CLK_EN_W {
        I2S_CLK_EN_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn i2s_tx_clk_sel(&mut self) -> I2S_TX_CLK_SEL_W {
        I2S_TX_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn i2s_tx_clk_active(&mut self) -> I2S_TX_CLK_ACTIVE_W {
        I2S_TX_CLK_ACTIVE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s_tx_clkm_div_num(&mut self) -> I2S_TX_CLKM_DIV_NUM_W {
        I2S_TX_CLKM_DIV_NUM_W { w: self }
    }
}
