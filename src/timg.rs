#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMG_T0CONFIG"]
    pub timg_t0config: TIMG_T0CONFIG,
    #[doc = "0x04 - TIMG_T0LO"]
    pub timg_t0lo: TIMG_T0LO,
    #[doc = "0x08 - TIMG_T0HI"]
    pub timg_t0hi: TIMG_T0HI,
    #[doc = "0x0c - TIMG_T0UPDATE"]
    pub timg_t0update: TIMG_T0UPDATE,
    #[doc = "0x10 - TIMG_T0ALARMLO"]
    pub timg_t0alarmlo: TIMG_T0ALARMLO,
    #[doc = "0x14 - TIMG_T0ALARMHI"]
    pub timg_t0alarmhi: TIMG_T0ALARMHI,
    #[doc = "0x18 - TIMG_T0LOADLO"]
    pub timg_t0loadlo: TIMG_T0LOADLO,
    #[doc = "0x1c - TIMG_T0LOADHI"]
    pub timg_t0loadhi: TIMG_T0LOADHI,
    #[doc = "0x20 - TIMG_T0LOAD"]
    pub timg_t0load: TIMG_T0LOAD,
    _reserved9: [u8; 36usize],
    #[doc = "0x48 - TIMG_WDTCONFIG0"]
    pub timg_wdtconfig0: TIMG_WDTCONFIG0,
    #[doc = "0x4c - TIMG_WDTCONFIG1"]
    pub timg_wdtconfig1: TIMG_WDTCONFIG1,
    #[doc = "0x50 - TIMG_WDTCONFIG2"]
    pub timg_wdtconfig2: TIMG_WDTCONFIG2,
    #[doc = "0x54 - TIMG_WDTCONFIG3"]
    pub timg_wdtconfig3: TIMG_WDTCONFIG3,
    #[doc = "0x58 - TIMG_WDTCONFIG4"]
    pub timg_wdtconfig4: TIMG_WDTCONFIG4,
    #[doc = "0x5c - TIMG_WDTCONFIG5"]
    pub timg_wdtconfig5: TIMG_WDTCONFIG5,
    #[doc = "0x60 - TIMG_WDTFEED"]
    pub timg_wdtfeed: TIMG_WDTFEED,
    #[doc = "0x64 - TIMG_WDTWPROTECT"]
    pub timg_wdtwprotect: TIMG_WDTWPROTECT,
    #[doc = "0x68 - TIMG_RTCCALICFG"]
    pub timg_rtccalicfg: TIMG_RTCCALICFG,
    #[doc = "0x6c - TIMG_RTCCALICFG1"]
    pub timg_rtccalicfg1: TIMG_RTCCALICFG1,
    #[doc = "0x70 - TIMG_INT_ENA_TIMERS"]
    pub timg_int_ena_timers: TIMG_INT_ENA_TIMERS,
    #[doc = "0x74 - TIMG_INT_RAW_TIMERS"]
    pub timg_int_raw_timers: TIMG_INT_RAW_TIMERS,
    #[doc = "0x78 - TIMG_INT_ST_TIMERS"]
    pub timg_int_st_timers: TIMG_INT_ST_TIMERS,
    #[doc = "0x7c - TIMG_INT_CLR_TIMERS"]
    pub timg_int_clr_timers: TIMG_INT_CLR_TIMERS,
    #[doc = "0x80 - TIMG_RTCCALICFG2"]
    pub timg_rtccalicfg2: TIMG_RTCCALICFG2,
    _reserved24: [u8; 116usize],
    #[doc = "0xf8 - TIMG_NTIMERS_DATE"]
    pub timg_ntimers_date: TIMG_NTIMERS_DATE,
    #[doc = "0xfc - TIMG_CLK"]
    pub timg_clk: TIMG_CLK,
}
#[doc = "TIMG_T0CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0config](timg_t0config) module"]
pub type TIMG_T0CONFIG = crate::Reg<u32, _TIMG_T0CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0CONFIG;
#[doc = "`read()` method returns [timg_t0config::R](timg_t0config::R) reader structure"]
impl crate::Readable for TIMG_T0CONFIG {}
#[doc = "`write(|w| ..)` method takes [timg_t0config::W](timg_t0config::W) writer structure"]
impl crate::Writable for TIMG_T0CONFIG {}
#[doc = "TIMG_T0CONFIG"]
pub mod timg_t0config;
#[doc = "TIMG_T0LO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0lo](timg_t0lo) module"]
pub type TIMG_T0LO = crate::Reg<u32, _TIMG_T0LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0LO;
#[doc = "`read()` method returns [timg_t0lo::R](timg_t0lo::R) reader structure"]
impl crate::Readable for TIMG_T0LO {}
#[doc = "TIMG_T0LO"]
pub mod timg_t0lo;
#[doc = "TIMG_T0HI\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0hi](timg_t0hi) module"]
pub type TIMG_T0HI = crate::Reg<u32, _TIMG_T0HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0HI;
#[doc = "`read()` method returns [timg_t0hi::R](timg_t0hi::R) reader structure"]
impl crate::Readable for TIMG_T0HI {}
#[doc = "TIMG_T0HI"]
pub mod timg_t0hi;
#[doc = "TIMG_T0UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0update](timg_t0update) module"]
pub type TIMG_T0UPDATE = crate::Reg<u32, _TIMG_T0UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0UPDATE;
#[doc = "`read()` method returns [timg_t0update::R](timg_t0update::R) reader structure"]
impl crate::Readable for TIMG_T0UPDATE {}
#[doc = "`write(|w| ..)` method takes [timg_t0update::W](timg_t0update::W) writer structure"]
impl crate::Writable for TIMG_T0UPDATE {}
#[doc = "TIMG_T0UPDATE"]
pub mod timg_t0update;
#[doc = "TIMG_T0ALARMLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0alarmlo](timg_t0alarmlo) module"]
pub type TIMG_T0ALARMLO = crate::Reg<u32, _TIMG_T0ALARMLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0ALARMLO;
#[doc = "`read()` method returns [timg_t0alarmlo::R](timg_t0alarmlo::R) reader structure"]
impl crate::Readable for TIMG_T0ALARMLO {}
#[doc = "`write(|w| ..)` method takes [timg_t0alarmlo::W](timg_t0alarmlo::W) writer structure"]
impl crate::Writable for TIMG_T0ALARMLO {}
#[doc = "TIMG_T0ALARMLO"]
pub mod timg_t0alarmlo;
#[doc = "TIMG_T0ALARMHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0alarmhi](timg_t0alarmhi) module"]
pub type TIMG_T0ALARMHI = crate::Reg<u32, _TIMG_T0ALARMHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0ALARMHI;
#[doc = "`read()` method returns [timg_t0alarmhi::R](timg_t0alarmhi::R) reader structure"]
impl crate::Readable for TIMG_T0ALARMHI {}
#[doc = "`write(|w| ..)` method takes [timg_t0alarmhi::W](timg_t0alarmhi::W) writer structure"]
impl crate::Writable for TIMG_T0ALARMHI {}
#[doc = "TIMG_T0ALARMHI"]
pub mod timg_t0alarmhi;
#[doc = "TIMG_T0LOADLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0loadlo](timg_t0loadlo) module"]
pub type TIMG_T0LOADLO = crate::Reg<u32, _TIMG_T0LOADLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0LOADLO;
#[doc = "`read()` method returns [timg_t0loadlo::R](timg_t0loadlo::R) reader structure"]
impl crate::Readable for TIMG_T0LOADLO {}
#[doc = "`write(|w| ..)` method takes [timg_t0loadlo::W](timg_t0loadlo::W) writer structure"]
impl crate::Writable for TIMG_T0LOADLO {}
#[doc = "TIMG_T0LOADLO"]
pub mod timg_t0loadlo;
#[doc = "TIMG_T0LOADHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0loadhi](timg_t0loadhi) module"]
pub type TIMG_T0LOADHI = crate::Reg<u32, _TIMG_T0LOADHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0LOADHI;
#[doc = "`read()` method returns [timg_t0loadhi::R](timg_t0loadhi::R) reader structure"]
impl crate::Readable for TIMG_T0LOADHI {}
#[doc = "`write(|w| ..)` method takes [timg_t0loadhi::W](timg_t0loadhi::W) writer structure"]
impl crate::Writable for TIMG_T0LOADHI {}
#[doc = "TIMG_T0LOADHI"]
pub mod timg_t0loadhi;
#[doc = "TIMG_T0LOAD\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_t0load](timg_t0load) module"]
pub type TIMG_T0LOAD = crate::Reg<u32, _TIMG_T0LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_T0LOAD;
#[doc = "`write(|w| ..)` method takes [timg_t0load::W](timg_t0load::W) writer structure"]
impl crate::Writable for TIMG_T0LOAD {}
#[doc = "TIMG_T0LOAD"]
pub mod timg_t0load;
#[doc = "TIMG_WDTCONFIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_wdtconfig0](timg_wdtconfig0) module"]
pub type TIMG_WDTCONFIG0 = crate::Reg<u32, _TIMG_WDTCONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG0;
#[doc = "`read()` method returns [timg_wdtconfig0::R](timg_wdtconfig0::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG0 {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig0::W](timg_wdtconfig0::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG0 {}
#[doc = "TIMG_WDTCONFIG0"]
pub mod timg_wdtconfig0;
#[doc = "TIMG_WDTCONFIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_wdtconfig1](timg_wdtconfig1) module"]
pub type TIMG_WDTCONFIG1 = crate::Reg<u32, _TIMG_WDTCONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG1;
#[doc = "`read()` method returns [timg_wdtconfig1::R](timg_wdtconfig1::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG1 {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig1::W](timg_wdtconfig1::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG1 {}
#[doc = "TIMG_WDTCONFIG1"]
pub mod timg_wdtconfig1;
#[doc = "TIMG_WDTCONFIG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_wdtconfig2](timg_wdtconfig2) module"]
pub type TIMG_WDTCONFIG2 = crate::Reg<u32, _TIMG_WDTCONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG2;
#[doc = "`read()` method returns [timg_wdtconfig2::R](timg_wdtconfig2::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG2 {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig2::W](timg_wdtconfig2::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG2 {}
#[doc = "TIMG_WDTCONFIG2"]
pub mod timg_wdtconfig2;
#[doc = "TIMG_WDTCONFIG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_wdtconfig3](timg_wdtconfig3) module"]
pub type TIMG_WDTCONFIG3 = crate::Reg<u32, _TIMG_WDTCONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG3;
#[doc = "`read()` method returns [timg_wdtconfig3::R](timg_wdtconfig3::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG3 {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig3::W](timg_wdtconfig3::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG3 {}
#[doc = "TIMG_WDTCONFIG3"]
pub mod timg_wdtconfig3;
#[doc = "TIMG_WDTCONFIG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_wdtconfig4](timg_wdtconfig4) module"]
pub type TIMG_WDTCONFIG4 = crate::Reg<u32, _TIMG_WDTCONFIG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG4;
#[doc = "`read()` method returns [timg_wdtconfig4::R](timg_wdtconfig4::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG4 {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig4::W](timg_wdtconfig4::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG4 {}
#[doc = "TIMG_WDTCONFIG4"]
pub mod timg_wdtconfig4;
#[doc = "TIMG_WDTCONFIG5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_wdtconfig5](timg_wdtconfig5) module"]
pub type TIMG_WDTCONFIG5 = crate::Reg<u32, _TIMG_WDTCONFIG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTCONFIG5;
#[doc = "`read()` method returns [timg_wdtconfig5::R](timg_wdtconfig5::R) reader structure"]
impl crate::Readable for TIMG_WDTCONFIG5 {}
#[doc = "`write(|w| ..)` method takes [timg_wdtconfig5::W](timg_wdtconfig5::W) writer structure"]
impl crate::Writable for TIMG_WDTCONFIG5 {}
#[doc = "TIMG_WDTCONFIG5"]
pub mod timg_wdtconfig5;
#[doc = "TIMG_WDTFEED\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_wdtfeed](timg_wdtfeed) module"]
pub type TIMG_WDTFEED = crate::Reg<u32, _TIMG_WDTFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTFEED;
#[doc = "`write(|w| ..)` method takes [timg_wdtfeed::W](timg_wdtfeed::W) writer structure"]
impl crate::Writable for TIMG_WDTFEED {}
#[doc = "TIMG_WDTFEED"]
pub mod timg_wdtfeed;
#[doc = "TIMG_WDTWPROTECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_wdtwprotect](timg_wdtwprotect) module"]
pub type TIMG_WDTWPROTECT = crate::Reg<u32, _TIMG_WDTWPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_WDTWPROTECT;
#[doc = "`read()` method returns [timg_wdtwprotect::R](timg_wdtwprotect::R) reader structure"]
impl crate::Readable for TIMG_WDTWPROTECT {}
#[doc = "`write(|w| ..)` method takes [timg_wdtwprotect::W](timg_wdtwprotect::W) writer structure"]
impl crate::Writable for TIMG_WDTWPROTECT {}
#[doc = "TIMG_WDTWPROTECT"]
pub mod timg_wdtwprotect;
#[doc = "TIMG_RTCCALICFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_rtccalicfg](timg_rtccalicfg) module"]
pub type TIMG_RTCCALICFG = crate::Reg<u32, _TIMG_RTCCALICFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_RTCCALICFG;
#[doc = "`read()` method returns [timg_rtccalicfg::R](timg_rtccalicfg::R) reader structure"]
impl crate::Readable for TIMG_RTCCALICFG {}
#[doc = "`write(|w| ..)` method takes [timg_rtccalicfg::W](timg_rtccalicfg::W) writer structure"]
impl crate::Writable for TIMG_RTCCALICFG {}
#[doc = "TIMG_RTCCALICFG"]
pub mod timg_rtccalicfg;
#[doc = "TIMG_RTCCALICFG1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_rtccalicfg1](timg_rtccalicfg1) module"]
pub type TIMG_RTCCALICFG1 = crate::Reg<u32, _TIMG_RTCCALICFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_RTCCALICFG1;
#[doc = "`read()` method returns [timg_rtccalicfg1::R](timg_rtccalicfg1::R) reader structure"]
impl crate::Readable for TIMG_RTCCALICFG1 {}
#[doc = "TIMG_RTCCALICFG1"]
pub mod timg_rtccalicfg1;
#[doc = "TIMG_INT_ENA_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_int_ena_timers](timg_int_ena_timers) module"]
pub type TIMG_INT_ENA_TIMERS = crate::Reg<u32, _TIMG_INT_ENA_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_INT_ENA_TIMERS;
#[doc = "`read()` method returns [timg_int_ena_timers::R](timg_int_ena_timers::R) reader structure"]
impl crate::Readable for TIMG_INT_ENA_TIMERS {}
#[doc = "`write(|w| ..)` method takes [timg_int_ena_timers::W](timg_int_ena_timers::W) writer structure"]
impl crate::Writable for TIMG_INT_ENA_TIMERS {}
#[doc = "TIMG_INT_ENA_TIMERS"]
pub mod timg_int_ena_timers;
#[doc = "TIMG_INT_RAW_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_int_raw_timers](timg_int_raw_timers) module"]
pub type TIMG_INT_RAW_TIMERS = crate::Reg<u32, _TIMG_INT_RAW_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_INT_RAW_TIMERS;
#[doc = "`read()` method returns [timg_int_raw_timers::R](timg_int_raw_timers::R) reader structure"]
impl crate::Readable for TIMG_INT_RAW_TIMERS {}
#[doc = "`write(|w| ..)` method takes [timg_int_raw_timers::W](timg_int_raw_timers::W) writer structure"]
impl crate::Writable for TIMG_INT_RAW_TIMERS {}
#[doc = "TIMG_INT_RAW_TIMERS"]
pub mod timg_int_raw_timers;
#[doc = "TIMG_INT_ST_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_int_st_timers](timg_int_st_timers) module"]
pub type TIMG_INT_ST_TIMERS = crate::Reg<u32, _TIMG_INT_ST_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_INT_ST_TIMERS;
#[doc = "`read()` method returns [timg_int_st_timers::R](timg_int_st_timers::R) reader structure"]
impl crate::Readable for TIMG_INT_ST_TIMERS {}
#[doc = "TIMG_INT_ST_TIMERS"]
pub mod timg_int_st_timers;
#[doc = "TIMG_INT_CLR_TIMERS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_int_clr_timers](timg_int_clr_timers) module"]
pub type TIMG_INT_CLR_TIMERS = crate::Reg<u32, _TIMG_INT_CLR_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_INT_CLR_TIMERS;
#[doc = "`write(|w| ..)` method takes [timg_int_clr_timers::W](timg_int_clr_timers::W) writer structure"]
impl crate::Writable for TIMG_INT_CLR_TIMERS {}
#[doc = "TIMG_INT_CLR_TIMERS"]
pub mod timg_int_clr_timers;
#[doc = "TIMG_RTCCALICFG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_rtccalicfg2](timg_rtccalicfg2) module"]
pub type TIMG_RTCCALICFG2 = crate::Reg<u32, _TIMG_RTCCALICFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_RTCCALICFG2;
#[doc = "`read()` method returns [timg_rtccalicfg2::R](timg_rtccalicfg2::R) reader structure"]
impl crate::Readable for TIMG_RTCCALICFG2 {}
#[doc = "`write(|w| ..)` method takes [timg_rtccalicfg2::W](timg_rtccalicfg2::W) writer structure"]
impl crate::Writable for TIMG_RTCCALICFG2 {}
#[doc = "TIMG_RTCCALICFG2"]
pub mod timg_rtccalicfg2;
#[doc = "TIMG_NTIMERS_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_ntimers_date](timg_ntimers_date) module"]
pub type TIMG_NTIMERS_DATE = crate::Reg<u32, _TIMG_NTIMERS_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_NTIMERS_DATE;
#[doc = "`read()` method returns [timg_ntimers_date::R](timg_ntimers_date::R) reader structure"]
impl crate::Readable for TIMG_NTIMERS_DATE {}
#[doc = "`write(|w| ..)` method takes [timg_ntimers_date::W](timg_ntimers_date::W) writer structure"]
impl crate::Writable for TIMG_NTIMERS_DATE {}
#[doc = "TIMG_NTIMERS_DATE"]
pub mod timg_ntimers_date;
#[doc = "TIMG_CLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timg_clk](timg_clk) module"]
pub type TIMG_CLK = crate::Reg<u32, _TIMG_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMG_CLK;
#[doc = "`read()` method returns [timg_clk::R](timg_clk::R) reader structure"]
impl crate::Readable for TIMG_CLK {}
#[doc = "`write(|w| ..)` method takes [timg_clk::W](timg_clk::W) writer structure"]
impl crate::Writable for TIMG_CLK {}
#[doc = "TIMG_CLK"]
pub mod timg_clk;
