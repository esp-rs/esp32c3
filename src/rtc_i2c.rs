#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_I2C_SCL_LOW_PERIOD"]
    pub rtc_i2c_scl_low_period: RTC_I2C_SCL_LOW_PERIOD,
    #[doc = "0x04 - RTC_I2C_CTRL"]
    pub rtc_i2c_ctrl: RTC_I2C_CTRL,
    #[doc = "0x08 - RTC_I2C_STATUS"]
    pub rtc_i2c_status: RTC_I2C_STATUS,
    #[doc = "0x0c - RTC_I2C_TIMEOUT"]
    pub rtc_i2c_timeout: RTC_I2C_TIMEOUT,
    #[doc = "0x10 - RTC_I2C_SLAVE_ADDR"]
    pub rtc_i2c_slave_addr: RTC_I2C_SLAVE_ADDR,
    #[doc = "0x14 - RTC_I2C_SCL_HIGH"]
    pub rtc_i2c_scl_high: RTC_I2C_SCL_HIGH,
    #[doc = "0x18 - RTC_I2C_SDA_DUTY"]
    pub rtc_i2c_sda_duty: RTC_I2C_SDA_DUTY,
    #[doc = "0x1c - RTC_I2C_SCL_START_PERIOD"]
    pub rtc_i2c_scl_start_period: RTC_I2C_SCL_START_PERIOD,
    #[doc = "0x20 - RTC_I2C_SCL_STOP_PERIOD"]
    pub rtc_i2c_scl_stop_period: RTC_I2C_SCL_STOP_PERIOD,
    #[doc = "0x24 - RTC_I2C_INT_CLR"]
    pub rtc_i2c_int_clr: RTC_I2C_INT_CLR,
    #[doc = "0x28 - RTC_I2C_INT_RAW"]
    pub rtc_i2c_int_raw: RTC_I2C_INT_RAW,
    #[doc = "0x2c - RTC_I2C_INT_ST"]
    pub rtc_i2c_int_st: RTC_I2C_INT_ST,
    #[doc = "0x30 - RTC_I2C_INT_ENA"]
    pub rtc_i2c_int_ena: RTC_I2C_INT_ENA,
    #[doc = "0x34 - RTC_I2C_DATA"]
    pub rtc_i2c_data: RTC_I2C_DATA,
    #[doc = "0x38 - RTC_I2C_CMD0"]
    pub rtc_i2c_cmd0: RTC_I2C_CMD0,
    #[doc = "0x3c - RTC_I2C_CMD1"]
    pub rtc_i2c_cmd1: RTC_I2C_CMD1,
    #[doc = "0x40 - RTC_I2C_CMD2"]
    pub rtc_i2c_cmd2: RTC_I2C_CMD2,
    #[doc = "0x44 - RTC_I2C_CMD3"]
    pub rtc_i2c_cmd3: RTC_I2C_CMD3,
    #[doc = "0x48 - RTC_I2C_CMD4"]
    pub rtc_i2c_cmd4: RTC_I2C_CMD4,
    #[doc = "0x4c - RTC_I2C_CMD5"]
    pub rtc_i2c_cmd5: RTC_I2C_CMD5,
    #[doc = "0x50 - RTC_I2C_CMD6"]
    pub rtc_i2c_cmd6: RTC_I2C_CMD6,
    #[doc = "0x54 - RTC_I2C_CMD7"]
    pub rtc_i2c_cmd7: RTC_I2C_CMD7,
    #[doc = "0x58 - RTC_I2C_CMD8"]
    pub rtc_i2c_cmd8: RTC_I2C_CMD8,
    #[doc = "0x5c - RTC_I2C_CMD9"]
    pub rtc_i2c_cmd9: RTC_I2C_CMD9,
    #[doc = "0x60 - RTC_I2C_CMD10"]
    pub rtc_i2c_cmd10: RTC_I2C_CMD10,
    #[doc = "0x64 - RTC_I2C_CMD11"]
    pub rtc_i2c_cmd11: RTC_I2C_CMD11,
    #[doc = "0x68 - RTC_I2C_CMD12"]
    pub rtc_i2c_cmd12: RTC_I2C_CMD12,
    #[doc = "0x6c - RTC_I2C_CMD13"]
    pub rtc_i2c_cmd13: RTC_I2C_CMD13,
    #[doc = "0x70 - RTC_I2C_CMD14"]
    pub rtc_i2c_cmd14: RTC_I2C_CMD14,
    #[doc = "0x74 - RTC_I2C_CMD15"]
    pub rtc_i2c_cmd15: RTC_I2C_CMD15,
    _reserved30: [u8; 132usize],
    #[doc = "0xfc - RTC_I2C_DATE"]
    pub rtc_i2c_date: RTC_I2C_DATE,
}
#[doc = "RTC_I2C_SCL_LOW_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_scl_low_period](rtc_i2c_scl_low_period) module"]
pub type RTC_I2C_SCL_LOW_PERIOD = crate::Reg<u32, _RTC_I2C_SCL_LOW_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SCL_LOW_PERIOD;
#[doc = "`read()` method returns [rtc_i2c_scl_low_period::R](rtc_i2c_scl_low_period::R) reader structure"]
impl crate::Readable for RTC_I2C_SCL_LOW_PERIOD {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_scl_low_period::W](rtc_i2c_scl_low_period::W) writer structure"]
impl crate::Writable for RTC_I2C_SCL_LOW_PERIOD {}
#[doc = "RTC_I2C_SCL_LOW_PERIOD"]
pub mod rtc_i2c_scl_low_period;
#[doc = "RTC_I2C_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_ctrl](rtc_i2c_ctrl) module"]
pub type RTC_I2C_CTRL = crate::Reg<u32, _RTC_I2C_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CTRL;
#[doc = "`read()` method returns [rtc_i2c_ctrl::R](rtc_i2c_ctrl::R) reader structure"]
impl crate::Readable for RTC_I2C_CTRL {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_ctrl::W](rtc_i2c_ctrl::W) writer structure"]
impl crate::Writable for RTC_I2C_CTRL {}
#[doc = "RTC_I2C_CTRL"]
pub mod rtc_i2c_ctrl;
#[doc = "RTC_I2C_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_status](rtc_i2c_status) module"]
pub type RTC_I2C_STATUS = crate::Reg<u32, _RTC_I2C_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_STATUS;
#[doc = "`read()` method returns [rtc_i2c_status::R](rtc_i2c_status::R) reader structure"]
impl crate::Readable for RTC_I2C_STATUS {}
#[doc = "RTC_I2C_STATUS"]
pub mod rtc_i2c_status;
#[doc = "RTC_I2C_TIMEOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_timeout](rtc_i2c_timeout) module"]
pub type RTC_I2C_TIMEOUT = crate::Reg<u32, _RTC_I2C_TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_TIMEOUT;
#[doc = "`read()` method returns [rtc_i2c_timeout::R](rtc_i2c_timeout::R) reader structure"]
impl crate::Readable for RTC_I2C_TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_timeout::W](rtc_i2c_timeout::W) writer structure"]
impl crate::Writable for RTC_I2C_TIMEOUT {}
#[doc = "RTC_I2C_TIMEOUT"]
pub mod rtc_i2c_timeout;
#[doc = "RTC_I2C_SLAVE_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_slave_addr](rtc_i2c_slave_addr) module"]
pub type RTC_I2C_SLAVE_ADDR = crate::Reg<u32, _RTC_I2C_SLAVE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SLAVE_ADDR;
#[doc = "`read()` method returns [rtc_i2c_slave_addr::R](rtc_i2c_slave_addr::R) reader structure"]
impl crate::Readable for RTC_I2C_SLAVE_ADDR {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_slave_addr::W](rtc_i2c_slave_addr::W) writer structure"]
impl crate::Writable for RTC_I2C_SLAVE_ADDR {}
#[doc = "RTC_I2C_SLAVE_ADDR"]
pub mod rtc_i2c_slave_addr;
#[doc = "RTC_I2C_SCL_HIGH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_scl_high](rtc_i2c_scl_high) module"]
pub type RTC_I2C_SCL_HIGH = crate::Reg<u32, _RTC_I2C_SCL_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SCL_HIGH;
#[doc = "`read()` method returns [rtc_i2c_scl_high::R](rtc_i2c_scl_high::R) reader structure"]
impl crate::Readable for RTC_I2C_SCL_HIGH {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_scl_high::W](rtc_i2c_scl_high::W) writer structure"]
impl crate::Writable for RTC_I2C_SCL_HIGH {}
#[doc = "RTC_I2C_SCL_HIGH"]
pub mod rtc_i2c_scl_high;
#[doc = "RTC_I2C_SDA_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_sda_duty](rtc_i2c_sda_duty) module"]
pub type RTC_I2C_SDA_DUTY = crate::Reg<u32, _RTC_I2C_SDA_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SDA_DUTY;
#[doc = "`read()` method returns [rtc_i2c_sda_duty::R](rtc_i2c_sda_duty::R) reader structure"]
impl crate::Readable for RTC_I2C_SDA_DUTY {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_sda_duty::W](rtc_i2c_sda_duty::W) writer structure"]
impl crate::Writable for RTC_I2C_SDA_DUTY {}
#[doc = "RTC_I2C_SDA_DUTY"]
pub mod rtc_i2c_sda_duty;
#[doc = "RTC_I2C_SCL_START_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_scl_start_period](rtc_i2c_scl_start_period) module"]
pub type RTC_I2C_SCL_START_PERIOD = crate::Reg<u32, _RTC_I2C_SCL_START_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SCL_START_PERIOD;
#[doc = "`read()` method returns [rtc_i2c_scl_start_period::R](rtc_i2c_scl_start_period::R) reader structure"]
impl crate::Readable for RTC_I2C_SCL_START_PERIOD {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_scl_start_period::W](rtc_i2c_scl_start_period::W) writer structure"]
impl crate::Writable for RTC_I2C_SCL_START_PERIOD {}
#[doc = "RTC_I2C_SCL_START_PERIOD"]
pub mod rtc_i2c_scl_start_period;
#[doc = "RTC_I2C_SCL_STOP_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_scl_stop_period](rtc_i2c_scl_stop_period) module"]
pub type RTC_I2C_SCL_STOP_PERIOD = crate::Reg<u32, _RTC_I2C_SCL_STOP_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_SCL_STOP_PERIOD;
#[doc = "`read()` method returns [rtc_i2c_scl_stop_period::R](rtc_i2c_scl_stop_period::R) reader structure"]
impl crate::Readable for RTC_I2C_SCL_STOP_PERIOD {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_scl_stop_period::W](rtc_i2c_scl_stop_period::W) writer structure"]
impl crate::Writable for RTC_I2C_SCL_STOP_PERIOD {}
#[doc = "RTC_I2C_SCL_STOP_PERIOD"]
pub mod rtc_i2c_scl_stop_period;
#[doc = "RTC_I2C_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_int_clr](rtc_i2c_int_clr) module"]
pub type RTC_I2C_INT_CLR = crate::Reg<u32, _RTC_I2C_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_INT_CLR;
#[doc = "`write(|w| ..)` method takes [rtc_i2c_int_clr::W](rtc_i2c_int_clr::W) writer structure"]
impl crate::Writable for RTC_I2C_INT_CLR {}
#[doc = "RTC_I2C_INT_CLR"]
pub mod rtc_i2c_int_clr;
#[doc = "RTC_I2C_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_int_raw](rtc_i2c_int_raw) module"]
pub type RTC_I2C_INT_RAW = crate::Reg<u32, _RTC_I2C_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_INT_RAW;
#[doc = "`read()` method returns [rtc_i2c_int_raw::R](rtc_i2c_int_raw::R) reader structure"]
impl crate::Readable for RTC_I2C_INT_RAW {}
#[doc = "RTC_I2C_INT_RAW"]
pub mod rtc_i2c_int_raw;
#[doc = "RTC_I2C_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_int_st](rtc_i2c_int_st) module"]
pub type RTC_I2C_INT_ST = crate::Reg<u32, _RTC_I2C_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_INT_ST;
#[doc = "`read()` method returns [rtc_i2c_int_st::R](rtc_i2c_int_st::R) reader structure"]
impl crate::Readable for RTC_I2C_INT_ST {}
#[doc = "RTC_I2C_INT_ST"]
pub mod rtc_i2c_int_st;
#[doc = "RTC_I2C_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_int_ena](rtc_i2c_int_ena) module"]
pub type RTC_I2C_INT_ENA = crate::Reg<u32, _RTC_I2C_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_INT_ENA;
#[doc = "`read()` method returns [rtc_i2c_int_ena::R](rtc_i2c_int_ena::R) reader structure"]
impl crate::Readable for RTC_I2C_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_int_ena::W](rtc_i2c_int_ena::W) writer structure"]
impl crate::Writable for RTC_I2C_INT_ENA {}
#[doc = "RTC_I2C_INT_ENA"]
pub mod rtc_i2c_int_ena;
#[doc = "RTC_I2C_DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_data](rtc_i2c_data) module"]
pub type RTC_I2C_DATA = crate::Reg<u32, _RTC_I2C_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_DATA;
#[doc = "`read()` method returns [rtc_i2c_data::R](rtc_i2c_data::R) reader structure"]
impl crate::Readable for RTC_I2C_DATA {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_data::W](rtc_i2c_data::W) writer structure"]
impl crate::Writable for RTC_I2C_DATA {}
#[doc = "RTC_I2C_DATA"]
pub mod rtc_i2c_data;
#[doc = "RTC_I2C_CMD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd0](rtc_i2c_cmd0) module"]
pub type RTC_I2C_CMD0 = crate::Reg<u32, _RTC_I2C_CMD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD0;
#[doc = "`read()` method returns [rtc_i2c_cmd0::R](rtc_i2c_cmd0::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD0 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd0::W](rtc_i2c_cmd0::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD0 {}
#[doc = "RTC_I2C_CMD0"]
pub mod rtc_i2c_cmd0;
#[doc = "RTC_I2C_CMD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd1](rtc_i2c_cmd1) module"]
pub type RTC_I2C_CMD1 = crate::Reg<u32, _RTC_I2C_CMD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD1;
#[doc = "`read()` method returns [rtc_i2c_cmd1::R](rtc_i2c_cmd1::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD1 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd1::W](rtc_i2c_cmd1::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD1 {}
#[doc = "RTC_I2C_CMD1"]
pub mod rtc_i2c_cmd1;
#[doc = "RTC_I2C_CMD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd2](rtc_i2c_cmd2) module"]
pub type RTC_I2C_CMD2 = crate::Reg<u32, _RTC_I2C_CMD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD2;
#[doc = "`read()` method returns [rtc_i2c_cmd2::R](rtc_i2c_cmd2::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD2 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd2::W](rtc_i2c_cmd2::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD2 {}
#[doc = "RTC_I2C_CMD2"]
pub mod rtc_i2c_cmd2;
#[doc = "RTC_I2C_CMD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd3](rtc_i2c_cmd3) module"]
pub type RTC_I2C_CMD3 = crate::Reg<u32, _RTC_I2C_CMD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD3;
#[doc = "`read()` method returns [rtc_i2c_cmd3::R](rtc_i2c_cmd3::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD3 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd3::W](rtc_i2c_cmd3::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD3 {}
#[doc = "RTC_I2C_CMD3"]
pub mod rtc_i2c_cmd3;
#[doc = "RTC_I2C_CMD4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd4](rtc_i2c_cmd4) module"]
pub type RTC_I2C_CMD4 = crate::Reg<u32, _RTC_I2C_CMD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD4;
#[doc = "`read()` method returns [rtc_i2c_cmd4::R](rtc_i2c_cmd4::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD4 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd4::W](rtc_i2c_cmd4::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD4 {}
#[doc = "RTC_I2C_CMD4"]
pub mod rtc_i2c_cmd4;
#[doc = "RTC_I2C_CMD5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd5](rtc_i2c_cmd5) module"]
pub type RTC_I2C_CMD5 = crate::Reg<u32, _RTC_I2C_CMD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD5;
#[doc = "`read()` method returns [rtc_i2c_cmd5::R](rtc_i2c_cmd5::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD5 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd5::W](rtc_i2c_cmd5::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD5 {}
#[doc = "RTC_I2C_CMD5"]
pub mod rtc_i2c_cmd5;
#[doc = "RTC_I2C_CMD6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd6](rtc_i2c_cmd6) module"]
pub type RTC_I2C_CMD6 = crate::Reg<u32, _RTC_I2C_CMD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD6;
#[doc = "`read()` method returns [rtc_i2c_cmd6::R](rtc_i2c_cmd6::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD6 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd6::W](rtc_i2c_cmd6::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD6 {}
#[doc = "RTC_I2C_CMD6"]
pub mod rtc_i2c_cmd6;
#[doc = "RTC_I2C_CMD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd7](rtc_i2c_cmd7) module"]
pub type RTC_I2C_CMD7 = crate::Reg<u32, _RTC_I2C_CMD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD7;
#[doc = "`read()` method returns [rtc_i2c_cmd7::R](rtc_i2c_cmd7::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD7 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd7::W](rtc_i2c_cmd7::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD7 {}
#[doc = "RTC_I2C_CMD7"]
pub mod rtc_i2c_cmd7;
#[doc = "RTC_I2C_CMD8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd8](rtc_i2c_cmd8) module"]
pub type RTC_I2C_CMD8 = crate::Reg<u32, _RTC_I2C_CMD8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD8;
#[doc = "`read()` method returns [rtc_i2c_cmd8::R](rtc_i2c_cmd8::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD8 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd8::W](rtc_i2c_cmd8::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD8 {}
#[doc = "RTC_I2C_CMD8"]
pub mod rtc_i2c_cmd8;
#[doc = "RTC_I2C_CMD9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd9](rtc_i2c_cmd9) module"]
pub type RTC_I2C_CMD9 = crate::Reg<u32, _RTC_I2C_CMD9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD9;
#[doc = "`read()` method returns [rtc_i2c_cmd9::R](rtc_i2c_cmd9::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD9 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd9::W](rtc_i2c_cmd9::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD9 {}
#[doc = "RTC_I2C_CMD9"]
pub mod rtc_i2c_cmd9;
#[doc = "RTC_I2C_CMD10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd10](rtc_i2c_cmd10) module"]
pub type RTC_I2C_CMD10 = crate::Reg<u32, _RTC_I2C_CMD10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD10;
#[doc = "`read()` method returns [rtc_i2c_cmd10::R](rtc_i2c_cmd10::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD10 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd10::W](rtc_i2c_cmd10::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD10 {}
#[doc = "RTC_I2C_CMD10"]
pub mod rtc_i2c_cmd10;
#[doc = "RTC_I2C_CMD11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd11](rtc_i2c_cmd11) module"]
pub type RTC_I2C_CMD11 = crate::Reg<u32, _RTC_I2C_CMD11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD11;
#[doc = "`read()` method returns [rtc_i2c_cmd11::R](rtc_i2c_cmd11::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD11 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd11::W](rtc_i2c_cmd11::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD11 {}
#[doc = "RTC_I2C_CMD11"]
pub mod rtc_i2c_cmd11;
#[doc = "RTC_I2C_CMD12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd12](rtc_i2c_cmd12) module"]
pub type RTC_I2C_CMD12 = crate::Reg<u32, _RTC_I2C_CMD12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD12;
#[doc = "`read()` method returns [rtc_i2c_cmd12::R](rtc_i2c_cmd12::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD12 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd12::W](rtc_i2c_cmd12::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD12 {}
#[doc = "RTC_I2C_CMD12"]
pub mod rtc_i2c_cmd12;
#[doc = "RTC_I2C_CMD13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd13](rtc_i2c_cmd13) module"]
pub type RTC_I2C_CMD13 = crate::Reg<u32, _RTC_I2C_CMD13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD13;
#[doc = "`read()` method returns [rtc_i2c_cmd13::R](rtc_i2c_cmd13::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD13 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd13::W](rtc_i2c_cmd13::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD13 {}
#[doc = "RTC_I2C_CMD13"]
pub mod rtc_i2c_cmd13;
#[doc = "RTC_I2C_CMD14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd14](rtc_i2c_cmd14) module"]
pub type RTC_I2C_CMD14 = crate::Reg<u32, _RTC_I2C_CMD14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD14;
#[doc = "`read()` method returns [rtc_i2c_cmd14::R](rtc_i2c_cmd14::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD14 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd14::W](rtc_i2c_cmd14::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD14 {}
#[doc = "RTC_I2C_CMD14"]
pub mod rtc_i2c_cmd14;
#[doc = "RTC_I2C_CMD15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_cmd15](rtc_i2c_cmd15) module"]
pub type RTC_I2C_CMD15 = crate::Reg<u32, _RTC_I2C_CMD15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_CMD15;
#[doc = "`read()` method returns [rtc_i2c_cmd15::R](rtc_i2c_cmd15::R) reader structure"]
impl crate::Readable for RTC_I2C_CMD15 {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_cmd15::W](rtc_i2c_cmd15::W) writer structure"]
impl crate::Writable for RTC_I2C_CMD15 {}
#[doc = "RTC_I2C_CMD15"]
pub mod rtc_i2c_cmd15;
#[doc = "RTC_I2C_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_i2c_date](rtc_i2c_date) module"]
pub type RTC_I2C_DATE = crate::Reg<u32, _RTC_I2C_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_I2C_DATE;
#[doc = "`read()` method returns [rtc_i2c_date::R](rtc_i2c_date::R) reader structure"]
impl crate::Readable for RTC_I2C_DATE {}
#[doc = "`write(|w| ..)` method takes [rtc_i2c_date::W](rtc_i2c_date::W) writer structure"]
impl crate::Writable for RTC_I2C_DATE {}
#[doc = "RTC_I2C_DATE"]
pub mod rtc_i2c_date;
