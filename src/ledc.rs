#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LEDC_LSCH0_CONF0"]
    pub ledc_lsch0_conf0: LEDC_LSCH0_CONF0,
    #[doc = "0x04 - LEDC_LSCH0_HPOINT"]
    pub ledc_lsch0_hpoint: LEDC_LSCH0_HPOINT,
    #[doc = "0x08 - LEDC_LSCH0_DUTY"]
    pub ledc_lsch0_duty: LEDC_LSCH0_DUTY,
    #[doc = "0x0c - LEDC_LSCH0_CONF1"]
    pub ledc_lsch0_conf1: LEDC_LSCH0_CONF1,
    #[doc = "0x10 - LEDC_LSCH0_DUTY_R"]
    pub ledc_lsch0_duty_r: LEDC_LSCH0_DUTY_R,
    #[doc = "0x14 - LEDC_LSCH1_CONF0"]
    pub ledc_lsch1_conf0: LEDC_LSCH1_CONF0,
    #[doc = "0x18 - LEDC_LSCH1_HPOINT"]
    pub ledc_lsch1_hpoint: LEDC_LSCH1_HPOINT,
    #[doc = "0x1c - LEDC_LSCH1_DUTY"]
    pub ledc_lsch1_duty: LEDC_LSCH1_DUTY,
    #[doc = "0x20 - LEDC_LSCH1_CONF1"]
    pub ledc_lsch1_conf1: LEDC_LSCH1_CONF1,
    #[doc = "0x24 - LEDC_LSCH1_DUTY_R"]
    pub ledc_lsch1_duty_r: LEDC_LSCH1_DUTY_R,
    #[doc = "0x28 - LEDC_LSCH2_CONF0"]
    pub ledc_lsch2_conf0: LEDC_LSCH2_CONF0,
    #[doc = "0x2c - LEDC_LSCH2_HPOINT"]
    pub ledc_lsch2_hpoint: LEDC_LSCH2_HPOINT,
    #[doc = "0x30 - LEDC_LSCH2_DUTY"]
    pub ledc_lsch2_duty: LEDC_LSCH2_DUTY,
    #[doc = "0x34 - LEDC_LSCH2_CONF1"]
    pub ledc_lsch2_conf1: LEDC_LSCH2_CONF1,
    #[doc = "0x38 - LEDC_LSCH2_DUTY_R"]
    pub ledc_lsch2_duty_r: LEDC_LSCH2_DUTY_R,
    #[doc = "0x3c - LEDC_LSCH3_CONF0"]
    pub ledc_lsch3_conf0: LEDC_LSCH3_CONF0,
    #[doc = "0x40 - LEDC_LSCH3_HPOINT"]
    pub ledc_lsch3_hpoint: LEDC_LSCH3_HPOINT,
    #[doc = "0x44 - LEDC_LSCH3_DUTY"]
    pub ledc_lsch3_duty: LEDC_LSCH3_DUTY,
    #[doc = "0x48 - LEDC_LSCH3_CONF1"]
    pub ledc_lsch3_conf1: LEDC_LSCH3_CONF1,
    #[doc = "0x4c - LEDC_LSCH3_DUTY_R"]
    pub ledc_lsch3_duty_r: LEDC_LSCH3_DUTY_R,
    #[doc = "0x50 - LEDC_LSCH4_CONF0"]
    pub ledc_lsch4_conf0: LEDC_LSCH4_CONF0,
    #[doc = "0x54 - LEDC_LSCH4_HPOINT"]
    pub ledc_lsch4_hpoint: LEDC_LSCH4_HPOINT,
    #[doc = "0x58 - LEDC_LSCH4_DUTY"]
    pub ledc_lsch4_duty: LEDC_LSCH4_DUTY,
    #[doc = "0x5c - LEDC_LSCH4_CONF1"]
    pub ledc_lsch4_conf1: LEDC_LSCH4_CONF1,
    #[doc = "0x60 - LEDC_LSCH4_DUTY_R"]
    pub ledc_lsch4_duty_r: LEDC_LSCH4_DUTY_R,
    #[doc = "0x64 - LEDC_LSCH5_CONF0"]
    pub ledc_lsch5_conf0: LEDC_LSCH5_CONF0,
    #[doc = "0x68 - LEDC_LSCH5_HPOINT"]
    pub ledc_lsch5_hpoint: LEDC_LSCH5_HPOINT,
    #[doc = "0x6c - LEDC_LSCH5_DUTY"]
    pub ledc_lsch5_duty: LEDC_LSCH5_DUTY,
    #[doc = "0x70 - LEDC_LSCH5_CONF1"]
    pub ledc_lsch5_conf1: LEDC_LSCH5_CONF1,
    #[doc = "0x74 - LEDC_LSCH5_DUTY_R"]
    pub ledc_lsch5_duty_r: LEDC_LSCH5_DUTY_R,
    _reserved30: [u8; 40usize],
    #[doc = "0xa0 - LEDC_LSTIMER0_CONF"]
    pub ledc_lstimer0_conf: LEDC_LSTIMER0_CONF,
    #[doc = "0xa4 - LEDC_LSTIMER0_VALUE"]
    pub ledc_lstimer0_value: LEDC_LSTIMER0_VALUE,
    #[doc = "0xa8 - LEDC_LSTIMER1_CONF"]
    pub ledc_lstimer1_conf: LEDC_LSTIMER1_CONF,
    #[doc = "0xac - LEDC_LSTIMER1_VALUE"]
    pub ledc_lstimer1_value: LEDC_LSTIMER1_VALUE,
    #[doc = "0xb0 - LEDC_LSTIMER2_CONF"]
    pub ledc_lstimer2_conf: LEDC_LSTIMER2_CONF,
    #[doc = "0xb4 - LEDC_LSTIMER2_VALUE"]
    pub ledc_lstimer2_value: LEDC_LSTIMER2_VALUE,
    #[doc = "0xb8 - LEDC_LSTIMER3_CONF"]
    pub ledc_lstimer3_conf: LEDC_LSTIMER3_CONF,
    #[doc = "0xbc - LEDC_LSTIMER3_VALUE"]
    pub ledc_lstimer3_value: LEDC_LSTIMER3_VALUE,
    #[doc = "0xc0 - LEDC_INT_RAW"]
    pub ledc_int_raw: LEDC_INT_RAW,
    #[doc = "0xc4 - LEDC_INT_ST"]
    pub ledc_int_st: LEDC_INT_ST,
    #[doc = "0xc8 - LEDC_INT_ENA"]
    pub ledc_int_ena: LEDC_INT_ENA,
    #[doc = "0xcc - LEDC_INT_CLR"]
    pub ledc_int_clr: LEDC_INT_CLR,
    #[doc = "0xd0 - LEDC_CONF"]
    pub ledc_conf: LEDC_CONF,
    _reserved43: [u8; 40usize],
    #[doc = "0xfc - LEDC_DATE"]
    pub ledc_date: LEDC_DATE,
}
#[doc = "LEDC_LSCH0_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch0_conf0](ledc_lsch0_conf0) module"]
pub type LEDC_LSCH0_CONF0 = crate::Reg<u32, _LEDC_LSCH0_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_CONF0;
#[doc = "`read()` method returns [ledc_lsch0_conf0::R](ledc_lsch0_conf0::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_CONF0 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_conf0::W](ledc_lsch0_conf0::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_CONF0 {}
#[doc = "LEDC_LSCH0_CONF0"]
pub mod ledc_lsch0_conf0;
#[doc = "LEDC_LSCH0_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch0_hpoint](ledc_lsch0_hpoint) module"]
pub type LEDC_LSCH0_HPOINT = crate::Reg<u32, _LEDC_LSCH0_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_HPOINT;
#[doc = "`read()` method returns [ledc_lsch0_hpoint::R](ledc_lsch0_hpoint::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_HPOINT {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_hpoint::W](ledc_lsch0_hpoint::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_HPOINT {}
#[doc = "LEDC_LSCH0_HPOINT"]
pub mod ledc_lsch0_hpoint;
#[doc = "LEDC_LSCH0_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch0_duty](ledc_lsch0_duty) module"]
pub type LEDC_LSCH0_DUTY = crate::Reg<u32, _LEDC_LSCH0_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_DUTY;
#[doc = "`read()` method returns [ledc_lsch0_duty::R](ledc_lsch0_duty::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_DUTY {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_duty::W](ledc_lsch0_duty::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_DUTY {}
#[doc = "LEDC_LSCH0_DUTY"]
pub mod ledc_lsch0_duty;
#[doc = "LEDC_LSCH0_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch0_conf1](ledc_lsch0_conf1) module"]
pub type LEDC_LSCH0_CONF1 = crate::Reg<u32, _LEDC_LSCH0_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_CONF1;
#[doc = "`read()` method returns [ledc_lsch0_conf1::R](ledc_lsch0_conf1::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_CONF1 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch0_conf1::W](ledc_lsch0_conf1::W) writer structure"]
impl crate::Writable for LEDC_LSCH0_CONF1 {}
#[doc = "LEDC_LSCH0_CONF1"]
pub mod ledc_lsch0_conf1;
#[doc = "LEDC_LSCH0_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch0_duty_r](ledc_lsch0_duty_r) module"]
pub type LEDC_LSCH0_DUTY_R = crate::Reg<u32, _LEDC_LSCH0_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH0_DUTY_R;
#[doc = "`read()` method returns [ledc_lsch0_duty_r::R](ledc_lsch0_duty_r::R) reader structure"]
impl crate::Readable for LEDC_LSCH0_DUTY_R {}
#[doc = "LEDC_LSCH0_DUTY_R"]
pub mod ledc_lsch0_duty_r;
#[doc = "LEDC_LSCH1_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch1_conf0](ledc_lsch1_conf0) module"]
pub type LEDC_LSCH1_CONF0 = crate::Reg<u32, _LEDC_LSCH1_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_CONF0;
#[doc = "`read()` method returns [ledc_lsch1_conf0::R](ledc_lsch1_conf0::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_CONF0 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_conf0::W](ledc_lsch1_conf0::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_CONF0 {}
#[doc = "LEDC_LSCH1_CONF0"]
pub mod ledc_lsch1_conf0;
#[doc = "LEDC_LSCH1_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch1_hpoint](ledc_lsch1_hpoint) module"]
pub type LEDC_LSCH1_HPOINT = crate::Reg<u32, _LEDC_LSCH1_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_HPOINT;
#[doc = "`read()` method returns [ledc_lsch1_hpoint::R](ledc_lsch1_hpoint::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_HPOINT {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_hpoint::W](ledc_lsch1_hpoint::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_HPOINT {}
#[doc = "LEDC_LSCH1_HPOINT"]
pub mod ledc_lsch1_hpoint;
#[doc = "LEDC_LSCH1_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch1_duty](ledc_lsch1_duty) module"]
pub type LEDC_LSCH1_DUTY = crate::Reg<u32, _LEDC_LSCH1_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_DUTY;
#[doc = "`read()` method returns [ledc_lsch1_duty::R](ledc_lsch1_duty::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_DUTY {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_duty::W](ledc_lsch1_duty::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_DUTY {}
#[doc = "LEDC_LSCH1_DUTY"]
pub mod ledc_lsch1_duty;
#[doc = "LEDC_LSCH1_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch1_conf1](ledc_lsch1_conf1) module"]
pub type LEDC_LSCH1_CONF1 = crate::Reg<u32, _LEDC_LSCH1_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_CONF1;
#[doc = "`read()` method returns [ledc_lsch1_conf1::R](ledc_lsch1_conf1::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_CONF1 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch1_conf1::W](ledc_lsch1_conf1::W) writer structure"]
impl crate::Writable for LEDC_LSCH1_CONF1 {}
#[doc = "LEDC_LSCH1_CONF1"]
pub mod ledc_lsch1_conf1;
#[doc = "LEDC_LSCH1_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch1_duty_r](ledc_lsch1_duty_r) module"]
pub type LEDC_LSCH1_DUTY_R = crate::Reg<u32, _LEDC_LSCH1_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH1_DUTY_R;
#[doc = "`read()` method returns [ledc_lsch1_duty_r::R](ledc_lsch1_duty_r::R) reader structure"]
impl crate::Readable for LEDC_LSCH1_DUTY_R {}
#[doc = "LEDC_LSCH1_DUTY_R"]
pub mod ledc_lsch1_duty_r;
#[doc = "LEDC_LSCH2_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch2_conf0](ledc_lsch2_conf0) module"]
pub type LEDC_LSCH2_CONF0 = crate::Reg<u32, _LEDC_LSCH2_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_CONF0;
#[doc = "`read()` method returns [ledc_lsch2_conf0::R](ledc_lsch2_conf0::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_CONF0 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_conf0::W](ledc_lsch2_conf0::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_CONF0 {}
#[doc = "LEDC_LSCH2_CONF0"]
pub mod ledc_lsch2_conf0;
#[doc = "LEDC_LSCH2_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch2_hpoint](ledc_lsch2_hpoint) module"]
pub type LEDC_LSCH2_HPOINT = crate::Reg<u32, _LEDC_LSCH2_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_HPOINT;
#[doc = "`read()` method returns [ledc_lsch2_hpoint::R](ledc_lsch2_hpoint::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_HPOINT {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_hpoint::W](ledc_lsch2_hpoint::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_HPOINT {}
#[doc = "LEDC_LSCH2_HPOINT"]
pub mod ledc_lsch2_hpoint;
#[doc = "LEDC_LSCH2_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch2_duty](ledc_lsch2_duty) module"]
pub type LEDC_LSCH2_DUTY = crate::Reg<u32, _LEDC_LSCH2_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_DUTY;
#[doc = "`read()` method returns [ledc_lsch2_duty::R](ledc_lsch2_duty::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_DUTY {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_duty::W](ledc_lsch2_duty::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_DUTY {}
#[doc = "LEDC_LSCH2_DUTY"]
pub mod ledc_lsch2_duty;
#[doc = "LEDC_LSCH2_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch2_conf1](ledc_lsch2_conf1) module"]
pub type LEDC_LSCH2_CONF1 = crate::Reg<u32, _LEDC_LSCH2_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_CONF1;
#[doc = "`read()` method returns [ledc_lsch2_conf1::R](ledc_lsch2_conf1::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_CONF1 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch2_conf1::W](ledc_lsch2_conf1::W) writer structure"]
impl crate::Writable for LEDC_LSCH2_CONF1 {}
#[doc = "LEDC_LSCH2_CONF1"]
pub mod ledc_lsch2_conf1;
#[doc = "LEDC_LSCH2_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch2_duty_r](ledc_lsch2_duty_r) module"]
pub type LEDC_LSCH2_DUTY_R = crate::Reg<u32, _LEDC_LSCH2_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH2_DUTY_R;
#[doc = "`read()` method returns [ledc_lsch2_duty_r::R](ledc_lsch2_duty_r::R) reader structure"]
impl crate::Readable for LEDC_LSCH2_DUTY_R {}
#[doc = "LEDC_LSCH2_DUTY_R"]
pub mod ledc_lsch2_duty_r;
#[doc = "LEDC_LSCH3_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch3_conf0](ledc_lsch3_conf0) module"]
pub type LEDC_LSCH3_CONF0 = crate::Reg<u32, _LEDC_LSCH3_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_CONF0;
#[doc = "`read()` method returns [ledc_lsch3_conf0::R](ledc_lsch3_conf0::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_CONF0 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_conf0::W](ledc_lsch3_conf0::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_CONF0 {}
#[doc = "LEDC_LSCH3_CONF0"]
pub mod ledc_lsch3_conf0;
#[doc = "LEDC_LSCH3_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch3_hpoint](ledc_lsch3_hpoint) module"]
pub type LEDC_LSCH3_HPOINT = crate::Reg<u32, _LEDC_LSCH3_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_HPOINT;
#[doc = "`read()` method returns [ledc_lsch3_hpoint::R](ledc_lsch3_hpoint::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_HPOINT {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_hpoint::W](ledc_lsch3_hpoint::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_HPOINT {}
#[doc = "LEDC_LSCH3_HPOINT"]
pub mod ledc_lsch3_hpoint;
#[doc = "LEDC_LSCH3_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch3_duty](ledc_lsch3_duty) module"]
pub type LEDC_LSCH3_DUTY = crate::Reg<u32, _LEDC_LSCH3_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_DUTY;
#[doc = "`read()` method returns [ledc_lsch3_duty::R](ledc_lsch3_duty::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_DUTY {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_duty::W](ledc_lsch3_duty::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_DUTY {}
#[doc = "LEDC_LSCH3_DUTY"]
pub mod ledc_lsch3_duty;
#[doc = "LEDC_LSCH3_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch3_conf1](ledc_lsch3_conf1) module"]
pub type LEDC_LSCH3_CONF1 = crate::Reg<u32, _LEDC_LSCH3_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_CONF1;
#[doc = "`read()` method returns [ledc_lsch3_conf1::R](ledc_lsch3_conf1::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_CONF1 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch3_conf1::W](ledc_lsch3_conf1::W) writer structure"]
impl crate::Writable for LEDC_LSCH3_CONF1 {}
#[doc = "LEDC_LSCH3_CONF1"]
pub mod ledc_lsch3_conf1;
#[doc = "LEDC_LSCH3_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch3_duty_r](ledc_lsch3_duty_r) module"]
pub type LEDC_LSCH3_DUTY_R = crate::Reg<u32, _LEDC_LSCH3_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH3_DUTY_R;
#[doc = "`read()` method returns [ledc_lsch3_duty_r::R](ledc_lsch3_duty_r::R) reader structure"]
impl crate::Readable for LEDC_LSCH3_DUTY_R {}
#[doc = "LEDC_LSCH3_DUTY_R"]
pub mod ledc_lsch3_duty_r;
#[doc = "LEDC_LSCH4_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch4_conf0](ledc_lsch4_conf0) module"]
pub type LEDC_LSCH4_CONF0 = crate::Reg<u32, _LEDC_LSCH4_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_CONF0;
#[doc = "`read()` method returns [ledc_lsch4_conf0::R](ledc_lsch4_conf0::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_CONF0 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_conf0::W](ledc_lsch4_conf0::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_CONF0 {}
#[doc = "LEDC_LSCH4_CONF0"]
pub mod ledc_lsch4_conf0;
#[doc = "LEDC_LSCH4_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch4_hpoint](ledc_lsch4_hpoint) module"]
pub type LEDC_LSCH4_HPOINT = crate::Reg<u32, _LEDC_LSCH4_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_HPOINT;
#[doc = "`read()` method returns [ledc_lsch4_hpoint::R](ledc_lsch4_hpoint::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_HPOINT {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_hpoint::W](ledc_lsch4_hpoint::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_HPOINT {}
#[doc = "LEDC_LSCH4_HPOINT"]
pub mod ledc_lsch4_hpoint;
#[doc = "LEDC_LSCH4_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch4_duty](ledc_lsch4_duty) module"]
pub type LEDC_LSCH4_DUTY = crate::Reg<u32, _LEDC_LSCH4_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_DUTY;
#[doc = "`read()` method returns [ledc_lsch4_duty::R](ledc_lsch4_duty::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_DUTY {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_duty::W](ledc_lsch4_duty::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_DUTY {}
#[doc = "LEDC_LSCH4_DUTY"]
pub mod ledc_lsch4_duty;
#[doc = "LEDC_LSCH4_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch4_conf1](ledc_lsch4_conf1) module"]
pub type LEDC_LSCH4_CONF1 = crate::Reg<u32, _LEDC_LSCH4_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_CONF1;
#[doc = "`read()` method returns [ledc_lsch4_conf1::R](ledc_lsch4_conf1::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_CONF1 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch4_conf1::W](ledc_lsch4_conf1::W) writer structure"]
impl crate::Writable for LEDC_LSCH4_CONF1 {}
#[doc = "LEDC_LSCH4_CONF1"]
pub mod ledc_lsch4_conf1;
#[doc = "LEDC_LSCH4_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch4_duty_r](ledc_lsch4_duty_r) module"]
pub type LEDC_LSCH4_DUTY_R = crate::Reg<u32, _LEDC_LSCH4_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH4_DUTY_R;
#[doc = "`read()` method returns [ledc_lsch4_duty_r::R](ledc_lsch4_duty_r::R) reader structure"]
impl crate::Readable for LEDC_LSCH4_DUTY_R {}
#[doc = "LEDC_LSCH4_DUTY_R"]
pub mod ledc_lsch4_duty_r;
#[doc = "LEDC_LSCH5_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch5_conf0](ledc_lsch5_conf0) module"]
pub type LEDC_LSCH5_CONF0 = crate::Reg<u32, _LEDC_LSCH5_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_CONF0;
#[doc = "`read()` method returns [ledc_lsch5_conf0::R](ledc_lsch5_conf0::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_CONF0 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_conf0::W](ledc_lsch5_conf0::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_CONF0 {}
#[doc = "LEDC_LSCH5_CONF0"]
pub mod ledc_lsch5_conf0;
#[doc = "LEDC_LSCH5_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch5_hpoint](ledc_lsch5_hpoint) module"]
pub type LEDC_LSCH5_HPOINT = crate::Reg<u32, _LEDC_LSCH5_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_HPOINT;
#[doc = "`read()` method returns [ledc_lsch5_hpoint::R](ledc_lsch5_hpoint::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_HPOINT {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_hpoint::W](ledc_lsch5_hpoint::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_HPOINT {}
#[doc = "LEDC_LSCH5_HPOINT"]
pub mod ledc_lsch5_hpoint;
#[doc = "LEDC_LSCH5_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch5_duty](ledc_lsch5_duty) module"]
pub type LEDC_LSCH5_DUTY = crate::Reg<u32, _LEDC_LSCH5_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_DUTY;
#[doc = "`read()` method returns [ledc_lsch5_duty::R](ledc_lsch5_duty::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_DUTY {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_duty::W](ledc_lsch5_duty::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_DUTY {}
#[doc = "LEDC_LSCH5_DUTY"]
pub mod ledc_lsch5_duty;
#[doc = "LEDC_LSCH5_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch5_conf1](ledc_lsch5_conf1) module"]
pub type LEDC_LSCH5_CONF1 = crate::Reg<u32, _LEDC_LSCH5_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_CONF1;
#[doc = "`read()` method returns [ledc_lsch5_conf1::R](ledc_lsch5_conf1::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_CONF1 {}
#[doc = "`write(|w| ..)` method takes [ledc_lsch5_conf1::W](ledc_lsch5_conf1::W) writer structure"]
impl crate::Writable for LEDC_LSCH5_CONF1 {}
#[doc = "LEDC_LSCH5_CONF1"]
pub mod ledc_lsch5_conf1;
#[doc = "LEDC_LSCH5_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lsch5_duty_r](ledc_lsch5_duty_r) module"]
pub type LEDC_LSCH5_DUTY_R = crate::Reg<u32, _LEDC_LSCH5_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSCH5_DUTY_R;
#[doc = "`read()` method returns [ledc_lsch5_duty_r::R](ledc_lsch5_duty_r::R) reader structure"]
impl crate::Readable for LEDC_LSCH5_DUTY_R {}
#[doc = "LEDC_LSCH5_DUTY_R"]
pub mod ledc_lsch5_duty_r;
#[doc = "LEDC_LSTIMER0_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lstimer0_conf](ledc_lstimer0_conf) module"]
pub type LEDC_LSTIMER0_CONF = crate::Reg<u32, _LEDC_LSTIMER0_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER0_CONF;
#[doc = "`read()` method returns [ledc_lstimer0_conf::R](ledc_lstimer0_conf::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER0_CONF {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer0_conf::W](ledc_lstimer0_conf::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER0_CONF {}
#[doc = "LEDC_LSTIMER0_CONF"]
pub mod ledc_lstimer0_conf;
#[doc = "LEDC_LSTIMER0_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lstimer0_value](ledc_lstimer0_value) module"]
pub type LEDC_LSTIMER0_VALUE = crate::Reg<u32, _LEDC_LSTIMER0_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER0_VALUE;
#[doc = "`read()` method returns [ledc_lstimer0_value::R](ledc_lstimer0_value::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER0_VALUE {}
#[doc = "LEDC_LSTIMER0_VALUE"]
pub mod ledc_lstimer0_value;
#[doc = "LEDC_LSTIMER1_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lstimer1_conf](ledc_lstimer1_conf) module"]
pub type LEDC_LSTIMER1_CONF = crate::Reg<u32, _LEDC_LSTIMER1_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER1_CONF;
#[doc = "`read()` method returns [ledc_lstimer1_conf::R](ledc_lstimer1_conf::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER1_CONF {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer1_conf::W](ledc_lstimer1_conf::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER1_CONF {}
#[doc = "LEDC_LSTIMER1_CONF"]
pub mod ledc_lstimer1_conf;
#[doc = "LEDC_LSTIMER1_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lstimer1_value](ledc_lstimer1_value) module"]
pub type LEDC_LSTIMER1_VALUE = crate::Reg<u32, _LEDC_LSTIMER1_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER1_VALUE;
#[doc = "`read()` method returns [ledc_lstimer1_value::R](ledc_lstimer1_value::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER1_VALUE {}
#[doc = "LEDC_LSTIMER1_VALUE"]
pub mod ledc_lstimer1_value;
#[doc = "LEDC_LSTIMER2_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lstimer2_conf](ledc_lstimer2_conf) module"]
pub type LEDC_LSTIMER2_CONF = crate::Reg<u32, _LEDC_LSTIMER2_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER2_CONF;
#[doc = "`read()` method returns [ledc_lstimer2_conf::R](ledc_lstimer2_conf::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER2_CONF {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer2_conf::W](ledc_lstimer2_conf::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER2_CONF {}
#[doc = "LEDC_LSTIMER2_CONF"]
pub mod ledc_lstimer2_conf;
#[doc = "LEDC_LSTIMER2_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lstimer2_value](ledc_lstimer2_value) module"]
pub type LEDC_LSTIMER2_VALUE = crate::Reg<u32, _LEDC_LSTIMER2_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER2_VALUE;
#[doc = "`read()` method returns [ledc_lstimer2_value::R](ledc_lstimer2_value::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER2_VALUE {}
#[doc = "LEDC_LSTIMER2_VALUE"]
pub mod ledc_lstimer2_value;
#[doc = "LEDC_LSTIMER3_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lstimer3_conf](ledc_lstimer3_conf) module"]
pub type LEDC_LSTIMER3_CONF = crate::Reg<u32, _LEDC_LSTIMER3_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER3_CONF;
#[doc = "`read()` method returns [ledc_lstimer3_conf::R](ledc_lstimer3_conf::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER3_CONF {}
#[doc = "`write(|w| ..)` method takes [ledc_lstimer3_conf::W](ledc_lstimer3_conf::W) writer structure"]
impl crate::Writable for LEDC_LSTIMER3_CONF {}
#[doc = "LEDC_LSTIMER3_CONF"]
pub mod ledc_lstimer3_conf;
#[doc = "LEDC_LSTIMER3_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_lstimer3_value](ledc_lstimer3_value) module"]
pub type LEDC_LSTIMER3_VALUE = crate::Reg<u32, _LEDC_LSTIMER3_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_LSTIMER3_VALUE;
#[doc = "`read()` method returns [ledc_lstimer3_value::R](ledc_lstimer3_value::R) reader structure"]
impl crate::Readable for LEDC_LSTIMER3_VALUE {}
#[doc = "LEDC_LSTIMER3_VALUE"]
pub mod ledc_lstimer3_value;
#[doc = "LEDC_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_int_raw](ledc_int_raw) module"]
pub type LEDC_INT_RAW = crate::Reg<u32, _LEDC_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_INT_RAW;
#[doc = "`read()` method returns [ledc_int_raw::R](ledc_int_raw::R) reader structure"]
impl crate::Readable for LEDC_INT_RAW {}
#[doc = "LEDC_INT_RAW"]
pub mod ledc_int_raw;
#[doc = "LEDC_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_int_st](ledc_int_st) module"]
pub type LEDC_INT_ST = crate::Reg<u32, _LEDC_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_INT_ST;
#[doc = "`read()` method returns [ledc_int_st::R](ledc_int_st::R) reader structure"]
impl crate::Readable for LEDC_INT_ST {}
#[doc = "LEDC_INT_ST"]
pub mod ledc_int_st;
#[doc = "LEDC_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_int_ena](ledc_int_ena) module"]
pub type LEDC_INT_ENA = crate::Reg<u32, _LEDC_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_INT_ENA;
#[doc = "`read()` method returns [ledc_int_ena::R](ledc_int_ena::R) reader structure"]
impl crate::Readable for LEDC_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [ledc_int_ena::W](ledc_int_ena::W) writer structure"]
impl crate::Writable for LEDC_INT_ENA {}
#[doc = "LEDC_INT_ENA"]
pub mod ledc_int_ena;
#[doc = "LEDC_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_int_clr](ledc_int_clr) module"]
pub type LEDC_INT_CLR = crate::Reg<u32, _LEDC_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_INT_CLR;
#[doc = "`write(|w| ..)` method takes [ledc_int_clr::W](ledc_int_clr::W) writer structure"]
impl crate::Writable for LEDC_INT_CLR {}
#[doc = "LEDC_INT_CLR"]
pub mod ledc_int_clr;
#[doc = "LEDC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_conf](ledc_conf) module"]
pub type LEDC_CONF = crate::Reg<u32, _LEDC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_CONF;
#[doc = "`read()` method returns [ledc_conf::R](ledc_conf::R) reader structure"]
impl crate::Readable for LEDC_CONF {}
#[doc = "`write(|w| ..)` method takes [ledc_conf::W](ledc_conf::W) writer structure"]
impl crate::Writable for LEDC_CONF {}
#[doc = "LEDC_CONF"]
pub mod ledc_conf;
#[doc = "LEDC_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_date](ledc_date) module"]
pub type LEDC_DATE = crate::Reg<u32, _LEDC_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDC_DATE;
#[doc = "`read()` method returns [ledc_date::R](ledc_date::R) reader structure"]
impl crate::Readable for LEDC_DATE {}
#[doc = "`write(|w| ..)` method takes [ledc_date::W](ledc_date::W) writer structure"]
impl crate::Writable for LEDC_DATE {}
#[doc = "LEDC_DATE"]
pub mod ledc_date;
