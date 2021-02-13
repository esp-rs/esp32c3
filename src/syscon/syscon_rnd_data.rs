#[doc = "Reader of register SYSCON_RND_DATA"]
pub type R = crate::R<u32, super::SYSCON_RND_DATA>;
#[doc = "Reader of field `SYSCON_RND_DATA`"]
pub type SYSCON_RND_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syscon_rnd_data(&self) -> SYSCON_RND_DATA_R {
        SYSCON_RND_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
