#[doc = "Writer for register EFUSE_INT_CLR"]
pub type W = crate::W<u32, super::EFUSE_INT_CLR>;
#[doc = "Register EFUSE_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EFUSE_PGM_DONE_INT_CLR`"]
pub struct EFUSE_PGM_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PGM_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `EFUSE_READ_DONE_INT_CLR`"]
pub struct EFUSE_READ_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_READ_DONE_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_pgm_done_int_clr(&mut self) -> EFUSE_PGM_DONE_INT_CLR_W {
        EFUSE_PGM_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_read_done_int_clr(&mut self) -> EFUSE_READ_DONE_INT_CLR_W {
        EFUSE_READ_DONE_INT_CLR_W { w: self }
    }
}
