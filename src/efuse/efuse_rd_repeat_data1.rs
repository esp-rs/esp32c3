#[doc = "Reader of register EFUSE_RD_REPEAT_DATA1"]
pub type R = crate::R<u32, super::EFUSE_RD_REPEAT_DATA1>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_1`"]
pub type EFUSE_KEY_PURPOSE_1_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_0`"]
pub type EFUSE_KEY_PURPOSE_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SECURE_BOOT_KEY_REVOKE2`"]
pub type EFUSE_SECURE_BOOT_KEY_REVOKE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_SECURE_BOOT_KEY_REVOKE1`"]
pub type EFUSE_SECURE_BOOT_KEY_REVOKE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_SECURE_BOOT_KEY_REVOKE0`"]
pub type EFUSE_SECURE_BOOT_KEY_REVOKE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_SPI_BOOT_CRYPT_CNT`"]
pub type EFUSE_SPI_BOOT_CRYPT_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_WDT_DELAY_SEL`"]
pub type EFUSE_WDT_DELAY_SEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED2`"]
pub type EFUSE_RPT4_RESERVED2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn efuse_key_purpose_1(&self) -> EFUSE_KEY_PURPOSE_1_R {
        EFUSE_KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn efuse_key_purpose_0(&self) -> EFUSE_KEY_PURPOSE_0_R {
        EFUSE_KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn efuse_secure_boot_key_revoke2(&self) -> EFUSE_SECURE_BOOT_KEY_REVOKE2_R {
        EFUSE_SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn efuse_secure_boot_key_revoke1(&self) -> EFUSE_SECURE_BOOT_KEY_REVOKE1_R {
        EFUSE_SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn efuse_secure_boot_key_revoke0(&self) -> EFUSE_SECURE_BOOT_KEY_REVOKE0_R {
        EFUSE_SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn efuse_spi_boot_crypt_cnt(&self) -> EFUSE_SPI_BOOT_CRYPT_CNT_R {
        EFUSE_SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn efuse_wdt_delay_sel(&self) -> EFUSE_WDT_DELAY_SEL_R {
        EFUSE_WDT_DELAY_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved2(&self) -> EFUSE_RPT4_RESERVED2_R {
        EFUSE_RPT4_RESERVED2_R::new((self.bits & 0xffff) as u16)
    }
}
