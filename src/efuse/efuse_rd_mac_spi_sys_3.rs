#[doc = "Reader of register EFUSE_RD_MAC_SPI_SYS_3"]
pub type R = crate::R<u32, super::EFUSE_RD_MAC_SPI_SYS_3>;
#[doc = "Reader of field `EFUSE_SYS_DATA_PART0_0`"]
pub type EFUSE_SYS_DATA_PART0_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_PKG_VERSION`"]
pub type EFUSE_PKG_VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_WAFER_VERSION`"]
pub type EFUSE_WAFER_VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SPI_PAD_CONF_2`"]
pub type EFUSE_SPI_PAD_CONF_2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn efuse_sys_data_part0_0(&self) -> EFUSE_SYS_DATA_PART0_0_R {
        EFUSE_SYS_DATA_PART0_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn efuse_pkg_version(&self) -> EFUSE_PKG_VERSION_R {
        EFUSE_PKG_VERSION_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn efuse_wafer_version(&self) -> EFUSE_WAFER_VERSION_R {
        EFUSE_WAFER_VERSION_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn efuse_spi_pad_conf_2(&self) -> EFUSE_SPI_PAD_CONF_2_R {
        EFUSE_SPI_PAD_CONF_2_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
