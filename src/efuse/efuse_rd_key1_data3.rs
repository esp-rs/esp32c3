#[doc = "Reader of register EFUSE_RD_KEY1_DATA3"]
pub type R = crate::R<u32, super::EFUSE_RD_KEY1_DATA3>;
#[doc = "Reader of field `EFUSE_KEY1_DATA3`"]
pub type EFUSE_KEY1_DATA3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_key1_data3(&self) -> EFUSE_KEY1_DATA3_R {
        EFUSE_KEY1_DATA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
