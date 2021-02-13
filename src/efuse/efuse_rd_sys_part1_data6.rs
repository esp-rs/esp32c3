#[doc = "Reader of register EFUSE_RD_SYS_PART1_DATA6"]
pub type R = crate::R<u32, super::EFUSE_RD_SYS_PART1_DATA6>;
#[doc = "Reader of field `EFUSE_SYS_DATA_PART1_6`"]
pub type EFUSE_SYS_DATA_PART1_6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_sys_data_part1_6(&self) -> EFUSE_SYS_DATA_PART1_6_R {
        EFUSE_SYS_DATA_PART1_6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
