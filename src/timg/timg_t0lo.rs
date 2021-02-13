#[doc = "Reader of register TIMG_T0LO"]
pub type R = crate::R<u32, super::TIMG_T0LO>;
#[doc = "Reader of field `TIMG_T0_LO`"]
pub type TIMG_T0_LO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timg_t0_lo(&self) -> TIMG_T0_LO_R {
        TIMG_T0_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
