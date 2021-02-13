#[doc = "Reader of register EFUSE_RD_REPEAT_ERR4"]
pub type R = crate::R<u32, super::EFUSE_RD_REPEAT_ERR4>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED4_ERR`"]
pub type EFUSE_RPT4_RESERVED4_ERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved4_err(&self) -> EFUSE_RPT4_RESERVED4_ERR_R {
        EFUSE_RPT4_RESERVED4_ERR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
