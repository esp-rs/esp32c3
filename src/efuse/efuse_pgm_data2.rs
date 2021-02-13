#[doc = "Reader of register EFUSE_PGM_DATA2"]
pub type R = crate::R<u32, super::EFUSE_PGM_DATA2>;
#[doc = "Writer for register EFUSE_PGM_DATA2"]
pub type W = crate::W<u32, super::EFUSE_PGM_DATA2>;
#[doc = "Register EFUSE_PGM_DATA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_PGM_DATA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_1`"]
pub type EFUSE_KEY_PURPOSE_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_KEY_PURPOSE_1`"]
pub struct EFUSE_KEY_PURPOSE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_KEY_PURPOSE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_KEY_PURPOSE_0`"]
pub type EFUSE_KEY_PURPOSE_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_KEY_PURPOSE_0`"]
pub struct EFUSE_KEY_PURPOSE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_KEY_PURPOSE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_SECURE_BOOT_KEY_REVOKE2`"]
pub type EFUSE_SECURE_BOOT_KEY_REVOKE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_SECURE_BOOT_KEY_REVOKE2`"]
pub struct EFUSE_SECURE_BOOT_KEY_REVOKE2_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SECURE_BOOT_KEY_REVOKE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_SECURE_BOOT_KEY_REVOKE1`"]
pub type EFUSE_SECURE_BOOT_KEY_REVOKE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_SECURE_BOOT_KEY_REVOKE1`"]
pub struct EFUSE_SECURE_BOOT_KEY_REVOKE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SECURE_BOOT_KEY_REVOKE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_SECURE_BOOT_KEY_REVOKE0`"]
pub type EFUSE_SECURE_BOOT_KEY_REVOKE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_SECURE_BOOT_KEY_REVOKE0`"]
pub struct EFUSE_SECURE_BOOT_KEY_REVOKE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SECURE_BOOT_KEY_REVOKE0_W<'a> {
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
#[doc = "Reader of field `EFUSE_SPI_BOOT_CRYPT_CNT`"]
pub type EFUSE_SPI_BOOT_CRYPT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_SPI_BOOT_CRYPT_CNT`"]
pub struct EFUSE_SPI_BOOT_CRYPT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SPI_BOOT_CRYPT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_WAT_DELAY_SEL`"]
pub type EFUSE_WAT_DELAY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_WAT_DELAY_SEL`"]
pub struct EFUSE_WAT_DELAY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_WAT_DELAY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
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
    pub fn efuse_wat_delay_sel(&self) -> EFUSE_WAT_DELAY_SEL_R {
        EFUSE_WAT_DELAY_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved2(&self) -> EFUSE_RPT4_RESERVED2_R {
        EFUSE_RPT4_RESERVED2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn efuse_key_purpose_1(&mut self) -> EFUSE_KEY_PURPOSE_1_W {
        EFUSE_KEY_PURPOSE_1_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn efuse_key_purpose_0(&mut self) -> EFUSE_KEY_PURPOSE_0_W {
        EFUSE_KEY_PURPOSE_0_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn efuse_secure_boot_key_revoke2(&mut self) -> EFUSE_SECURE_BOOT_KEY_REVOKE2_W {
        EFUSE_SECURE_BOOT_KEY_REVOKE2_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn efuse_secure_boot_key_revoke1(&mut self) -> EFUSE_SECURE_BOOT_KEY_REVOKE1_W {
        EFUSE_SECURE_BOOT_KEY_REVOKE1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn efuse_secure_boot_key_revoke0(&mut self) -> EFUSE_SECURE_BOOT_KEY_REVOKE0_W {
        EFUSE_SECURE_BOOT_KEY_REVOKE0_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn efuse_spi_boot_crypt_cnt(&mut self) -> EFUSE_SPI_BOOT_CRYPT_CNT_W {
        EFUSE_SPI_BOOT_CRYPT_CNT_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn efuse_wat_delay_sel(&mut self) -> EFUSE_WAT_DELAY_SEL_W {
        EFUSE_WAT_DELAY_SEL_W { w: self }
    }
}
