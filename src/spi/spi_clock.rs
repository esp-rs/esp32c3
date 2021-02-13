#[doc = "Reader of register SPI_CLOCK"]
pub type R = crate::R<u32, super::SPI_CLOCK>;
#[doc = "Writer for register SPI_CLOCK"]
pub type W = crate::W<u32, super::SPI_CLOCK>;
#[doc = "Register SPI_CLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_CLK_EQU_SYSCLK`"]
pub type SPI_CLK_EQU_SYSCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CLK_EQU_SYSCLK`"]
pub struct SPI_CLK_EQU_SYSCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_EQU_SYSCLK_W<'a> {
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
#[doc = "Reader of field `SPI_CLKDIV_PRE`"]
pub type SPI_CLKDIV_PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CLKDIV_PRE`"]
pub struct SPI_CLKDIV_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLKDIV_PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPI_CLKCNT_N`"]
pub type SPI_CLKCNT_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CLKCNT_N`"]
pub struct SPI_CLKCNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLKCNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPI_CLKCNT_H`"]
pub type SPI_CLKCNT_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CLKCNT_H`"]
pub struct SPI_CLKCNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLKCNT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_CLKCNT_L`"]
pub type SPI_CLKCNT_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CLKCNT_L`"]
pub struct SPI_CLKCNT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLKCNT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_clk_equ_sysclk(&self) -> SPI_CLK_EQU_SYSCLK_R {
        SPI_CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn spi_clkdiv_pre(&self) -> SPI_CLKDIV_PRE_R {
        SPI_CLKDIV_PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn spi_clkcnt_n(&self) -> SPI_CLKCNT_N_R {
        SPI_CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn spi_clkcnt_h(&self) -> SPI_CLKCNT_H_R {
        SPI_CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn spi_clkcnt_l(&self) -> SPI_CLKCNT_L_R {
        SPI_CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_clk_equ_sysclk(&mut self) -> SPI_CLK_EQU_SYSCLK_W {
        SPI_CLK_EQU_SYSCLK_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn spi_clkdiv_pre(&mut self) -> SPI_CLKDIV_PRE_W {
        SPI_CLKDIV_PRE_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn spi_clkcnt_n(&mut self) -> SPI_CLKCNT_N_W {
        SPI_CLKCNT_N_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn spi_clkcnt_h(&mut self) -> SPI_CLKCNT_H_W {
        SPI_CLKCNT_H_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn spi_clkcnt_l(&mut self) -> SPI_CLKCNT_L_W {
        SPI_CLKCNT_L_W { w: self }
    }
}
