#[doc = "Reader of register SPI_MEM_DIN_NUM"]
pub type R = crate::R<u32, super::SPI_MEM_DIN_NUM>;
#[doc = "Writer for register SPI_MEM_DIN_NUM"]
pub type W = crate::W<u32, super::SPI_MEM_DIN_NUM>;
#[doc = "Register SPI_MEM_DIN_NUM `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_DIN_NUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_DIN3_NUM`"]
pub type SPI_MEM_DIN3_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_DIN3_NUM`"]
pub struct SPI_MEM_DIN3_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_DIN3_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_DIN2_NUM`"]
pub type SPI_MEM_DIN2_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_DIN2_NUM`"]
pub struct SPI_MEM_DIN2_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_DIN2_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_DIN1_NUM`"]
pub type SPI_MEM_DIN1_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_DIN1_NUM`"]
pub struct SPI_MEM_DIN1_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_DIN1_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_DIN0_NUM`"]
pub type SPI_MEM_DIN0_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_DIN0_NUM`"]
pub struct SPI_MEM_DIN0_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_DIN0_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn spi_mem_din3_num(&self) -> SPI_MEM_DIN3_NUM_R {
        SPI_MEM_DIN3_NUM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi_mem_din2_num(&self) -> SPI_MEM_DIN2_NUM_R {
        SPI_MEM_DIN2_NUM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi_mem_din1_num(&self) -> SPI_MEM_DIN1_NUM_R {
        SPI_MEM_DIN1_NUM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_mem_din0_num(&self) -> SPI_MEM_DIN0_NUM_R {
        SPI_MEM_DIN0_NUM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn spi_mem_din3_num(&mut self) -> SPI_MEM_DIN3_NUM_W {
        SPI_MEM_DIN3_NUM_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi_mem_din2_num(&mut self) -> SPI_MEM_DIN2_NUM_W {
        SPI_MEM_DIN2_NUM_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi_mem_din1_num(&mut self) -> SPI_MEM_DIN1_NUM_W {
        SPI_MEM_DIN1_NUM_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_mem_din0_num(&mut self) -> SPI_MEM_DIN0_NUM_W {
        SPI_MEM_DIN0_NUM_W { w: self }
    }
}
