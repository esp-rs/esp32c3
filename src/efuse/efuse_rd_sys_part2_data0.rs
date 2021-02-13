#[doc = "Reader of register EFUSE_RD_SYS_PART2_DATA0"]
pub type R = crate::R<u32, super::EFUSE_RD_SYS_PART2_DATA0>;
#[doc = "Reader of field `EFUSE_SYS_DATA_PART2_0`"]
pub type EFUSE_SYS_DATA_PART2_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_sys_data_part2_0(&self) -> EFUSE_SYS_DATA_PART2_0_R {
        EFUSE_SYS_DATA_PART2_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
