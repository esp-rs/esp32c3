#[doc = "Reader of register EFUSE_RD_SYS_PART1_DATA4"]
pub type R = crate::R<u32, super::EFUSE_RD_SYS_PART1_DATA4>;
#[doc = "Reader of field `EFUSE_SYS_DATA_PART1_4`"]
pub type EFUSE_SYS_DATA_PART1_4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_sys_data_part1_4(&self) -> EFUSE_SYS_DATA_PART1_4_R {
        EFUSE_SYS_DATA_PART1_4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
