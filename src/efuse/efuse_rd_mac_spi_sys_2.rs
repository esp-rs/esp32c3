#[doc = "Reader of register EFUSE_RD_MAC_SPI_SYS_2"]
pub type R = crate::R<u32, super::EFUSE_RD_MAC_SPI_SYS_2>;
#[doc = "Reader of field `EFUSE_SPI_PAD_CONF_1`"]
pub type EFUSE_SPI_PAD_CONF_1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_spi_pad_conf_1(&self) -> EFUSE_SPI_PAD_CONF_1_R {
        EFUSE_SPI_PAD_CONF_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
