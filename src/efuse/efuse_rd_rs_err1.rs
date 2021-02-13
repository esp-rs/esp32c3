#[doc = "Reader of register EFUSE_RD_RS_ERR1"]
pub type R = crate::R<u32, super::EFUSE_RD_RS_ERR1>;
#[doc = "Reader of field `EFUSE_SYS_PART2_FAIL`"]
pub type EFUSE_SYS_PART2_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_SYS_PART2_ERR_NUM`"]
pub type EFUSE_SYS_PART2_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY5_FAIL`"]
pub type EFUSE_KEY5_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_KEY5_ERR_NUM`"]
pub type EFUSE_KEY5_ERR_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn efuse_sys_part2_fail(&self) -> EFUSE_SYS_PART2_FAIL_R {
        EFUSE_SYS_PART2_FAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn efuse_sys_part2_err_num(&self) -> EFUSE_SYS_PART2_ERR_NUM_R {
        EFUSE_SYS_PART2_ERR_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_key5_fail(&self) -> EFUSE_KEY5_FAIL_R {
        EFUSE_KEY5_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn efuse_key5_err_num(&self) -> EFUSE_KEY5_ERR_NUM_R {
        EFUSE_KEY5_ERR_NUM_R::new((self.bits & 0x07) as u8)
    }
}
