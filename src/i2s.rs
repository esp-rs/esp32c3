#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - I2S_INT_RAW"]
    pub i2s_int_raw: I2S_INT_RAW,
    #[doc = "0x10 - I2S_INT_ST"]
    pub i2s_int_st: I2S_INT_ST,
    #[doc = "0x14 - I2S_INT_ENA"]
    pub i2s_int_ena: I2S_INT_ENA,
    #[doc = "0x18 - I2S_INT_CLR"]
    pub i2s_int_clr: I2S_INT_CLR,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - I2S_RX_CONF"]
    pub i2s_rx_conf: I2S_RX_CONF,
    #[doc = "0x24 - I2S_TX_CONF"]
    pub i2s_tx_conf: I2S_TX_CONF,
    #[doc = "0x28 - I2S_RX_CONF1"]
    pub i2s_rx_conf1: I2S_RX_CONF1,
    #[doc = "0x2c - I2S_TX_CONF1"]
    pub i2s_tx_conf1: I2S_TX_CONF1,
    #[doc = "0x30 - I2S_RX_CLKM_CONF"]
    pub i2s_rx_clkm_conf: I2S_RX_CLKM_CONF,
    #[doc = "0x34 - I2S_TX_CLKM_CONF"]
    pub i2s_tx_clkm_conf: I2S_TX_CLKM_CONF,
    #[doc = "0x38 - I2S_RX_CLKM_DIV_CONF"]
    pub i2s_rx_clkm_div_conf: I2S_RX_CLKM_DIV_CONF,
    #[doc = "0x3c - I2S_TX_CLKM_DIV_CONF"]
    pub i2s_tx_clkm_div_conf: I2S_TX_CLKM_DIV_CONF,
    #[doc = "0x40 - I2S_TX_PCM2PDM_CONF"]
    pub i2s_tx_pcm2pdm_conf: I2S_TX_PCM2PDM_CONF,
    #[doc = "0x44 - I2S_TX_PCM2PDM_CONF1"]
    pub i2s_tx_pcm2pdm_conf1: I2S_TX_PCM2PDM_CONF1,
    _reserved14: [u8; 8usize],
    #[doc = "0x50 - I2S_RX_TDM_CTRL"]
    pub i2s_rx_tdm_ctrl: I2S_RX_TDM_CTRL,
    #[doc = "0x54 - I2S_TX_TDM_CTRL"]
    pub i2s_tx_tdm_ctrl: I2S_TX_TDM_CTRL,
    #[doc = "0x58 - I2S_RX_TIMING"]
    pub i2s_rx_timing: I2S_RX_TIMING,
    #[doc = "0x5c - I2S_TX_TIMING"]
    pub i2s_tx_timing: I2S_TX_TIMING,
    #[doc = "0x60 - I2S_LC_HUNG_CONF"]
    pub i2s_lc_hung_conf: I2S_LC_HUNG_CONF,
    #[doc = "0x64 - I2S_RXEOF_NUM"]
    pub i2s_rxeof_num: I2S_RXEOF_NUM,
    #[doc = "0x68 - I2S_CONF_SIGLE_DATA"]
    pub i2s_conf_sigle_data: I2S_CONF_SIGLE_DATA,
    #[doc = "0x6c - I2S_STATE"]
    pub i2s_state: I2S_STATE,
    _reserved22: [u8; 16usize],
    #[doc = "0x80 - I2S_DATE"]
    pub i2s_date: I2S_DATE,
}
#[doc = "I2S_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_int_raw](i2s_int_raw) module"]
pub type I2S_INT_RAW = crate::Reg<u32, _I2S_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_RAW;
#[doc = "`read()` method returns [i2s_int_raw::R](i2s_int_raw::R) reader structure"]
impl crate::Readable for I2S_INT_RAW {}
#[doc = "I2S_INT_RAW"]
pub mod i2s_int_raw;
#[doc = "I2S_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_int_st](i2s_int_st) module"]
pub type I2S_INT_ST = crate::Reg<u32, _I2S_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_ST;
#[doc = "`read()` method returns [i2s_int_st::R](i2s_int_st::R) reader structure"]
impl crate::Readable for I2S_INT_ST {}
#[doc = "I2S_INT_ST"]
pub mod i2s_int_st;
#[doc = "I2S_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_int_ena](i2s_int_ena) module"]
pub type I2S_INT_ENA = crate::Reg<u32, _I2S_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_ENA;
#[doc = "`read()` method returns [i2s_int_ena::R](i2s_int_ena::R) reader structure"]
impl crate::Readable for I2S_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [i2s_int_ena::W](i2s_int_ena::W) writer structure"]
impl crate::Writable for I2S_INT_ENA {}
#[doc = "I2S_INT_ENA"]
pub mod i2s_int_ena;
#[doc = "I2S_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_int_clr](i2s_int_clr) module"]
pub type I2S_INT_CLR = crate::Reg<u32, _I2S_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_INT_CLR;
#[doc = "`write(|w| ..)` method takes [i2s_int_clr::W](i2s_int_clr::W) writer structure"]
impl crate::Writable for I2S_INT_CLR {}
#[doc = "I2S_INT_CLR"]
pub mod i2s_int_clr;
#[doc = "I2S_RX_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rx_conf](i2s_rx_conf) module"]
pub type I2S_RX_CONF = crate::Reg<u32, _I2S_RX_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RX_CONF;
#[doc = "`read()` method returns [i2s_rx_conf::R](i2s_rx_conf::R) reader structure"]
impl crate::Readable for I2S_RX_CONF {}
#[doc = "`write(|w| ..)` method takes [i2s_rx_conf::W](i2s_rx_conf::W) writer structure"]
impl crate::Writable for I2S_RX_CONF {}
#[doc = "I2S_RX_CONF"]
pub mod i2s_rx_conf;
#[doc = "I2S_TX_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_conf](i2s_tx_conf) module"]
pub type I2S_TX_CONF = crate::Reg<u32, _I2S_TX_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TX_CONF;
#[doc = "`read()` method returns [i2s_tx_conf::R](i2s_tx_conf::R) reader structure"]
impl crate::Readable for I2S_TX_CONF {}
#[doc = "`write(|w| ..)` method takes [i2s_tx_conf::W](i2s_tx_conf::W) writer structure"]
impl crate::Writable for I2S_TX_CONF {}
#[doc = "I2S_TX_CONF"]
pub mod i2s_tx_conf;
#[doc = "I2S_RX_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rx_conf1](i2s_rx_conf1) module"]
pub type I2S_RX_CONF1 = crate::Reg<u32, _I2S_RX_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RX_CONF1;
#[doc = "`read()` method returns [i2s_rx_conf1::R](i2s_rx_conf1::R) reader structure"]
impl crate::Readable for I2S_RX_CONF1 {}
#[doc = "`write(|w| ..)` method takes [i2s_rx_conf1::W](i2s_rx_conf1::W) writer structure"]
impl crate::Writable for I2S_RX_CONF1 {}
#[doc = "I2S_RX_CONF1"]
pub mod i2s_rx_conf1;
#[doc = "I2S_TX_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_conf1](i2s_tx_conf1) module"]
pub type I2S_TX_CONF1 = crate::Reg<u32, _I2S_TX_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TX_CONF1;
#[doc = "`read()` method returns [i2s_tx_conf1::R](i2s_tx_conf1::R) reader structure"]
impl crate::Readable for I2S_TX_CONF1 {}
#[doc = "`write(|w| ..)` method takes [i2s_tx_conf1::W](i2s_tx_conf1::W) writer structure"]
impl crate::Writable for I2S_TX_CONF1 {}
#[doc = "I2S_TX_CONF1"]
pub mod i2s_tx_conf1;
#[doc = "I2S_RX_CLKM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rx_clkm_conf](i2s_rx_clkm_conf) module"]
pub type I2S_RX_CLKM_CONF = crate::Reg<u32, _I2S_RX_CLKM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RX_CLKM_CONF;
#[doc = "`read()` method returns [i2s_rx_clkm_conf::R](i2s_rx_clkm_conf::R) reader structure"]
impl crate::Readable for I2S_RX_CLKM_CONF {}
#[doc = "`write(|w| ..)` method takes [i2s_rx_clkm_conf::W](i2s_rx_clkm_conf::W) writer structure"]
impl crate::Writable for I2S_RX_CLKM_CONF {}
#[doc = "I2S_RX_CLKM_CONF"]
pub mod i2s_rx_clkm_conf;
#[doc = "I2S_TX_CLKM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_clkm_conf](i2s_tx_clkm_conf) module"]
pub type I2S_TX_CLKM_CONF = crate::Reg<u32, _I2S_TX_CLKM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TX_CLKM_CONF;
#[doc = "`read()` method returns [i2s_tx_clkm_conf::R](i2s_tx_clkm_conf::R) reader structure"]
impl crate::Readable for I2S_TX_CLKM_CONF {}
#[doc = "`write(|w| ..)` method takes [i2s_tx_clkm_conf::W](i2s_tx_clkm_conf::W) writer structure"]
impl crate::Writable for I2S_TX_CLKM_CONF {}
#[doc = "I2S_TX_CLKM_CONF"]
pub mod i2s_tx_clkm_conf;
#[doc = "I2S_RX_CLKM_DIV_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rx_clkm_div_conf](i2s_rx_clkm_div_conf) module"]
pub type I2S_RX_CLKM_DIV_CONF = crate::Reg<u32, _I2S_RX_CLKM_DIV_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RX_CLKM_DIV_CONF;
#[doc = "`read()` method returns [i2s_rx_clkm_div_conf::R](i2s_rx_clkm_div_conf::R) reader structure"]
impl crate::Readable for I2S_RX_CLKM_DIV_CONF {}
#[doc = "`write(|w| ..)` method takes [i2s_rx_clkm_div_conf::W](i2s_rx_clkm_div_conf::W) writer structure"]
impl crate::Writable for I2S_RX_CLKM_DIV_CONF {}
#[doc = "I2S_RX_CLKM_DIV_CONF"]
pub mod i2s_rx_clkm_div_conf;
#[doc = "I2S_TX_CLKM_DIV_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_clkm_div_conf](i2s_tx_clkm_div_conf) module"]
pub type I2S_TX_CLKM_DIV_CONF = crate::Reg<u32, _I2S_TX_CLKM_DIV_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TX_CLKM_DIV_CONF;
#[doc = "`read()` method returns [i2s_tx_clkm_div_conf::R](i2s_tx_clkm_div_conf::R) reader structure"]
impl crate::Readable for I2S_TX_CLKM_DIV_CONF {}
#[doc = "`write(|w| ..)` method takes [i2s_tx_clkm_div_conf::W](i2s_tx_clkm_div_conf::W) writer structure"]
impl crate::Writable for I2S_TX_CLKM_DIV_CONF {}
#[doc = "I2S_TX_CLKM_DIV_CONF"]
pub mod i2s_tx_clkm_div_conf;
#[doc = "I2S_TX_PCM2PDM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_pcm2pdm_conf](i2s_tx_pcm2pdm_conf) module"]
pub type I2S_TX_PCM2PDM_CONF = crate::Reg<u32, _I2S_TX_PCM2PDM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TX_PCM2PDM_CONF;
#[doc = "`read()` method returns [i2s_tx_pcm2pdm_conf::R](i2s_tx_pcm2pdm_conf::R) reader structure"]
impl crate::Readable for I2S_TX_PCM2PDM_CONF {}
#[doc = "`write(|w| ..)` method takes [i2s_tx_pcm2pdm_conf::W](i2s_tx_pcm2pdm_conf::W) writer structure"]
impl crate::Writable for I2S_TX_PCM2PDM_CONF {}
#[doc = "I2S_TX_PCM2PDM_CONF"]
pub mod i2s_tx_pcm2pdm_conf;
#[doc = "I2S_TX_PCM2PDM_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_pcm2pdm_conf1](i2s_tx_pcm2pdm_conf1) module"]
pub type I2S_TX_PCM2PDM_CONF1 = crate::Reg<u32, _I2S_TX_PCM2PDM_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TX_PCM2PDM_CONF1;
#[doc = "`read()` method returns [i2s_tx_pcm2pdm_conf1::R](i2s_tx_pcm2pdm_conf1::R) reader structure"]
impl crate::Readable for I2S_TX_PCM2PDM_CONF1 {}
#[doc = "`write(|w| ..)` method takes [i2s_tx_pcm2pdm_conf1::W](i2s_tx_pcm2pdm_conf1::W) writer structure"]
impl crate::Writable for I2S_TX_PCM2PDM_CONF1 {}
#[doc = "I2S_TX_PCM2PDM_CONF1"]
pub mod i2s_tx_pcm2pdm_conf1;
#[doc = "I2S_RX_TDM_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rx_tdm_ctrl](i2s_rx_tdm_ctrl) module"]
pub type I2S_RX_TDM_CTRL = crate::Reg<u32, _I2S_RX_TDM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RX_TDM_CTRL;
#[doc = "`read()` method returns [i2s_rx_tdm_ctrl::R](i2s_rx_tdm_ctrl::R) reader structure"]
impl crate::Readable for I2S_RX_TDM_CTRL {}
#[doc = "`write(|w| ..)` method takes [i2s_rx_tdm_ctrl::W](i2s_rx_tdm_ctrl::W) writer structure"]
impl crate::Writable for I2S_RX_TDM_CTRL {}
#[doc = "I2S_RX_TDM_CTRL"]
pub mod i2s_rx_tdm_ctrl;
#[doc = "I2S_TX_TDM_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_tdm_ctrl](i2s_tx_tdm_ctrl) module"]
pub type I2S_TX_TDM_CTRL = crate::Reg<u32, _I2S_TX_TDM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TX_TDM_CTRL;
#[doc = "`read()` method returns [i2s_tx_tdm_ctrl::R](i2s_tx_tdm_ctrl::R) reader structure"]
impl crate::Readable for I2S_TX_TDM_CTRL {}
#[doc = "`write(|w| ..)` method takes [i2s_tx_tdm_ctrl::W](i2s_tx_tdm_ctrl::W) writer structure"]
impl crate::Writable for I2S_TX_TDM_CTRL {}
#[doc = "I2S_TX_TDM_CTRL"]
pub mod i2s_tx_tdm_ctrl;
#[doc = "I2S_RX_TIMING\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rx_timing](i2s_rx_timing) module"]
pub type I2S_RX_TIMING = crate::Reg<u32, _I2S_RX_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RX_TIMING;
#[doc = "`read()` method returns [i2s_rx_timing::R](i2s_rx_timing::R) reader structure"]
impl crate::Readable for I2S_RX_TIMING {}
#[doc = "`write(|w| ..)` method takes [i2s_rx_timing::W](i2s_rx_timing::W) writer structure"]
impl crate::Writable for I2S_RX_TIMING {}
#[doc = "I2S_RX_TIMING"]
pub mod i2s_rx_timing;
#[doc = "I2S_TX_TIMING\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_timing](i2s_tx_timing) module"]
pub type I2S_TX_TIMING = crate::Reg<u32, _I2S_TX_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TX_TIMING;
#[doc = "`read()` method returns [i2s_tx_timing::R](i2s_tx_timing::R) reader structure"]
impl crate::Readable for I2S_TX_TIMING {}
#[doc = "`write(|w| ..)` method takes [i2s_tx_timing::W](i2s_tx_timing::W) writer structure"]
impl crate::Writable for I2S_TX_TIMING {}
#[doc = "I2S_TX_TIMING"]
pub mod i2s_tx_timing;
#[doc = "I2S_LC_HUNG_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_lc_hung_conf](i2s_lc_hung_conf) module"]
pub type I2S_LC_HUNG_CONF = crate::Reg<u32, _I2S_LC_HUNG_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_LC_HUNG_CONF;
#[doc = "`read()` method returns [i2s_lc_hung_conf::R](i2s_lc_hung_conf::R) reader structure"]
impl crate::Readable for I2S_LC_HUNG_CONF {}
#[doc = "`write(|w| ..)` method takes [i2s_lc_hung_conf::W](i2s_lc_hung_conf::W) writer structure"]
impl crate::Writable for I2S_LC_HUNG_CONF {}
#[doc = "I2S_LC_HUNG_CONF"]
pub mod i2s_lc_hung_conf;
#[doc = "I2S_RXEOF_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rxeof_num](i2s_rxeof_num) module"]
pub type I2S_RXEOF_NUM = crate::Reg<u32, _I2S_RXEOF_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RXEOF_NUM;
#[doc = "`read()` method returns [i2s_rxeof_num::R](i2s_rxeof_num::R) reader structure"]
impl crate::Readable for I2S_RXEOF_NUM {}
#[doc = "`write(|w| ..)` method takes [i2s_rxeof_num::W](i2s_rxeof_num::W) writer structure"]
impl crate::Writable for I2S_RXEOF_NUM {}
#[doc = "I2S_RXEOF_NUM"]
pub mod i2s_rxeof_num;
#[doc = "I2S_CONF_SIGLE_DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_conf_sigle_data](i2s_conf_sigle_data) module"]
pub type I2S_CONF_SIGLE_DATA = crate::Reg<u32, _I2S_CONF_SIGLE_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CONF_SIGLE_DATA;
#[doc = "`read()` method returns [i2s_conf_sigle_data::R](i2s_conf_sigle_data::R) reader structure"]
impl crate::Readable for I2S_CONF_SIGLE_DATA {}
#[doc = "`write(|w| ..)` method takes [i2s_conf_sigle_data::W](i2s_conf_sigle_data::W) writer structure"]
impl crate::Writable for I2S_CONF_SIGLE_DATA {}
#[doc = "I2S_CONF_SIGLE_DATA"]
pub mod i2s_conf_sigle_data;
#[doc = "I2S_STATE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_state](i2s_state) module"]
pub type I2S_STATE = crate::Reg<u32, _I2S_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_STATE;
#[doc = "`read()` method returns [i2s_state::R](i2s_state::R) reader structure"]
impl crate::Readable for I2S_STATE {}
#[doc = "I2S_STATE"]
pub mod i2s_state;
#[doc = "I2S_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_date](i2s_date) module"]
pub type I2S_DATE = crate::Reg<u32, _I2S_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_DATE;
#[doc = "`read()` method returns [i2s_date::R](i2s_date::R) reader structure"]
impl crate::Readable for I2S_DATE {}
#[doc = "`write(|w| ..)` method takes [i2s_date::W](i2s_date::W) writer structure"]
impl crate::Writable for I2S_DATE {}
#[doc = "I2S_DATE"]
pub mod i2s_date;
