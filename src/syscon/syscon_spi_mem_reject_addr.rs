#[doc = "Reader of register SYSCON_SPI_MEM_REJECT_ADDR"]
pub type R = crate::R<u32, super::SYSCON_SPI_MEM_REJECT_ADDR>;
#[doc = "Reader of field `SYSCON_SPI_MEM_REJECT_ADDR`"]
pub type SYSCON_SPI_MEM_REJECT_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syscon_spi_mem_reject_addr(&self) -> SYSCON_SPI_MEM_REJECT_ADDR_R {
        SYSCON_SPI_MEM_REJECT_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
