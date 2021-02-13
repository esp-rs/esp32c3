#[doc = "Reader of register APB_CTRL_RND_DATA"]
pub type R = crate::R<u32, super::APB_CTRL_RND_DATA>;
#[doc = "Reader of field `APB_CTRL_RND_DATA`"]
pub type APB_CTRL_RND_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_ctrl_rnd_data(&self) -> APB_CTRL_RND_DATA_R {
        APB_CTRL_RND_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
