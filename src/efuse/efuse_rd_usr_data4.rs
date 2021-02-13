#[doc = "Reader of register EFUSE_RD_USR_DATA4"]
pub type R = crate::R<u32, super::EFUSE_RD_USR_DATA4>;
#[doc = "Reader of field `EFUSE_USR_DATA4`"]
pub type EFUSE_USR_DATA4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_usr_data4(&self) -> EFUSE_USR_DATA4_R {
        EFUSE_USR_DATA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
