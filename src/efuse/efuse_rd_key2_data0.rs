#[doc = "Reader of register EFUSE_RD_KEY2_DATA0"]
pub type R = crate::R<u32, super::EFUSE_RD_KEY2_DATA0>;
#[doc = "Reader of field `EFUSE_KEY2_DATA0`"]
pub type EFUSE_KEY2_DATA0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_key2_data0(&self) -> EFUSE_KEY2_DATA0_R {
        EFUSE_KEY2_DATA0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
