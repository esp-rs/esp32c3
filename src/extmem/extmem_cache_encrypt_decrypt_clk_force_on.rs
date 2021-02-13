#[doc = "Reader of register EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON"]
pub type R = crate::R<u32, super::EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON>;
#[doc = "Writer for register EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON>;
#[doc = "Register EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_CLK_FORCE_ON_CRYPT`"]
pub type EXTMEM_CLK_FORCE_ON_CRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CLK_FORCE_ON_CRYPT`"]
pub struct EXTMEM_CLK_FORCE_ON_CRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CLK_FORCE_ON_CRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_CLK_FORCE_ON_AUTO_CRYPT`"]
pub type EXTMEM_CLK_FORCE_ON_AUTO_CRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CLK_FORCE_ON_AUTO_CRYPT`"]
pub struct EXTMEM_CLK_FORCE_ON_AUTO_CRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CLK_FORCE_ON_AUTO_CRYPT_W<'a> {
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
#[doc = "Reader of field `EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT`"]
pub type EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT`"]
pub struct EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_clk_force_on_crypt(&self) -> EXTMEM_CLK_FORCE_ON_CRYPT_R {
        EXTMEM_CLK_FORCE_ON_CRYPT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_clk_force_on_auto_crypt(&self) -> EXTMEM_CLK_FORCE_ON_AUTO_CRYPT_R {
        EXTMEM_CLK_FORCE_ON_AUTO_CRYPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_clk_force_on_manual_crypt(&self) -> EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT_R {
        EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_clk_force_on_crypt(&mut self) -> EXTMEM_CLK_FORCE_ON_CRYPT_W {
        EXTMEM_CLK_FORCE_ON_CRYPT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_clk_force_on_auto_crypt(&mut self) -> EXTMEM_CLK_FORCE_ON_AUTO_CRYPT_W {
        EXTMEM_CLK_FORCE_ON_AUTO_CRYPT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_clk_force_on_manual_crypt(&mut self) -> EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT_W {
        EXTMEM_CLK_FORCE_ON_MANUAL_CRYPT_W { w: self }
    }
}
