#[doc = "Reader of register EFUSE_RD_MAC_SPI_SYS_0"]
pub type R = crate::R<u32, super::EFUSE_RD_MAC_SPI_SYS_0>;
#[doc = "Reader of field `EFUSE_MAC_0`"]
pub type EFUSE_MAC_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_mac_0(&self) -> EFUSE_MAC_0_R {
        EFUSE_MAC_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
