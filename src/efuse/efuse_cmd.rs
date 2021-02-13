#[doc = "Reader of register EFUSE_CMD"]
pub type R = crate::R<u32, super::EFUSE_CMD>;
#[doc = "Writer for register EFUSE_CMD"]
pub type W = crate::W<u32, super::EFUSE_CMD>;
#[doc = "Register EFUSE_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_BLK_NUM`"]
pub type EFUSE_BLK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_BLK_NUM`"]
pub struct EFUSE_BLK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_BLK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_PGM_CMD`"]
pub type EFUSE_PGM_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_PGM_CMD`"]
pub struct EFUSE_PGM_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PGM_CMD_W<'a> {
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
#[doc = "Reader of field `EFUSE_READ_CMD`"]
pub type EFUSE_READ_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_READ_CMD`"]
pub struct EFUSE_READ_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_READ_CMD_W<'a> {
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
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn efuse_blk_num(&self) -> EFUSE_BLK_NUM_R {
        EFUSE_BLK_NUM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_pgm_cmd(&self) -> EFUSE_PGM_CMD_R {
        EFUSE_PGM_CMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_read_cmd(&self) -> EFUSE_READ_CMD_R {
        EFUSE_READ_CMD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn efuse_blk_num(&mut self) -> EFUSE_BLK_NUM_W {
        EFUSE_BLK_NUM_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_pgm_cmd(&mut self) -> EFUSE_PGM_CMD_W {
        EFUSE_PGM_CMD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_read_cmd(&mut self) -> EFUSE_READ_CMD_W {
        EFUSE_READ_CMD_W { w: self }
    }
}
