#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ASSIST_DEBUG_CORE_0_INTR_ENA"]
    pub assist_debug_core_0_intr_ena: ASSIST_DEBUG_CORE_0_INTR_ENA,
    #[doc = "0x04 - ASSIST_DEBUG_CORE_0_INTR_RAW"]
    pub assist_debug_core_0_intr_raw: ASSIST_DEBUG_CORE_0_INTR_RAW,
    #[doc = "0x08 - ASSIST_DEBUG_CORE_0_INTR_RLS"]
    pub assist_debug_core_0_intr_rls: ASSIST_DEBUG_CORE_0_INTR_RLS,
    #[doc = "0x0c - ASSIST_DEBUG_CORE_0_INTR_CLR"]
    pub assist_debug_core_0_intr_clr: ASSIST_DEBUG_CORE_0_INTR_CLR,
    #[doc = "0x10 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN"]
    pub assist_debug_core_0_area_dram0_0_min: ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN,
    #[doc = "0x14 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX"]
    pub assist_debug_core_0_area_dram0_0_max: ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX,
    #[doc = "0x18 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN"]
    pub assist_debug_core_0_area_dram0_1_min: ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN,
    #[doc = "0x1c - ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX"]
    pub assist_debug_core_0_area_dram0_1_max: ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX,
    #[doc = "0x20 - ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN"]
    pub assist_debug_core_0_area_pif_0_min: ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN,
    #[doc = "0x24 - ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX"]
    pub assist_debug_core_0_area_pif_0_max: ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX,
    #[doc = "0x28 - ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN"]
    pub assist_debug_core_0_area_pif_1_min: ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN,
    #[doc = "0x2c - ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX"]
    pub assist_debug_core_0_area_pif_1_max: ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX,
    #[doc = "0x30 - ASSIST_DEBUG_CORE_0_AREA_PC"]
    pub assist_debug_core_0_area_pc: ASSIST_DEBUG_CORE_0_AREA_PC,
    #[doc = "0x34 - ASSIST_DEBUG_CORE_0_AREA_SP"]
    pub assist_debug_core_0_area_sp: ASSIST_DEBUG_CORE_0_AREA_SP,
    #[doc = "0x38 - ASSIST_DEBUG_CORE_0_SP_MIN"]
    pub assist_debug_core_0_sp_min: ASSIST_DEBUG_CORE_0_SP_MIN,
    #[doc = "0x3c - ASSIST_DEBUG_CORE_0_SP_MAX"]
    pub assist_debug_core_0_sp_max: ASSIST_DEBUG_CORE_0_SP_MAX,
    #[doc = "0x40 - ASSIST_DEBUG_CORE_0_SP_PC"]
    pub assist_debug_core_0_sp_pc: ASSIST_DEBUG_CORE_0_SP_PC,
    #[doc = "0x44 - ASSIST_DEBUG_CORE_0_RCD_EN"]
    pub assist_debug_core_0_rcd_en: ASSIST_DEBUG_CORE_0_RCD_EN,
    #[doc = "0x48 - ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC"]
    pub assist_debug_core_0_rcd_pdebugpc: ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC,
    #[doc = "0x4c - ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP"]
    pub assist_debug_core_0_rcd_pdebugsp: ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP,
    #[doc = "0x50 - ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0"]
    pub assist_debug_core_0_iram0_exception_monitor_0:
        ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x54 - ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1"]
    pub assist_debug_core_0_iram0_exception_monitor_1:
        ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x58 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0"]
    pub assist_debug_core_0_dram0_exception_monitor_0:
        ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x5c - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1"]
    pub assist_debug_core_0_dram0_exception_monitor_1:
        ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x60 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2"]
    pub assist_debug_core_0_dram0_exception_monitor_2:
        ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2,
    #[doc = "0x64 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3"]
    pub assist_debug_core_0_dram0_exception_monitor_3:
        ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3,
    #[doc = "0x68 - ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0"]
    pub assist_debug_core_x_iram0_dram0_exception_monitor_0:
        ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0,
    #[doc = "0x6c - ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1"]
    pub assist_debug_core_x_iram0_dram0_exception_monitor_1:
        ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1,
    #[doc = "0x70 - ASSIST_DEBUG_LOG_SETTING"]
    pub assist_debug_log_setting: ASSIST_DEBUG_LOG_SETTING,
    #[doc = "0x74 - ASSIST_DEBUG_LOG_DATA_0"]
    pub assist_debug_log_data_0: ASSIST_DEBUG_LOG_DATA_0,
    #[doc = "0x78 - ASSIST_DEBUG_LOG_DATA_MASK"]
    pub assist_debug_log_data_mask: ASSIST_DEBUG_LOG_DATA_MASK,
    #[doc = "0x7c - ASSIST_DEBUG_LOG_MIN"]
    pub assist_debug_log_min: ASSIST_DEBUG_LOG_MIN,
    #[doc = "0x80 - ASSIST_DEBUG_LOG_MAX"]
    pub assist_debug_log_max: ASSIST_DEBUG_LOG_MAX,
    #[doc = "0x84 - ASSIST_DEBUG_LOG_MEM_START"]
    pub assist_debug_log_mem_start: ASSIST_DEBUG_LOG_MEM_START,
    #[doc = "0x88 - ASSIST_DEBUG_LOG_MEM_END"]
    pub assist_debug_log_mem_end: ASSIST_DEBUG_LOG_MEM_END,
    #[doc = "0x8c - ASSIST_DEBUG_LOG_MEM_WRITING_ADDR"]
    pub assist_debug_log_mem_writing_addr: ASSIST_DEBUG_LOG_MEM_WRITING_ADDR,
    #[doc = "0x90 - ASSIST_DEBUG_LOG_MEM_FULL_FLAG"]
    pub assist_debug_log_mem_full_flag: ASSIST_DEBUG_LOG_MEM_FULL_FLAG,
    #[doc = "0x94 - ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
    pub assist_debug_c0re_0_lastpc_before_exception: ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION,
    #[doc = "0x98 - ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
    pub assist_debug_c0re_0_debug_mode: ASSIST_DEBUG_C0RE_0_DEBUG_MODE,
    _reserved39: [u8; 352usize],
    #[doc = "0x1fc - ASSIST_DEBUG_DATE"]
    pub assist_debug_date: ASSIST_DEBUG_DATE,
}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_intr_ena](assist_debug_core_0_intr_ena) module"]
pub type ASSIST_DEBUG_CORE_0_INTR_ENA = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_INTR_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_INTR_ENA;
#[doc = "`read()` method returns [assist_debug_core_0_intr_ena::R](assist_debug_core_0_intr_ena::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_INTR_ENA {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_intr_ena::W](assist_debug_core_0_intr_ena::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_INTR_ENA {}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_ENA"]
pub mod assist_debug_core_0_intr_ena;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_intr_raw](assist_debug_core_0_intr_raw) module"]
pub type ASSIST_DEBUG_CORE_0_INTR_RAW = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_INTR_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_INTR_RAW;
#[doc = "`read()` method returns [assist_debug_core_0_intr_raw::R](assist_debug_core_0_intr_raw::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_INTR_RAW {}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW"]
pub mod assist_debug_core_0_intr_raw;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RLS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_intr_rls](assist_debug_core_0_intr_rls) module"]
pub type ASSIST_DEBUG_CORE_0_INTR_RLS = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_INTR_RLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_INTR_RLS;
#[doc = "`read()` method returns [assist_debug_core_0_intr_rls::R](assist_debug_core_0_intr_rls::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_INTR_RLS {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_intr_rls::W](assist_debug_core_0_intr_rls::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_INTR_RLS {}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RLS"]
pub mod assist_debug_core_0_intr_rls;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_intr_clr](assist_debug_core_0_intr_clr) module"]
pub type ASSIST_DEBUG_CORE_0_INTR_CLR = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_INTR_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_INTR_CLR;
#[doc = "`read()` method returns [assist_debug_core_0_intr_clr::R](assist_debug_core_0_intr_clr::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_INTR_CLR {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_intr_clr::W](assist_debug_core_0_intr_clr::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_INTR_CLR {}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_CLR"]
pub mod assist_debug_core_0_intr_clr;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_dram0_0_min](assist_debug_core_0_area_dram0_0_min) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN;
#[doc = "`read()` method returns [assist_debug_core_0_area_dram0_0_min::R](assist_debug_core_0_area_dram0_0_min::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_area_dram0_0_min::W](assist_debug_core_0_area_dram0_0_min::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN"]
pub mod assist_debug_core_0_area_dram0_0_min;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_dram0_0_max](assist_debug_core_0_area_dram0_0_max) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX;
#[doc = "`read()` method returns [assist_debug_core_0_area_dram0_0_max::R](assist_debug_core_0_area_dram0_0_max::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_area_dram0_0_max::W](assist_debug_core_0_area_dram0_0_max::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX"]
pub mod assist_debug_core_0_area_dram0_0_max;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_dram0_1_min](assist_debug_core_0_area_dram0_1_min) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN;
#[doc = "`read()` method returns [assist_debug_core_0_area_dram0_1_min::R](assist_debug_core_0_area_dram0_1_min::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_area_dram0_1_min::W](assist_debug_core_0_area_dram0_1_min::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN"]
pub mod assist_debug_core_0_area_dram0_1_min;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_dram0_1_max](assist_debug_core_0_area_dram0_1_max) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX;
#[doc = "`read()` method returns [assist_debug_core_0_area_dram0_1_max::R](assist_debug_core_0_area_dram0_1_max::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_area_dram0_1_max::W](assist_debug_core_0_area_dram0_1_max::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX"]
pub mod assist_debug_core_0_area_dram0_1_max;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_pif_0_min](assist_debug_core_0_area_pif_0_min) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN;
#[doc = "`read()` method returns [assist_debug_core_0_area_pif_0_min::R](assist_debug_core_0_area_pif_0_min::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_area_pif_0_min::W](assist_debug_core_0_area_pif_0_min::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN"]
pub mod assist_debug_core_0_area_pif_0_min;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_pif_0_max](assist_debug_core_0_area_pif_0_max) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX;
#[doc = "`read()` method returns [assist_debug_core_0_area_pif_0_max::R](assist_debug_core_0_area_pif_0_max::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_area_pif_0_max::W](assist_debug_core_0_area_pif_0_max::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX"]
pub mod assist_debug_core_0_area_pif_0_max;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_pif_1_min](assist_debug_core_0_area_pif_1_min) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN;
#[doc = "`read()` method returns [assist_debug_core_0_area_pif_1_min::R](assist_debug_core_0_area_pif_1_min::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_area_pif_1_min::W](assist_debug_core_0_area_pif_1_min::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN"]
pub mod assist_debug_core_0_area_pif_1_min;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_pif_1_max](assist_debug_core_0_area_pif_1_max) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX;
#[doc = "`read()` method returns [assist_debug_core_0_area_pif_1_max::R](assist_debug_core_0_area_pif_1_max::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_area_pif_1_max::W](assist_debug_core_0_area_pif_1_max::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX"]
pub mod assist_debug_core_0_area_pif_1_max;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_pc](assist_debug_core_0_area_pc) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_PC = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_PC;
#[doc = "`read()` method returns [assist_debug_core_0_area_pc::R](assist_debug_core_0_area_pc::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_PC {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PC"]
pub mod assist_debug_core_0_area_pc;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_SP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_area_sp](assist_debug_core_0_area_sp) module"]
pub type ASSIST_DEBUG_CORE_0_AREA_SP = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_AREA_SP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_AREA_SP;
#[doc = "`read()` method returns [assist_debug_core_0_area_sp::R](assist_debug_core_0_area_sp::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_AREA_SP {}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_SP"]
pub mod assist_debug_core_0_area_sp;
#[doc = "ASSIST_DEBUG_CORE_0_SP_MIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_sp_min](assist_debug_core_0_sp_min) module"]
pub type ASSIST_DEBUG_CORE_0_SP_MIN = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_SP_MIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_SP_MIN;
#[doc = "`read()` method returns [assist_debug_core_0_sp_min::R](assist_debug_core_0_sp_min::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_SP_MIN {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_sp_min::W](assist_debug_core_0_sp_min::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_SP_MIN {}
#[doc = "ASSIST_DEBUG_CORE_0_SP_MIN"]
pub mod assist_debug_core_0_sp_min;
#[doc = "ASSIST_DEBUG_CORE_0_SP_MAX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_sp_max](assist_debug_core_0_sp_max) module"]
pub type ASSIST_DEBUG_CORE_0_SP_MAX = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_SP_MAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_SP_MAX;
#[doc = "`read()` method returns [assist_debug_core_0_sp_max::R](assist_debug_core_0_sp_max::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_SP_MAX {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_sp_max::W](assist_debug_core_0_sp_max::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_SP_MAX {}
#[doc = "ASSIST_DEBUG_CORE_0_SP_MAX"]
pub mod assist_debug_core_0_sp_max;
#[doc = "ASSIST_DEBUG_CORE_0_SP_PC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_sp_pc](assist_debug_core_0_sp_pc) module"]
pub type ASSIST_DEBUG_CORE_0_SP_PC = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_SP_PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_SP_PC;
#[doc = "`read()` method returns [assist_debug_core_0_sp_pc::R](assist_debug_core_0_sp_pc::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_SP_PC {}
#[doc = "ASSIST_DEBUG_CORE_0_SP_PC"]
pub mod assist_debug_core_0_sp_pc;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_rcd_en](assist_debug_core_0_rcd_en) module"]
pub type ASSIST_DEBUG_CORE_0_RCD_EN = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_RCD_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_RCD_EN;
#[doc = "`read()` method returns [assist_debug_core_0_rcd_en::R](assist_debug_core_0_rcd_en::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_RCD_EN {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_0_rcd_en::W](assist_debug_core_0_rcd_en::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_0_RCD_EN {}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_EN"]
pub mod assist_debug_core_0_rcd_en;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_rcd_pdebugpc](assist_debug_core_0_rcd_pdebugpc) module"]
pub type ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC;
#[doc = "`read()` method returns [assist_debug_core_0_rcd_pdebugpc::R](assist_debug_core_0_rcd_pdebugpc::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC {}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC"]
pub mod assist_debug_core_0_rcd_pdebugpc;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_rcd_pdebugsp](assist_debug_core_0_rcd_pdebugsp) module"]
pub type ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP = crate::Reg<u32, _ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP;
#[doc = "`read()` method returns [assist_debug_core_0_rcd_pdebugsp::R](assist_debug_core_0_rcd_pdebugsp::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP {}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP"]
pub mod assist_debug_core_0_rcd_pdebugsp;
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_iram0_exception_monitor_0](assist_debug_core_0_iram0_exception_monitor_0) module"]
pub type ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0;
#[doc = "`read()` method returns [assist_debug_core_0_iram0_exception_monitor_0::R](assist_debug_core_0_iram0_exception_monitor_0::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0 {}
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0"]
pub mod assist_debug_core_0_iram0_exception_monitor_0;
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_iram0_exception_monitor_1](assist_debug_core_0_iram0_exception_monitor_1) module"]
pub type ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1;
#[doc = "`read()` method returns [assist_debug_core_0_iram0_exception_monitor_1::R](assist_debug_core_0_iram0_exception_monitor_1::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1 {}
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1"]
pub mod assist_debug_core_0_iram0_exception_monitor_1;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_dram0_exception_monitor_0](assist_debug_core_0_dram0_exception_monitor_0) module"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0;
#[doc = "`read()` method returns [assist_debug_core_0_dram0_exception_monitor_0::R](assist_debug_core_0_dram0_exception_monitor_0::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0 {}
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0"]
pub mod assist_debug_core_0_dram0_exception_monitor_0;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_dram0_exception_monitor_1](assist_debug_core_0_dram0_exception_monitor_1) module"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1;
#[doc = "`read()` method returns [assist_debug_core_0_dram0_exception_monitor_1::R](assist_debug_core_0_dram0_exception_monitor_1::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1 {}
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1"]
pub mod assist_debug_core_0_dram0_exception_monitor_1;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_dram0_exception_monitor_2](assist_debug_core_0_dram0_exception_monitor_2) module"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2;
#[doc = "`read()` method returns [assist_debug_core_0_dram0_exception_monitor_2::R](assist_debug_core_0_dram0_exception_monitor_2::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2 {}
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2"]
pub mod assist_debug_core_0_dram0_exception_monitor_2;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_0_dram0_exception_monitor_3](assist_debug_core_0_dram0_exception_monitor_3) module"]
pub type ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3;
#[doc = "`read()` method returns [assist_debug_core_0_dram0_exception_monitor_3::R](assist_debug_core_0_dram0_exception_monitor_3::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3 {}
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3"]
pub mod assist_debug_core_0_dram0_exception_monitor_3;
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_x_iram0_dram0_exception_monitor_0](assist_debug_core_x_iram0_dram0_exception_monitor_0) module"]
pub type ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0;
#[doc = "`read()` method returns [assist_debug_core_x_iram0_dram0_exception_monitor_0::R](assist_debug_core_x_iram0_dram0_exception_monitor_0::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_x_iram0_dram0_exception_monitor_0::W](assist_debug_core_x_iram0_dram0_exception_monitor_0::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 {}
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0"]
pub mod assist_debug_core_x_iram0_dram0_exception_monitor_0;
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_core_x_iram0_dram0_exception_monitor_1](assist_debug_core_x_iram0_dram0_exception_monitor_1) module"]
pub type ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<u32, _ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1;
#[doc = "`read()` method returns [assist_debug_core_x_iram0_dram0_exception_monitor_1::R](assist_debug_core_x_iram0_dram0_exception_monitor_1::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 {}
#[doc = "`write(|w| ..)` method takes [assist_debug_core_x_iram0_dram0_exception_monitor_1::W](assist_debug_core_x_iram0_dram0_exception_monitor_1::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 {}
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1"]
pub mod assist_debug_core_x_iram0_dram0_exception_monitor_1;
#[doc = "ASSIST_DEBUG_LOG_SETTING\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_setting](assist_debug_log_setting) module"]
pub type ASSIST_DEBUG_LOG_SETTING = crate::Reg<u32, _ASSIST_DEBUG_LOG_SETTING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_SETTING;
#[doc = "`read()` method returns [assist_debug_log_setting::R](assist_debug_log_setting::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_SETTING {}
#[doc = "`write(|w| ..)` method takes [assist_debug_log_setting::W](assist_debug_log_setting::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_LOG_SETTING {}
#[doc = "ASSIST_DEBUG_LOG_SETTING"]
pub mod assist_debug_log_setting;
#[doc = "ASSIST_DEBUG_LOG_DATA_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_data_0](assist_debug_log_data_0) module"]
pub type ASSIST_DEBUG_LOG_DATA_0 = crate::Reg<u32, _ASSIST_DEBUG_LOG_DATA_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_DATA_0;
#[doc = "`read()` method returns [assist_debug_log_data_0::R](assist_debug_log_data_0::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_DATA_0 {}
#[doc = "`write(|w| ..)` method takes [assist_debug_log_data_0::W](assist_debug_log_data_0::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_LOG_DATA_0 {}
#[doc = "ASSIST_DEBUG_LOG_DATA_0"]
pub mod assist_debug_log_data_0;
#[doc = "ASSIST_DEBUG_LOG_DATA_MASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_data_mask](assist_debug_log_data_mask) module"]
pub type ASSIST_DEBUG_LOG_DATA_MASK = crate::Reg<u32, _ASSIST_DEBUG_LOG_DATA_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_DATA_MASK;
#[doc = "`read()` method returns [assist_debug_log_data_mask::R](assist_debug_log_data_mask::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_DATA_MASK {}
#[doc = "`write(|w| ..)` method takes [assist_debug_log_data_mask::W](assist_debug_log_data_mask::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_LOG_DATA_MASK {}
#[doc = "ASSIST_DEBUG_LOG_DATA_MASK"]
pub mod assist_debug_log_data_mask;
#[doc = "ASSIST_DEBUG_LOG_MIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_min](assist_debug_log_min) module"]
pub type ASSIST_DEBUG_LOG_MIN = crate::Reg<u32, _ASSIST_DEBUG_LOG_MIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_MIN;
#[doc = "`read()` method returns [assist_debug_log_min::R](assist_debug_log_min::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_MIN {}
#[doc = "`write(|w| ..)` method takes [assist_debug_log_min::W](assist_debug_log_min::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_LOG_MIN {}
#[doc = "ASSIST_DEBUG_LOG_MIN"]
pub mod assist_debug_log_min;
#[doc = "ASSIST_DEBUG_LOG_MAX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_max](assist_debug_log_max) module"]
pub type ASSIST_DEBUG_LOG_MAX = crate::Reg<u32, _ASSIST_DEBUG_LOG_MAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_MAX;
#[doc = "`read()` method returns [assist_debug_log_max::R](assist_debug_log_max::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_MAX {}
#[doc = "`write(|w| ..)` method takes [assist_debug_log_max::W](assist_debug_log_max::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_LOG_MAX {}
#[doc = "ASSIST_DEBUG_LOG_MAX"]
pub mod assist_debug_log_max;
#[doc = "ASSIST_DEBUG_LOG_MEM_START\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_mem_start](assist_debug_log_mem_start) module"]
pub type ASSIST_DEBUG_LOG_MEM_START = crate::Reg<u32, _ASSIST_DEBUG_LOG_MEM_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_MEM_START;
#[doc = "`read()` method returns [assist_debug_log_mem_start::R](assist_debug_log_mem_start::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_MEM_START {}
#[doc = "`write(|w| ..)` method takes [assist_debug_log_mem_start::W](assist_debug_log_mem_start::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_LOG_MEM_START {}
#[doc = "ASSIST_DEBUG_LOG_MEM_START"]
pub mod assist_debug_log_mem_start;
#[doc = "ASSIST_DEBUG_LOG_MEM_END\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_mem_end](assist_debug_log_mem_end) module"]
pub type ASSIST_DEBUG_LOG_MEM_END = crate::Reg<u32, _ASSIST_DEBUG_LOG_MEM_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_MEM_END;
#[doc = "`read()` method returns [assist_debug_log_mem_end::R](assist_debug_log_mem_end::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_MEM_END {}
#[doc = "`write(|w| ..)` method takes [assist_debug_log_mem_end::W](assist_debug_log_mem_end::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_LOG_MEM_END {}
#[doc = "ASSIST_DEBUG_LOG_MEM_END"]
pub mod assist_debug_log_mem_end;
#[doc = "ASSIST_DEBUG_LOG_MEM_WRITING_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_mem_writing_addr](assist_debug_log_mem_writing_addr) module"]
pub type ASSIST_DEBUG_LOG_MEM_WRITING_ADDR = crate::Reg<u32, _ASSIST_DEBUG_LOG_MEM_WRITING_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_MEM_WRITING_ADDR;
#[doc = "`read()` method returns [assist_debug_log_mem_writing_addr::R](assist_debug_log_mem_writing_addr::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_MEM_WRITING_ADDR {}
#[doc = "ASSIST_DEBUG_LOG_MEM_WRITING_ADDR"]
pub mod assist_debug_log_mem_writing_addr;
#[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_log_mem_full_flag](assist_debug_log_mem_full_flag) module"]
pub type ASSIST_DEBUG_LOG_MEM_FULL_FLAG = crate::Reg<u32, _ASSIST_DEBUG_LOG_MEM_FULL_FLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_LOG_MEM_FULL_FLAG;
#[doc = "`read()` method returns [assist_debug_log_mem_full_flag::R](assist_debug_log_mem_full_flag::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_LOG_MEM_FULL_FLAG {}
#[doc = "`write(|w| ..)` method takes [assist_debug_log_mem_full_flag::W](assist_debug_log_mem_full_flag::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_LOG_MEM_FULL_FLAG {}
#[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG"]
pub mod assist_debug_log_mem_full_flag;
#[doc = "ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_c0re_0_lastpc_before_exception](assist_debug_c0re_0_lastpc_before_exception) module"]
pub type ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION =
    crate::Reg<u32, _ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION;
#[doc = "`read()` method returns [assist_debug_c0re_0_lastpc_before_exception::R](assist_debug_c0re_0_lastpc_before_exception::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION {}
#[doc = "ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
pub mod assist_debug_c0re_0_lastpc_before_exception;
#[doc = "ASSIST_DEBUG_C0RE_0_DEBUG_MODE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_c0re_0_debug_mode](assist_debug_c0re_0_debug_mode) module"]
pub type ASSIST_DEBUG_C0RE_0_DEBUG_MODE = crate::Reg<u32, _ASSIST_DEBUG_C0RE_0_DEBUG_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_C0RE_0_DEBUG_MODE;
#[doc = "`read()` method returns [assist_debug_c0re_0_debug_mode::R](assist_debug_c0re_0_debug_mode::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_C0RE_0_DEBUG_MODE {}
#[doc = "ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
pub mod assist_debug_c0re_0_debug_mode;
#[doc = "ASSIST_DEBUG_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_date](assist_debug_date) module"]
pub type ASSIST_DEBUG_DATE = crate::Reg<u32, _ASSIST_DEBUG_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSIST_DEBUG_DATE;
#[doc = "`read()` method returns [assist_debug_date::R](assist_debug_date::R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_DATE {}
#[doc = "`write(|w| ..)` method takes [assist_debug_date::W](assist_debug_date::W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_DATE {}
#[doc = "ASSIST_DEBUG_DATE"]
pub mod assist_debug_date;
