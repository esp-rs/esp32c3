#[doc = "Reader of register SYSTEM_RTC_FASTMEM_CRC"]
pub type R = crate::R<u32, super::SYSTEM_RTC_FASTMEM_CRC>;
#[doc = "Reader of field `SYSTEM_RTC_MEM_CRC_RES`"]
pub type SYSTEM_RTC_MEM_CRC_RES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn system_rtc_mem_crc_res(&self) -> SYSTEM_RTC_MEM_CRC_RES_R {
        SYSTEM_RTC_MEM_CRC_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
