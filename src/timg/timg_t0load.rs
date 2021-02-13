#[doc = "Writer for register TIMG_T0LOAD"]
pub type W = crate::W<u32, super::TIMG_T0LOAD>;
#[doc = "Register TIMG_T0LOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_T0LOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TIMG_T0_LOAD`"]
pub struct TIMG_T0_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timg_t0_load(&mut self) -> TIMG_T0_LOAD_W {
        TIMG_T0_LOAD_W { w: self }
    }
}
