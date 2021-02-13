#[doc = "Reader of register SPI_MEM_ADDR"]
pub type R = crate::R<u32, super::SPI_MEM_ADDR>;
#[doc = "Writer for register SPI_MEM_ADDR"]
pub type W = crate::W<u32, super::SPI_MEM_ADDR>;
#[doc = "Register SPI_MEM_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_USR_ADDR_VALUE`"]
pub type SPI_MEM_USR_ADDR_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_MEM_USR_ADDR_VALUE`"]
pub struct SPI_MEM_USR_ADDR_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_USR_ADDR_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_mem_usr_addr_value(&self) -> SPI_MEM_USR_ADDR_VALUE_R {
        SPI_MEM_USR_ADDR_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_mem_usr_addr_value(&mut self) -> SPI_MEM_USR_ADDR_VALUE_W {
        SPI_MEM_USR_ADDR_VALUE_W { w: self }
    }
}
