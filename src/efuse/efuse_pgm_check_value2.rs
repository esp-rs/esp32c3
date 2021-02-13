#[doc = "Reader of register EFUSE_PGM_CHECK_VALUE2"]
pub type R = crate::R<u32, super::EFUSE_PGM_CHECK_VALUE2>;
#[doc = "Writer for register EFUSE_PGM_CHECK_VALUE2"]
pub type W = crate::W<u32, super::EFUSE_PGM_CHECK_VALUE2>;
#[doc = "Register EFUSE_PGM_CHECK_VALUE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_PGM_CHECK_VALUE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_PGM_RS_DATA_2`"]
pub type EFUSE_PGM_RS_DATA_2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_PGM_RS_DATA_2`"]
pub struct EFUSE_PGM_RS_DATA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PGM_RS_DATA_2_W<'a> {
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
    pub fn efuse_pgm_rs_data_2(&self) -> EFUSE_PGM_RS_DATA_2_R {
        EFUSE_PGM_RS_DATA_2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_pgm_rs_data_2(&mut self) -> EFUSE_PGM_RS_DATA_2_W {
        EFUSE_PGM_RS_DATA_2_W { w: self }
    }
}
