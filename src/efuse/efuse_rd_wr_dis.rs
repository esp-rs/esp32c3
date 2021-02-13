#[doc = "Reader of register EFUSE_RD_WR_DIS"]
pub type R = crate::R<u32, super::EFUSE_RD_WR_DIS>;
#[doc = "Reader of field `EFUSE_WR_DIS`"]
pub type EFUSE_WR_DIS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_wr_dis(&self) -> EFUSE_WR_DIS_R {
        EFUSE_WR_DIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
