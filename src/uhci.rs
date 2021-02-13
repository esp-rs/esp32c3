#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UHCI_CONF0"]
    pub uhci_conf0: UHCI_CONF0,
    #[doc = "0x04 - UHCI_INT_RAW"]
    pub uhci_int_raw: UHCI_INT_RAW,
    #[doc = "0x08 - UHCI_INT_ST"]
    pub uhci_int_st: UHCI_INT_ST,
    #[doc = "0x0c - UHCI_INT_ENA"]
    pub uhci_int_ena: UHCI_INT_ENA,
    #[doc = "0x10 - UHCI_INT_CLR"]
    pub uhci_int_clr: UHCI_INT_CLR,
    #[doc = "0x14 - UHCI_CONF1"]
    pub uhci_conf1: UHCI_CONF1,
    #[doc = "0x18 - UHCI_STATE0"]
    pub uhci_state0: UHCI_STATE0,
    #[doc = "0x1c - UHCI_STATE1"]
    pub uhci_state1: UHCI_STATE1,
    #[doc = "0x20 - UHCI_ESCAPE_CONF"]
    pub uhci_escape_conf: UHCI_ESCAPE_CONF,
    #[doc = "0x24 - UHCI_HUNG_CONF"]
    pub uhci_hung_conf: UHCI_HUNG_CONF,
    #[doc = "0x28 - UHCI_ACK_NUM"]
    pub uhci_ack_num: UHCI_ACK_NUM,
    #[doc = "0x2c - UHCI_RX_HEAD"]
    pub uhci_rx_head: UHCI_RX_HEAD,
    #[doc = "0x30 - UHCI_QUICK_SENT"]
    pub uhci_quick_sent: UHCI_QUICK_SENT,
    #[doc = "0x34 - UHCI_Q0_WORD0"]
    pub uhci_q0_word0: UHCI_Q0_WORD0,
    #[doc = "0x38 - UHCI_Q0_WORD1"]
    pub uhci_q0_word1: UHCI_Q0_WORD1,
    #[doc = "0x3c - UHCI_Q1_WORD0"]
    pub uhci_q1_word0: UHCI_Q1_WORD0,
    #[doc = "0x40 - UHCI_Q1_WORD1"]
    pub uhci_q1_word1: UHCI_Q1_WORD1,
    #[doc = "0x44 - UHCI_Q2_WORD0"]
    pub uhci_q2_word0: UHCI_Q2_WORD0,
    #[doc = "0x48 - UHCI_Q2_WORD1"]
    pub uhci_q2_word1: UHCI_Q2_WORD1,
    #[doc = "0x4c - UHCI_Q3_WORD0"]
    pub uhci_q3_word0: UHCI_Q3_WORD0,
    #[doc = "0x50 - UHCI_Q3_WORD1"]
    pub uhci_q3_word1: UHCI_Q3_WORD1,
    #[doc = "0x54 - UHCI_Q4_WORD0"]
    pub uhci_q4_word0: UHCI_Q4_WORD0,
    #[doc = "0x58 - UHCI_Q4_WORD1"]
    pub uhci_q4_word1: UHCI_Q4_WORD1,
    #[doc = "0x5c - UHCI_Q5_WORD0"]
    pub uhci_q5_word0: UHCI_Q5_WORD0,
    #[doc = "0x60 - UHCI_Q5_WORD1"]
    pub uhci_q5_word1: UHCI_Q5_WORD1,
    #[doc = "0x64 - UHCI_Q6_WORD0"]
    pub uhci_q6_word0: UHCI_Q6_WORD0,
    #[doc = "0x68 - UHCI_Q6_WORD1"]
    pub uhci_q6_word1: UHCI_Q6_WORD1,
    #[doc = "0x6c - UHCI_ESC_CONF0"]
    pub uhci_esc_conf0: UHCI_ESC_CONF0,
    #[doc = "0x70 - UHCI_ESC_CONF1"]
    pub uhci_esc_conf1: UHCI_ESC_CONF1,
    #[doc = "0x74 - UHCI_ESC_CONF2"]
    pub uhci_esc_conf2: UHCI_ESC_CONF2,
    #[doc = "0x78 - UHCI_ESC_CONF3"]
    pub uhci_esc_conf3: UHCI_ESC_CONF3,
    #[doc = "0x7c - UHCI_PKT_THRES"]
    pub uhci_pkt_thres: UHCI_PKT_THRES,
    #[doc = "0x80 - UHCI_DATE"]
    pub uhci_date: UHCI_DATE,
}
#[doc = "UHCI_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_conf0](uhci_conf0) module"]
pub type UHCI_CONF0 = crate::Reg<u32, _UHCI_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_CONF0;
#[doc = "`read()` method returns [uhci_conf0::R](uhci_conf0::R) reader structure"]
impl crate::Readable for UHCI_CONF0 {}
#[doc = "`write(|w| ..)` method takes [uhci_conf0::W](uhci_conf0::W) writer structure"]
impl crate::Writable for UHCI_CONF0 {}
#[doc = "UHCI_CONF0"]
pub mod uhci_conf0;
#[doc = "UHCI_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_int_raw](uhci_int_raw) module"]
pub type UHCI_INT_RAW = crate::Reg<u32, _UHCI_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_INT_RAW;
#[doc = "`read()` method returns [uhci_int_raw::R](uhci_int_raw::R) reader structure"]
impl crate::Readable for UHCI_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [uhci_int_raw::W](uhci_int_raw::W) writer structure"]
impl crate::Writable for UHCI_INT_RAW {}
#[doc = "UHCI_INT_RAW"]
pub mod uhci_int_raw;
#[doc = "UHCI_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_int_st](uhci_int_st) module"]
pub type UHCI_INT_ST = crate::Reg<u32, _UHCI_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_INT_ST;
#[doc = "`read()` method returns [uhci_int_st::R](uhci_int_st::R) reader structure"]
impl crate::Readable for UHCI_INT_ST {}
#[doc = "UHCI_INT_ST"]
pub mod uhci_int_st;
#[doc = "UHCI_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_int_ena](uhci_int_ena) module"]
pub type UHCI_INT_ENA = crate::Reg<u32, _UHCI_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_INT_ENA;
#[doc = "`read()` method returns [uhci_int_ena::R](uhci_int_ena::R) reader structure"]
impl crate::Readable for UHCI_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [uhci_int_ena::W](uhci_int_ena::W) writer structure"]
impl crate::Writable for UHCI_INT_ENA {}
#[doc = "UHCI_INT_ENA"]
pub mod uhci_int_ena;
#[doc = "UHCI_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_int_clr](uhci_int_clr) module"]
pub type UHCI_INT_CLR = crate::Reg<u32, _UHCI_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_INT_CLR;
#[doc = "`write(|w| ..)` method takes [uhci_int_clr::W](uhci_int_clr::W) writer structure"]
impl crate::Writable for UHCI_INT_CLR {}
#[doc = "UHCI_INT_CLR"]
pub mod uhci_int_clr;
#[doc = "UHCI_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_conf1](uhci_conf1) module"]
pub type UHCI_CONF1 = crate::Reg<u32, _UHCI_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_CONF1;
#[doc = "`read()` method returns [uhci_conf1::R](uhci_conf1::R) reader structure"]
impl crate::Readable for UHCI_CONF1 {}
#[doc = "`write(|w| ..)` method takes [uhci_conf1::W](uhci_conf1::W) writer structure"]
impl crate::Writable for UHCI_CONF1 {}
#[doc = "UHCI_CONF1"]
pub mod uhci_conf1;
#[doc = "UHCI_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_state0](uhci_state0) module"]
pub type UHCI_STATE0 = crate::Reg<u32, _UHCI_STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_STATE0;
#[doc = "`read()` method returns [uhci_state0::R](uhci_state0::R) reader structure"]
impl crate::Readable for UHCI_STATE0 {}
#[doc = "UHCI_STATE0"]
pub mod uhci_state0;
#[doc = "UHCI_STATE1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_state1](uhci_state1) module"]
pub type UHCI_STATE1 = crate::Reg<u32, _UHCI_STATE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_STATE1;
#[doc = "`read()` method returns [uhci_state1::R](uhci_state1::R) reader structure"]
impl crate::Readable for UHCI_STATE1 {}
#[doc = "UHCI_STATE1"]
pub mod uhci_state1;
#[doc = "UHCI_ESCAPE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_escape_conf](uhci_escape_conf) module"]
pub type UHCI_ESCAPE_CONF = crate::Reg<u32, _UHCI_ESCAPE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESCAPE_CONF;
#[doc = "`read()` method returns [uhci_escape_conf::R](uhci_escape_conf::R) reader structure"]
impl crate::Readable for UHCI_ESCAPE_CONF {}
#[doc = "`write(|w| ..)` method takes [uhci_escape_conf::W](uhci_escape_conf::W) writer structure"]
impl crate::Writable for UHCI_ESCAPE_CONF {}
#[doc = "UHCI_ESCAPE_CONF"]
pub mod uhci_escape_conf;
#[doc = "UHCI_HUNG_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_hung_conf](uhci_hung_conf) module"]
pub type UHCI_HUNG_CONF = crate::Reg<u32, _UHCI_HUNG_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_HUNG_CONF;
#[doc = "`read()` method returns [uhci_hung_conf::R](uhci_hung_conf::R) reader structure"]
impl crate::Readable for UHCI_HUNG_CONF {}
#[doc = "`write(|w| ..)` method takes [uhci_hung_conf::W](uhci_hung_conf::W) writer structure"]
impl crate::Writable for UHCI_HUNG_CONF {}
#[doc = "UHCI_HUNG_CONF"]
pub mod uhci_hung_conf;
#[doc = "UHCI_ACK_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_ack_num](uhci_ack_num) module"]
pub type UHCI_ACK_NUM = crate::Reg<u32, _UHCI_ACK_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ACK_NUM;
#[doc = "`read()` method returns [uhci_ack_num::R](uhci_ack_num::R) reader structure"]
impl crate::Readable for UHCI_ACK_NUM {}
#[doc = "`write(|w| ..)` method takes [uhci_ack_num::W](uhci_ack_num::W) writer structure"]
impl crate::Writable for UHCI_ACK_NUM {}
#[doc = "UHCI_ACK_NUM"]
pub mod uhci_ack_num;
#[doc = "UHCI_RX_HEAD\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_rx_head](uhci_rx_head) module"]
pub type UHCI_RX_HEAD = crate::Reg<u32, _UHCI_RX_HEAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_RX_HEAD;
#[doc = "`read()` method returns [uhci_rx_head::R](uhci_rx_head::R) reader structure"]
impl crate::Readable for UHCI_RX_HEAD {}
#[doc = "UHCI_RX_HEAD"]
pub mod uhci_rx_head;
#[doc = "UHCI_QUICK_SENT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_quick_sent](uhci_quick_sent) module"]
pub type UHCI_QUICK_SENT = crate::Reg<u32, _UHCI_QUICK_SENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_QUICK_SENT;
#[doc = "`read()` method returns [uhci_quick_sent::R](uhci_quick_sent::R) reader structure"]
impl crate::Readable for UHCI_QUICK_SENT {}
#[doc = "`write(|w| ..)` method takes [uhci_quick_sent::W](uhci_quick_sent::W) writer structure"]
impl crate::Writable for UHCI_QUICK_SENT {}
#[doc = "UHCI_QUICK_SENT"]
pub mod uhci_quick_sent;
#[doc = "UHCI_Q0_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q0_word0](uhci_q0_word0) module"]
pub type UHCI_Q0_WORD0 = crate::Reg<u32, _UHCI_Q0_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q0_WORD0;
#[doc = "`read()` method returns [uhci_q0_word0::R](uhci_q0_word0::R) reader structure"]
impl crate::Readable for UHCI_Q0_WORD0 {}
#[doc = "`write(|w| ..)` method takes [uhci_q0_word0::W](uhci_q0_word0::W) writer structure"]
impl crate::Writable for UHCI_Q0_WORD0 {}
#[doc = "UHCI_Q0_WORD0"]
pub mod uhci_q0_word0;
#[doc = "UHCI_Q0_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q0_word1](uhci_q0_word1) module"]
pub type UHCI_Q0_WORD1 = crate::Reg<u32, _UHCI_Q0_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q0_WORD1;
#[doc = "`read()` method returns [uhci_q0_word1::R](uhci_q0_word1::R) reader structure"]
impl crate::Readable for UHCI_Q0_WORD1 {}
#[doc = "`write(|w| ..)` method takes [uhci_q0_word1::W](uhci_q0_word1::W) writer structure"]
impl crate::Writable for UHCI_Q0_WORD1 {}
#[doc = "UHCI_Q0_WORD1"]
pub mod uhci_q0_word1;
#[doc = "UHCI_Q1_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q1_word0](uhci_q1_word0) module"]
pub type UHCI_Q1_WORD0 = crate::Reg<u32, _UHCI_Q1_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q1_WORD0;
#[doc = "`read()` method returns [uhci_q1_word0::R](uhci_q1_word0::R) reader structure"]
impl crate::Readable for UHCI_Q1_WORD0 {}
#[doc = "`write(|w| ..)` method takes [uhci_q1_word0::W](uhci_q1_word0::W) writer structure"]
impl crate::Writable for UHCI_Q1_WORD0 {}
#[doc = "UHCI_Q1_WORD0"]
pub mod uhci_q1_word0;
#[doc = "UHCI_Q1_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q1_word1](uhci_q1_word1) module"]
pub type UHCI_Q1_WORD1 = crate::Reg<u32, _UHCI_Q1_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q1_WORD1;
#[doc = "`read()` method returns [uhci_q1_word1::R](uhci_q1_word1::R) reader structure"]
impl crate::Readable for UHCI_Q1_WORD1 {}
#[doc = "`write(|w| ..)` method takes [uhci_q1_word1::W](uhci_q1_word1::W) writer structure"]
impl crate::Writable for UHCI_Q1_WORD1 {}
#[doc = "UHCI_Q1_WORD1"]
pub mod uhci_q1_word1;
#[doc = "UHCI_Q2_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q2_word0](uhci_q2_word0) module"]
pub type UHCI_Q2_WORD0 = crate::Reg<u32, _UHCI_Q2_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q2_WORD0;
#[doc = "`read()` method returns [uhci_q2_word0::R](uhci_q2_word0::R) reader structure"]
impl crate::Readable for UHCI_Q2_WORD0 {}
#[doc = "`write(|w| ..)` method takes [uhci_q2_word0::W](uhci_q2_word0::W) writer structure"]
impl crate::Writable for UHCI_Q2_WORD0 {}
#[doc = "UHCI_Q2_WORD0"]
pub mod uhci_q2_word0;
#[doc = "UHCI_Q2_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q2_word1](uhci_q2_word1) module"]
pub type UHCI_Q2_WORD1 = crate::Reg<u32, _UHCI_Q2_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q2_WORD1;
#[doc = "`read()` method returns [uhci_q2_word1::R](uhci_q2_word1::R) reader structure"]
impl crate::Readable for UHCI_Q2_WORD1 {}
#[doc = "`write(|w| ..)` method takes [uhci_q2_word1::W](uhci_q2_word1::W) writer structure"]
impl crate::Writable for UHCI_Q2_WORD1 {}
#[doc = "UHCI_Q2_WORD1"]
pub mod uhci_q2_word1;
#[doc = "UHCI_Q3_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q3_word0](uhci_q3_word0) module"]
pub type UHCI_Q3_WORD0 = crate::Reg<u32, _UHCI_Q3_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q3_WORD0;
#[doc = "`read()` method returns [uhci_q3_word0::R](uhci_q3_word0::R) reader structure"]
impl crate::Readable for UHCI_Q3_WORD0 {}
#[doc = "`write(|w| ..)` method takes [uhci_q3_word0::W](uhci_q3_word0::W) writer structure"]
impl crate::Writable for UHCI_Q3_WORD0 {}
#[doc = "UHCI_Q3_WORD0"]
pub mod uhci_q3_word0;
#[doc = "UHCI_Q3_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q3_word1](uhci_q3_word1) module"]
pub type UHCI_Q3_WORD1 = crate::Reg<u32, _UHCI_Q3_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q3_WORD1;
#[doc = "`read()` method returns [uhci_q3_word1::R](uhci_q3_word1::R) reader structure"]
impl crate::Readable for UHCI_Q3_WORD1 {}
#[doc = "`write(|w| ..)` method takes [uhci_q3_word1::W](uhci_q3_word1::W) writer structure"]
impl crate::Writable for UHCI_Q3_WORD1 {}
#[doc = "UHCI_Q3_WORD1"]
pub mod uhci_q3_word1;
#[doc = "UHCI_Q4_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q4_word0](uhci_q4_word0) module"]
pub type UHCI_Q4_WORD0 = crate::Reg<u32, _UHCI_Q4_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q4_WORD0;
#[doc = "`read()` method returns [uhci_q4_word0::R](uhci_q4_word0::R) reader structure"]
impl crate::Readable for UHCI_Q4_WORD0 {}
#[doc = "`write(|w| ..)` method takes [uhci_q4_word0::W](uhci_q4_word0::W) writer structure"]
impl crate::Writable for UHCI_Q4_WORD0 {}
#[doc = "UHCI_Q4_WORD0"]
pub mod uhci_q4_word0;
#[doc = "UHCI_Q4_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q4_word1](uhci_q4_word1) module"]
pub type UHCI_Q4_WORD1 = crate::Reg<u32, _UHCI_Q4_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q4_WORD1;
#[doc = "`read()` method returns [uhci_q4_word1::R](uhci_q4_word1::R) reader structure"]
impl crate::Readable for UHCI_Q4_WORD1 {}
#[doc = "`write(|w| ..)` method takes [uhci_q4_word1::W](uhci_q4_word1::W) writer structure"]
impl crate::Writable for UHCI_Q4_WORD1 {}
#[doc = "UHCI_Q4_WORD1"]
pub mod uhci_q4_word1;
#[doc = "UHCI_Q5_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q5_word0](uhci_q5_word0) module"]
pub type UHCI_Q5_WORD0 = crate::Reg<u32, _UHCI_Q5_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q5_WORD0;
#[doc = "`read()` method returns [uhci_q5_word0::R](uhci_q5_word0::R) reader structure"]
impl crate::Readable for UHCI_Q5_WORD0 {}
#[doc = "`write(|w| ..)` method takes [uhci_q5_word0::W](uhci_q5_word0::W) writer structure"]
impl crate::Writable for UHCI_Q5_WORD0 {}
#[doc = "UHCI_Q5_WORD0"]
pub mod uhci_q5_word0;
#[doc = "UHCI_Q5_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q5_word1](uhci_q5_word1) module"]
pub type UHCI_Q5_WORD1 = crate::Reg<u32, _UHCI_Q5_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q5_WORD1;
#[doc = "`read()` method returns [uhci_q5_word1::R](uhci_q5_word1::R) reader structure"]
impl crate::Readable for UHCI_Q5_WORD1 {}
#[doc = "`write(|w| ..)` method takes [uhci_q5_word1::W](uhci_q5_word1::W) writer structure"]
impl crate::Writable for UHCI_Q5_WORD1 {}
#[doc = "UHCI_Q5_WORD1"]
pub mod uhci_q5_word1;
#[doc = "UHCI_Q6_WORD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q6_word0](uhci_q6_word0) module"]
pub type UHCI_Q6_WORD0 = crate::Reg<u32, _UHCI_Q6_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q6_WORD0;
#[doc = "`read()` method returns [uhci_q6_word0::R](uhci_q6_word0::R) reader structure"]
impl crate::Readable for UHCI_Q6_WORD0 {}
#[doc = "`write(|w| ..)` method takes [uhci_q6_word0::W](uhci_q6_word0::W) writer structure"]
impl crate::Writable for UHCI_Q6_WORD0 {}
#[doc = "UHCI_Q6_WORD0"]
pub mod uhci_q6_word0;
#[doc = "UHCI_Q6_WORD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_q6_word1](uhci_q6_word1) module"]
pub type UHCI_Q6_WORD1 = crate::Reg<u32, _UHCI_Q6_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_Q6_WORD1;
#[doc = "`read()` method returns [uhci_q6_word1::R](uhci_q6_word1::R) reader structure"]
impl crate::Readable for UHCI_Q6_WORD1 {}
#[doc = "`write(|w| ..)` method takes [uhci_q6_word1::W](uhci_q6_word1::W) writer structure"]
impl crate::Writable for UHCI_Q6_WORD1 {}
#[doc = "UHCI_Q6_WORD1"]
pub mod uhci_q6_word1;
#[doc = "UHCI_ESC_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_esc_conf0](uhci_esc_conf0) module"]
pub type UHCI_ESC_CONF0 = crate::Reg<u32, _UHCI_ESC_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESC_CONF0;
#[doc = "`read()` method returns [uhci_esc_conf0::R](uhci_esc_conf0::R) reader structure"]
impl crate::Readable for UHCI_ESC_CONF0 {}
#[doc = "`write(|w| ..)` method takes [uhci_esc_conf0::W](uhci_esc_conf0::W) writer structure"]
impl crate::Writable for UHCI_ESC_CONF0 {}
#[doc = "UHCI_ESC_CONF0"]
pub mod uhci_esc_conf0;
#[doc = "UHCI_ESC_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_esc_conf1](uhci_esc_conf1) module"]
pub type UHCI_ESC_CONF1 = crate::Reg<u32, _UHCI_ESC_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESC_CONF1;
#[doc = "`read()` method returns [uhci_esc_conf1::R](uhci_esc_conf1::R) reader structure"]
impl crate::Readable for UHCI_ESC_CONF1 {}
#[doc = "`write(|w| ..)` method takes [uhci_esc_conf1::W](uhci_esc_conf1::W) writer structure"]
impl crate::Writable for UHCI_ESC_CONF1 {}
#[doc = "UHCI_ESC_CONF1"]
pub mod uhci_esc_conf1;
#[doc = "UHCI_ESC_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_esc_conf2](uhci_esc_conf2) module"]
pub type UHCI_ESC_CONF2 = crate::Reg<u32, _UHCI_ESC_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESC_CONF2;
#[doc = "`read()` method returns [uhci_esc_conf2::R](uhci_esc_conf2::R) reader structure"]
impl crate::Readable for UHCI_ESC_CONF2 {}
#[doc = "`write(|w| ..)` method takes [uhci_esc_conf2::W](uhci_esc_conf2::W) writer structure"]
impl crate::Writable for UHCI_ESC_CONF2 {}
#[doc = "UHCI_ESC_CONF2"]
pub mod uhci_esc_conf2;
#[doc = "UHCI_ESC_CONF3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_esc_conf3](uhci_esc_conf3) module"]
pub type UHCI_ESC_CONF3 = crate::Reg<u32, _UHCI_ESC_CONF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_ESC_CONF3;
#[doc = "`read()` method returns [uhci_esc_conf3::R](uhci_esc_conf3::R) reader structure"]
impl crate::Readable for UHCI_ESC_CONF3 {}
#[doc = "`write(|w| ..)` method takes [uhci_esc_conf3::W](uhci_esc_conf3::W) writer structure"]
impl crate::Writable for UHCI_ESC_CONF3 {}
#[doc = "UHCI_ESC_CONF3"]
pub mod uhci_esc_conf3;
#[doc = "UHCI_PKT_THRES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_pkt_thres](uhci_pkt_thres) module"]
pub type UHCI_PKT_THRES = crate::Reg<u32, _UHCI_PKT_THRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_PKT_THRES;
#[doc = "`read()` method returns [uhci_pkt_thres::R](uhci_pkt_thres::R) reader structure"]
impl crate::Readable for UHCI_PKT_THRES {}
#[doc = "`write(|w| ..)` method takes [uhci_pkt_thres::W](uhci_pkt_thres::W) writer structure"]
impl crate::Writable for UHCI_PKT_THRES {}
#[doc = "UHCI_PKT_THRES"]
pub mod uhci_pkt_thres;
#[doc = "UHCI_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhci_date](uhci_date) module"]
pub type UHCI_DATE = crate::Reg<u32, _UHCI_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UHCI_DATE;
#[doc = "`read()` method returns [uhci_date::R](uhci_date::R) reader structure"]
impl crate::Readable for UHCI_DATE {}
#[doc = "`write(|w| ..)` method takes [uhci_date::W](uhci_date::W) writer structure"]
impl crate::Writable for UHCI_DATE {}
#[doc = "UHCI_DATE"]
pub mod uhci_date;
