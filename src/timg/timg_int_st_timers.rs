#[doc = "Reader of register TIMG_INT_ST_TIMERS"]
pub type R = crate::R<u32, super::TIMG_INT_ST_TIMERS>;
#[doc = "Reader of field `TIMG_WDT_INT_ST`"]
pub type TIMG_WDT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMG_T0_INT_ST`"]
pub type TIMG_T0_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timg_wdt_int_st(&self) -> TIMG_WDT_INT_ST_R {
        TIMG_WDT_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timg_t0_int_st(&self) -> TIMG_T0_INT_ST_R {
        TIMG_T0_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
