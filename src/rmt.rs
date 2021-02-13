#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - RMT_CH0CONF0"]
    pub rmt_ch0conf0: RMT_CH0CONF0,
    #[doc = "0x14 - RMT_CH1CONF0"]
    pub rmt_ch1conf0: RMT_CH1CONF0,
    #[doc = "0x18 - RMT_CH2CONF0"]
    pub rmt_ch2conf0: RMT_CH2CONF0,
    #[doc = "0x1c - RMT_CH2CONF1"]
    pub rmt_ch2conf1: RMT_CH2CONF1,
    #[doc = "0x20 - RMT_CH3CONF0"]
    pub rmt_ch3conf0: RMT_CH3CONF0,
    #[doc = "0x24 - RMT_CH3CONF1"]
    pub rmt_ch3conf1: RMT_CH3CONF1,
    #[doc = "0x28 - RMT_CH0STATUS"]
    pub rmt_ch0status: RMT_CH0STATUS,
    #[doc = "0x2c - RMT_CH1STATUS"]
    pub rmt_ch1status: RMT_CH1STATUS,
    #[doc = "0x30 - RMT_CH2STATUS"]
    pub rmt_ch2status: RMT_CH2STATUS,
    #[doc = "0x34 - RMT_CH3STATUS"]
    pub rmt_ch3status: RMT_CH3STATUS,
    #[doc = "0x38 - RMT_INT_RAW"]
    pub rmt_int_raw: RMT_INT_RAW,
    #[doc = "0x3c - RMT_INT_ST"]
    pub rmt_int_st: RMT_INT_ST,
    #[doc = "0x40 - RMT_INT_ENA"]
    pub rmt_int_ena: RMT_INT_ENA,
    #[doc = "0x44 - RMT_INT_CLR"]
    pub rmt_int_clr: RMT_INT_CLR,
    #[doc = "0x48 - RMT_CH0CARRIER_DUTY"]
    pub rmt_ch0carrier_duty: RMT_CH0CARRIER_DUTY,
    #[doc = "0x4c - RMT_CH1CARRIER_DUTY"]
    pub rmt_ch1carrier_duty: RMT_CH1CARRIER_DUTY,
    #[doc = "0x50 - RMT_CH2_RX_CARRIER_RM"]
    pub rmt_ch2_rx_carrier_rm: RMT_CH2_RX_CARRIER_RM,
    #[doc = "0x54 - RMT_CH3_RX_CARRIER_RM"]
    pub rmt_ch3_rx_carrier_rm: RMT_CH3_RX_CARRIER_RM,
    #[doc = "0x58 - RMT_CH0_TX_LIM"]
    pub rmt_ch0_tx_lim: RMT_CH0_TX_LIM,
    #[doc = "0x5c - RMT_CH1_TX_LIM"]
    pub rmt_ch1_tx_lim: RMT_CH1_TX_LIM,
    #[doc = "0x60 - RMT_CH2_RX_LIM"]
    pub rmt_ch2_rx_lim: RMT_CH2_RX_LIM,
    #[doc = "0x64 - RMT_CH3_RX_LIM"]
    pub rmt_ch3_rx_lim: RMT_CH3_RX_LIM,
    #[doc = "0x68 - RMT_SYS_CONF"]
    pub rmt_sys_conf: RMT_SYS_CONF,
    #[doc = "0x6c - RMT_TX_SIM"]
    pub rmt_tx_sim: RMT_TX_SIM,
    #[doc = "0x70 - RMT_REF_CNT_RST"]
    pub rmt_ref_cnt_rst: RMT_REF_CNT_RST,
    _reserved25: [u8; 88usize],
    #[doc = "0xcc - RMT_DATE"]
    pub rmt_date: RMT_DATE,
}
#[doc = "RMT_CH0CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch0conf0](rmt_ch0conf0) module"]
pub type RMT_CH0CONF0 = crate::Reg<u32, _RMT_CH0CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0CONF0;
#[doc = "`read()` method returns [rmt_ch0conf0::R](rmt_ch0conf0::R) reader structure"]
impl crate::Readable for RMT_CH0CONF0 {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0conf0::W](rmt_ch0conf0::W) writer structure"]
impl crate::Writable for RMT_CH0CONF0 {}
#[doc = "RMT_CH0CONF0"]
pub mod rmt_ch0conf0;
#[doc = "RMT_CH1CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch1conf0](rmt_ch1conf0) module"]
pub type RMT_CH1CONF0 = crate::Reg<u32, _RMT_CH1CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1CONF0;
#[doc = "`read()` method returns [rmt_ch1conf0::R](rmt_ch1conf0::R) reader structure"]
impl crate::Readable for RMT_CH1CONF0 {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1conf0::W](rmt_ch1conf0::W) writer structure"]
impl crate::Writable for RMT_CH1CONF0 {}
#[doc = "RMT_CH1CONF0"]
pub mod rmt_ch1conf0;
#[doc = "RMT_CH2CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch2conf0](rmt_ch2conf0) module"]
pub type RMT_CH2CONF0 = crate::Reg<u32, _RMT_CH2CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2CONF0;
#[doc = "`read()` method returns [rmt_ch2conf0::R](rmt_ch2conf0::R) reader structure"]
impl crate::Readable for RMT_CH2CONF0 {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2conf0::W](rmt_ch2conf0::W) writer structure"]
impl crate::Writable for RMT_CH2CONF0 {}
#[doc = "RMT_CH2CONF0"]
pub mod rmt_ch2conf0;
#[doc = "RMT_CH2CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch2conf1](rmt_ch2conf1) module"]
pub type RMT_CH2CONF1 = crate::Reg<u32, _RMT_CH2CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2CONF1;
#[doc = "`read()` method returns [rmt_ch2conf1::R](rmt_ch2conf1::R) reader structure"]
impl crate::Readable for RMT_CH2CONF1 {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2conf1::W](rmt_ch2conf1::W) writer structure"]
impl crate::Writable for RMT_CH2CONF1 {}
#[doc = "RMT_CH2CONF1"]
pub mod rmt_ch2conf1;
#[doc = "RMT_CH3CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch3conf0](rmt_ch3conf0) module"]
pub type RMT_CH3CONF0 = crate::Reg<u32, _RMT_CH3CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3CONF0;
#[doc = "`read()` method returns [rmt_ch3conf0::R](rmt_ch3conf0::R) reader structure"]
impl crate::Readable for RMT_CH3CONF0 {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3conf0::W](rmt_ch3conf0::W) writer structure"]
impl crate::Writable for RMT_CH3CONF0 {}
#[doc = "RMT_CH3CONF0"]
pub mod rmt_ch3conf0;
#[doc = "RMT_CH3CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch3conf1](rmt_ch3conf1) module"]
pub type RMT_CH3CONF1 = crate::Reg<u32, _RMT_CH3CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3CONF1;
#[doc = "`read()` method returns [rmt_ch3conf1::R](rmt_ch3conf1::R) reader structure"]
impl crate::Readable for RMT_CH3CONF1 {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3conf1::W](rmt_ch3conf1::W) writer structure"]
impl crate::Writable for RMT_CH3CONF1 {}
#[doc = "RMT_CH3CONF1"]
pub mod rmt_ch3conf1;
#[doc = "RMT_CH0STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch0status](rmt_ch0status) module"]
pub type RMT_CH0STATUS = crate::Reg<u32, _RMT_CH0STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0STATUS;
#[doc = "`read()` method returns [rmt_ch0status::R](rmt_ch0status::R) reader structure"]
impl crate::Readable for RMT_CH0STATUS {}
#[doc = "RMT_CH0STATUS"]
pub mod rmt_ch0status;
#[doc = "RMT_CH1STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch1status](rmt_ch1status) module"]
pub type RMT_CH1STATUS = crate::Reg<u32, _RMT_CH1STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1STATUS;
#[doc = "`read()` method returns [rmt_ch1status::R](rmt_ch1status::R) reader structure"]
impl crate::Readable for RMT_CH1STATUS {}
#[doc = "RMT_CH1STATUS"]
pub mod rmt_ch1status;
#[doc = "RMT_CH2STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch2status](rmt_ch2status) module"]
pub type RMT_CH2STATUS = crate::Reg<u32, _RMT_CH2STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2STATUS;
#[doc = "`read()` method returns [rmt_ch2status::R](rmt_ch2status::R) reader structure"]
impl crate::Readable for RMT_CH2STATUS {}
#[doc = "RMT_CH2STATUS"]
pub mod rmt_ch2status;
#[doc = "RMT_CH3STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch3status](rmt_ch3status) module"]
pub type RMT_CH3STATUS = crate::Reg<u32, _RMT_CH3STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3STATUS;
#[doc = "`read()` method returns [rmt_ch3status::R](rmt_ch3status::R) reader structure"]
impl crate::Readable for RMT_CH3STATUS {}
#[doc = "RMT_CH3STATUS"]
pub mod rmt_ch3status;
#[doc = "RMT_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_int_raw](rmt_int_raw) module"]
pub type RMT_INT_RAW = crate::Reg<u32, _RMT_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_INT_RAW;
#[doc = "`read()` method returns [rmt_int_raw::R](rmt_int_raw::R) reader structure"]
impl crate::Readable for RMT_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [rmt_int_raw::W](rmt_int_raw::W) writer structure"]
impl crate::Writable for RMT_INT_RAW {}
#[doc = "RMT_INT_RAW"]
pub mod rmt_int_raw;
#[doc = "RMT_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_int_st](rmt_int_st) module"]
pub type RMT_INT_ST = crate::Reg<u32, _RMT_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_INT_ST;
#[doc = "`read()` method returns [rmt_int_st::R](rmt_int_st::R) reader structure"]
impl crate::Readable for RMT_INT_ST {}
#[doc = "RMT_INT_ST"]
pub mod rmt_int_st;
#[doc = "RMT_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_int_ena](rmt_int_ena) module"]
pub type RMT_INT_ENA = crate::Reg<u32, _RMT_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_INT_ENA;
#[doc = "`read()` method returns [rmt_int_ena::R](rmt_int_ena::R) reader structure"]
impl crate::Readable for RMT_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [rmt_int_ena::W](rmt_int_ena::W) writer structure"]
impl crate::Writable for RMT_INT_ENA {}
#[doc = "RMT_INT_ENA"]
pub mod rmt_int_ena;
#[doc = "RMT_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_int_clr](rmt_int_clr) module"]
pub type RMT_INT_CLR = crate::Reg<u32, _RMT_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_INT_CLR;
#[doc = "`write(|w| ..)` method takes [rmt_int_clr::W](rmt_int_clr::W) writer structure"]
impl crate::Writable for RMT_INT_CLR {}
#[doc = "RMT_INT_CLR"]
pub mod rmt_int_clr;
#[doc = "RMT_CH0CARRIER_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch0carrier_duty](rmt_ch0carrier_duty) module"]
pub type RMT_CH0CARRIER_DUTY = crate::Reg<u32, _RMT_CH0CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0CARRIER_DUTY;
#[doc = "`read()` method returns [rmt_ch0carrier_duty::R](rmt_ch0carrier_duty::R) reader structure"]
impl crate::Readable for RMT_CH0CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0carrier_duty::W](rmt_ch0carrier_duty::W) writer structure"]
impl crate::Writable for RMT_CH0CARRIER_DUTY {}
#[doc = "RMT_CH0CARRIER_DUTY"]
pub mod rmt_ch0carrier_duty;
#[doc = "RMT_CH1CARRIER_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch1carrier_duty](rmt_ch1carrier_duty) module"]
pub type RMT_CH1CARRIER_DUTY = crate::Reg<u32, _RMT_CH1CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1CARRIER_DUTY;
#[doc = "`read()` method returns [rmt_ch1carrier_duty::R](rmt_ch1carrier_duty::R) reader structure"]
impl crate::Readable for RMT_CH1CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1carrier_duty::W](rmt_ch1carrier_duty::W) writer structure"]
impl crate::Writable for RMT_CH1CARRIER_DUTY {}
#[doc = "RMT_CH1CARRIER_DUTY"]
pub mod rmt_ch1carrier_duty;
#[doc = "RMT_CH2_RX_CARRIER_RM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch2_rx_carrier_rm](rmt_ch2_rx_carrier_rm) module"]
pub type RMT_CH2_RX_CARRIER_RM = crate::Reg<u32, _RMT_CH2_RX_CARRIER_RM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2_RX_CARRIER_RM;
#[doc = "`read()` method returns [rmt_ch2_rx_carrier_rm::R](rmt_ch2_rx_carrier_rm::R) reader structure"]
impl crate::Readable for RMT_CH2_RX_CARRIER_RM {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2_rx_carrier_rm::W](rmt_ch2_rx_carrier_rm::W) writer structure"]
impl crate::Writable for RMT_CH2_RX_CARRIER_RM {}
#[doc = "RMT_CH2_RX_CARRIER_RM"]
pub mod rmt_ch2_rx_carrier_rm;
#[doc = "RMT_CH3_RX_CARRIER_RM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch3_rx_carrier_rm](rmt_ch3_rx_carrier_rm) module"]
pub type RMT_CH3_RX_CARRIER_RM = crate::Reg<u32, _RMT_CH3_RX_CARRIER_RM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3_RX_CARRIER_RM;
#[doc = "`read()` method returns [rmt_ch3_rx_carrier_rm::R](rmt_ch3_rx_carrier_rm::R) reader structure"]
impl crate::Readable for RMT_CH3_RX_CARRIER_RM {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3_rx_carrier_rm::W](rmt_ch3_rx_carrier_rm::W) writer structure"]
impl crate::Writable for RMT_CH3_RX_CARRIER_RM {}
#[doc = "RMT_CH3_RX_CARRIER_RM"]
pub mod rmt_ch3_rx_carrier_rm;
#[doc = "RMT_CH0_TX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch0_tx_lim](rmt_ch0_tx_lim) module"]
pub type RMT_CH0_TX_LIM = crate::Reg<u32, _RMT_CH0_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0_TX_LIM;
#[doc = "`read()` method returns [rmt_ch0_tx_lim::R](rmt_ch0_tx_lim::R) reader structure"]
impl crate::Readable for RMT_CH0_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0_tx_lim::W](rmt_ch0_tx_lim::W) writer structure"]
impl crate::Writable for RMT_CH0_TX_LIM {}
#[doc = "RMT_CH0_TX_LIM"]
pub mod rmt_ch0_tx_lim;
#[doc = "RMT_CH1_TX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch1_tx_lim](rmt_ch1_tx_lim) module"]
pub type RMT_CH1_TX_LIM = crate::Reg<u32, _RMT_CH1_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1_TX_LIM;
#[doc = "`read()` method returns [rmt_ch1_tx_lim::R](rmt_ch1_tx_lim::R) reader structure"]
impl crate::Readable for RMT_CH1_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1_tx_lim::W](rmt_ch1_tx_lim::W) writer structure"]
impl crate::Writable for RMT_CH1_TX_LIM {}
#[doc = "RMT_CH1_TX_LIM"]
pub mod rmt_ch1_tx_lim;
#[doc = "RMT_CH2_RX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch2_rx_lim](rmt_ch2_rx_lim) module"]
pub type RMT_CH2_RX_LIM = crate::Reg<u32, _RMT_CH2_RX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2_RX_LIM;
#[doc = "`read()` method returns [rmt_ch2_rx_lim::R](rmt_ch2_rx_lim::R) reader structure"]
impl crate::Readable for RMT_CH2_RX_LIM {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2_rx_lim::W](rmt_ch2_rx_lim::W) writer structure"]
impl crate::Writable for RMT_CH2_RX_LIM {}
#[doc = "RMT_CH2_RX_LIM"]
pub mod rmt_ch2_rx_lim;
#[doc = "RMT_CH3_RX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ch3_rx_lim](rmt_ch3_rx_lim) module"]
pub type RMT_CH3_RX_LIM = crate::Reg<u32, _RMT_CH3_RX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3_RX_LIM;
#[doc = "`read()` method returns [rmt_ch3_rx_lim::R](rmt_ch3_rx_lim::R) reader structure"]
impl crate::Readable for RMT_CH3_RX_LIM {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3_rx_lim::W](rmt_ch3_rx_lim::W) writer structure"]
impl crate::Writable for RMT_CH3_RX_LIM {}
#[doc = "RMT_CH3_RX_LIM"]
pub mod rmt_ch3_rx_lim;
#[doc = "RMT_SYS_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_sys_conf](rmt_sys_conf) module"]
pub type RMT_SYS_CONF = crate::Reg<u32, _RMT_SYS_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_SYS_CONF;
#[doc = "`read()` method returns [rmt_sys_conf::R](rmt_sys_conf::R) reader structure"]
impl crate::Readable for RMT_SYS_CONF {}
#[doc = "`write(|w| ..)` method takes [rmt_sys_conf::W](rmt_sys_conf::W) writer structure"]
impl crate::Writable for RMT_SYS_CONF {}
#[doc = "RMT_SYS_CONF"]
pub mod rmt_sys_conf;
#[doc = "RMT_TX_SIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_tx_sim](rmt_tx_sim) module"]
pub type RMT_TX_SIM = crate::Reg<u32, _RMT_TX_SIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_TX_SIM;
#[doc = "`read()` method returns [rmt_tx_sim::R](rmt_tx_sim::R) reader structure"]
impl crate::Readable for RMT_TX_SIM {}
#[doc = "`write(|w| ..)` method takes [rmt_tx_sim::W](rmt_tx_sim::W) writer structure"]
impl crate::Writable for RMT_TX_SIM {}
#[doc = "RMT_TX_SIM"]
pub mod rmt_tx_sim;
#[doc = "RMT_REF_CNT_RST\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_ref_cnt_rst](rmt_ref_cnt_rst) module"]
pub type RMT_REF_CNT_RST = crate::Reg<u32, _RMT_REF_CNT_RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_REF_CNT_RST;
#[doc = "`write(|w| ..)` method takes [rmt_ref_cnt_rst::W](rmt_ref_cnt_rst::W) writer structure"]
impl crate::Writable for RMT_REF_CNT_RST {}
#[doc = "RMT_REF_CNT_RST"]
pub mod rmt_ref_cnt_rst;
#[doc = "RMT_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_date](rmt_date) module"]
pub type RMT_DATE = crate::Reg<u32, _RMT_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_DATE;
#[doc = "`read()` method returns [rmt_date::R](rmt_date::R) reader structure"]
impl crate::Readable for RMT_DATE {}
#[doc = "`write(|w| ..)` method takes [rmt_date::W](rmt_date::W) writer structure"]
impl crate::Writable for RMT_DATE {}
#[doc = "RMT_DATE"]
pub mod rmt_date;
