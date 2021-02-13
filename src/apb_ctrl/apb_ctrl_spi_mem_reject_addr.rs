#[doc = "Reader of register APB_CTRL_SPI_MEM_REJECT_ADDR"]
pub type R = crate::R<u32, super::APB_CTRL_SPI_MEM_REJECT_ADDR>;
#[doc = "Reader of field `APB_CTRL_SPI_MEM_REJECT_ADDR`"]
pub type APB_CTRL_SPI_MEM_REJECT_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_ctrl_spi_mem_reject_addr(&self) -> APB_CTRL_SPI_MEM_REJECT_ADDR_R {
        APB_CTRL_SPI_MEM_REJECT_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
