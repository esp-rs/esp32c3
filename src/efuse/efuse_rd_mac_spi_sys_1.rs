#[doc = "Reader of register EFUSE_RD_MAC_SPI_SYS_1"]
pub type R = crate::R<u32, super::EFUSE_RD_MAC_SPI_SYS_1>;
#[doc = "Reader of field `EFUSE_SPI_PAD_CONF_0`"]
pub type EFUSE_SPI_PAD_CONF_0_R = crate::R<u16, u16>;
#[doc = "Reader of field `EFUSE_MAC_1`"]
pub type EFUSE_MAC_1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn efuse_spi_pad_conf_0(&self) -> EFUSE_SPI_PAD_CONF_0_R {
        EFUSE_SPI_PAD_CONF_0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn efuse_mac_1(&self) -> EFUSE_MAC_1_R {
        EFUSE_MAC_1_R::new((self.bits & 0xffff) as u16)
    }
}
