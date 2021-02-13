#[doc = "Reader of register EFUSE_RD_SYS_PART1_DATA1"]
pub type R = crate::R<u32, super::EFUSE_RD_SYS_PART1_DATA1>;
#[doc = "Reader of field `EFUSE_SYS_DATA_PART1_1`"]
pub type EFUSE_SYS_DATA_PART1_1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_sys_data_part1_1(&self) -> EFUSE_SYS_DATA_PART1_1_R {
        EFUSE_SYS_DATA_PART1_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
