#[doc = "Reader of register EFUSE_DAC_CONF"]
pub type R = crate::R<u32, super::EFUSE_DAC_CONF>;
#[doc = "Writer for register EFUSE_DAC_CONF"]
pub type W = crate::W<u32, super::EFUSE_DAC_CONF>;
#[doc = "Register EFUSE_DAC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_DAC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_OE_CLR`"]
pub type EFUSE_OE_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_OE_CLR`"]
pub struct EFUSE_OE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_OE_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_DAC_NUM`"]
pub type EFUSE_DAC_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_DAC_NUM`"]
pub struct EFUSE_DAC_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DAC_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | (((value as u32) & 0xff) << 9);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_DAC_CLK_PAD_SEL`"]
pub type EFUSE_DAC_CLK_PAD_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DAC_CLK_PAD_SEL`"]
pub struct EFUSE_DAC_CLK_PAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DAC_CLK_PAD_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_DAC_CLK_DIV`"]
pub type EFUSE_DAC_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_DAC_CLK_DIV`"]
pub struct EFUSE_DAC_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DAC_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn efuse_oe_clr(&self) -> EFUSE_OE_CLR_R {
        EFUSE_OE_CLR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn efuse_dac_num(&self) -> EFUSE_DAC_NUM_R {
        EFUSE_DAC_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_dac_clk_pad_sel(&self) -> EFUSE_DAC_CLK_PAD_SEL_R {
        EFUSE_DAC_CLK_PAD_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn efuse_dac_clk_div(&self) -> EFUSE_DAC_CLK_DIV_R {
        EFUSE_DAC_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn efuse_oe_clr(&mut self) -> EFUSE_OE_CLR_W {
        EFUSE_OE_CLR_W { w: self }
    }
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn efuse_dac_num(&mut self) -> EFUSE_DAC_NUM_W {
        EFUSE_DAC_NUM_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_dac_clk_pad_sel(&mut self) -> EFUSE_DAC_CLK_PAD_SEL_W {
        EFUSE_DAC_CLK_PAD_SEL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn efuse_dac_clk_div(&mut self) -> EFUSE_DAC_CLK_DIV_W {
        EFUSE_DAC_CLK_DIV_W { w: self }
    }
}
