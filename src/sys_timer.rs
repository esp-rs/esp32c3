#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYS_TIMER_SYSTIMER_CONF"]
    pub sys_timer_systimer_conf: SYS_TIMER_SYSTIMER_CONF,
    #[doc = "0x04 - SYS_TIMER_SYSTIMER_UNIT0_OP"]
    pub sys_timer_systimer_unit0_op: SYS_TIMER_SYSTIMER_UNIT0_OP,
    #[doc = "0x08 - SYS_TIMER_SYSTIMER_UNIT1_OP"]
    pub sys_timer_systimer_unit1_op: SYS_TIMER_SYSTIMER_UNIT1_OP,
    #[doc = "0x0c - SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI"]
    pub sys_timer_systimer_unit0_load_hi: SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI,
    #[doc = "0x10 - SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO"]
    pub sys_timer_systimer_unit0_load_lo: SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO,
    #[doc = "0x14 - SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI"]
    pub sys_timer_systimer_unit1_load_hi: SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI,
    #[doc = "0x18 - SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO"]
    pub sys_timer_systimer_unit1_load_lo: SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO,
    #[doc = "0x1c - SYS_TIMER_SYSTIMER_TARGET0_HI"]
    pub sys_timer_systimer_target0_hi: SYS_TIMER_SYSTIMER_TARGET0_HI,
    #[doc = "0x20 - SYS_TIMER_SYSTIMER_TARGET0_LO"]
    pub sys_timer_systimer_target0_lo: SYS_TIMER_SYSTIMER_TARGET0_LO,
    #[doc = "0x24 - SYS_TIMER_SYSTIMER_TARGET1_HI"]
    pub sys_timer_systimer_target1_hi: SYS_TIMER_SYSTIMER_TARGET1_HI,
    #[doc = "0x28 - SYS_TIMER_SYSTIMER_TARGET1_LO"]
    pub sys_timer_systimer_target1_lo: SYS_TIMER_SYSTIMER_TARGET1_LO,
    #[doc = "0x2c - SYS_TIMER_SYSTIMER_TARGET2_HI"]
    pub sys_timer_systimer_target2_hi: SYS_TIMER_SYSTIMER_TARGET2_HI,
    #[doc = "0x30 - SYS_TIMER_SYSTIMER_TARGET2_LO"]
    pub sys_timer_systimer_target2_lo: SYS_TIMER_SYSTIMER_TARGET2_LO,
    #[doc = "0x34 - SYS_TIMER_SYSTIMER_TARGET0_CONF"]
    pub sys_timer_systimer_target0_conf: SYS_TIMER_SYSTIMER_TARGET0_CONF,
    #[doc = "0x38 - SYS_TIMER_SYSTIMER_TARGET1_CONF"]
    pub sys_timer_systimer_target1_conf: SYS_TIMER_SYSTIMER_TARGET1_CONF,
    #[doc = "0x3c - SYS_TIMER_SYSTIMER_TARGET2_CONF"]
    pub sys_timer_systimer_target2_conf: SYS_TIMER_SYSTIMER_TARGET2_CONF,
    #[doc = "0x40 - SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI"]
    pub sys_timer_systimer_unit0_value_hi: SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI,
    #[doc = "0x44 - SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO"]
    pub sys_timer_systimer_unit0_value_lo: SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO,
    #[doc = "0x48 - SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI"]
    pub sys_timer_systimer_unit1_value_hi: SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI,
    #[doc = "0x4c - SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO"]
    pub sys_timer_systimer_unit1_value_lo: SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO,
    #[doc = "0x50 - SYS_TIMER_SYSTIMER_COMP0_LOAD"]
    pub sys_timer_systimer_comp0_load: SYS_TIMER_SYSTIMER_COMP0_LOAD,
    #[doc = "0x54 - SYS_TIMER_SYSTIMER_COMP1_LOAD"]
    pub sys_timer_systimer_comp1_load: SYS_TIMER_SYSTIMER_COMP1_LOAD,
    #[doc = "0x58 - SYS_TIMER_SYSTIMER_COMP2_LOAD"]
    pub sys_timer_systimer_comp2_load: SYS_TIMER_SYSTIMER_COMP2_LOAD,
    #[doc = "0x5c - SYS_TIMER_SYSTIMER_UNIT0_LOAD"]
    pub sys_timer_systimer_unit0_load: SYS_TIMER_SYSTIMER_UNIT0_LOAD,
    #[doc = "0x60 - SYS_TIMER_SYSTIMER_UNIT1_LOAD"]
    pub sys_timer_systimer_unit1_load: SYS_TIMER_SYSTIMER_UNIT1_LOAD,
    #[doc = "0x64 - SYS_TIMER_SYSTIMER_INT_ENA"]
    pub sys_timer_systimer_int_ena: SYS_TIMER_SYSTIMER_INT_ENA,
    #[doc = "0x68 - SYS_TIMER_SYSTIMER_INT_RAW"]
    pub sys_timer_systimer_int_raw: SYS_TIMER_SYSTIMER_INT_RAW,
    #[doc = "0x6c - SYS_TIMER_SYSTIMER_INT_CLR"]
    pub sys_timer_systimer_int_clr: SYS_TIMER_SYSTIMER_INT_CLR,
    #[doc = "0x70 - SYS_TIMER_SYSTIMER_INT_ST"]
    pub sys_timer_systimer_int_st: SYS_TIMER_SYSTIMER_INT_ST,
    _reserved29: [u8; 136usize],
    #[doc = "0xfc - SYS_TIMER_SYSTIMER_DATE"]
    pub sys_timer_systimer_date: SYS_TIMER_SYSTIMER_DATE,
}
#[doc = "SYS_TIMER_SYSTIMER_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_conf](sys_timer_systimer_conf) module"]
pub type SYS_TIMER_SYSTIMER_CONF = crate::Reg<u32, _SYS_TIMER_SYSTIMER_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_CONF;
#[doc = "`read()` method returns [sys_timer_systimer_conf::R](sys_timer_systimer_conf::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_CONF {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_conf::W](sys_timer_systimer_conf::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_CONF {}
#[doc = "SYS_TIMER_SYSTIMER_CONF"]
pub mod sys_timer_systimer_conf;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_OP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit0_op](sys_timer_systimer_unit0_op) module"]
pub type SYS_TIMER_SYSTIMER_UNIT0_OP = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT0_OP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT0_OP;
#[doc = "`read()` method returns [sys_timer_systimer_unit0_op::R](sys_timer_systimer_unit0_op::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT0_OP {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_unit0_op::W](sys_timer_systimer_unit0_op::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_UNIT0_OP {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_OP"]
pub mod sys_timer_systimer_unit0_op;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_OP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit1_op](sys_timer_systimer_unit1_op) module"]
pub type SYS_TIMER_SYSTIMER_UNIT1_OP = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT1_OP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT1_OP;
#[doc = "`read()` method returns [sys_timer_systimer_unit1_op::R](sys_timer_systimer_unit1_op::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT1_OP {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_unit1_op::W](sys_timer_systimer_unit1_op::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_UNIT1_OP {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_OP"]
pub mod sys_timer_systimer_unit1_op;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit0_load_hi](sys_timer_systimer_unit0_load_hi) module"]
pub type SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI;
#[doc = "`read()` method returns [sys_timer_systimer_unit0_load_hi::R](sys_timer_systimer_unit0_load_hi::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_unit0_load_hi::W](sys_timer_systimer_unit0_load_hi::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI"]
pub mod sys_timer_systimer_unit0_load_hi;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit0_load_lo](sys_timer_systimer_unit0_load_lo) module"]
pub type SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO;
#[doc = "`read()` method returns [sys_timer_systimer_unit0_load_lo::R](sys_timer_systimer_unit0_load_lo::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_unit0_load_lo::W](sys_timer_systimer_unit0_load_lo::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO"]
pub mod sys_timer_systimer_unit0_load_lo;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit1_load_hi](sys_timer_systimer_unit1_load_hi) module"]
pub type SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI;
#[doc = "`read()` method returns [sys_timer_systimer_unit1_load_hi::R](sys_timer_systimer_unit1_load_hi::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_unit1_load_hi::W](sys_timer_systimer_unit1_load_hi::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI"]
pub mod sys_timer_systimer_unit1_load_hi;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit1_load_lo](sys_timer_systimer_unit1_load_lo) module"]
pub type SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO;
#[doc = "`read()` method returns [sys_timer_systimer_unit1_load_lo::R](sys_timer_systimer_unit1_load_lo::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_unit1_load_lo::W](sys_timer_systimer_unit1_load_lo::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO"]
pub mod sys_timer_systimer_unit1_load_lo;
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_HI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target0_hi](sys_timer_systimer_target0_hi) module"]
pub type SYS_TIMER_SYSTIMER_TARGET0_HI = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET0_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET0_HI;
#[doc = "`read()` method returns [sys_timer_systimer_target0_hi::R](sys_timer_systimer_target0_hi::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET0_HI {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target0_hi::W](sys_timer_systimer_target0_hi::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET0_HI {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_HI"]
pub mod sys_timer_systimer_target0_hi;
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_LO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target0_lo](sys_timer_systimer_target0_lo) module"]
pub type SYS_TIMER_SYSTIMER_TARGET0_LO = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET0_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET0_LO;
#[doc = "`read()` method returns [sys_timer_systimer_target0_lo::R](sys_timer_systimer_target0_lo::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET0_LO {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target0_lo::W](sys_timer_systimer_target0_lo::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET0_LO {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_LO"]
pub mod sys_timer_systimer_target0_lo;
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_HI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target1_hi](sys_timer_systimer_target1_hi) module"]
pub type SYS_TIMER_SYSTIMER_TARGET1_HI = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET1_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET1_HI;
#[doc = "`read()` method returns [sys_timer_systimer_target1_hi::R](sys_timer_systimer_target1_hi::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET1_HI {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target1_hi::W](sys_timer_systimer_target1_hi::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET1_HI {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_HI"]
pub mod sys_timer_systimer_target1_hi;
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_LO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target1_lo](sys_timer_systimer_target1_lo) module"]
pub type SYS_TIMER_SYSTIMER_TARGET1_LO = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET1_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET1_LO;
#[doc = "`read()` method returns [sys_timer_systimer_target1_lo::R](sys_timer_systimer_target1_lo::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET1_LO {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target1_lo::W](sys_timer_systimer_target1_lo::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET1_LO {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_LO"]
pub mod sys_timer_systimer_target1_lo;
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_HI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target2_hi](sys_timer_systimer_target2_hi) module"]
pub type SYS_TIMER_SYSTIMER_TARGET2_HI = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET2_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET2_HI;
#[doc = "`read()` method returns [sys_timer_systimer_target2_hi::R](sys_timer_systimer_target2_hi::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET2_HI {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target2_hi::W](sys_timer_systimer_target2_hi::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET2_HI {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_HI"]
pub mod sys_timer_systimer_target2_hi;
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_LO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target2_lo](sys_timer_systimer_target2_lo) module"]
pub type SYS_TIMER_SYSTIMER_TARGET2_LO = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET2_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET2_LO;
#[doc = "`read()` method returns [sys_timer_systimer_target2_lo::R](sys_timer_systimer_target2_lo::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET2_LO {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target2_lo::W](sys_timer_systimer_target2_lo::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET2_LO {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_LO"]
pub mod sys_timer_systimer_target2_lo;
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target0_conf](sys_timer_systimer_target0_conf) module"]
pub type SYS_TIMER_SYSTIMER_TARGET0_CONF = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET0_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET0_CONF;
#[doc = "`read()` method returns [sys_timer_systimer_target0_conf::R](sys_timer_systimer_target0_conf::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET0_CONF {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target0_conf::W](sys_timer_systimer_target0_conf::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET0_CONF {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_CONF"]
pub mod sys_timer_systimer_target0_conf;
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target1_conf](sys_timer_systimer_target1_conf) module"]
pub type SYS_TIMER_SYSTIMER_TARGET1_CONF = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET1_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET1_CONF;
#[doc = "`read()` method returns [sys_timer_systimer_target1_conf::R](sys_timer_systimer_target1_conf::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET1_CONF {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target1_conf::W](sys_timer_systimer_target1_conf::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET1_CONF {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_CONF"]
pub mod sys_timer_systimer_target1_conf;
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_target2_conf](sys_timer_systimer_target2_conf) module"]
pub type SYS_TIMER_SYSTIMER_TARGET2_CONF = crate::Reg<u32, _SYS_TIMER_SYSTIMER_TARGET2_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_TARGET2_CONF;
#[doc = "`read()` method returns [sys_timer_systimer_target2_conf::R](sys_timer_systimer_target2_conf::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_TARGET2_CONF {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_target2_conf::W](sys_timer_systimer_target2_conf::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_TARGET2_CONF {}
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_CONF"]
pub mod sys_timer_systimer_target2_conf;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit0_value_hi](sys_timer_systimer_unit0_value_hi) module"]
pub type SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI;
#[doc = "`read()` method returns [sys_timer_systimer_unit0_value_hi::R](sys_timer_systimer_unit0_value_hi::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI"]
pub mod sys_timer_systimer_unit0_value_hi;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit0_value_lo](sys_timer_systimer_unit0_value_lo) module"]
pub type SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO;
#[doc = "`read()` method returns [sys_timer_systimer_unit0_value_lo::R](sys_timer_systimer_unit0_value_lo::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO"]
pub mod sys_timer_systimer_unit0_value_lo;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit1_value_hi](sys_timer_systimer_unit1_value_hi) module"]
pub type SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI;
#[doc = "`read()` method returns [sys_timer_systimer_unit1_value_hi::R](sys_timer_systimer_unit1_value_hi::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI"]
pub mod sys_timer_systimer_unit1_value_hi;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit1_value_lo](sys_timer_systimer_unit1_value_lo) module"]
pub type SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO;
#[doc = "`read()` method returns [sys_timer_systimer_unit1_value_lo::R](sys_timer_systimer_unit1_value_lo::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO"]
pub mod sys_timer_systimer_unit1_value_lo;
#[doc = "SYS_TIMER_SYSTIMER_COMP0_LOAD\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_comp0_load](sys_timer_systimer_comp0_load) module"]
pub type SYS_TIMER_SYSTIMER_COMP0_LOAD = crate::Reg<u32, _SYS_TIMER_SYSTIMER_COMP0_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_COMP0_LOAD;
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_comp0_load::W](sys_timer_systimer_comp0_load::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_COMP0_LOAD {}
#[doc = "SYS_TIMER_SYSTIMER_COMP0_LOAD"]
pub mod sys_timer_systimer_comp0_load;
#[doc = "SYS_TIMER_SYSTIMER_COMP1_LOAD\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_comp1_load](sys_timer_systimer_comp1_load) module"]
pub type SYS_TIMER_SYSTIMER_COMP1_LOAD = crate::Reg<u32, _SYS_TIMER_SYSTIMER_COMP1_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_COMP1_LOAD;
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_comp1_load::W](sys_timer_systimer_comp1_load::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_COMP1_LOAD {}
#[doc = "SYS_TIMER_SYSTIMER_COMP1_LOAD"]
pub mod sys_timer_systimer_comp1_load;
#[doc = "SYS_TIMER_SYSTIMER_COMP2_LOAD\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_comp2_load](sys_timer_systimer_comp2_load) module"]
pub type SYS_TIMER_SYSTIMER_COMP2_LOAD = crate::Reg<u32, _SYS_TIMER_SYSTIMER_COMP2_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_COMP2_LOAD;
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_comp2_load::W](sys_timer_systimer_comp2_load::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_COMP2_LOAD {}
#[doc = "SYS_TIMER_SYSTIMER_COMP2_LOAD"]
pub mod sys_timer_systimer_comp2_load;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit0_load](sys_timer_systimer_unit0_load) module"]
pub type SYS_TIMER_SYSTIMER_UNIT0_LOAD = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT0_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT0_LOAD;
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_unit0_load::W](sys_timer_systimer_unit0_load::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_UNIT0_LOAD {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD"]
pub mod sys_timer_systimer_unit0_load;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_unit1_load](sys_timer_systimer_unit1_load) module"]
pub type SYS_TIMER_SYSTIMER_UNIT1_LOAD = crate::Reg<u32, _SYS_TIMER_SYSTIMER_UNIT1_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_UNIT1_LOAD;
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_unit1_load::W](sys_timer_systimer_unit1_load::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_UNIT1_LOAD {}
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD"]
pub mod sys_timer_systimer_unit1_load;
#[doc = "SYS_TIMER_SYSTIMER_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_int_ena](sys_timer_systimer_int_ena) module"]
pub type SYS_TIMER_SYSTIMER_INT_ENA = crate::Reg<u32, _SYS_TIMER_SYSTIMER_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_INT_ENA;
#[doc = "`read()` method returns [sys_timer_systimer_int_ena::R](sys_timer_systimer_int_ena::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_int_ena::W](sys_timer_systimer_int_ena::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_INT_ENA {}
#[doc = "SYS_TIMER_SYSTIMER_INT_ENA"]
pub mod sys_timer_systimer_int_ena;
#[doc = "SYS_TIMER_SYSTIMER_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_int_raw](sys_timer_systimer_int_raw) module"]
pub type SYS_TIMER_SYSTIMER_INT_RAW = crate::Reg<u32, _SYS_TIMER_SYSTIMER_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_INT_RAW;
#[doc = "`read()` method returns [sys_timer_systimer_int_raw::R](sys_timer_systimer_int_raw::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_int_raw::W](sys_timer_systimer_int_raw::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_INT_RAW {}
#[doc = "SYS_TIMER_SYSTIMER_INT_RAW"]
pub mod sys_timer_systimer_int_raw;
#[doc = "SYS_TIMER_SYSTIMER_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_int_clr](sys_timer_systimer_int_clr) module"]
pub type SYS_TIMER_SYSTIMER_INT_CLR = crate::Reg<u32, _SYS_TIMER_SYSTIMER_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_INT_CLR;
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_int_clr::W](sys_timer_systimer_int_clr::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_INT_CLR {}
#[doc = "SYS_TIMER_SYSTIMER_INT_CLR"]
pub mod sys_timer_systimer_int_clr;
#[doc = "SYS_TIMER_SYSTIMER_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_int_st](sys_timer_systimer_int_st) module"]
pub type SYS_TIMER_SYSTIMER_INT_ST = crate::Reg<u32, _SYS_TIMER_SYSTIMER_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_INT_ST;
#[doc = "`read()` method returns [sys_timer_systimer_int_st::R](sys_timer_systimer_int_st::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_INT_ST {}
#[doc = "SYS_TIMER_SYSTIMER_INT_ST"]
pub mod sys_timer_systimer_int_st;
#[doc = "SYS_TIMER_SYSTIMER_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_timer_systimer_date](sys_timer_systimer_date) module"]
pub type SYS_TIMER_SYSTIMER_DATE = crate::Reg<u32, _SYS_TIMER_SYSTIMER_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_TIMER_SYSTIMER_DATE;
#[doc = "`read()` method returns [sys_timer_systimer_date::R](sys_timer_systimer_date::R) reader structure"]
impl crate::Readable for SYS_TIMER_SYSTIMER_DATE {}
#[doc = "`write(|w| ..)` method takes [sys_timer_systimer_date::W](sys_timer_systimer_date::W) writer structure"]
impl crate::Writable for SYS_TIMER_SYSTIMER_DATE {}
#[doc = "SYS_TIMER_SYSTIMER_DATE"]
pub mod sys_timer_systimer_date;
