#[doc = "Reader of register SPI_MEM_CTRL2"]
pub type R = crate::R<u32, super::SPI_MEM_CTRL2>;
#[doc = "Writer for register SPI_MEM_CTRL2"]
pub type W = crate::W<u32, super::SPI_MEM_CTRL2>;
#[doc = "Register SPI_MEM_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SPI_MEM_SYNC_RESET`"]
pub struct SPI_MEM_SYNC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_SYNC_RESET_W<'a> {
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
#[doc = "Reader of field `SPI_MEM_CS_HOLD_DELAY`"]
pub type SPI_MEM_CS_HOLD_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_CS_HOLD_DELAY`"]
pub struct SPI_MEM_CS_HOLD_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CS_HOLD_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 25)) | (((value as u32) & 0x3f) << 25);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_CS_HOLD_TIME`"]
pub type SPI_MEM_CS_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_CS_HOLD_TIME`"]
pub struct SPI_MEM_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_CS_SETUP_TIME`"]
pub type SPI_MEM_CS_SETUP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_CS_SETUP_TIME`"]
pub struct SPI_MEM_CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:30"]
    #[inline(always)]
    pub fn spi_mem_cs_hold_delay(&self) -> SPI_MEM_CS_HOLD_DELAY_R {
        SPI_MEM_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn spi_mem_cs_hold_time(&self) -> SPI_MEM_CS_HOLD_TIME_R {
        SPI_MEM_CS_HOLD_TIME_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_mem_cs_setup_time(&self) -> SPI_MEM_CS_SETUP_TIME_R {
        SPI_MEM_CS_SETUP_TIME_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_mem_sync_reset(&mut self) -> SPI_MEM_SYNC_RESET_W {
        SPI_MEM_SYNC_RESET_W { w: self }
    }
    #[doc = "Bits 25:30"]
    #[inline(always)]
    pub fn spi_mem_cs_hold_delay(&mut self) -> SPI_MEM_CS_HOLD_DELAY_W {
        SPI_MEM_CS_HOLD_DELAY_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn spi_mem_cs_hold_time(&mut self) -> SPI_MEM_CS_HOLD_TIME_W {
        SPI_MEM_CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_mem_cs_setup_time(&mut self) -> SPI_MEM_CS_SETUP_TIME_W {
        SPI_MEM_CS_SETUP_TIME_W { w: self }
    }
}
