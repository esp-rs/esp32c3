#[doc = "Reader of register EFUSE_RD_REPEAT_DATA2"]
pub type R = crate::R<u32, super::EFUSE_RD_REPEAT_DATA2>;
#[doc = "Reader of field `EFUSE_FLASH_TPUW`"]
pub type EFUSE_FLASH_TPUW_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED0`"]
pub type EFUSE_RPT4_RESERVED0_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE`"]
pub type EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_SECURE_BOOT_EN`"]
pub type EFUSE_SECURE_BOOT_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED3`"]
pub type EFUSE_RPT4_RESERVED3_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_5`"]
pub type EFUSE_KEY_PURPOSE_5_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_4`"]
pub type EFUSE_KEY_PURPOSE_4_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_3`"]
pub type EFUSE_KEY_PURPOSE_3_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_2`"]
pub type EFUSE_KEY_PURPOSE_2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn efuse_flash_tpuw(&self) -> EFUSE_FLASH_TPUW_R {
        EFUSE_FLASH_TPUW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved0(&self) -> EFUSE_RPT4_RESERVED0_R {
        EFUSE_RPT4_RESERVED0_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn efuse_secure_boot_aggressive_revoke(&self) -> EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn efuse_secure_boot_en(&self) -> EFUSE_SECURE_BOOT_EN_R {
        EFUSE_SECURE_BOOT_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved3(&self) -> EFUSE_RPT4_RESERVED3_R {
        EFUSE_RPT4_RESERVED3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn efuse_key_purpose_5(&self) -> EFUSE_KEY_PURPOSE_5_R {
        EFUSE_KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn efuse_key_purpose_4(&self) -> EFUSE_KEY_PURPOSE_4_R {
        EFUSE_KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn efuse_key_purpose_3(&self) -> EFUSE_KEY_PURPOSE_3_R {
        EFUSE_KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn efuse_key_purpose_2(&self) -> EFUSE_KEY_PURPOSE_2_R {
        EFUSE_KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
}
