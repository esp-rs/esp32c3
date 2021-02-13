#[doc = "Reader of register EFUSE_RD_SYS_PART1_DATA7"]
pub type R = crate::R<u32, super::EFUSE_RD_SYS_PART1_DATA7>;
#[doc = "Reader of field `EFUSE_SYS_DATA_PART1_7`"]
pub type EFUSE_SYS_DATA_PART1_7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_sys_data_part1_7(&self) -> EFUSE_SYS_DATA_PART1_7_R {
        EFUSE_SYS_DATA_PART1_7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
