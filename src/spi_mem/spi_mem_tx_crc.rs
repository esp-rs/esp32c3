#[doc = "Reader of register SPI_MEM_TX_CRC"]
pub type R = crate::R<u32, super::SPI_MEM_TX_CRC>;
#[doc = "Reader of field `SPI_MEM_TX_CRC_DATA`"]
pub type SPI_MEM_TX_CRC_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_mem_tx_crc_data(&self) -> SPI_MEM_TX_CRC_DATA_R {
        SPI_MEM_TX_CRC_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
