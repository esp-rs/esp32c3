#[doc = "Reader of register EFUSE_RD_REPEAT_ERR2"]
pub type R = crate::R<u32, super::EFUSE_RD_REPEAT_ERR2>;
#[doc = "Reader of field `EFUSE_FLASH_TPUW_ERR`"]
pub type EFUSE_FLASH_TPUW_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED0_ERR`"]
pub type EFUSE_RPT4_RESERVED0_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_ERR`"]
pub type EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_SECURE_BOOT_EN_ERR`"]
pub type EFUSE_SECURE_BOOT_EN_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED3_ERR`"]
pub type EFUSE_RPT4_RESERVED3_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_5_ERR`"]
pub type EFUSE_KEY_PURPOSE_5_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_4_ERR`"]
pub type EFUSE_KEY_PURPOSE_4_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_3_ERR`"]
pub type EFUSE_KEY_PURPOSE_3_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_2_ERR`"]
pub type EFUSE_KEY_PURPOSE_2_ERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn efuse_flash_tpuw_err(&self) -> EFUSE_FLASH_TPUW_ERR_R {
        EFUSE_FLASH_TPUW_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved0_err(&self) -> EFUSE_RPT4_RESERVED0_ERR_R {
        EFUSE_RPT4_RESERVED0_ERR_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn efuse_secure_boot_aggressive_revoke_err(
        &self,
    ) -> EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
        EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn efuse_secure_boot_en_err(&self) -> EFUSE_SECURE_BOOT_EN_ERR_R {
        EFUSE_SECURE_BOOT_EN_ERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved3_err(&self) -> EFUSE_RPT4_RESERVED3_ERR_R {
        EFUSE_RPT4_RESERVED3_ERR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn efuse_key_purpose_5_err(&self) -> EFUSE_KEY_PURPOSE_5_ERR_R {
        EFUSE_KEY_PURPOSE_5_ERR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn efuse_key_purpose_4_err(&self) -> EFUSE_KEY_PURPOSE_4_ERR_R {
        EFUSE_KEY_PURPOSE_4_ERR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn efuse_key_purpose_3_err(&self) -> EFUSE_KEY_PURPOSE_3_ERR_R {
        EFUSE_KEY_PURPOSE_3_ERR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn efuse_key_purpose_2_err(&self) -> EFUSE_KEY_PURPOSE_2_ERR_R {
        EFUSE_KEY_PURPOSE_2_ERR_R::new((self.bits & 0x0f) as u8)
    }
}
