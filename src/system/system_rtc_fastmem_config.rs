#[doc = "Reader of register SYSTEM_RTC_FASTMEM_CONFIG"]
pub type R = crate::R<u32, super::SYSTEM_RTC_FASTMEM_CONFIG>;
#[doc = "Writer for register SYSTEM_RTC_FASTMEM_CONFIG"]
pub type W = crate::W<u32, super::SYSTEM_RTC_FASTMEM_CONFIG>;
#[doc = "Register SYSTEM_RTC_FASTMEM_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_RTC_FASTMEM_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_RTC_MEM_CRC_FINISH`"]
pub type SYSTEM_RTC_MEM_CRC_FINISH_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSTEM_RTC_MEM_CRC_LEN`"]
pub type SYSTEM_RTC_MEM_CRC_LEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSTEM_RTC_MEM_CRC_LEN`"]
pub struct SYSTEM_RTC_MEM_CRC_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_RTC_MEM_CRC_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 20)) | (((value as u32) & 0x07ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_RTC_MEM_CRC_ADDR`"]
pub type SYSTEM_RTC_MEM_CRC_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSTEM_RTC_MEM_CRC_ADDR`"]
pub struct SYSTEM_RTC_MEM_CRC_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_RTC_MEM_CRC_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 9)) | (((value as u32) & 0x07ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_RTC_MEM_CRC_START`"]
pub type SYSTEM_RTC_MEM_CRC_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_RTC_MEM_CRC_START`"]
pub struct SYSTEM_RTC_MEM_CRC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_RTC_MEM_CRC_START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn system_rtc_mem_crc_finish(&self) -> SYSTEM_RTC_MEM_CRC_FINISH_R {
        SYSTEM_RTC_MEM_CRC_FINISH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    pub fn system_rtc_mem_crc_len(&self) -> SYSTEM_RTC_MEM_CRC_LEN_R {
        SYSTEM_RTC_MEM_CRC_LEN_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bits 9:19"]
    #[inline(always)]
    pub fn system_rtc_mem_crc_addr(&self) -> SYSTEM_RTC_MEM_CRC_ADDR_R {
        SYSTEM_RTC_MEM_CRC_ADDR_R::new(((self.bits >> 9) & 0x07ff) as u16)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn system_rtc_mem_crc_start(&self) -> SYSTEM_RTC_MEM_CRC_START_R {
        SYSTEM_RTC_MEM_CRC_START_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:30"]
    #[inline(always)]
    pub fn system_rtc_mem_crc_len(&mut self) -> SYSTEM_RTC_MEM_CRC_LEN_W {
        SYSTEM_RTC_MEM_CRC_LEN_W { w: self }
    }
    #[doc = "Bits 9:19"]
    #[inline(always)]
    pub fn system_rtc_mem_crc_addr(&mut self) -> SYSTEM_RTC_MEM_CRC_ADDR_W {
        SYSTEM_RTC_MEM_CRC_ADDR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn system_rtc_mem_crc_start(&mut self) -> SYSTEM_RTC_MEM_CRC_START_W {
        SYSTEM_RTC_MEM_CRC_START_W { w: self }
    }
}
