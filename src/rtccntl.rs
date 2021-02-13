#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_CNTL_OPTIONS0"]
    pub rtc_cntl_options0: RTC_CNTL_OPTIONS0,
    #[doc = "0x04 - RTC_CNTL_SLP_TIMER0"]
    pub rtc_cntl_slp_timer0: RTC_CNTL_SLP_TIMER0,
    #[doc = "0x08 - RTC_CNTL_SLP_TIMER1"]
    pub rtc_cntl_slp_timer1: RTC_CNTL_SLP_TIMER1,
    #[doc = "0x0c - RTC_CNTL_TIME_UPDATE"]
    pub rtc_cntl_time_update: RTC_CNTL_TIME_UPDATE,
    #[doc = "0x10 - RTC_CNTL_TIME_LOW0"]
    pub rtc_cntl_time_low0: RTC_CNTL_TIME_LOW0,
    #[doc = "0x14 - RTC_CNTL_TIME_HIGH0"]
    pub rtc_cntl_time_high0: RTC_CNTL_TIME_HIGH0,
    #[doc = "0x18 - RTC_CNTL_STATE0"]
    pub rtc_cntl_state0: RTC_CNTL_STATE0,
    #[doc = "0x1c - RTC_CNTL_TIMER1"]
    pub rtc_cntl_timer1: RTC_CNTL_TIMER1,
    #[doc = "0x20 - RTC_CNTL_TIMER2"]
    pub rtc_cntl_timer2: RTC_CNTL_TIMER2,
    #[doc = "0x24 - RTC_CNTL_TIMER3"]
    pub rtc_cntl_timer3: RTC_CNTL_TIMER3,
    #[doc = "0x28 - RTC_CNTL_TIMER4"]
    pub rtc_cntl_timer4: RTC_CNTL_TIMER4,
    #[doc = "0x2c - RTC_CNTL_TIMER5"]
    pub rtc_cntl_timer5: RTC_CNTL_TIMER5,
    #[doc = "0x30 - RTC_CNTL_TIMER6"]
    pub rtc_cntl_timer6: RTC_CNTL_TIMER6,
    #[doc = "0x34 - RTC_CNTL_ANA_CONF"]
    pub rtc_cntl_ana_conf: RTC_CNTL_ANA_CONF,
    #[doc = "0x38 - RTC_CNTL_RESET_STATE"]
    pub rtc_cntl_reset_state: RTC_CNTL_RESET_STATE,
    #[doc = "0x3c - RTC_CNTL_WAKEUP_STATE"]
    pub rtc_cntl_wakeup_state: RTC_CNTL_WAKEUP_STATE,
    #[doc = "0x40 - RTC_CNTL_INT_ENA"]
    pub rtc_cntl_int_ena: RTC_CNTL_INT_ENA,
    #[doc = "0x44 - RTC_CNTL_INT_RAW"]
    pub rtc_cntl_int_raw: RTC_CNTL_INT_RAW,
    #[doc = "0x48 - RTC_CNTL_INT_ST"]
    pub rtc_cntl_int_st: RTC_CNTL_INT_ST,
    #[doc = "0x4c - RTC_CNTL_INT_CLR"]
    pub rtc_cntl_int_clr: RTC_CNTL_INT_CLR,
    #[doc = "0x50 - RTC_CNTL_STORE0"]
    pub rtc_cntl_store0: RTC_CNTL_STORE0,
    #[doc = "0x54 - RTC_CNTL_STORE1"]
    pub rtc_cntl_store1: RTC_CNTL_STORE1,
    #[doc = "0x58 - RTC_CNTL_STORE2"]
    pub rtc_cntl_store2: RTC_CNTL_STORE2,
    #[doc = "0x5c - RTC_CNTL_STORE3"]
    pub rtc_cntl_store3: RTC_CNTL_STORE3,
    #[doc = "0x60 - RTC_CNTL_EXT_XTL_CONF"]
    pub rtc_cntl_ext_xtl_conf: RTC_CNTL_EXT_XTL_CONF,
    #[doc = "0x64 - RTC_CNTL_EXT_WAKEUP_CONF"]
    pub rtc_cntl_ext_wakeup_conf: RTC_CNTL_EXT_WAKEUP_CONF,
    #[doc = "0x68 - RTC_CNTL_SLP_REJECT_CONF"]
    pub rtc_cntl_slp_reject_conf: RTC_CNTL_SLP_REJECT_CONF,
    #[doc = "0x6c - RTC_CNTL_CPU_PERIOD_CONF"]
    pub rtc_cntl_cpu_period_conf: RTC_CNTL_CPU_PERIOD_CONF,
    #[doc = "0x70 - RTC_CNTL_CLK_CONF"]
    pub rtc_cntl_clk_conf: RTC_CNTL_CLK_CONF,
    #[doc = "0x74 - RTC_CNTL_SLOW_CLK_CONF"]
    pub rtc_cntl_slow_clk_conf: RTC_CNTL_SLOW_CLK_CONF,
    #[doc = "0x78 - RTC_CNTL_SDIO_CONF"]
    pub rtc_cntl_sdio_conf: RTC_CNTL_SDIO_CONF,
    #[doc = "0x7c - RTC_CNTL_BIAS_CONF"]
    pub rtc_cntl_bias_conf: RTC_CNTL_BIAS_CONF,
    #[doc = "0x80 - RTC_CNTL"]
    pub rtc_cntl: RTC_CNTL,
    #[doc = "0x84 - RTC_CNTL_PWC"]
    pub rtc_cntl_pwc: RTC_CNTL_PWC,
    #[doc = "0x88 - RTC_CNTL_DIG_PWC"]
    pub rtc_cntl_dig_pwc: RTC_CNTL_DIG_PWC,
    #[doc = "0x8c - RTC_CNTL_DIG_ISO"]
    pub rtc_cntl_dig_iso: RTC_CNTL_DIG_ISO,
    #[doc = "0x90 - RTC_CNTL_WDTCONFIG0"]
    pub rtc_cntl_wdtconfig0: RTC_CNTL_WDTCONFIG0,
    #[doc = "0x94 - RTC_CNTL_WDTCONFIG1"]
    pub rtc_cntl_wdtconfig1: RTC_CNTL_WDTCONFIG1,
    #[doc = "0x98 - RTC_CNTL_WDTCONFIG2"]
    pub rtc_cntl_wdtconfig2: RTC_CNTL_WDTCONFIG2,
    #[doc = "0x9c - RTC_CNTL_WDTCONFIG3"]
    pub rtc_cntl_wdtconfig3: RTC_CNTL_WDTCONFIG3,
    #[doc = "0xa0 - RTC_CNTL_WDTCONFIG4"]
    pub rtc_cntl_wdtconfig4: RTC_CNTL_WDTCONFIG4,
    #[doc = "0xa4 - RTC_CNTL_WDTFEED"]
    pub rtc_cntl_wdtfeed: RTC_CNTL_WDTFEED,
    #[doc = "0xa8 - RTC_CNTL_WDTWPROTECT"]
    pub rtc_cntl_wdtwprotect: RTC_CNTL_WDTWPROTECT,
    #[doc = "0xac - RTC_CNTL_SWD_CONF"]
    pub rtc_cntl_swd_conf: RTC_CNTL_SWD_CONF,
    #[doc = "0xb0 - RTC_CNTL_SWD_WPROTECT"]
    pub rtc_cntl_swd_wprotect: RTC_CNTL_SWD_WPROTECT,
    #[doc = "0xb4 - RTC_CNTL_SW_CPU_STALL"]
    pub rtc_cntl_sw_cpu_stall: RTC_CNTL_SW_CPU_STALL,
    #[doc = "0xb8 - RTC_CNTL_STORE4"]
    pub rtc_cntl_store4: RTC_CNTL_STORE4,
    #[doc = "0xbc - RTC_CNTL_STORE5"]
    pub rtc_cntl_store5: RTC_CNTL_STORE5,
    #[doc = "0xc0 - RTC_CNTL_STORE6"]
    pub rtc_cntl_store6: RTC_CNTL_STORE6,
    #[doc = "0xc4 - RTC_CNTL_STORE7"]
    pub rtc_cntl_store7: RTC_CNTL_STORE7,
    #[doc = "0xc8 - RTC_CNTL_LOW_POWER_ST"]
    pub rtc_cntl_low_power_st: RTC_CNTL_LOW_POWER_ST,
    #[doc = "0xcc - RTC_CNTL_DIAG0"]
    pub rtc_cntl_diag0: RTC_CNTL_DIAG0,
    #[doc = "0xd0 - RTC_CNTL_PAD_HOLD"]
    pub rtc_cntl_pad_hold: RTC_CNTL_PAD_HOLD,
    #[doc = "0xd4 - RTC_CNTL_DIG_PAD_HOLD"]
    pub rtc_cntl_dig_pad_hold: RTC_CNTL_DIG_PAD_HOLD,
    #[doc = "0xd8 - RTC_CNTL_BROWN_OUT"]
    pub rtc_cntl_brown_out: RTC_CNTL_BROWN_OUT,
    #[doc = "0xdc - RTC_CNTL_TIME_LOW1"]
    pub rtc_cntl_time_low1: RTC_CNTL_TIME_LOW1,
    #[doc = "0xe0 - RTC_CNTL_TIME_HIGH1"]
    pub rtc_cntl_time_high1: RTC_CNTL_TIME_HIGH1,
    #[doc = "0xe4 - RTC_CNTL_XTAL32K_CLK_FACTOR"]
    pub rtc_cntl_xtal32k_clk_factor: RTC_CNTL_XTAL32K_CLK_FACTOR,
    #[doc = "0xe8 - RTC_CNTL_XTAL32K_CONF"]
    pub rtc_cntl_xtal32k_conf: RTC_CNTL_XTAL32K_CONF,
    #[doc = "0xec - RTC_CNTL_USB_CONF"]
    pub rtc_cntl_usb_conf: RTC_CNTL_USB_CONF,
    #[doc = "0xf0 - RTC_CNTL_SLP_REJECT_CAUSE"]
    pub rtc_cntl_slp_reject_cause: RTC_CNTL_SLP_REJECT_CAUSE,
    #[doc = "0xf4 - RTC_CNTL_OPTION1"]
    pub rtc_cntl_option1: RTC_CNTL_OPTION1,
    #[doc = "0xf8 - RTC_CNTL_SLP_WAKEUP_CAUSE"]
    pub rtc_cntl_slp_wakeup_cause: RTC_CNTL_SLP_WAKEUP_CAUSE,
    #[doc = "0xfc - RTC_CNTL_ULP_CP_TIMER_1"]
    pub rtc_cntl_ulp_cp_timer_1: RTC_CNTL_ULP_CP_TIMER_1,
    #[doc = "0x100 - RTC_CNTL_INT_ENA_W1TS"]
    pub rtc_cntl_int_ena_w1ts: RTC_CNTL_INT_ENA_W1TS,
    #[doc = "0x104 - RTC_CNTL_INT_ENA_W1TC"]
    pub rtc_cntl_int_ena_w1tc: RTC_CNTL_INT_ENA_W1TC,
    #[doc = "0x108 - RTC_CNTL_RETENTION_CTRL"]
    pub rtc_cntl_retention_ctrl: RTC_CNTL_RETENTION_CTRL,
    #[doc = "0x10c - RTC_CNTL_FIB_SEL"]
    pub rtc_cntl_fib_sel: RTC_CNTL_FIB_SEL,
    #[doc = "0x110 - RTC_CNTL_GPIO_WAKEUP"]
    pub rtc_cntl_gpio_wakeup: RTC_CNTL_GPIO_WAKEUP,
    #[doc = "0x114 - RTC_CNTL_DBG_SEL"]
    pub rtc_cntl_dbg_sel: RTC_CNTL_DBG_SEL,
    #[doc = "0x118 - RTC_CNTL_DBG_MAP"]
    pub rtc_cntl_dbg_map: RTC_CNTL_DBG_MAP,
    #[doc = "0x11c - RTC_CNTL_SENSOR_CTRL"]
    pub rtc_cntl_sensor_ctrl: RTC_CNTL_SENSOR_CTRL,
    #[doc = "0x120 - RTC_CNTL_DBG_SAR_SEL"]
    pub rtc_cntl_dbg_sar_sel: RTC_CNTL_DBG_SAR_SEL,
    #[doc = "0x124 - RTC_CNTL_PG_CTRL"]
    pub rtc_cntl_pg_ctrl: RTC_CNTL_PG_CTRL,
    _reserved74: [u8; 212usize],
    #[doc = "0x1fc - RTC_CNTL_DATE"]
    pub rtc_cntl_date: RTC_CNTL_DATE,
}
#[doc = "RTC_CNTL_OPTIONS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_options0](rtc_cntl_options0) module"]
pub type RTC_CNTL_OPTIONS0 = crate::Reg<u32, _RTC_CNTL_OPTIONS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_OPTIONS0;
#[doc = "`read()` method returns [rtc_cntl_options0::R](rtc_cntl_options0::R) reader structure"]
impl crate::Readable for RTC_CNTL_OPTIONS0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_options0::W](rtc_cntl_options0::W) writer structure"]
impl crate::Writable for RTC_CNTL_OPTIONS0 {}
#[doc = "RTC_CNTL_OPTIONS0"]
pub mod rtc_cntl_options0;
#[doc = "RTC_CNTL_SLP_TIMER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_slp_timer0](rtc_cntl_slp_timer0) module"]
pub type RTC_CNTL_SLP_TIMER0 = crate::Reg<u32, _RTC_CNTL_SLP_TIMER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_TIMER0;
#[doc = "`read()` method returns [rtc_cntl_slp_timer0::R](rtc_cntl_slp_timer0::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_TIMER0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_timer0::W](rtc_cntl_slp_timer0::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_TIMER0 {}
#[doc = "RTC_CNTL_SLP_TIMER0"]
pub mod rtc_cntl_slp_timer0;
#[doc = "RTC_CNTL_SLP_TIMER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_slp_timer1](rtc_cntl_slp_timer1) module"]
pub type RTC_CNTL_SLP_TIMER1 = crate::Reg<u32, _RTC_CNTL_SLP_TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_TIMER1;
#[doc = "`read()` method returns [rtc_cntl_slp_timer1::R](rtc_cntl_slp_timer1::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_TIMER1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_timer1::W](rtc_cntl_slp_timer1::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_TIMER1 {}
#[doc = "RTC_CNTL_SLP_TIMER1"]
pub mod rtc_cntl_slp_timer1;
#[doc = "RTC_CNTL_TIME_UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_time_update](rtc_cntl_time_update) module"]
pub type RTC_CNTL_TIME_UPDATE = crate::Reg<u32, _RTC_CNTL_TIME_UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME_UPDATE;
#[doc = "`read()` method returns [rtc_cntl_time_update::R](rtc_cntl_time_update::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME_UPDATE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_time_update::W](rtc_cntl_time_update::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIME_UPDATE {}
#[doc = "RTC_CNTL_TIME_UPDATE"]
pub mod rtc_cntl_time_update;
#[doc = "RTC_CNTL_TIME_LOW0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_time_low0](rtc_cntl_time_low0) module"]
pub type RTC_CNTL_TIME_LOW0 = crate::Reg<u32, _RTC_CNTL_TIME_LOW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME_LOW0;
#[doc = "`read()` method returns [rtc_cntl_time_low0::R](rtc_cntl_time_low0::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME_LOW0 {}
#[doc = "RTC_CNTL_TIME_LOW0"]
pub mod rtc_cntl_time_low0;
#[doc = "RTC_CNTL_TIME_HIGH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_time_high0](rtc_cntl_time_high0) module"]
pub type RTC_CNTL_TIME_HIGH0 = crate::Reg<u32, _RTC_CNTL_TIME_HIGH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME_HIGH0;
#[doc = "`read()` method returns [rtc_cntl_time_high0::R](rtc_cntl_time_high0::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME_HIGH0 {}
#[doc = "RTC_CNTL_TIME_HIGH0"]
pub mod rtc_cntl_time_high0;
#[doc = "RTC_CNTL_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_state0](rtc_cntl_state0) module"]
pub type RTC_CNTL_STATE0 = crate::Reg<u32, _RTC_CNTL_STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STATE0;
#[doc = "`read()` method returns [rtc_cntl_state0::R](rtc_cntl_state0::R) reader structure"]
impl crate::Readable for RTC_CNTL_STATE0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_state0::W](rtc_cntl_state0::W) writer structure"]
impl crate::Writable for RTC_CNTL_STATE0 {}
#[doc = "RTC_CNTL_STATE0"]
pub mod rtc_cntl_state0;
#[doc = "RTC_CNTL_TIMER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_timer1](rtc_cntl_timer1) module"]
pub type RTC_CNTL_TIMER1 = crate::Reg<u32, _RTC_CNTL_TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER1;
#[doc = "`read()` method returns [rtc_cntl_timer1::R](rtc_cntl_timer1::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer1::W](rtc_cntl_timer1::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER1 {}
#[doc = "RTC_CNTL_TIMER1"]
pub mod rtc_cntl_timer1;
#[doc = "RTC_CNTL_TIMER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_timer2](rtc_cntl_timer2) module"]
pub type RTC_CNTL_TIMER2 = crate::Reg<u32, _RTC_CNTL_TIMER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER2;
#[doc = "`read()` method returns [rtc_cntl_timer2::R](rtc_cntl_timer2::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER2 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer2::W](rtc_cntl_timer2::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER2 {}
#[doc = "RTC_CNTL_TIMER2"]
pub mod rtc_cntl_timer2;
#[doc = "RTC_CNTL_TIMER3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_timer3](rtc_cntl_timer3) module"]
pub type RTC_CNTL_TIMER3 = crate::Reg<u32, _RTC_CNTL_TIMER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER3;
#[doc = "`read()` method returns [rtc_cntl_timer3::R](rtc_cntl_timer3::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER3 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer3::W](rtc_cntl_timer3::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER3 {}
#[doc = "RTC_CNTL_TIMER3"]
pub mod rtc_cntl_timer3;
#[doc = "RTC_CNTL_TIMER4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_timer4](rtc_cntl_timer4) module"]
pub type RTC_CNTL_TIMER4 = crate::Reg<u32, _RTC_CNTL_TIMER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER4;
#[doc = "`read()` method returns [rtc_cntl_timer4::R](rtc_cntl_timer4::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER4 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer4::W](rtc_cntl_timer4::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER4 {}
#[doc = "RTC_CNTL_TIMER4"]
pub mod rtc_cntl_timer4;
#[doc = "RTC_CNTL_TIMER5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_timer5](rtc_cntl_timer5) module"]
pub type RTC_CNTL_TIMER5 = crate::Reg<u32, _RTC_CNTL_TIMER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER5;
#[doc = "`read()` method returns [rtc_cntl_timer5::R](rtc_cntl_timer5::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER5 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer5::W](rtc_cntl_timer5::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER5 {}
#[doc = "RTC_CNTL_TIMER5"]
pub mod rtc_cntl_timer5;
#[doc = "RTC_CNTL_TIMER6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_timer6](rtc_cntl_timer6) module"]
pub type RTC_CNTL_TIMER6 = crate::Reg<u32, _RTC_CNTL_TIMER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIMER6;
#[doc = "`read()` method returns [rtc_cntl_timer6::R](rtc_cntl_timer6::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIMER6 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_timer6::W](rtc_cntl_timer6::W) writer structure"]
impl crate::Writable for RTC_CNTL_TIMER6 {}
#[doc = "RTC_CNTL_TIMER6"]
pub mod rtc_cntl_timer6;
#[doc = "RTC_CNTL_ANA_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_ana_conf](rtc_cntl_ana_conf) module"]
pub type RTC_CNTL_ANA_CONF = crate::Reg<u32, _RTC_CNTL_ANA_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_ANA_CONF;
#[doc = "`read()` method returns [rtc_cntl_ana_conf::R](rtc_cntl_ana_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_ANA_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ana_conf::W](rtc_cntl_ana_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_ANA_CONF {}
#[doc = "RTC_CNTL_ANA_CONF"]
pub mod rtc_cntl_ana_conf;
#[doc = "RTC_CNTL_RESET_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_reset_state](rtc_cntl_reset_state) module"]
pub type RTC_CNTL_RESET_STATE = crate::Reg<u32, _RTC_CNTL_RESET_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_RESET_STATE;
#[doc = "`read()` method returns [rtc_cntl_reset_state::R](rtc_cntl_reset_state::R) reader structure"]
impl crate::Readable for RTC_CNTL_RESET_STATE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_reset_state::W](rtc_cntl_reset_state::W) writer structure"]
impl crate::Writable for RTC_CNTL_RESET_STATE {}
#[doc = "RTC_CNTL_RESET_STATE"]
pub mod rtc_cntl_reset_state;
#[doc = "RTC_CNTL_WAKEUP_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_wakeup_state](rtc_cntl_wakeup_state) module"]
pub type RTC_CNTL_WAKEUP_STATE = crate::Reg<u32, _RTC_CNTL_WAKEUP_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WAKEUP_STATE;
#[doc = "`read()` method returns [rtc_cntl_wakeup_state::R](rtc_cntl_wakeup_state::R) reader structure"]
impl crate::Readable for RTC_CNTL_WAKEUP_STATE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wakeup_state::W](rtc_cntl_wakeup_state::W) writer structure"]
impl crate::Writable for RTC_CNTL_WAKEUP_STATE {}
#[doc = "RTC_CNTL_WAKEUP_STATE"]
pub mod rtc_cntl_wakeup_state;
#[doc = "RTC_CNTL_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_int_ena](rtc_cntl_int_ena) module"]
pub type RTC_CNTL_INT_ENA = crate::Reg<u32, _RTC_CNTL_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_ENA;
#[doc = "`read()` method returns [rtc_cntl_int_ena::R](rtc_cntl_int_ena::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_ena::W](rtc_cntl_int_ena::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_ENA {}
#[doc = "RTC_CNTL_INT_ENA"]
pub mod rtc_cntl_int_ena;
#[doc = "RTC_CNTL_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_int_raw](rtc_cntl_int_raw) module"]
pub type RTC_CNTL_INT_RAW = crate::Reg<u32, _RTC_CNTL_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_RAW;
#[doc = "`read()` method returns [rtc_cntl_int_raw::R](rtc_cntl_int_raw::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_RAW {}
#[doc = "RTC_CNTL_INT_RAW"]
pub mod rtc_cntl_int_raw;
#[doc = "RTC_CNTL_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_int_st](rtc_cntl_int_st) module"]
pub type RTC_CNTL_INT_ST = crate::Reg<u32, _RTC_CNTL_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_ST;
#[doc = "`read()` method returns [rtc_cntl_int_st::R](rtc_cntl_int_st::R) reader structure"]
impl crate::Readable for RTC_CNTL_INT_ST {}
#[doc = "RTC_CNTL_INT_ST"]
pub mod rtc_cntl_int_st;
#[doc = "RTC_CNTL_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_int_clr](rtc_cntl_int_clr) module"]
pub type RTC_CNTL_INT_CLR = crate::Reg<u32, _RTC_CNTL_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_CLR;
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_clr::W](rtc_cntl_int_clr::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_CLR {}
#[doc = "RTC_CNTL_INT_CLR"]
pub mod rtc_cntl_int_clr;
#[doc = "RTC_CNTL_STORE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_store0](rtc_cntl_store0) module"]
pub type RTC_CNTL_STORE0 = crate::Reg<u32, _RTC_CNTL_STORE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE0;
#[doc = "`read()` method returns [rtc_cntl_store0::R](rtc_cntl_store0::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store0::W](rtc_cntl_store0::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE0 {}
#[doc = "RTC_CNTL_STORE0"]
pub mod rtc_cntl_store0;
#[doc = "RTC_CNTL_STORE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_store1](rtc_cntl_store1) module"]
pub type RTC_CNTL_STORE1 = crate::Reg<u32, _RTC_CNTL_STORE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE1;
#[doc = "`read()` method returns [rtc_cntl_store1::R](rtc_cntl_store1::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store1::W](rtc_cntl_store1::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE1 {}
#[doc = "RTC_CNTL_STORE1"]
pub mod rtc_cntl_store1;
#[doc = "RTC_CNTL_STORE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_store2](rtc_cntl_store2) module"]
pub type RTC_CNTL_STORE2 = crate::Reg<u32, _RTC_CNTL_STORE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE2;
#[doc = "`read()` method returns [rtc_cntl_store2::R](rtc_cntl_store2::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE2 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store2::W](rtc_cntl_store2::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE2 {}
#[doc = "RTC_CNTL_STORE2"]
pub mod rtc_cntl_store2;
#[doc = "RTC_CNTL_STORE3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_store3](rtc_cntl_store3) module"]
pub type RTC_CNTL_STORE3 = crate::Reg<u32, _RTC_CNTL_STORE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE3;
#[doc = "`read()` method returns [rtc_cntl_store3::R](rtc_cntl_store3::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE3 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store3::W](rtc_cntl_store3::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE3 {}
#[doc = "RTC_CNTL_STORE3"]
pub mod rtc_cntl_store3;
#[doc = "RTC_CNTL_EXT_XTL_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_ext_xtl_conf](rtc_cntl_ext_xtl_conf) module"]
pub type RTC_CNTL_EXT_XTL_CONF = crate::Reg<u32, _RTC_CNTL_EXT_XTL_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_XTL_CONF;
#[doc = "`read()` method returns [rtc_cntl_ext_xtl_conf::R](rtc_cntl_ext_xtl_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_XTL_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_xtl_conf::W](rtc_cntl_ext_xtl_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_XTL_CONF {}
#[doc = "RTC_CNTL_EXT_XTL_CONF"]
pub mod rtc_cntl_ext_xtl_conf;
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_ext_wakeup_conf](rtc_cntl_ext_wakeup_conf) module"]
pub type RTC_CNTL_EXT_WAKEUP_CONF = crate::Reg<u32, _RTC_CNTL_EXT_WAKEUP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_EXT_WAKEUP_CONF;
#[doc = "`read()` method returns [rtc_cntl_ext_wakeup_conf::R](rtc_cntl_ext_wakeup_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_EXT_WAKEUP_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ext_wakeup_conf::W](rtc_cntl_ext_wakeup_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_EXT_WAKEUP_CONF {}
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF"]
pub mod rtc_cntl_ext_wakeup_conf;
#[doc = "RTC_CNTL_SLP_REJECT_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_slp_reject_conf](rtc_cntl_slp_reject_conf) module"]
pub type RTC_CNTL_SLP_REJECT_CONF = crate::Reg<u32, _RTC_CNTL_SLP_REJECT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_REJECT_CONF;
#[doc = "`read()` method returns [rtc_cntl_slp_reject_conf::R](rtc_cntl_slp_reject_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_REJECT_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slp_reject_conf::W](rtc_cntl_slp_reject_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLP_REJECT_CONF {}
#[doc = "RTC_CNTL_SLP_REJECT_CONF"]
pub mod rtc_cntl_slp_reject_conf;
#[doc = "RTC_CNTL_CPU_PERIOD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_cpu_period_conf](rtc_cntl_cpu_period_conf) module"]
pub type RTC_CNTL_CPU_PERIOD_CONF = crate::Reg<u32, _RTC_CNTL_CPU_PERIOD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_CPU_PERIOD_CONF;
#[doc = "`read()` method returns [rtc_cntl_cpu_period_conf::R](rtc_cntl_cpu_period_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_CPU_PERIOD_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_cpu_period_conf::W](rtc_cntl_cpu_period_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_CPU_PERIOD_CONF {}
#[doc = "RTC_CNTL_CPU_PERIOD_CONF"]
pub mod rtc_cntl_cpu_period_conf;
#[doc = "RTC_CNTL_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_clk_conf](rtc_cntl_clk_conf) module"]
pub type RTC_CNTL_CLK_CONF = crate::Reg<u32, _RTC_CNTL_CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_CLK_CONF;
#[doc = "`read()` method returns [rtc_cntl_clk_conf::R](rtc_cntl_clk_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_clk_conf::W](rtc_cntl_clk_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_CLK_CONF {}
#[doc = "RTC_CNTL_CLK_CONF"]
pub mod rtc_cntl_clk_conf;
#[doc = "RTC_CNTL_SLOW_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_slow_clk_conf](rtc_cntl_slow_clk_conf) module"]
pub type RTC_CNTL_SLOW_CLK_CONF = crate::Reg<u32, _RTC_CNTL_SLOW_CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLOW_CLK_CONF;
#[doc = "`read()` method returns [rtc_cntl_slow_clk_conf::R](rtc_cntl_slow_clk_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLOW_CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_slow_clk_conf::W](rtc_cntl_slow_clk_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_SLOW_CLK_CONF {}
#[doc = "RTC_CNTL_SLOW_CLK_CONF"]
pub mod rtc_cntl_slow_clk_conf;
#[doc = "RTC_CNTL_SDIO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_sdio_conf](rtc_cntl_sdio_conf) module"]
pub type RTC_CNTL_SDIO_CONF = crate::Reg<u32, _RTC_CNTL_SDIO_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SDIO_CONF;
#[doc = "`read()` method returns [rtc_cntl_sdio_conf::R](rtc_cntl_sdio_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_SDIO_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sdio_conf::W](rtc_cntl_sdio_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_SDIO_CONF {}
#[doc = "RTC_CNTL_SDIO_CONF"]
pub mod rtc_cntl_sdio_conf;
#[doc = "RTC_CNTL_BIAS_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_bias_conf](rtc_cntl_bias_conf) module"]
pub type RTC_CNTL_BIAS_CONF = crate::Reg<u32, _RTC_CNTL_BIAS_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_BIAS_CONF;
#[doc = "`read()` method returns [rtc_cntl_bias_conf::R](rtc_cntl_bias_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_BIAS_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_bias_conf::W](rtc_cntl_bias_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_BIAS_CONF {}
#[doc = "RTC_CNTL_BIAS_CONF"]
pub mod rtc_cntl_bias_conf;
#[doc = "RTC_CNTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl](rtc_cntl) module"]
pub type RTC_CNTL = crate::Reg<u32, _RTC_CNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL;
#[doc = "`read()` method returns [rtc_cntl::R](rtc_cntl::R) reader structure"]
impl crate::Readable for RTC_CNTL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl::W](rtc_cntl::W) writer structure"]
impl crate::Writable for RTC_CNTL {}
#[doc = "RTC_CNTL"]
pub mod rtc_cntl;
#[doc = "RTC_CNTL_PWC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_pwc](rtc_cntl_pwc) module"]
pub type RTC_CNTL_PWC = crate::Reg<u32, _RTC_CNTL_PWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_PWC;
#[doc = "`read()` method returns [rtc_cntl_pwc::R](rtc_cntl_pwc::R) reader structure"]
impl crate::Readable for RTC_CNTL_PWC {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_pwc::W](rtc_cntl_pwc::W) writer structure"]
impl crate::Writable for RTC_CNTL_PWC {}
#[doc = "RTC_CNTL_PWC"]
pub mod rtc_cntl_pwc;
#[doc = "RTC_CNTL_DIG_PWC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dig_pwc](rtc_cntl_dig_pwc) module"]
pub type RTC_CNTL_DIG_PWC = crate::Reg<u32, _RTC_CNTL_DIG_PWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIG_PWC;
#[doc = "`read()` method returns [rtc_cntl_dig_pwc::R](rtc_cntl_dig_pwc::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIG_PWC {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dig_pwc::W](rtc_cntl_dig_pwc::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIG_PWC {}
#[doc = "RTC_CNTL_DIG_PWC"]
pub mod rtc_cntl_dig_pwc;
#[doc = "RTC_CNTL_DIG_ISO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dig_iso](rtc_cntl_dig_iso) module"]
pub type RTC_CNTL_DIG_ISO = crate::Reg<u32, _RTC_CNTL_DIG_ISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIG_ISO;
#[doc = "`read()` method returns [rtc_cntl_dig_iso::R](rtc_cntl_dig_iso::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIG_ISO {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dig_iso::W](rtc_cntl_dig_iso::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIG_ISO {}
#[doc = "RTC_CNTL_DIG_ISO"]
pub mod rtc_cntl_dig_iso;
#[doc = "RTC_CNTL_WDTCONFIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_wdtconfig0](rtc_cntl_wdtconfig0) module"]
pub type RTC_CNTL_WDTCONFIG0 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG0;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig0::R](rtc_cntl_wdtconfig0::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG0 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig0::W](rtc_cntl_wdtconfig0::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG0 {}
#[doc = "RTC_CNTL_WDTCONFIG0"]
pub mod rtc_cntl_wdtconfig0;
#[doc = "RTC_CNTL_WDTCONFIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_wdtconfig1](rtc_cntl_wdtconfig1) module"]
pub type RTC_CNTL_WDTCONFIG1 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG1;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig1::R](rtc_cntl_wdtconfig1::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig1::W](rtc_cntl_wdtconfig1::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG1 {}
#[doc = "RTC_CNTL_WDTCONFIG1"]
pub mod rtc_cntl_wdtconfig1;
#[doc = "RTC_CNTL_WDTCONFIG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_wdtconfig2](rtc_cntl_wdtconfig2) module"]
pub type RTC_CNTL_WDTCONFIG2 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG2;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig2::R](rtc_cntl_wdtconfig2::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG2 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig2::W](rtc_cntl_wdtconfig2::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG2 {}
#[doc = "RTC_CNTL_WDTCONFIG2"]
pub mod rtc_cntl_wdtconfig2;
#[doc = "RTC_CNTL_WDTCONFIG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_wdtconfig3](rtc_cntl_wdtconfig3) module"]
pub type RTC_CNTL_WDTCONFIG3 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG3;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig3::R](rtc_cntl_wdtconfig3::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG3 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig3::W](rtc_cntl_wdtconfig3::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG3 {}
#[doc = "RTC_CNTL_WDTCONFIG3"]
pub mod rtc_cntl_wdtconfig3;
#[doc = "RTC_CNTL_WDTCONFIG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_wdtconfig4](rtc_cntl_wdtconfig4) module"]
pub type RTC_CNTL_WDTCONFIG4 = crate::Reg<u32, _RTC_CNTL_WDTCONFIG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTCONFIG4;
#[doc = "`read()` method returns [rtc_cntl_wdtconfig4::R](rtc_cntl_wdtconfig4::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTCONFIG4 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtconfig4::W](rtc_cntl_wdtconfig4::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTCONFIG4 {}
#[doc = "RTC_CNTL_WDTCONFIG4"]
pub mod rtc_cntl_wdtconfig4;
#[doc = "RTC_CNTL_WDTFEED\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_wdtfeed](rtc_cntl_wdtfeed) module"]
pub type RTC_CNTL_WDTFEED = crate::Reg<u32, _RTC_CNTL_WDTFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTFEED;
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtfeed::W](rtc_cntl_wdtfeed::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTFEED {}
#[doc = "RTC_CNTL_WDTFEED"]
pub mod rtc_cntl_wdtfeed;
#[doc = "RTC_CNTL_WDTWPROTECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_wdtwprotect](rtc_cntl_wdtwprotect) module"]
pub type RTC_CNTL_WDTWPROTECT = crate::Reg<u32, _RTC_CNTL_WDTWPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_WDTWPROTECT;
#[doc = "`read()` method returns [rtc_cntl_wdtwprotect::R](rtc_cntl_wdtwprotect::R) reader structure"]
impl crate::Readable for RTC_CNTL_WDTWPROTECT {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_wdtwprotect::W](rtc_cntl_wdtwprotect::W) writer structure"]
impl crate::Writable for RTC_CNTL_WDTWPROTECT {}
#[doc = "RTC_CNTL_WDTWPROTECT"]
pub mod rtc_cntl_wdtwprotect;
#[doc = "RTC_CNTL_SWD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_swd_conf](rtc_cntl_swd_conf) module"]
pub type RTC_CNTL_SWD_CONF = crate::Reg<u32, _RTC_CNTL_SWD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SWD_CONF;
#[doc = "`read()` method returns [rtc_cntl_swd_conf::R](rtc_cntl_swd_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_SWD_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_swd_conf::W](rtc_cntl_swd_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_SWD_CONF {}
#[doc = "RTC_CNTL_SWD_CONF"]
pub mod rtc_cntl_swd_conf;
#[doc = "RTC_CNTL_SWD_WPROTECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_swd_wprotect](rtc_cntl_swd_wprotect) module"]
pub type RTC_CNTL_SWD_WPROTECT = crate::Reg<u32, _RTC_CNTL_SWD_WPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SWD_WPROTECT;
#[doc = "`read()` method returns [rtc_cntl_swd_wprotect::R](rtc_cntl_swd_wprotect::R) reader structure"]
impl crate::Readable for RTC_CNTL_SWD_WPROTECT {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_swd_wprotect::W](rtc_cntl_swd_wprotect::W) writer structure"]
impl crate::Writable for RTC_CNTL_SWD_WPROTECT {}
#[doc = "RTC_CNTL_SWD_WPROTECT"]
pub mod rtc_cntl_swd_wprotect;
#[doc = "RTC_CNTL_SW_CPU_STALL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_sw_cpu_stall](rtc_cntl_sw_cpu_stall) module"]
pub type RTC_CNTL_SW_CPU_STALL = crate::Reg<u32, _RTC_CNTL_SW_CPU_STALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SW_CPU_STALL;
#[doc = "`read()` method returns [rtc_cntl_sw_cpu_stall::R](rtc_cntl_sw_cpu_stall::R) reader structure"]
impl crate::Readable for RTC_CNTL_SW_CPU_STALL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sw_cpu_stall::W](rtc_cntl_sw_cpu_stall::W) writer structure"]
impl crate::Writable for RTC_CNTL_SW_CPU_STALL {}
#[doc = "RTC_CNTL_SW_CPU_STALL"]
pub mod rtc_cntl_sw_cpu_stall;
#[doc = "RTC_CNTL_STORE4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_store4](rtc_cntl_store4) module"]
pub type RTC_CNTL_STORE4 = crate::Reg<u32, _RTC_CNTL_STORE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE4;
#[doc = "`read()` method returns [rtc_cntl_store4::R](rtc_cntl_store4::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE4 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store4::W](rtc_cntl_store4::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE4 {}
#[doc = "RTC_CNTL_STORE4"]
pub mod rtc_cntl_store4;
#[doc = "RTC_CNTL_STORE5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_store5](rtc_cntl_store5) module"]
pub type RTC_CNTL_STORE5 = crate::Reg<u32, _RTC_CNTL_STORE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE5;
#[doc = "`read()` method returns [rtc_cntl_store5::R](rtc_cntl_store5::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE5 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store5::W](rtc_cntl_store5::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE5 {}
#[doc = "RTC_CNTL_STORE5"]
pub mod rtc_cntl_store5;
#[doc = "RTC_CNTL_STORE6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_store6](rtc_cntl_store6) module"]
pub type RTC_CNTL_STORE6 = crate::Reg<u32, _RTC_CNTL_STORE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE6;
#[doc = "`read()` method returns [rtc_cntl_store6::R](rtc_cntl_store6::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE6 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store6::W](rtc_cntl_store6::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE6 {}
#[doc = "RTC_CNTL_STORE6"]
pub mod rtc_cntl_store6;
#[doc = "RTC_CNTL_STORE7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_store7](rtc_cntl_store7) module"]
pub type RTC_CNTL_STORE7 = crate::Reg<u32, _RTC_CNTL_STORE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_STORE7;
#[doc = "`read()` method returns [rtc_cntl_store7::R](rtc_cntl_store7::R) reader structure"]
impl crate::Readable for RTC_CNTL_STORE7 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_store7::W](rtc_cntl_store7::W) writer structure"]
impl crate::Writable for RTC_CNTL_STORE7 {}
#[doc = "RTC_CNTL_STORE7"]
pub mod rtc_cntl_store7;
#[doc = "RTC_CNTL_LOW_POWER_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_low_power_st](rtc_cntl_low_power_st) module"]
pub type RTC_CNTL_LOW_POWER_ST = crate::Reg<u32, _RTC_CNTL_LOW_POWER_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_LOW_POWER_ST;
#[doc = "`read()` method returns [rtc_cntl_low_power_st::R](rtc_cntl_low_power_st::R) reader structure"]
impl crate::Readable for RTC_CNTL_LOW_POWER_ST {}
#[doc = "RTC_CNTL_LOW_POWER_ST"]
pub mod rtc_cntl_low_power_st;
#[doc = "RTC_CNTL_DIAG0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_diag0](rtc_cntl_diag0) module"]
pub type RTC_CNTL_DIAG0 = crate::Reg<u32, _RTC_CNTL_DIAG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIAG0;
#[doc = "`read()` method returns [rtc_cntl_diag0::R](rtc_cntl_diag0::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIAG0 {}
#[doc = "RTC_CNTL_DIAG0"]
pub mod rtc_cntl_diag0;
#[doc = "RTC_CNTL_PAD_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_pad_hold](rtc_cntl_pad_hold) module"]
pub type RTC_CNTL_PAD_HOLD = crate::Reg<u32, _RTC_CNTL_PAD_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_PAD_HOLD;
#[doc = "`read()` method returns [rtc_cntl_pad_hold::R](rtc_cntl_pad_hold::R) reader structure"]
impl crate::Readable for RTC_CNTL_PAD_HOLD {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_pad_hold::W](rtc_cntl_pad_hold::W) writer structure"]
impl crate::Writable for RTC_CNTL_PAD_HOLD {}
#[doc = "RTC_CNTL_PAD_HOLD"]
pub mod rtc_cntl_pad_hold;
#[doc = "RTC_CNTL_DIG_PAD_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dig_pad_hold](rtc_cntl_dig_pad_hold) module"]
pub type RTC_CNTL_DIG_PAD_HOLD = crate::Reg<u32, _RTC_CNTL_DIG_PAD_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DIG_PAD_HOLD;
#[doc = "`read()` method returns [rtc_cntl_dig_pad_hold::R](rtc_cntl_dig_pad_hold::R) reader structure"]
impl crate::Readable for RTC_CNTL_DIG_PAD_HOLD {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dig_pad_hold::W](rtc_cntl_dig_pad_hold::W) writer structure"]
impl crate::Writable for RTC_CNTL_DIG_PAD_HOLD {}
#[doc = "RTC_CNTL_DIG_PAD_HOLD"]
pub mod rtc_cntl_dig_pad_hold;
#[doc = "RTC_CNTL_BROWN_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_brown_out](rtc_cntl_brown_out) module"]
pub type RTC_CNTL_BROWN_OUT = crate::Reg<u32, _RTC_CNTL_BROWN_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_BROWN_OUT;
#[doc = "`read()` method returns [rtc_cntl_brown_out::R](rtc_cntl_brown_out::R) reader structure"]
impl crate::Readable for RTC_CNTL_BROWN_OUT {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_brown_out::W](rtc_cntl_brown_out::W) writer structure"]
impl crate::Writable for RTC_CNTL_BROWN_OUT {}
#[doc = "RTC_CNTL_BROWN_OUT"]
pub mod rtc_cntl_brown_out;
#[doc = "RTC_CNTL_TIME_LOW1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_time_low1](rtc_cntl_time_low1) module"]
pub type RTC_CNTL_TIME_LOW1 = crate::Reg<u32, _RTC_CNTL_TIME_LOW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME_LOW1;
#[doc = "`read()` method returns [rtc_cntl_time_low1::R](rtc_cntl_time_low1::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME_LOW1 {}
#[doc = "RTC_CNTL_TIME_LOW1"]
pub mod rtc_cntl_time_low1;
#[doc = "RTC_CNTL_TIME_HIGH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_time_high1](rtc_cntl_time_high1) module"]
pub type RTC_CNTL_TIME_HIGH1 = crate::Reg<u32, _RTC_CNTL_TIME_HIGH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_TIME_HIGH1;
#[doc = "`read()` method returns [rtc_cntl_time_high1::R](rtc_cntl_time_high1::R) reader structure"]
impl crate::Readable for RTC_CNTL_TIME_HIGH1 {}
#[doc = "RTC_CNTL_TIME_HIGH1"]
pub mod rtc_cntl_time_high1;
#[doc = "RTC_CNTL_XTAL32K_CLK_FACTOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_xtal32k_clk_factor](rtc_cntl_xtal32k_clk_factor) module"]
pub type RTC_CNTL_XTAL32K_CLK_FACTOR = crate::Reg<u32, _RTC_CNTL_XTAL32K_CLK_FACTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_XTAL32K_CLK_FACTOR;
#[doc = "`read()` method returns [rtc_cntl_xtal32k_clk_factor::R](rtc_cntl_xtal32k_clk_factor::R) reader structure"]
impl crate::Readable for RTC_CNTL_XTAL32K_CLK_FACTOR {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_xtal32k_clk_factor::W](rtc_cntl_xtal32k_clk_factor::W) writer structure"]
impl crate::Writable for RTC_CNTL_XTAL32K_CLK_FACTOR {}
#[doc = "RTC_CNTL_XTAL32K_CLK_FACTOR"]
pub mod rtc_cntl_xtal32k_clk_factor;
#[doc = "RTC_CNTL_XTAL32K_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_xtal32k_conf](rtc_cntl_xtal32k_conf) module"]
pub type RTC_CNTL_XTAL32K_CONF = crate::Reg<u32, _RTC_CNTL_XTAL32K_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_XTAL32K_CONF;
#[doc = "`read()` method returns [rtc_cntl_xtal32k_conf::R](rtc_cntl_xtal32k_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_XTAL32K_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_xtal32k_conf::W](rtc_cntl_xtal32k_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_XTAL32K_CONF {}
#[doc = "RTC_CNTL_XTAL32K_CONF"]
pub mod rtc_cntl_xtal32k_conf;
#[doc = "RTC_CNTL_USB_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_usb_conf](rtc_cntl_usb_conf) module"]
pub type RTC_CNTL_USB_CONF = crate::Reg<u32, _RTC_CNTL_USB_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_USB_CONF;
#[doc = "`read()` method returns [rtc_cntl_usb_conf::R](rtc_cntl_usb_conf::R) reader structure"]
impl crate::Readable for RTC_CNTL_USB_CONF {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_usb_conf::W](rtc_cntl_usb_conf::W) writer structure"]
impl crate::Writable for RTC_CNTL_USB_CONF {}
#[doc = "RTC_CNTL_USB_CONF"]
pub mod rtc_cntl_usb_conf;
#[doc = "RTC_CNTL_SLP_REJECT_CAUSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_slp_reject_cause](rtc_cntl_slp_reject_cause) module"]
pub type RTC_CNTL_SLP_REJECT_CAUSE = crate::Reg<u32, _RTC_CNTL_SLP_REJECT_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_REJECT_CAUSE;
#[doc = "`read()` method returns [rtc_cntl_slp_reject_cause::R](rtc_cntl_slp_reject_cause::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_REJECT_CAUSE {}
#[doc = "RTC_CNTL_SLP_REJECT_CAUSE"]
pub mod rtc_cntl_slp_reject_cause;
#[doc = "RTC_CNTL_OPTION1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_option1](rtc_cntl_option1) module"]
pub type RTC_CNTL_OPTION1 = crate::Reg<u32, _RTC_CNTL_OPTION1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_OPTION1;
#[doc = "`read()` method returns [rtc_cntl_option1::R](rtc_cntl_option1::R) reader structure"]
impl crate::Readable for RTC_CNTL_OPTION1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_option1::W](rtc_cntl_option1::W) writer structure"]
impl crate::Writable for RTC_CNTL_OPTION1 {}
#[doc = "RTC_CNTL_OPTION1"]
pub mod rtc_cntl_option1;
#[doc = "RTC_CNTL_SLP_WAKEUP_CAUSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_slp_wakeup_cause](rtc_cntl_slp_wakeup_cause) module"]
pub type RTC_CNTL_SLP_WAKEUP_CAUSE = crate::Reg<u32, _RTC_CNTL_SLP_WAKEUP_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SLP_WAKEUP_CAUSE;
#[doc = "`read()` method returns [rtc_cntl_slp_wakeup_cause::R](rtc_cntl_slp_wakeup_cause::R) reader structure"]
impl crate::Readable for RTC_CNTL_SLP_WAKEUP_CAUSE {}
#[doc = "RTC_CNTL_SLP_WAKEUP_CAUSE"]
pub mod rtc_cntl_slp_wakeup_cause;
#[doc = "RTC_CNTL_ULP_CP_TIMER_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_ulp_cp_timer_1](rtc_cntl_ulp_cp_timer_1) module"]
pub type RTC_CNTL_ULP_CP_TIMER_1 = crate::Reg<u32, _RTC_CNTL_ULP_CP_TIMER_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_ULP_CP_TIMER_1;
#[doc = "`read()` method returns [rtc_cntl_ulp_cp_timer_1::R](rtc_cntl_ulp_cp_timer_1::R) reader structure"]
impl crate::Readable for RTC_CNTL_ULP_CP_TIMER_1 {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_ulp_cp_timer_1::W](rtc_cntl_ulp_cp_timer_1::W) writer structure"]
impl crate::Writable for RTC_CNTL_ULP_CP_TIMER_1 {}
#[doc = "RTC_CNTL_ULP_CP_TIMER_1"]
pub mod rtc_cntl_ulp_cp_timer_1;
#[doc = "RTC_CNTL_INT_ENA_W1TS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_int_ena_w1ts](rtc_cntl_int_ena_w1ts) module"]
pub type RTC_CNTL_INT_ENA_W1TS = crate::Reg<u32, _RTC_CNTL_INT_ENA_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_ENA_W1TS;
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_ena_w1ts::W](rtc_cntl_int_ena_w1ts::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_ENA_W1TS {}
#[doc = "RTC_CNTL_INT_ENA_W1TS"]
pub mod rtc_cntl_int_ena_w1ts;
#[doc = "RTC_CNTL_INT_ENA_W1TC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_int_ena_w1tc](rtc_cntl_int_ena_w1tc) module"]
pub type RTC_CNTL_INT_ENA_W1TC = crate::Reg<u32, _RTC_CNTL_INT_ENA_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_INT_ENA_W1TC;
#[doc = "`write(|w| ..)` method takes [rtc_cntl_int_ena_w1tc::W](rtc_cntl_int_ena_w1tc::W) writer structure"]
impl crate::Writable for RTC_CNTL_INT_ENA_W1TC {}
#[doc = "RTC_CNTL_INT_ENA_W1TC"]
pub mod rtc_cntl_int_ena_w1tc;
#[doc = "RTC_CNTL_RETENTION_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_retention_ctrl](rtc_cntl_retention_ctrl) module"]
pub type RTC_CNTL_RETENTION_CTRL = crate::Reg<u32, _RTC_CNTL_RETENTION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_RETENTION_CTRL;
#[doc = "`read()` method returns [rtc_cntl_retention_ctrl::R](rtc_cntl_retention_ctrl::R) reader structure"]
impl crate::Readable for RTC_CNTL_RETENTION_CTRL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_retention_ctrl::W](rtc_cntl_retention_ctrl::W) writer structure"]
impl crate::Writable for RTC_CNTL_RETENTION_CTRL {}
#[doc = "RTC_CNTL_RETENTION_CTRL"]
pub mod rtc_cntl_retention_ctrl;
#[doc = "RTC_CNTL_FIB_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_fib_sel](rtc_cntl_fib_sel) module"]
pub type RTC_CNTL_FIB_SEL = crate::Reg<u32, _RTC_CNTL_FIB_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_FIB_SEL;
#[doc = "`read()` method returns [rtc_cntl_fib_sel::R](rtc_cntl_fib_sel::R) reader structure"]
impl crate::Readable for RTC_CNTL_FIB_SEL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_fib_sel::W](rtc_cntl_fib_sel::W) writer structure"]
impl crate::Writable for RTC_CNTL_FIB_SEL {}
#[doc = "RTC_CNTL_FIB_SEL"]
pub mod rtc_cntl_fib_sel;
#[doc = "RTC_CNTL_GPIO_WAKEUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_gpio_wakeup](rtc_cntl_gpio_wakeup) module"]
pub type RTC_CNTL_GPIO_WAKEUP = crate::Reg<u32, _RTC_CNTL_GPIO_WAKEUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_GPIO_WAKEUP;
#[doc = "`read()` method returns [rtc_cntl_gpio_wakeup::R](rtc_cntl_gpio_wakeup::R) reader structure"]
impl crate::Readable for RTC_CNTL_GPIO_WAKEUP {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_gpio_wakeup::W](rtc_cntl_gpio_wakeup::W) writer structure"]
impl crate::Writable for RTC_CNTL_GPIO_WAKEUP {}
#[doc = "RTC_CNTL_GPIO_WAKEUP"]
pub mod rtc_cntl_gpio_wakeup;
#[doc = "RTC_CNTL_DBG_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dbg_sel](rtc_cntl_dbg_sel) module"]
pub type RTC_CNTL_DBG_SEL = crate::Reg<u32, _RTC_CNTL_DBG_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DBG_SEL;
#[doc = "`read()` method returns [rtc_cntl_dbg_sel::R](rtc_cntl_dbg_sel::R) reader structure"]
impl crate::Readable for RTC_CNTL_DBG_SEL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dbg_sel::W](rtc_cntl_dbg_sel::W) writer structure"]
impl crate::Writable for RTC_CNTL_DBG_SEL {}
#[doc = "RTC_CNTL_DBG_SEL"]
pub mod rtc_cntl_dbg_sel;
#[doc = "RTC_CNTL_DBG_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dbg_map](rtc_cntl_dbg_map) module"]
pub type RTC_CNTL_DBG_MAP = crate::Reg<u32, _RTC_CNTL_DBG_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DBG_MAP;
#[doc = "`read()` method returns [rtc_cntl_dbg_map::R](rtc_cntl_dbg_map::R) reader structure"]
impl crate::Readable for RTC_CNTL_DBG_MAP {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dbg_map::W](rtc_cntl_dbg_map::W) writer structure"]
impl crate::Writable for RTC_CNTL_DBG_MAP {}
#[doc = "RTC_CNTL_DBG_MAP"]
pub mod rtc_cntl_dbg_map;
#[doc = "RTC_CNTL_SENSOR_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_sensor_ctrl](rtc_cntl_sensor_ctrl) module"]
pub type RTC_CNTL_SENSOR_CTRL = crate::Reg<u32, _RTC_CNTL_SENSOR_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_SENSOR_CTRL;
#[doc = "`read()` method returns [rtc_cntl_sensor_ctrl::R](rtc_cntl_sensor_ctrl::R) reader structure"]
impl crate::Readable for RTC_CNTL_SENSOR_CTRL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_sensor_ctrl::W](rtc_cntl_sensor_ctrl::W) writer structure"]
impl crate::Writable for RTC_CNTL_SENSOR_CTRL {}
#[doc = "RTC_CNTL_SENSOR_CTRL"]
pub mod rtc_cntl_sensor_ctrl;
#[doc = "RTC_CNTL_DBG_SAR_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dbg_sar_sel](rtc_cntl_dbg_sar_sel) module"]
pub type RTC_CNTL_DBG_SAR_SEL = crate::Reg<u32, _RTC_CNTL_DBG_SAR_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DBG_SAR_SEL;
#[doc = "`read()` method returns [rtc_cntl_dbg_sar_sel::R](rtc_cntl_dbg_sar_sel::R) reader structure"]
impl crate::Readable for RTC_CNTL_DBG_SAR_SEL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dbg_sar_sel::W](rtc_cntl_dbg_sar_sel::W) writer structure"]
impl crate::Writable for RTC_CNTL_DBG_SAR_SEL {}
#[doc = "RTC_CNTL_DBG_SAR_SEL"]
pub mod rtc_cntl_dbg_sar_sel;
#[doc = "RTC_CNTL_PG_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_pg_ctrl](rtc_cntl_pg_ctrl) module"]
pub type RTC_CNTL_PG_CTRL = crate::Reg<u32, _RTC_CNTL_PG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_PG_CTRL;
#[doc = "`read()` method returns [rtc_cntl_pg_ctrl::R](rtc_cntl_pg_ctrl::R) reader structure"]
impl crate::Readable for RTC_CNTL_PG_CTRL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_pg_ctrl::W](rtc_cntl_pg_ctrl::W) writer structure"]
impl crate::Writable for RTC_CNTL_PG_CTRL {}
#[doc = "RTC_CNTL_PG_CTRL"]
pub mod rtc_cntl_pg_ctrl;
#[doc = "RTC_CNTL_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_date](rtc_cntl_date) module"]
pub type RTC_CNTL_DATE = crate::Reg<u32, _RTC_CNTL_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL_DATE;
#[doc = "`read()` method returns [rtc_cntl_date::R](rtc_cntl_date::R) reader structure"]
impl crate::Readable for RTC_CNTL_DATE {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_date::W](rtc_cntl_date::W) writer structure"]
impl crate::Writable for RTC_CNTL_DATE {}
#[doc = "RTC_CNTL_DATE"]
pub mod rtc_cntl_date;
