#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - APB_SARADC_CTRL"]
    pub apb_saradc_ctrl: APB_SARADC_CTRL,
    #[doc = "0x04 - APB_SARADC_CTRL2"]
    pub apb_saradc_ctrl2: APB_SARADC_CTRL2,
    #[doc = "0x08 - APB_SARADC_FILTER_CTRL1"]
    pub apb_saradc_filter_ctrl1: APB_SARADC_FILTER_CTRL1,
    #[doc = "0x0c - APB_SARADC_FSM_WAIT"]
    pub apb_saradc_fsm_wait: APB_SARADC_FSM_WAIT,
    #[doc = "0x10 - APB_SARADC_SAR1_STATUS"]
    pub apb_saradc_sar1_status: APB_SARADC_SAR1_STATUS,
    #[doc = "0x14 - APB_SARADC_SAR2_STATUS"]
    pub apb_saradc_sar2_status: APB_SARADC_SAR2_STATUS,
    #[doc = "0x18 - APB_SARADC_SAR_PATT_TAB1"]
    pub apb_saradc_sar_patt_tab1: APB_SARADC_SAR_PATT_TAB1,
    #[doc = "0x1c - APB_SARADC_SAR_PATT_TAB2"]
    pub apb_saradc_sar_patt_tab2: APB_SARADC_SAR_PATT_TAB2,
    #[doc = "0x20 - APB_SARADC_ONETIME_SAMPLE"]
    pub apb_saradc_onetime_sample: APB_SARADC_ONETIME_SAMPLE,
    #[doc = "0x24 - APB_SARADC_APB_ADC_ARB_CTRL"]
    pub apb_saradc_apb_adc_arb_ctrl: APB_SARADC_APB_ADC_ARB_CTRL,
    #[doc = "0x28 - APB_SARADC_FILTER_CTRL0"]
    pub apb_saradc_filter_ctrl0: APB_SARADC_FILTER_CTRL0,
    #[doc = "0x2c - APB_SARADC_1_DATA_STATUS"]
    pub apb_saradc_1_data_status: APB_SARADC_1_DATA_STATUS,
    #[doc = "0x30 - APB_SARADC_2_DATA_STATUS"]
    pub apb_saradc_2_data_status: APB_SARADC_2_DATA_STATUS,
    #[doc = "0x34 - APB_SARADC_THRES0_CTRL"]
    pub apb_saradc_thres0_ctrl: APB_SARADC_THRES0_CTRL,
    #[doc = "0x38 - APB_SARADC_THRES1_CTRL"]
    pub apb_saradc_thres1_ctrl: APB_SARADC_THRES1_CTRL,
    #[doc = "0x3c - APB_SARADC_THRES_CTRL"]
    pub apb_saradc_thres_ctrl: APB_SARADC_THRES_CTRL,
    #[doc = "0x40 - APB_SARADC_INT_ENA"]
    pub apb_saradc_int_ena: APB_SARADC_INT_ENA,
    #[doc = "0x44 - APB_SARADC_INT_RAW"]
    pub apb_saradc_int_raw: APB_SARADC_INT_RAW,
    #[doc = "0x48 - APB_SARADC_INT_ST"]
    pub apb_saradc_int_st: APB_SARADC_INT_ST,
    #[doc = "0x4c - APB_SARADC_INT_CLR"]
    pub apb_saradc_int_clr: APB_SARADC_INT_CLR,
    #[doc = "0x50 - APB_SARADC_DMA_CONF"]
    pub apb_saradc_dma_conf: APB_SARADC_DMA_CONF,
    #[doc = "0x54 - APB_SARADC_APB_ADC_CLKM_CONF"]
    pub apb_saradc_apb_adc_clkm_conf: APB_SARADC_APB_ADC_CLKM_CONF,
    #[doc = "0x58 - APB_SARADC_APB_TSENS_CTRL"]
    pub apb_saradc_apb_tsens_ctrl: APB_SARADC_APB_TSENS_CTRL,
    #[doc = "0x5c - APB_SARADC_APB_TSENS_CTRL2"]
    pub apb_saradc_apb_tsens_ctrl2: APB_SARADC_APB_TSENS_CTRL2,
    #[doc = "0x60 - APB_SARADC_CALI"]
    pub apb_saradc_cali: APB_SARADC_CALI,
    _reserved25: [u8; 920usize],
    #[doc = "0x3fc - APB_SARADC_APB_CTRL_DATE"]
    pub apb_saradc_apb_ctrl_date: APB_SARADC_APB_CTRL_DATE,
}
#[doc = "APB_SARADC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_ctrl](apb_saradc_ctrl) module"]
pub type APB_SARADC_CTRL = crate::Reg<u32, _APB_SARADC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_CTRL;
#[doc = "`read()` method returns [apb_saradc_ctrl::R](apb_saradc_ctrl::R) reader structure"]
impl crate::Readable for APB_SARADC_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_ctrl::W](apb_saradc_ctrl::W) writer structure"]
impl crate::Writable for APB_SARADC_CTRL {}
#[doc = "APB_SARADC_CTRL"]
pub mod apb_saradc_ctrl;
#[doc = "APB_SARADC_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_ctrl2](apb_saradc_ctrl2) module"]
pub type APB_SARADC_CTRL2 = crate::Reg<u32, _APB_SARADC_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_CTRL2;
#[doc = "`read()` method returns [apb_saradc_ctrl2::R](apb_saradc_ctrl2::R) reader structure"]
impl crate::Readable for APB_SARADC_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_ctrl2::W](apb_saradc_ctrl2::W) writer structure"]
impl crate::Writable for APB_SARADC_CTRL2 {}
#[doc = "APB_SARADC_CTRL2"]
pub mod apb_saradc_ctrl2;
#[doc = "APB_SARADC_FILTER_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_filter_ctrl1](apb_saradc_filter_ctrl1) module"]
pub type APB_SARADC_FILTER_CTRL1 = crate::Reg<u32, _APB_SARADC_FILTER_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_FILTER_CTRL1;
#[doc = "`read()` method returns [apb_saradc_filter_ctrl1::R](apb_saradc_filter_ctrl1::R) reader structure"]
impl crate::Readable for APB_SARADC_FILTER_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_filter_ctrl1::W](apb_saradc_filter_ctrl1::W) writer structure"]
impl crate::Writable for APB_SARADC_FILTER_CTRL1 {}
#[doc = "APB_SARADC_FILTER_CTRL1"]
pub mod apb_saradc_filter_ctrl1;
#[doc = "APB_SARADC_FSM_WAIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_fsm_wait](apb_saradc_fsm_wait) module"]
pub type APB_SARADC_FSM_WAIT = crate::Reg<u32, _APB_SARADC_FSM_WAIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_FSM_WAIT;
#[doc = "`read()` method returns [apb_saradc_fsm_wait::R](apb_saradc_fsm_wait::R) reader structure"]
impl crate::Readable for APB_SARADC_FSM_WAIT {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_fsm_wait::W](apb_saradc_fsm_wait::W) writer structure"]
impl crate::Writable for APB_SARADC_FSM_WAIT {}
#[doc = "APB_SARADC_FSM_WAIT"]
pub mod apb_saradc_fsm_wait;
#[doc = "APB_SARADC_SAR1_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_sar1_status](apb_saradc_sar1_status) module"]
pub type APB_SARADC_SAR1_STATUS = crate::Reg<u32, _APB_SARADC_SAR1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_SAR1_STATUS;
#[doc = "`read()` method returns [apb_saradc_sar1_status::R](apb_saradc_sar1_status::R) reader structure"]
impl crate::Readable for APB_SARADC_SAR1_STATUS {}
#[doc = "APB_SARADC_SAR1_STATUS"]
pub mod apb_saradc_sar1_status;
#[doc = "APB_SARADC_SAR2_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_sar2_status](apb_saradc_sar2_status) module"]
pub type APB_SARADC_SAR2_STATUS = crate::Reg<u32, _APB_SARADC_SAR2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_SAR2_STATUS;
#[doc = "`read()` method returns [apb_saradc_sar2_status::R](apb_saradc_sar2_status::R) reader structure"]
impl crate::Readable for APB_SARADC_SAR2_STATUS {}
#[doc = "APB_SARADC_SAR2_STATUS"]
pub mod apb_saradc_sar2_status;
#[doc = "APB_SARADC_SAR_PATT_TAB1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_sar_patt_tab1](apb_saradc_sar_patt_tab1) module"]
pub type APB_SARADC_SAR_PATT_TAB1 = crate::Reg<u32, _APB_SARADC_SAR_PATT_TAB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_SAR_PATT_TAB1;
#[doc = "`read()` method returns [apb_saradc_sar_patt_tab1::R](apb_saradc_sar_patt_tab1::R) reader structure"]
impl crate::Readable for APB_SARADC_SAR_PATT_TAB1 {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_sar_patt_tab1::W](apb_saradc_sar_patt_tab1::W) writer structure"]
impl crate::Writable for APB_SARADC_SAR_PATT_TAB1 {}
#[doc = "APB_SARADC_SAR_PATT_TAB1"]
pub mod apb_saradc_sar_patt_tab1;
#[doc = "APB_SARADC_SAR_PATT_TAB2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_sar_patt_tab2](apb_saradc_sar_patt_tab2) module"]
pub type APB_SARADC_SAR_PATT_TAB2 = crate::Reg<u32, _APB_SARADC_SAR_PATT_TAB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_SAR_PATT_TAB2;
#[doc = "`read()` method returns [apb_saradc_sar_patt_tab2::R](apb_saradc_sar_patt_tab2::R) reader structure"]
impl crate::Readable for APB_SARADC_SAR_PATT_TAB2 {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_sar_patt_tab2::W](apb_saradc_sar_patt_tab2::W) writer structure"]
impl crate::Writable for APB_SARADC_SAR_PATT_TAB2 {}
#[doc = "APB_SARADC_SAR_PATT_TAB2"]
pub mod apb_saradc_sar_patt_tab2;
#[doc = "APB_SARADC_ONETIME_SAMPLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_onetime_sample](apb_saradc_onetime_sample) module"]
pub type APB_SARADC_ONETIME_SAMPLE = crate::Reg<u32, _APB_SARADC_ONETIME_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_ONETIME_SAMPLE;
#[doc = "`read()` method returns [apb_saradc_onetime_sample::R](apb_saradc_onetime_sample::R) reader structure"]
impl crate::Readable for APB_SARADC_ONETIME_SAMPLE {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_onetime_sample::W](apb_saradc_onetime_sample::W) writer structure"]
impl crate::Writable for APB_SARADC_ONETIME_SAMPLE {}
#[doc = "APB_SARADC_ONETIME_SAMPLE"]
pub mod apb_saradc_onetime_sample;
#[doc = "APB_SARADC_APB_ADC_ARB_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_apb_adc_arb_ctrl](apb_saradc_apb_adc_arb_ctrl) module"]
pub type APB_SARADC_APB_ADC_ARB_CTRL = crate::Reg<u32, _APB_SARADC_APB_ADC_ARB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_APB_ADC_ARB_CTRL;
#[doc = "`read()` method returns [apb_saradc_apb_adc_arb_ctrl::R](apb_saradc_apb_adc_arb_ctrl::R) reader structure"]
impl crate::Readable for APB_SARADC_APB_ADC_ARB_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_apb_adc_arb_ctrl::W](apb_saradc_apb_adc_arb_ctrl::W) writer structure"]
impl crate::Writable for APB_SARADC_APB_ADC_ARB_CTRL {}
#[doc = "APB_SARADC_APB_ADC_ARB_CTRL"]
pub mod apb_saradc_apb_adc_arb_ctrl;
#[doc = "APB_SARADC_FILTER_CTRL0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_filter_ctrl0](apb_saradc_filter_ctrl0) module"]
pub type APB_SARADC_FILTER_CTRL0 = crate::Reg<u32, _APB_SARADC_FILTER_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_FILTER_CTRL0;
#[doc = "`read()` method returns [apb_saradc_filter_ctrl0::R](apb_saradc_filter_ctrl0::R) reader structure"]
impl crate::Readable for APB_SARADC_FILTER_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_filter_ctrl0::W](apb_saradc_filter_ctrl0::W) writer structure"]
impl crate::Writable for APB_SARADC_FILTER_CTRL0 {}
#[doc = "APB_SARADC_FILTER_CTRL0"]
pub mod apb_saradc_filter_ctrl0;
#[doc = "APB_SARADC_1_DATA_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_1_data_status](apb_saradc_1_data_status) module"]
pub type APB_SARADC_1_DATA_STATUS = crate::Reg<u32, _APB_SARADC_1_DATA_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_1_DATA_STATUS;
#[doc = "`read()` method returns [apb_saradc_1_data_status::R](apb_saradc_1_data_status::R) reader structure"]
impl crate::Readable for APB_SARADC_1_DATA_STATUS {}
#[doc = "APB_SARADC_1_DATA_STATUS"]
pub mod apb_saradc_1_data_status;
#[doc = "APB_SARADC_2_DATA_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_2_data_status](apb_saradc_2_data_status) module"]
pub type APB_SARADC_2_DATA_STATUS = crate::Reg<u32, _APB_SARADC_2_DATA_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_2_DATA_STATUS;
#[doc = "`read()` method returns [apb_saradc_2_data_status::R](apb_saradc_2_data_status::R) reader structure"]
impl crate::Readable for APB_SARADC_2_DATA_STATUS {}
#[doc = "APB_SARADC_2_DATA_STATUS"]
pub mod apb_saradc_2_data_status;
#[doc = "APB_SARADC_THRES0_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_thres0_ctrl](apb_saradc_thres0_ctrl) module"]
pub type APB_SARADC_THRES0_CTRL = crate::Reg<u32, _APB_SARADC_THRES0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_THRES0_CTRL;
#[doc = "`read()` method returns [apb_saradc_thres0_ctrl::R](apb_saradc_thres0_ctrl::R) reader structure"]
impl crate::Readable for APB_SARADC_THRES0_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_thres0_ctrl::W](apb_saradc_thres0_ctrl::W) writer structure"]
impl crate::Writable for APB_SARADC_THRES0_CTRL {}
#[doc = "APB_SARADC_THRES0_CTRL"]
pub mod apb_saradc_thres0_ctrl;
#[doc = "APB_SARADC_THRES1_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_thres1_ctrl](apb_saradc_thres1_ctrl) module"]
pub type APB_SARADC_THRES1_CTRL = crate::Reg<u32, _APB_SARADC_THRES1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_THRES1_CTRL;
#[doc = "`read()` method returns [apb_saradc_thres1_ctrl::R](apb_saradc_thres1_ctrl::R) reader structure"]
impl crate::Readable for APB_SARADC_THRES1_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_thres1_ctrl::W](apb_saradc_thres1_ctrl::W) writer structure"]
impl crate::Writable for APB_SARADC_THRES1_CTRL {}
#[doc = "APB_SARADC_THRES1_CTRL"]
pub mod apb_saradc_thres1_ctrl;
#[doc = "APB_SARADC_THRES_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_thres_ctrl](apb_saradc_thres_ctrl) module"]
pub type APB_SARADC_THRES_CTRL = crate::Reg<u32, _APB_SARADC_THRES_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_THRES_CTRL;
#[doc = "`read()` method returns [apb_saradc_thres_ctrl::R](apb_saradc_thres_ctrl::R) reader structure"]
impl crate::Readable for APB_SARADC_THRES_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_thres_ctrl::W](apb_saradc_thres_ctrl::W) writer structure"]
impl crate::Writable for APB_SARADC_THRES_CTRL {}
#[doc = "APB_SARADC_THRES_CTRL"]
pub mod apb_saradc_thres_ctrl;
#[doc = "APB_SARADC_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_int_ena](apb_saradc_int_ena) module"]
pub type APB_SARADC_INT_ENA = crate::Reg<u32, _APB_SARADC_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_INT_ENA;
#[doc = "`read()` method returns [apb_saradc_int_ena::R](apb_saradc_int_ena::R) reader structure"]
impl crate::Readable for APB_SARADC_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_int_ena::W](apb_saradc_int_ena::W) writer structure"]
impl crate::Writable for APB_SARADC_INT_ENA {}
#[doc = "APB_SARADC_INT_ENA"]
pub mod apb_saradc_int_ena;
#[doc = "APB_SARADC_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_int_raw](apb_saradc_int_raw) module"]
pub type APB_SARADC_INT_RAW = crate::Reg<u32, _APB_SARADC_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_INT_RAW;
#[doc = "`read()` method returns [apb_saradc_int_raw::R](apb_saradc_int_raw::R) reader structure"]
impl crate::Readable for APB_SARADC_INT_RAW {}
#[doc = "APB_SARADC_INT_RAW"]
pub mod apb_saradc_int_raw;
#[doc = "APB_SARADC_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_int_st](apb_saradc_int_st) module"]
pub type APB_SARADC_INT_ST = crate::Reg<u32, _APB_SARADC_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_INT_ST;
#[doc = "`read()` method returns [apb_saradc_int_st::R](apb_saradc_int_st::R) reader structure"]
impl crate::Readable for APB_SARADC_INT_ST {}
#[doc = "APB_SARADC_INT_ST"]
pub mod apb_saradc_int_st;
#[doc = "APB_SARADC_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_int_clr](apb_saradc_int_clr) module"]
pub type APB_SARADC_INT_CLR = crate::Reg<u32, _APB_SARADC_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_INT_CLR;
#[doc = "`write(|w| ..)` method takes [apb_saradc_int_clr::W](apb_saradc_int_clr::W) writer structure"]
impl crate::Writable for APB_SARADC_INT_CLR {}
#[doc = "APB_SARADC_INT_CLR"]
pub mod apb_saradc_int_clr;
#[doc = "APB_SARADC_DMA_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_dma_conf](apb_saradc_dma_conf) module"]
pub type APB_SARADC_DMA_CONF = crate::Reg<u32, _APB_SARADC_DMA_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_DMA_CONF;
#[doc = "`read()` method returns [apb_saradc_dma_conf::R](apb_saradc_dma_conf::R) reader structure"]
impl crate::Readable for APB_SARADC_DMA_CONF {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_dma_conf::W](apb_saradc_dma_conf::W) writer structure"]
impl crate::Writable for APB_SARADC_DMA_CONF {}
#[doc = "APB_SARADC_DMA_CONF"]
pub mod apb_saradc_dma_conf;
#[doc = "APB_SARADC_APB_ADC_CLKM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_apb_adc_clkm_conf](apb_saradc_apb_adc_clkm_conf) module"]
pub type APB_SARADC_APB_ADC_CLKM_CONF = crate::Reg<u32, _APB_SARADC_APB_ADC_CLKM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_APB_ADC_CLKM_CONF;
#[doc = "`read()` method returns [apb_saradc_apb_adc_clkm_conf::R](apb_saradc_apb_adc_clkm_conf::R) reader structure"]
impl crate::Readable for APB_SARADC_APB_ADC_CLKM_CONF {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_apb_adc_clkm_conf::W](apb_saradc_apb_adc_clkm_conf::W) writer structure"]
impl crate::Writable for APB_SARADC_APB_ADC_CLKM_CONF {}
#[doc = "APB_SARADC_APB_ADC_CLKM_CONF"]
pub mod apb_saradc_apb_adc_clkm_conf;
#[doc = "APB_SARADC_APB_TSENS_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_apb_tsens_ctrl](apb_saradc_apb_tsens_ctrl) module"]
pub type APB_SARADC_APB_TSENS_CTRL = crate::Reg<u32, _APB_SARADC_APB_TSENS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_APB_TSENS_CTRL;
#[doc = "`read()` method returns [apb_saradc_apb_tsens_ctrl::R](apb_saradc_apb_tsens_ctrl::R) reader structure"]
impl crate::Readable for APB_SARADC_APB_TSENS_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_apb_tsens_ctrl::W](apb_saradc_apb_tsens_ctrl::W) writer structure"]
impl crate::Writable for APB_SARADC_APB_TSENS_CTRL {}
#[doc = "APB_SARADC_APB_TSENS_CTRL"]
pub mod apb_saradc_apb_tsens_ctrl;
#[doc = "APB_SARADC_APB_TSENS_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_apb_tsens_ctrl2](apb_saradc_apb_tsens_ctrl2) module"]
pub type APB_SARADC_APB_TSENS_CTRL2 = crate::Reg<u32, _APB_SARADC_APB_TSENS_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_APB_TSENS_CTRL2;
#[doc = "`read()` method returns [apb_saradc_apb_tsens_ctrl2::R](apb_saradc_apb_tsens_ctrl2::R) reader structure"]
impl crate::Readable for APB_SARADC_APB_TSENS_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_apb_tsens_ctrl2::W](apb_saradc_apb_tsens_ctrl2::W) writer structure"]
impl crate::Writable for APB_SARADC_APB_TSENS_CTRL2 {}
#[doc = "APB_SARADC_APB_TSENS_CTRL2"]
pub mod apb_saradc_apb_tsens_ctrl2;
#[doc = "APB_SARADC_CALI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_cali](apb_saradc_cali) module"]
pub type APB_SARADC_CALI = crate::Reg<u32, _APB_SARADC_CALI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_CALI;
#[doc = "`read()` method returns [apb_saradc_cali::R](apb_saradc_cali::R) reader structure"]
impl crate::Readable for APB_SARADC_CALI {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_cali::W](apb_saradc_cali::W) writer structure"]
impl crate::Writable for APB_SARADC_CALI {}
#[doc = "APB_SARADC_CALI"]
pub mod apb_saradc_cali;
#[doc = "APB_SARADC_APB_CTRL_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_apb_ctrl_date](apb_saradc_apb_ctrl_date) module"]
pub type APB_SARADC_APB_CTRL_DATE = crate::Reg<u32, _APB_SARADC_APB_CTRL_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_SARADC_APB_CTRL_DATE;
#[doc = "`read()` method returns [apb_saradc_apb_ctrl_date::R](apb_saradc_apb_ctrl_date::R) reader structure"]
impl crate::Readable for APB_SARADC_APB_CTRL_DATE {}
#[doc = "`write(|w| ..)` method takes [apb_saradc_apb_ctrl_date::W](apb_saradc_apb_ctrl_date::W) writer structure"]
impl crate::Writable for APB_SARADC_APB_CTRL_DATE {}
#[doc = "APB_SARADC_APB_CTRL_DATE"]
pub mod apb_saradc_apb_ctrl_date;
