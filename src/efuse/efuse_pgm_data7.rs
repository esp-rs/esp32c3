#[doc = "Reader of register EFUSE_PGM_DATA7"]
pub type R = crate::R<u32, super::EFUSE_PGM_DATA7>;
#[doc = "Writer for register EFUSE_PGM_DATA7"]
pub type W = crate::W<u32, super::EFUSE_PGM_DATA7>;
#[doc = "Register EFUSE_PGM_DATA7 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_PGM_DATA7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_PGM_DATA_7`"]
pub type EFUSE_PGM_DATA_7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_PGM_DATA_7`"]
pub struct EFUSE_PGM_DATA_7_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PGM_DATA_7_W<'a> {
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
    pub fn efuse_pgm_data_7(&self) -> EFUSE_PGM_DATA_7_R {
        EFUSE_PGM_DATA_7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_pgm_data_7(&mut self) -> EFUSE_PGM_DATA_7_W {
        EFUSE_PGM_DATA_7_W { w: self }
    }
}
