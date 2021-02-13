#[doc = "Reader of register EFUSE_RD_SYS_PART1_DATA2"]
pub type R = crate::R<u32, super::EFUSE_RD_SYS_PART1_DATA2>;
#[doc = "Reader of field `EFUSE_SYS_DATA_PART1_2`"]
pub type EFUSE_SYS_DATA_PART1_2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_sys_data_part1_2(&self) -> EFUSE_SYS_DATA_PART1_2_R {
        EFUSE_SYS_DATA_PART1_2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
