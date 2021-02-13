#[doc = "Reader of register SPI_MEM_CORE_CLK_SEL"]
pub type R = crate::R<u32, super::SPI_MEM_CORE_CLK_SEL>;
#[doc = "Writer for register SPI_MEM_CORE_CLK_SEL"]
pub type W = crate::W<u32, super::SPI_MEM_CORE_CLK_SEL>;
#[doc = "Register SPI_MEM_CORE_CLK_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_CORE_CLK_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_SPI01_CLK_SEL`"]
pub type SPI_MEM_SPI01_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_SPI01_CLK_SEL`"]
pub struct SPI_MEM_SPI01_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_SPI01_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_mem_spi01_clk_sel(&self) -> SPI_MEM_SPI01_CLK_SEL_R {
        SPI_MEM_SPI01_CLK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_mem_spi01_clk_sel(&mut self) -> SPI_MEM_SPI01_CLK_SEL_W {
        SPI_MEM_SPI01_CLK_SEL_W { w: self }
    }
}
