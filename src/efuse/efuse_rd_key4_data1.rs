#[doc = "Reader of register EFUSE_RD_KEY4_DATA1"]
pub type R = crate::R<u32, super::EFUSE_RD_KEY4_DATA1>;
#[doc = "Reader of field `EFUSE_KEY4_DATA1`"]
pub type EFUSE_KEY4_DATA1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_key4_data1(&self) -> EFUSE_KEY4_DATA1_R {
        EFUSE_KEY4_DATA1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
