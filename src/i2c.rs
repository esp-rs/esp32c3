#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_SCL_LOW_PERIOD"]
    pub i2c_scl_low_period: I2C_SCL_LOW_PERIOD,
    #[doc = "0x04 - I2C_CTR"]
    pub i2c_ctr: I2C_CTR,
    #[doc = "0x08 - I2C_SR"]
    pub i2c_sr: I2C_SR,
    #[doc = "0x0c - I2C_TO"]
    pub i2c_to: I2C_TO,
    #[doc = "0x10 - I2C_SLAVE_ADDR"]
    pub i2c_slave_addr: I2C_SLAVE_ADDR,
    #[doc = "0x14 - I2C_FIFO_ST"]
    pub i2c_fifo_st: I2C_FIFO_ST,
    #[doc = "0x18 - I2C_FIFO_CONF"]
    pub i2c_fifo_conf: I2C_FIFO_CONF,
    #[doc = "0x1c - I2C_DATA"]
    pub i2c_data: I2C_DATA,
    #[doc = "0x20 - I2C_INT_RAW"]
    pub i2c_int_raw: I2C_INT_RAW,
    #[doc = "0x24 - I2C_INT_CLR"]
    pub i2c_int_clr: I2C_INT_CLR,
    #[doc = "0x28 - I2C_INT_ENA"]
    pub i2c_int_ena: I2C_INT_ENA,
    #[doc = "0x2c - I2C_INT_STATUS"]
    pub i2c_int_status: I2C_INT_STATUS,
    #[doc = "0x30 - I2C_SDA_HOLD"]
    pub i2c_sda_hold: I2C_SDA_HOLD,
    #[doc = "0x34 - I2C_SDA_SAMPLE"]
    pub i2c_sda_sample: I2C_SDA_SAMPLE,
    #[doc = "0x38 - I2C_SCL_HIGH_PERIOD"]
    pub i2c_scl_high_period: I2C_SCL_HIGH_PERIOD,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - I2C_SCL_START_HOLD"]
    pub i2c_scl_start_hold: I2C_SCL_START_HOLD,
    #[doc = "0x44 - I2C_SCL_RSTART_SETUP"]
    pub i2c_scl_rstart_setup: I2C_SCL_RSTART_SETUP,
    #[doc = "0x48 - I2C_SCL_STOP_HOLD"]
    pub i2c_scl_stop_hold: I2C_SCL_STOP_HOLD,
    #[doc = "0x4c - I2C_SCL_STOP_SETUP"]
    pub i2c_scl_stop_setup: I2C_SCL_STOP_SETUP,
    #[doc = "0x50 - I2C_FILTER_CFG"]
    pub i2c_filter_cfg: I2C_FILTER_CFG,
    #[doc = "0x54 - I2C_CLK_CONF"]
    pub i2c_clk_conf: I2C_CLK_CONF,
    #[doc = "0x58 - I2C_COMD0"]
    pub i2c_comd0: I2C_COMD0,
    #[doc = "0x5c - I2C_COMD1"]
    pub i2c_comd1: I2C_COMD1,
    #[doc = "0x60 - I2C_COMD2"]
    pub i2c_comd2: I2C_COMD2,
    #[doc = "0x64 - I2C_COMD3"]
    pub i2c_comd3: I2C_COMD3,
    #[doc = "0x68 - I2C_COMD4"]
    pub i2c_comd4: I2C_COMD4,
    #[doc = "0x6c - I2C_COMD5"]
    pub i2c_comd5: I2C_COMD5,
    #[doc = "0x70 - I2C_COMD6"]
    pub i2c_comd6: I2C_COMD6,
    #[doc = "0x74 - I2C_COMD7"]
    pub i2c_comd7: I2C_COMD7,
    #[doc = "0x78 - I2C_SCL_ST_TIME_OUT"]
    pub i2c_scl_st_time_out: I2C_SCL_ST_TIME_OUT,
    #[doc = "0x7c - I2C_SCL_MAIN_ST_TIME_OUT"]
    pub i2c_scl_main_st_time_out: I2C_SCL_MAIN_ST_TIME_OUT,
    #[doc = "0x80 - I2C_SCL_SP_CONF"]
    pub i2c_scl_sp_conf: I2C_SCL_SP_CONF,
    #[doc = "0x84 - I2C_SCL_STRETCH_CONF"]
    pub i2c_scl_stretch_conf: I2C_SCL_STRETCH_CONF,
    _reserved33: [u8; 112usize],
    #[doc = "0xf8 - I2C_DATE"]
    pub i2c_date: I2C_DATE,
}
#[doc = "I2C_SCL_LOW_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_low_period](i2c_scl_low_period) module"]
pub type I2C_SCL_LOW_PERIOD = crate::Reg<u32, _I2C_SCL_LOW_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_LOW_PERIOD;
#[doc = "`read()` method returns [i2c_scl_low_period::R](i2c_scl_low_period::R) reader structure"]
impl crate::Readable for I2C_SCL_LOW_PERIOD {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_low_period::W](i2c_scl_low_period::W) writer structure"]
impl crate::Writable for I2C_SCL_LOW_PERIOD {}
#[doc = "I2C_SCL_LOW_PERIOD"]
pub mod i2c_scl_low_period;
#[doc = "I2C_CTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctr](i2c_ctr) module"]
pub type I2C_CTR = crate::Reg<u32, _I2C_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CTR;
#[doc = "`read()` method returns [i2c_ctr::R](i2c_ctr::R) reader structure"]
impl crate::Readable for I2C_CTR {}
#[doc = "`write(|w| ..)` method takes [i2c_ctr::W](i2c_ctr::W) writer structure"]
impl crate::Writable for I2C_CTR {}
#[doc = "I2C_CTR"]
pub mod i2c_ctr;
#[doc = "I2C_SR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sr](i2c_sr) module"]
pub type I2C_SR = crate::Reg<u32, _I2C_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SR;
#[doc = "`read()` method returns [i2c_sr::R](i2c_sr::R) reader structure"]
impl crate::Readable for I2C_SR {}
#[doc = "I2C_SR"]
pub mod i2c_sr;
#[doc = "I2C_TO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_to](i2c_to) module"]
pub type I2C_TO = crate::Reg<u32, _I2C_TO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TO;
#[doc = "`read()` method returns [i2c_to::R](i2c_to::R) reader structure"]
impl crate::Readable for I2C_TO {}
#[doc = "`write(|w| ..)` method takes [i2c_to::W](i2c_to::W) writer structure"]
impl crate::Writable for I2C_TO {}
#[doc = "I2C_TO"]
pub mod i2c_to;
#[doc = "I2C_SLAVE_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_slave_addr](i2c_slave_addr) module"]
pub type I2C_SLAVE_ADDR = crate::Reg<u32, _I2C_SLAVE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SLAVE_ADDR;
#[doc = "`read()` method returns [i2c_slave_addr::R](i2c_slave_addr::R) reader structure"]
impl crate::Readable for I2C_SLAVE_ADDR {}
#[doc = "`write(|w| ..)` method takes [i2c_slave_addr::W](i2c_slave_addr::W) writer structure"]
impl crate::Writable for I2C_SLAVE_ADDR {}
#[doc = "I2C_SLAVE_ADDR"]
pub mod i2c_slave_addr;
#[doc = "I2C_FIFO_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_st](i2c_fifo_st) module"]
pub type I2C_FIFO_ST = crate::Reg<u32, _I2C_FIFO_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_FIFO_ST;
#[doc = "`read()` method returns [i2c_fifo_st::R](i2c_fifo_st::R) reader structure"]
impl crate::Readable for I2C_FIFO_ST {}
#[doc = "I2C_FIFO_ST"]
pub mod i2c_fifo_st;
#[doc = "I2C_FIFO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_conf](i2c_fifo_conf) module"]
pub type I2C_FIFO_CONF = crate::Reg<u32, _I2C_FIFO_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_FIFO_CONF;
#[doc = "`read()` method returns [i2c_fifo_conf::R](i2c_fifo_conf::R) reader structure"]
impl crate::Readable for I2C_FIFO_CONF {}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_conf::W](i2c_fifo_conf::W) writer structure"]
impl crate::Writable for I2C_FIFO_CONF {}
#[doc = "I2C_FIFO_CONF"]
pub mod i2c_fifo_conf;
#[doc = "I2C_DATA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_data](i2c_data) module"]
pub type I2C_DATA = crate::Reg<u32, _I2C_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_DATA;
#[doc = "`read()` method returns [i2c_data::R](i2c_data::R) reader structure"]
impl crate::Readable for I2C_DATA {}
#[doc = "I2C_DATA"]
pub mod i2c_data;
#[doc = "I2C_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_int_raw](i2c_int_raw) module"]
pub type I2C_INT_RAW = crate::Reg<u32, _I2C_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_RAW;
#[doc = "`read()` method returns [i2c_int_raw::R](i2c_int_raw::R) reader structure"]
impl crate::Readable for I2C_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [i2c_int_raw::W](i2c_int_raw::W) writer structure"]
impl crate::Writable for I2C_INT_RAW {}
#[doc = "I2C_INT_RAW"]
pub mod i2c_int_raw;
#[doc = "I2C_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_int_clr](i2c_int_clr) module"]
pub type I2C_INT_CLR = crate::Reg<u32, _I2C_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_CLR;
#[doc = "`write(|w| ..)` method takes [i2c_int_clr::W](i2c_int_clr::W) writer structure"]
impl crate::Writable for I2C_INT_CLR {}
#[doc = "I2C_INT_CLR"]
pub mod i2c_int_clr;
#[doc = "I2C_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_int_ena](i2c_int_ena) module"]
pub type I2C_INT_ENA = crate::Reg<u32, _I2C_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_ENA;
#[doc = "`read()` method returns [i2c_int_ena::R](i2c_int_ena::R) reader structure"]
impl crate::Readable for I2C_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [i2c_int_ena::W](i2c_int_ena::W) writer structure"]
impl crate::Writable for I2C_INT_ENA {}
#[doc = "I2C_INT_ENA"]
pub mod i2c_int_ena;
#[doc = "I2C_INT_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_int_status](i2c_int_status) module"]
pub type I2C_INT_STATUS = crate::Reg<u32, _I2C_INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_STATUS;
#[doc = "`read()` method returns [i2c_int_status::R](i2c_int_status::R) reader structure"]
impl crate::Readable for I2C_INT_STATUS {}
#[doc = "I2C_INT_STATUS"]
pub mod i2c_int_status;
#[doc = "I2C_SDA_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sda_hold](i2c_sda_hold) module"]
pub type I2C_SDA_HOLD = crate::Reg<u32, _I2C_SDA_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SDA_HOLD;
#[doc = "`read()` method returns [i2c_sda_hold::R](i2c_sda_hold::R) reader structure"]
impl crate::Readable for I2C_SDA_HOLD {}
#[doc = "`write(|w| ..)` method takes [i2c_sda_hold::W](i2c_sda_hold::W) writer structure"]
impl crate::Writable for I2C_SDA_HOLD {}
#[doc = "I2C_SDA_HOLD"]
pub mod i2c_sda_hold;
#[doc = "I2C_SDA_SAMPLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sda_sample](i2c_sda_sample) module"]
pub type I2C_SDA_SAMPLE = crate::Reg<u32, _I2C_SDA_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SDA_SAMPLE;
#[doc = "`read()` method returns [i2c_sda_sample::R](i2c_sda_sample::R) reader structure"]
impl crate::Readable for I2C_SDA_SAMPLE {}
#[doc = "`write(|w| ..)` method takes [i2c_sda_sample::W](i2c_sda_sample::W) writer structure"]
impl crate::Writable for I2C_SDA_SAMPLE {}
#[doc = "I2C_SDA_SAMPLE"]
pub mod i2c_sda_sample;
#[doc = "I2C_SCL_HIGH_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_high_period](i2c_scl_high_period) module"]
pub type I2C_SCL_HIGH_PERIOD = crate::Reg<u32, _I2C_SCL_HIGH_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_HIGH_PERIOD;
#[doc = "`read()` method returns [i2c_scl_high_period::R](i2c_scl_high_period::R) reader structure"]
impl crate::Readable for I2C_SCL_HIGH_PERIOD {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_high_period::W](i2c_scl_high_period::W) writer structure"]
impl crate::Writable for I2C_SCL_HIGH_PERIOD {}
#[doc = "I2C_SCL_HIGH_PERIOD"]
pub mod i2c_scl_high_period;
#[doc = "I2C_SCL_START_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_start_hold](i2c_scl_start_hold) module"]
pub type I2C_SCL_START_HOLD = crate::Reg<u32, _I2C_SCL_START_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_START_HOLD;
#[doc = "`read()` method returns [i2c_scl_start_hold::R](i2c_scl_start_hold::R) reader structure"]
impl crate::Readable for I2C_SCL_START_HOLD {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_start_hold::W](i2c_scl_start_hold::W) writer structure"]
impl crate::Writable for I2C_SCL_START_HOLD {}
#[doc = "I2C_SCL_START_HOLD"]
pub mod i2c_scl_start_hold;
#[doc = "I2C_SCL_RSTART_SETUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_rstart_setup](i2c_scl_rstart_setup) module"]
pub type I2C_SCL_RSTART_SETUP = crate::Reg<u32, _I2C_SCL_RSTART_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_RSTART_SETUP;
#[doc = "`read()` method returns [i2c_scl_rstart_setup::R](i2c_scl_rstart_setup::R) reader structure"]
impl crate::Readable for I2C_SCL_RSTART_SETUP {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_rstart_setup::W](i2c_scl_rstart_setup::W) writer structure"]
impl crate::Writable for I2C_SCL_RSTART_SETUP {}
#[doc = "I2C_SCL_RSTART_SETUP"]
pub mod i2c_scl_rstart_setup;
#[doc = "I2C_SCL_STOP_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_stop_hold](i2c_scl_stop_hold) module"]
pub type I2C_SCL_STOP_HOLD = crate::Reg<u32, _I2C_SCL_STOP_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_STOP_HOLD;
#[doc = "`read()` method returns [i2c_scl_stop_hold::R](i2c_scl_stop_hold::R) reader structure"]
impl crate::Readable for I2C_SCL_STOP_HOLD {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_stop_hold::W](i2c_scl_stop_hold::W) writer structure"]
impl crate::Writable for I2C_SCL_STOP_HOLD {}
#[doc = "I2C_SCL_STOP_HOLD"]
pub mod i2c_scl_stop_hold;
#[doc = "I2C_SCL_STOP_SETUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_stop_setup](i2c_scl_stop_setup) module"]
pub type I2C_SCL_STOP_SETUP = crate::Reg<u32, _I2C_SCL_STOP_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_STOP_SETUP;
#[doc = "`read()` method returns [i2c_scl_stop_setup::R](i2c_scl_stop_setup::R) reader structure"]
impl crate::Readable for I2C_SCL_STOP_SETUP {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_stop_setup::W](i2c_scl_stop_setup::W) writer structure"]
impl crate::Writable for I2C_SCL_STOP_SETUP {}
#[doc = "I2C_SCL_STOP_SETUP"]
pub mod i2c_scl_stop_setup;
#[doc = "I2C_FILTER_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_filter_cfg](i2c_filter_cfg) module"]
pub type I2C_FILTER_CFG = crate::Reg<u32, _I2C_FILTER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_FILTER_CFG;
#[doc = "`read()` method returns [i2c_filter_cfg::R](i2c_filter_cfg::R) reader structure"]
impl crate::Readable for I2C_FILTER_CFG {}
#[doc = "`write(|w| ..)` method takes [i2c_filter_cfg::W](i2c_filter_cfg::W) writer structure"]
impl crate::Writable for I2C_FILTER_CFG {}
#[doc = "I2C_FILTER_CFG"]
pub mod i2c_filter_cfg;
#[doc = "I2C_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_clk_conf](i2c_clk_conf) module"]
pub type I2C_CLK_CONF = crate::Reg<u32, _I2C_CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CLK_CONF;
#[doc = "`read()` method returns [i2c_clk_conf::R](i2c_clk_conf::R) reader structure"]
impl crate::Readable for I2C_CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [i2c_clk_conf::W](i2c_clk_conf::W) writer structure"]
impl crate::Writable for I2C_CLK_CONF {}
#[doc = "I2C_CLK_CONF"]
pub mod i2c_clk_conf;
#[doc = "I2C_COMD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd0](i2c_comd0) module"]
pub type I2C_COMD0 = crate::Reg<u32, _I2C_COMD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD0;
#[doc = "`read()` method returns [i2c_comd0::R](i2c_comd0::R) reader structure"]
impl crate::Readable for I2C_COMD0 {}
#[doc = "`write(|w| ..)` method takes [i2c_comd0::W](i2c_comd0::W) writer structure"]
impl crate::Writable for I2C_COMD0 {}
#[doc = "I2C_COMD0"]
pub mod i2c_comd0;
#[doc = "I2C_COMD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd1](i2c_comd1) module"]
pub type I2C_COMD1 = crate::Reg<u32, _I2C_COMD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD1;
#[doc = "`read()` method returns [i2c_comd1::R](i2c_comd1::R) reader structure"]
impl crate::Readable for I2C_COMD1 {}
#[doc = "`write(|w| ..)` method takes [i2c_comd1::W](i2c_comd1::W) writer structure"]
impl crate::Writable for I2C_COMD1 {}
#[doc = "I2C_COMD1"]
pub mod i2c_comd1;
#[doc = "I2C_COMD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd2](i2c_comd2) module"]
pub type I2C_COMD2 = crate::Reg<u32, _I2C_COMD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD2;
#[doc = "`read()` method returns [i2c_comd2::R](i2c_comd2::R) reader structure"]
impl crate::Readable for I2C_COMD2 {}
#[doc = "`write(|w| ..)` method takes [i2c_comd2::W](i2c_comd2::W) writer structure"]
impl crate::Writable for I2C_COMD2 {}
#[doc = "I2C_COMD2"]
pub mod i2c_comd2;
#[doc = "I2C_COMD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd3](i2c_comd3) module"]
pub type I2C_COMD3 = crate::Reg<u32, _I2C_COMD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD3;
#[doc = "`read()` method returns [i2c_comd3::R](i2c_comd3::R) reader structure"]
impl crate::Readable for I2C_COMD3 {}
#[doc = "`write(|w| ..)` method takes [i2c_comd3::W](i2c_comd3::W) writer structure"]
impl crate::Writable for I2C_COMD3 {}
#[doc = "I2C_COMD3"]
pub mod i2c_comd3;
#[doc = "I2C_COMD4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd4](i2c_comd4) module"]
pub type I2C_COMD4 = crate::Reg<u32, _I2C_COMD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD4;
#[doc = "`read()` method returns [i2c_comd4::R](i2c_comd4::R) reader structure"]
impl crate::Readable for I2C_COMD4 {}
#[doc = "`write(|w| ..)` method takes [i2c_comd4::W](i2c_comd4::W) writer structure"]
impl crate::Writable for I2C_COMD4 {}
#[doc = "I2C_COMD4"]
pub mod i2c_comd4;
#[doc = "I2C_COMD5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd5](i2c_comd5) module"]
pub type I2C_COMD5 = crate::Reg<u32, _I2C_COMD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD5;
#[doc = "`read()` method returns [i2c_comd5::R](i2c_comd5::R) reader structure"]
impl crate::Readable for I2C_COMD5 {}
#[doc = "`write(|w| ..)` method takes [i2c_comd5::W](i2c_comd5::W) writer structure"]
impl crate::Writable for I2C_COMD5 {}
#[doc = "I2C_COMD5"]
pub mod i2c_comd5;
#[doc = "I2C_COMD6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd6](i2c_comd6) module"]
pub type I2C_COMD6 = crate::Reg<u32, _I2C_COMD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD6;
#[doc = "`read()` method returns [i2c_comd6::R](i2c_comd6::R) reader structure"]
impl crate::Readable for I2C_COMD6 {}
#[doc = "`write(|w| ..)` method takes [i2c_comd6::W](i2c_comd6::W) writer structure"]
impl crate::Writable for I2C_COMD6 {}
#[doc = "I2C_COMD6"]
pub mod i2c_comd6;
#[doc = "I2C_COMD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd7](i2c_comd7) module"]
pub type I2C_COMD7 = crate::Reg<u32, _I2C_COMD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD7;
#[doc = "`read()` method returns [i2c_comd7::R](i2c_comd7::R) reader structure"]
impl crate::Readable for I2C_COMD7 {}
#[doc = "`write(|w| ..)` method takes [i2c_comd7::W](i2c_comd7::W) writer structure"]
impl crate::Writable for I2C_COMD7 {}
#[doc = "I2C_COMD7"]
pub mod i2c_comd7;
#[doc = "I2C_SCL_ST_TIME_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_st_time_out](i2c_scl_st_time_out) module"]
pub type I2C_SCL_ST_TIME_OUT = crate::Reg<u32, _I2C_SCL_ST_TIME_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_ST_TIME_OUT;
#[doc = "`read()` method returns [i2c_scl_st_time_out::R](i2c_scl_st_time_out::R) reader structure"]
impl crate::Readable for I2C_SCL_ST_TIME_OUT {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_st_time_out::W](i2c_scl_st_time_out::W) writer structure"]
impl crate::Writable for I2C_SCL_ST_TIME_OUT {}
#[doc = "I2C_SCL_ST_TIME_OUT"]
pub mod i2c_scl_st_time_out;
#[doc = "I2C_SCL_MAIN_ST_TIME_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_main_st_time_out](i2c_scl_main_st_time_out) module"]
pub type I2C_SCL_MAIN_ST_TIME_OUT = crate::Reg<u32, _I2C_SCL_MAIN_ST_TIME_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_MAIN_ST_TIME_OUT;
#[doc = "`read()` method returns [i2c_scl_main_st_time_out::R](i2c_scl_main_st_time_out::R) reader structure"]
impl crate::Readable for I2C_SCL_MAIN_ST_TIME_OUT {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_main_st_time_out::W](i2c_scl_main_st_time_out::W) writer structure"]
impl crate::Writable for I2C_SCL_MAIN_ST_TIME_OUT {}
#[doc = "I2C_SCL_MAIN_ST_TIME_OUT"]
pub mod i2c_scl_main_st_time_out;
#[doc = "I2C_SCL_SP_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_sp_conf](i2c_scl_sp_conf) module"]
pub type I2C_SCL_SP_CONF = crate::Reg<u32, _I2C_SCL_SP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_SP_CONF;
#[doc = "`read()` method returns [i2c_scl_sp_conf::R](i2c_scl_sp_conf::R) reader structure"]
impl crate::Readable for I2C_SCL_SP_CONF {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_sp_conf::W](i2c_scl_sp_conf::W) writer structure"]
impl crate::Writable for I2C_SCL_SP_CONF {}
#[doc = "I2C_SCL_SP_CONF"]
pub mod i2c_scl_sp_conf;
#[doc = "I2C_SCL_STRETCH_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_stretch_conf](i2c_scl_stretch_conf) module"]
pub type I2C_SCL_STRETCH_CONF = crate::Reg<u32, _I2C_SCL_STRETCH_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_STRETCH_CONF;
#[doc = "`read()` method returns [i2c_scl_stretch_conf::R](i2c_scl_stretch_conf::R) reader structure"]
impl crate::Readable for I2C_SCL_STRETCH_CONF {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_stretch_conf::W](i2c_scl_stretch_conf::W) writer structure"]
impl crate::Writable for I2C_SCL_STRETCH_CONF {}
#[doc = "I2C_SCL_STRETCH_CONF"]
pub mod i2c_scl_stretch_conf;
#[doc = "I2C_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_date](i2c_date) module"]
pub type I2C_DATE = crate::Reg<u32, _I2C_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_DATE;
#[doc = "`read()` method returns [i2c_date::R](i2c_date::R) reader structure"]
impl crate::Readable for I2C_DATE {}
#[doc = "`write(|w| ..)` method takes [i2c_date::W](i2c_date::W) writer structure"]
impl crate::Writable for I2C_DATE {}
#[doc = "I2C_DATE"]
pub mod i2c_date;
