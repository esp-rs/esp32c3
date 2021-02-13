#[doc = "Reader of register EFUSE_RD_KEY2_DATA4"]
pub type R = crate::R<u32, super::EFUSE_RD_KEY2_DATA4>;
#[doc = "Reader of field `EFUSE_KEY2_DATA4`"]
pub type EFUSE_KEY2_DATA4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_key2_data4(&self) -> EFUSE_KEY2_DATA4_R {
        EFUSE_KEY2_DATA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
