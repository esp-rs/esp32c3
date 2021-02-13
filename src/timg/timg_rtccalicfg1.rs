#[doc = "Reader of register TIMG_RTCCALICFG1"]
pub type R = crate::R<u32, super::TIMG_RTCCALICFG1>;
#[doc = "Reader of field `TIMG_RTC_CALI_VALUE`"]
pub type TIMG_RTC_CALI_VALUE_R = crate::R<u32, u32>;
#[doc = "Reader of field `TIMG_RTC_CALI_CYCLING_DATA_VLD`"]
pub type TIMG_RTC_CALI_CYCLING_DATA_VLD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn timg_rtc_cali_value(&self) -> TIMG_RTC_CALI_VALUE_R {
        TIMG_RTC_CALI_VALUE_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timg_rtc_cali_cycling_data_vld(&self) -> TIMG_RTC_CALI_CYCLING_DATA_VLD_R {
        TIMG_RTC_CALI_CYCLING_DATA_VLD_R::new((self.bits & 0x01) != 0)
    }
}
