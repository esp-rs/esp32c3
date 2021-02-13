#[doc = "Reader of register EFUSE_PGM_DATA0"]
pub type R = crate::R<u32, super::EFUSE_PGM_DATA0>;
#[doc = "Writer for register EFUSE_PGM_DATA0"]
pub type W = crate::W<u32, super::EFUSE_PGM_DATA0>;
#[doc = "Register EFUSE_PGM_DATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_PGM_DATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_WR_DIS`"]
pub type EFUSE_WR_DIS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_WR_DIS`"]
pub struct EFUSE_WR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_WR_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_wr_dis(&self) -> EFUSE_WR_DIS_R {
        EFUSE_WR_DIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_wr_dis(&mut self) -> EFUSE_WR_DIS_W {
        EFUSE_WR_DIS_W { w: self }
    }
}
