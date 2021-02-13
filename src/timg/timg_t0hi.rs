#[doc = "Reader of register TIMG_T0HI"]
pub type R = crate::R<u32, super::TIMG_T0HI>;
#[doc = "Reader of field `TIMG_T0_HI`"]
pub type TIMG_T0_HI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn timg_t0_hi(&self) -> TIMG_T0_HI_R {
        TIMG_T0_HI_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
