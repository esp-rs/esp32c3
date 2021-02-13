#[doc = "Reader of register EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE>;
#[doc = "Writer for register EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE>;
#[doc = "Register EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_RECORD_DISABLE_G0CB_DECRYPT`"]
pub type EXTMEM_RECORD_DISABLE_G0CB_DECRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_RECORD_DISABLE_G0CB_DECRYPT`"]
pub struct EXTMEM_RECORD_DISABLE_G0CB_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_RECORD_DISABLE_G0CB_DECRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_RECORD_DISABLE_DB_ENCRYPT`"]
pub type EXTMEM_RECORD_DISABLE_DB_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_RECORD_DISABLE_DB_ENCRYPT`"]
pub struct EXTMEM_RECORD_DISABLE_DB_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_RECORD_DISABLE_DB_ENCRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_record_disable_g0cb_decrypt(&self) -> EXTMEM_RECORD_DISABLE_G0CB_DECRYPT_R {
        EXTMEM_RECORD_DISABLE_G0CB_DECRYPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_record_disable_db_encrypt(&self) -> EXTMEM_RECORD_DISABLE_DB_ENCRYPT_R {
        EXTMEM_RECORD_DISABLE_DB_ENCRYPT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_record_disable_g0cb_decrypt(&mut self) -> EXTMEM_RECORD_DISABLE_G0CB_DECRYPT_W {
        EXTMEM_RECORD_DISABLE_G0CB_DECRYPT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_record_disable_db_encrypt(&mut self) -> EXTMEM_RECORD_DISABLE_DB_ENCRYPT_W {
        EXTMEM_RECORD_DISABLE_DB_ENCRYPT_W { w: self }
    }
}
