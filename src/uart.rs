#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART_FIFO"]
    pub uart_fifo: UART_FIFO,
    #[doc = "0x04 - UART_INT_RAW"]
    pub uart_int_raw: UART_INT_RAW,
    #[doc = "0x08 - UART_INT_ST"]
    pub uart_int_st: UART_INT_ST,
    #[doc = "0x0c - UART_INT_ENA"]
    pub uart_int_ena: UART_INT_ENA,
    #[doc = "0x10 - UART_INT_CLR"]
    pub uart_int_clr: UART_INT_CLR,
    #[doc = "0x14 - UART_CLKDIV"]
    pub uart_clkdiv: UART_CLKDIV,
    #[doc = "0x18 - UART_RX_FILT"]
    pub uart_rx_filt: UART_RX_FILT,
    #[doc = "0x1c - UART_STATUS"]
    pub uart_status: UART_STATUS,
    #[doc = "0x20 - UART_CONF0"]
    pub uart_conf0: UART_CONF0,
    #[doc = "0x24 - UART_CONF1"]
    pub uart_conf1: UART_CONF1,
    #[doc = "0x28 - UART_LOWPULSE"]
    pub uart_lowpulse: UART_LOWPULSE,
    #[doc = "0x2c - UART_HIGHPULSE"]
    pub uart_highpulse: UART_HIGHPULSE,
    #[doc = "0x30 - UART_RXD_CNT"]
    pub uart_rxd_cnt: UART_RXD_CNT,
    #[doc = "0x34 - UART_FLOW_CONF"]
    pub uart_flow_conf: UART_FLOW_CONF,
    #[doc = "0x38 - UART_SLEEP_CONF"]
    pub uart_sleep_conf: UART_SLEEP_CONF,
    #[doc = "0x3c - UART_SWFC_CONF0"]
    pub uart_swfc_conf0: UART_SWFC_CONF0,
    #[doc = "0x40 - UART_SWFC_CONF1"]
    pub uart_swfc_conf1: UART_SWFC_CONF1,
    #[doc = "0x44 - UART_TXBRK_CONF"]
    pub uart_txbrk_conf: UART_TXBRK_CONF,
    #[doc = "0x48 - UART_IDLE_CONF"]
    pub uart_idle_conf: UART_IDLE_CONF,
    #[doc = "0x4c - UART_RS485_CONF"]
    pub uart_rs485_conf: UART_RS485_CONF,
    #[doc = "0x50 - UART_AT_CMD_PRECNT"]
    pub uart_at_cmd_precnt: UART_AT_CMD_PRECNT,
    #[doc = "0x54 - UART_AT_CMD_POSTCNT"]
    pub uart_at_cmd_postcnt: UART_AT_CMD_POSTCNT,
    #[doc = "0x58 - UART_AT_CMD_GAPTOUT"]
    pub uart_at_cmd_gaptout: UART_AT_CMD_GAPTOUT,
    #[doc = "0x5c - UART_AT_CMD_CHAR"]
    pub uart_at_cmd_char: UART_AT_CMD_CHAR,
    #[doc = "0x60 - UART_MEM_CONF"]
    pub uart_mem_conf: UART_MEM_CONF,
    #[doc = "0x64 - UART_MEM_TX_STATUS"]
    pub uart_mem_tx_status: UART_MEM_TX_STATUS,
    #[doc = "0x68 - UART_MEM_RX_STATUS"]
    pub uart_mem_rx_status: UART_MEM_RX_STATUS,
    #[doc = "0x6c - UART_FSM_STATUS"]
    pub uart_fsm_status: UART_FSM_STATUS,
    #[doc = "0x70 - UART_POSPULSE"]
    pub uart_pospulse: UART_POSPULSE,
    #[doc = "0x74 - UART_NEGPULSE"]
    pub uart_negpulse: UART_NEGPULSE,
    #[doc = "0x78 - UART_CLK_CONF"]
    pub uart_clk_conf: UART_CLK_CONF,
    #[doc = "0x7c - UART_DATE"]
    pub uart_date: UART_DATE,
    #[doc = "0x80 - UART_ID"]
    pub uart_id: UART_ID,
}
#[doc = "UART_FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo](uart_fifo) module"]
pub type UART_FIFO = crate::Reg<u32, _UART_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FIFO;
#[doc = "`read()` method returns [uart_fifo::R](uart_fifo::R) reader structure"]
impl crate::Readable for UART_FIFO {}
#[doc = "UART_FIFO"]
pub mod uart_fifo;
#[doc = "UART_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_raw](uart_int_raw) module"]
pub type UART_INT_RAW = crate::Reg<u32, _UART_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_RAW;
#[doc = "`read()` method returns [uart_int_raw::R](uart_int_raw::R) reader structure"]
impl crate::Readable for UART_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [uart_int_raw::W](uart_int_raw::W) writer structure"]
impl crate::Writable for UART_INT_RAW {}
#[doc = "UART_INT_RAW"]
pub mod uart_int_raw;
#[doc = "UART_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_st](uart_int_st) module"]
pub type UART_INT_ST = crate::Reg<u32, _UART_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_ST;
#[doc = "`read()` method returns [uart_int_st::R](uart_int_st::R) reader structure"]
impl crate::Readable for UART_INT_ST {}
#[doc = "UART_INT_ST"]
pub mod uart_int_st;
#[doc = "UART_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_ena](uart_int_ena) module"]
pub type UART_INT_ENA = crate::Reg<u32, _UART_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_ENA;
#[doc = "`read()` method returns [uart_int_ena::R](uart_int_ena::R) reader structure"]
impl crate::Readable for UART_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [uart_int_ena::W](uart_int_ena::W) writer structure"]
impl crate::Writable for UART_INT_ENA {}
#[doc = "UART_INT_ENA"]
pub mod uart_int_ena;
#[doc = "UART_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_clr](uart_int_clr) module"]
pub type UART_INT_CLR = crate::Reg<u32, _UART_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_CLR;
#[doc = "`write(|w| ..)` method takes [uart_int_clr::W](uart_int_clr::W) writer structure"]
impl crate::Writable for UART_INT_CLR {}
#[doc = "UART_INT_CLR"]
pub mod uart_int_clr;
#[doc = "UART_CLKDIV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_clkdiv](uart_clkdiv) module"]
pub type UART_CLKDIV = crate::Reg<u32, _UART_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CLKDIV;
#[doc = "`read()` method returns [uart_clkdiv::R](uart_clkdiv::R) reader structure"]
impl crate::Readable for UART_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [uart_clkdiv::W](uart_clkdiv::W) writer structure"]
impl crate::Writable for UART_CLKDIV {}
#[doc = "UART_CLKDIV"]
pub mod uart_clkdiv;
#[doc = "UART_RX_FILT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rx_filt](uart_rx_filt) module"]
pub type UART_RX_FILT = crate::Reg<u32, _UART_RX_FILT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RX_FILT;
#[doc = "`read()` method returns [uart_rx_filt::R](uart_rx_filt::R) reader structure"]
impl crate::Readable for UART_RX_FILT {}
#[doc = "`write(|w| ..)` method takes [uart_rx_filt::W](uart_rx_filt::W) writer structure"]
impl crate::Writable for UART_RX_FILT {}
#[doc = "UART_RX_FILT"]
pub mod uart_rx_filt;
#[doc = "UART_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_status](uart_status) module"]
pub type UART_STATUS = crate::Reg<u32, _UART_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_STATUS;
#[doc = "`read()` method returns [uart_status::R](uart_status::R) reader structure"]
impl crate::Readable for UART_STATUS {}
#[doc = "UART_STATUS"]
pub mod uart_status;
#[doc = "UART_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_conf0](uart_conf0) module"]
pub type UART_CONF0 = crate::Reg<u32, _UART_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CONF0;
#[doc = "`read()` method returns [uart_conf0::R](uart_conf0::R) reader structure"]
impl crate::Readable for UART_CONF0 {}
#[doc = "`write(|w| ..)` method takes [uart_conf0::W](uart_conf0::W) writer structure"]
impl crate::Writable for UART_CONF0 {}
#[doc = "UART_CONF0"]
pub mod uart_conf0;
#[doc = "UART_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_conf1](uart_conf1) module"]
pub type UART_CONF1 = crate::Reg<u32, _UART_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CONF1;
#[doc = "`read()` method returns [uart_conf1::R](uart_conf1::R) reader structure"]
impl crate::Readable for UART_CONF1 {}
#[doc = "`write(|w| ..)` method takes [uart_conf1::W](uart_conf1::W) writer structure"]
impl crate::Writable for UART_CONF1 {}
#[doc = "UART_CONF1"]
pub mod uart_conf1;
#[doc = "UART_LOWPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_lowpulse](uart_lowpulse) module"]
pub type UART_LOWPULSE = crate::Reg<u32, _UART_LOWPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_LOWPULSE;
#[doc = "`read()` method returns [uart_lowpulse::R](uart_lowpulse::R) reader structure"]
impl crate::Readable for UART_LOWPULSE {}
#[doc = "UART_LOWPULSE"]
pub mod uart_lowpulse;
#[doc = "UART_HIGHPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_highpulse](uart_highpulse) module"]
pub type UART_HIGHPULSE = crate::Reg<u32, _UART_HIGHPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_HIGHPULSE;
#[doc = "`read()` method returns [uart_highpulse::R](uart_highpulse::R) reader structure"]
impl crate::Readable for UART_HIGHPULSE {}
#[doc = "UART_HIGHPULSE"]
pub mod uart_highpulse;
#[doc = "UART_RXD_CNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rxd_cnt](uart_rxd_cnt) module"]
pub type UART_RXD_CNT = crate::Reg<u32, _UART_RXD_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RXD_CNT;
#[doc = "`read()` method returns [uart_rxd_cnt::R](uart_rxd_cnt::R) reader structure"]
impl crate::Readable for UART_RXD_CNT {}
#[doc = "UART_RXD_CNT"]
pub mod uart_rxd_cnt;
#[doc = "UART_FLOW_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_flow_conf](uart_flow_conf) module"]
pub type UART_FLOW_CONF = crate::Reg<u32, _UART_FLOW_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FLOW_CONF;
#[doc = "`read()` method returns [uart_flow_conf::R](uart_flow_conf::R) reader structure"]
impl crate::Readable for UART_FLOW_CONF {}
#[doc = "`write(|w| ..)` method takes [uart_flow_conf::W](uart_flow_conf::W) writer structure"]
impl crate::Writable for UART_FLOW_CONF {}
#[doc = "UART_FLOW_CONF"]
pub mod uart_flow_conf;
#[doc = "UART_SLEEP_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sleep_conf](uart_sleep_conf) module"]
pub type UART_SLEEP_CONF = crate::Reg<u32, _UART_SLEEP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SLEEP_CONF;
#[doc = "`read()` method returns [uart_sleep_conf::R](uart_sleep_conf::R) reader structure"]
impl crate::Readable for UART_SLEEP_CONF {}
#[doc = "`write(|w| ..)` method takes [uart_sleep_conf::W](uart_sleep_conf::W) writer structure"]
impl crate::Writable for UART_SLEEP_CONF {}
#[doc = "UART_SLEEP_CONF"]
pub mod uart_sleep_conf;
#[doc = "UART_SWFC_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_swfc_conf0](uart_swfc_conf0) module"]
pub type UART_SWFC_CONF0 = crate::Reg<u32, _UART_SWFC_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SWFC_CONF0;
#[doc = "`read()` method returns [uart_swfc_conf0::R](uart_swfc_conf0::R) reader structure"]
impl crate::Readable for UART_SWFC_CONF0 {}
#[doc = "`write(|w| ..)` method takes [uart_swfc_conf0::W](uart_swfc_conf0::W) writer structure"]
impl crate::Writable for UART_SWFC_CONF0 {}
#[doc = "UART_SWFC_CONF0"]
pub mod uart_swfc_conf0;
#[doc = "UART_SWFC_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_swfc_conf1](uart_swfc_conf1) module"]
pub type UART_SWFC_CONF1 = crate::Reg<u32, _UART_SWFC_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SWFC_CONF1;
#[doc = "`read()` method returns [uart_swfc_conf1::R](uart_swfc_conf1::R) reader structure"]
impl crate::Readable for UART_SWFC_CONF1 {}
#[doc = "`write(|w| ..)` method takes [uart_swfc_conf1::W](uart_swfc_conf1::W) writer structure"]
impl crate::Writable for UART_SWFC_CONF1 {}
#[doc = "UART_SWFC_CONF1"]
pub mod uart_swfc_conf1;
#[doc = "UART_TXBRK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_txbrk_conf](uart_txbrk_conf) module"]
pub type UART_TXBRK_CONF = crate::Reg<u32, _UART_TXBRK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_TXBRK_CONF;
#[doc = "`read()` method returns [uart_txbrk_conf::R](uart_txbrk_conf::R) reader structure"]
impl crate::Readable for UART_TXBRK_CONF {}
#[doc = "`write(|w| ..)` method takes [uart_txbrk_conf::W](uart_txbrk_conf::W) writer structure"]
impl crate::Writable for UART_TXBRK_CONF {}
#[doc = "UART_TXBRK_CONF"]
pub mod uart_txbrk_conf;
#[doc = "UART_IDLE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_idle_conf](uart_idle_conf) module"]
pub type UART_IDLE_CONF = crate::Reg<u32, _UART_IDLE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_IDLE_CONF;
#[doc = "`read()` method returns [uart_idle_conf::R](uart_idle_conf::R) reader structure"]
impl crate::Readable for UART_IDLE_CONF {}
#[doc = "`write(|w| ..)` method takes [uart_idle_conf::W](uart_idle_conf::W) writer structure"]
impl crate::Writable for UART_IDLE_CONF {}
#[doc = "UART_IDLE_CONF"]
pub mod uart_idle_conf;
#[doc = "UART_RS485_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rs485_conf](uart_rs485_conf) module"]
pub type UART_RS485_CONF = crate::Reg<u32, _UART_RS485_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RS485_CONF;
#[doc = "`read()` method returns [uart_rs485_conf::R](uart_rs485_conf::R) reader structure"]
impl crate::Readable for UART_RS485_CONF {}
#[doc = "`write(|w| ..)` method takes [uart_rs485_conf::W](uart_rs485_conf::W) writer structure"]
impl crate::Writable for UART_RS485_CONF {}
#[doc = "UART_RS485_CONF"]
pub mod uart_rs485_conf;
#[doc = "UART_AT_CMD_PRECNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_at_cmd_precnt](uart_at_cmd_precnt) module"]
pub type UART_AT_CMD_PRECNT = crate::Reg<u32, _UART_AT_CMD_PRECNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_PRECNT;
#[doc = "`read()` method returns [uart_at_cmd_precnt::R](uart_at_cmd_precnt::R) reader structure"]
impl crate::Readable for UART_AT_CMD_PRECNT {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_precnt::W](uart_at_cmd_precnt::W) writer structure"]
impl crate::Writable for UART_AT_CMD_PRECNT {}
#[doc = "UART_AT_CMD_PRECNT"]
pub mod uart_at_cmd_precnt;
#[doc = "UART_AT_CMD_POSTCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_at_cmd_postcnt](uart_at_cmd_postcnt) module"]
pub type UART_AT_CMD_POSTCNT = crate::Reg<u32, _UART_AT_CMD_POSTCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_POSTCNT;
#[doc = "`read()` method returns [uart_at_cmd_postcnt::R](uart_at_cmd_postcnt::R) reader structure"]
impl crate::Readable for UART_AT_CMD_POSTCNT {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_postcnt::W](uart_at_cmd_postcnt::W) writer structure"]
impl crate::Writable for UART_AT_CMD_POSTCNT {}
#[doc = "UART_AT_CMD_POSTCNT"]
pub mod uart_at_cmd_postcnt;
#[doc = "UART_AT_CMD_GAPTOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_at_cmd_gaptout](uart_at_cmd_gaptout) module"]
pub type UART_AT_CMD_GAPTOUT = crate::Reg<u32, _UART_AT_CMD_GAPTOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_GAPTOUT;
#[doc = "`read()` method returns [uart_at_cmd_gaptout::R](uart_at_cmd_gaptout::R) reader structure"]
impl crate::Readable for UART_AT_CMD_GAPTOUT {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_gaptout::W](uart_at_cmd_gaptout::W) writer structure"]
impl crate::Writable for UART_AT_CMD_GAPTOUT {}
#[doc = "UART_AT_CMD_GAPTOUT"]
pub mod uart_at_cmd_gaptout;
#[doc = "UART_AT_CMD_CHAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_at_cmd_char](uart_at_cmd_char) module"]
pub type UART_AT_CMD_CHAR = crate::Reg<u32, _UART_AT_CMD_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_AT_CMD_CHAR;
#[doc = "`read()` method returns [uart_at_cmd_char::R](uart_at_cmd_char::R) reader structure"]
impl crate::Readable for UART_AT_CMD_CHAR {}
#[doc = "`write(|w| ..)` method takes [uart_at_cmd_char::W](uart_at_cmd_char::W) writer structure"]
impl crate::Writable for UART_AT_CMD_CHAR {}
#[doc = "UART_AT_CMD_CHAR"]
pub mod uart_at_cmd_char;
#[doc = "UART_MEM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_mem_conf](uart_mem_conf) module"]
pub type UART_MEM_CONF = crate::Reg<u32, _UART_MEM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_CONF;
#[doc = "`read()` method returns [uart_mem_conf::R](uart_mem_conf::R) reader structure"]
impl crate::Readable for UART_MEM_CONF {}
#[doc = "`write(|w| ..)` method takes [uart_mem_conf::W](uart_mem_conf::W) writer structure"]
impl crate::Writable for UART_MEM_CONF {}
#[doc = "UART_MEM_CONF"]
pub mod uart_mem_conf;
#[doc = "UART_MEM_TX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_mem_tx_status](uart_mem_tx_status) module"]
pub type UART_MEM_TX_STATUS = crate::Reg<u32, _UART_MEM_TX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_TX_STATUS;
#[doc = "`read()` method returns [uart_mem_tx_status::R](uart_mem_tx_status::R) reader structure"]
impl crate::Readable for UART_MEM_TX_STATUS {}
#[doc = "UART_MEM_TX_STATUS"]
pub mod uart_mem_tx_status;
#[doc = "UART_MEM_RX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_mem_rx_status](uart_mem_rx_status) module"]
pub type UART_MEM_RX_STATUS = crate::Reg<u32, _UART_MEM_RX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MEM_RX_STATUS;
#[doc = "`read()` method returns [uart_mem_rx_status::R](uart_mem_rx_status::R) reader structure"]
impl crate::Readable for UART_MEM_RX_STATUS {}
#[doc = "UART_MEM_RX_STATUS"]
pub mod uart_mem_rx_status;
#[doc = "UART_FSM_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fsm_status](uart_fsm_status) module"]
pub type UART_FSM_STATUS = crate::Reg<u32, _UART_FSM_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FSM_STATUS;
#[doc = "`read()` method returns [uart_fsm_status::R](uart_fsm_status::R) reader structure"]
impl crate::Readable for UART_FSM_STATUS {}
#[doc = "UART_FSM_STATUS"]
pub mod uart_fsm_status;
#[doc = "UART_POSPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_pospulse](uart_pospulse) module"]
pub type UART_POSPULSE = crate::Reg<u32, _UART_POSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_POSPULSE;
#[doc = "`read()` method returns [uart_pospulse::R](uart_pospulse::R) reader structure"]
impl crate::Readable for UART_POSPULSE {}
#[doc = "UART_POSPULSE"]
pub mod uart_pospulse;
#[doc = "UART_NEGPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_negpulse](uart_negpulse) module"]
pub type UART_NEGPULSE = crate::Reg<u32, _UART_NEGPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_NEGPULSE;
#[doc = "`read()` method returns [uart_negpulse::R](uart_negpulse::R) reader structure"]
impl crate::Readable for UART_NEGPULSE {}
#[doc = "UART_NEGPULSE"]
pub mod uart_negpulse;
#[doc = "UART_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_clk_conf](uart_clk_conf) module"]
pub type UART_CLK_CONF = crate::Reg<u32, _UART_CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CLK_CONF;
#[doc = "`read()` method returns [uart_clk_conf::R](uart_clk_conf::R) reader structure"]
impl crate::Readable for UART_CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [uart_clk_conf::W](uart_clk_conf::W) writer structure"]
impl crate::Writable for UART_CLK_CONF {}
#[doc = "UART_CLK_CONF"]
pub mod uart_clk_conf;
#[doc = "UART_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_date](uart_date) module"]
pub type UART_DATE = crate::Reg<u32, _UART_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_DATE;
#[doc = "`read()` method returns [uart_date::R](uart_date::R) reader structure"]
impl crate::Readable for UART_DATE {}
#[doc = "`write(|w| ..)` method takes [uart_date::W](uart_date::W) writer structure"]
impl crate::Writable for UART_DATE {}
#[doc = "UART_DATE"]
pub mod uart_date;
#[doc = "UART_ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_id](uart_id) module"]
pub type UART_ID = crate::Reg<u32, _UART_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_ID;
#[doc = "`read()` method returns [uart_id::R](uart_id::R) reader structure"]
impl crate::Readable for UART_ID {}
#[doc = "`write(|w| ..)` method takes [uart_id::W](uart_id::W) writer structure"]
impl crate::Writable for UART_ID {}
#[doc = "UART_ID"]
pub mod uart_id;
