#[doc = "Reader of register EFUSE_RD_REPEAT_DATA4"]
pub type R = crate::R<u32, super::EFUSE_RD_REPEAT_DATA4>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED4`"]
pub type EFUSE_RPT4_RESERVED4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved4(&self) -> EFUSE_RPT4_RESERVED4_R {
        EFUSE_RPT4_RESERVED4_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
