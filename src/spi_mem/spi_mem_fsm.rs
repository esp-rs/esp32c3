#[doc = "Reader of register SPI_MEM_FSM"]
pub type R = crate::R<u32, super::SPI_MEM_FSM>;
#[doc = "Writer for register SPI_MEM_FSM"]
pub type W = crate::W<u32, super::SPI_MEM_FSM>;
#[doc = "Register SPI_MEM_FSM `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_FSM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_CSPI_LOCK_DELAY_TIME`"]
pub type SPI_MEM_CSPI_LOCK_DELAY_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_CSPI_LOCK_DELAY_TIME`"]
pub struct SPI_MEM_CSPI_LOCK_DELAY_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_CSPI_LOCK_DELAY_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_EM_ST`"]
pub type SPI_MEM_EM_ST_R = crate::R<u8, u8>;
#[doc = "Reader of field `SPI_MEM_CSPI_ST`"]
pub type SPI_MEM_CSPI_ST_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn spi_mem_cspi_lock_delay_time(&self) -> SPI_MEM_CSPI_LOCK_DELAY_TIME_R {
        SPI_MEM_CSPI_LOCK_DELAY_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn spi_mem_em_st(&self) -> SPI_MEM_EM_ST_R {
        SPI_MEM_EM_ST_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn spi_mem_cspi_st(&self) -> SPI_MEM_CSPI_ST_R {
        SPI_MEM_CSPI_ST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn spi_mem_cspi_lock_delay_time(&mut self) -> SPI_MEM_CSPI_LOCK_DELAY_TIME_W {
        SPI_MEM_CSPI_LOCK_DELAY_TIME_W { w: self }
    }
}
