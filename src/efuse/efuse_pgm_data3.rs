#[doc = "Reader of register EFUSE_PGM_DATA3"]
pub type R = crate::R<u32, super::EFUSE_PGM_DATA3>;
#[doc = "Writer for register EFUSE_PGM_DATA3"]
pub type W = crate::W<u32, super::EFUSE_PGM_DATA3>;
#[doc = "Register EFUSE_PGM_DATA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_PGM_DATA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_FLASH_TPUW`"]
pub type EFUSE_FLASH_TPUW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_FLASH_TPUW`"]
pub struct EFUSE_FLASH_TPUW_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_FLASH_TPUW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RPT4_RESERVED0`"]
pub type EFUSE_RPT4_RESERVED0_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE`"]
pub type EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE`"]
pub struct EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_SECURE_BOOT_EN`"]
pub type EFUSE_SECURE_BOOT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_SECURE_BOOT_EN`"]
pub struct EFUSE_SECURE_BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SECURE_BOOT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RPT4_RESERVED3`"]
pub type EFUSE_RPT4_RESERVED3_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_5`"]
pub type EFUSE_KEY_PURPOSE_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_KEY_PURPOSE_5`"]
pub struct EFUSE_KEY_PURPOSE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_KEY_PURPOSE_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_4`"]
pub type EFUSE_KEY_PURPOSE_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_KEY_PURPOSE_4`"]
pub struct EFUSE_KEY_PURPOSE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_KEY_PURPOSE_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_3`"]
pub type EFUSE_KEY_PURPOSE_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_KEY_PURPOSE_3`"]
pub struct EFUSE_KEY_PURPOSE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_KEY_PURPOSE_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_2`"]
pub type EFUSE_KEY_PURPOSE_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_KEY_PURPOSE_2`"]
pub struct EFUSE_KEY_PURPOSE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_KEY_PURPOSE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
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
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn efuse_flash_tpuw(&mut self) -> EFUSE_FLASH_TPUW_W {
        EFUSE_FLASH_TPUW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn efuse_secure_boot_aggressive_revoke(&mut self) -> EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_W {
        EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn efuse_secure_boot_en(&mut self) -> EFUSE_SECURE_BOOT_EN_W {
        EFUSE_SECURE_BOOT_EN_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn efuse_key_purpose_5(&mut self) -> EFUSE_KEY_PURPOSE_5_W {
        EFUSE_KEY_PURPOSE_5_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn efuse_key_purpose_4(&mut self) -> EFUSE_KEY_PURPOSE_4_W {
        EFUSE_KEY_PURPOSE_4_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn efuse_key_purpose_3(&mut self) -> EFUSE_KEY_PURPOSE_3_W {
        EFUSE_KEY_PURPOSE_3_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn efuse_key_purpose_2(&mut self) -> EFUSE_KEY_PURPOSE_2_W {
        EFUSE_KEY_PURPOSE_2_W { w: self }
    }
}
