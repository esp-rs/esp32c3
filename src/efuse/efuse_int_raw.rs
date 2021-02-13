#[doc = "Reader of register EFUSE_INT_RAW"]
pub type R = crate::R<u32, super::EFUSE_INT_RAW>;
#[doc = "Reader of field `EFUSE_PGM_DONE_INT_RAW`"]
pub type EFUSE_PGM_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_READ_DONE_INT_RAW`"]
pub type EFUSE_READ_DONE_INT_RAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_pgm_done_int_raw(&self) -> EFUSE_PGM_DONE_INT_RAW_R {
        EFUSE_PGM_DONE_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_read_done_int_raw(&self) -> EFUSE_READ_DONE_INT_RAW_R {
        EFUSE_READ_DONE_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
