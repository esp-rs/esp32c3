#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_CMD"]
    pub spi_cmd: SPI_CMD,
    #[doc = "0x04 - SPI_ADDR"]
    pub spi_addr: SPI_ADDR,
    #[doc = "0x08 - SPI_CTRL"]
    pub spi_ctrl: SPI_CTRL,
    #[doc = "0x0c - SPI_CLOCK"]
    pub spi_clock: SPI_CLOCK,
    #[doc = "0x10 - SPI_USER"]
    pub spi_user: SPI_USER,
    #[doc = "0x14 - SPI_USER1"]
    pub spi_user1: SPI_USER1,
    #[doc = "0x18 - SPI_USER2"]
    pub spi_user2: SPI_USER2,
    #[doc = "0x1c - SPI_MS_DLEN"]
    pub spi_ms_dlen: SPI_MS_DLEN,
    #[doc = "0x20 - SPI_MISC"]
    pub spi_misc: SPI_MISC,
    #[doc = "0x24 - SPI_DIN_MODE"]
    pub spi_din_mode: SPI_DIN_MODE,
    #[doc = "0x28 - SPI_DIN_NUM"]
    pub spi_din_num: SPI_DIN_NUM,
    #[doc = "0x2c - SPI_DOUT_MODE"]
    pub spi_dout_mode: SPI_DOUT_MODE,
    #[doc = "0x30 - SPI_DMA_CONF"]
    pub spi_dma_conf: SPI_DMA_CONF,
    #[doc = "0x34 - SPI_DMA_INT_ENA"]
    pub spi_dma_int_ena: SPI_DMA_INT_ENA,
    #[doc = "0x38 - SPI_DMA_INT_CLR"]
    pub spi_dma_int_clr: SPI_DMA_INT_CLR,
    #[doc = "0x3c - SPI_DMA_INT_RAW"]
    pub spi_dma_int_raw: SPI_DMA_INT_RAW,
    #[doc = "0x40 - SPI_DMA_INT_ST"]
    pub spi_dma_int_st: SPI_DMA_INT_ST,
    _reserved17: [u8; 84usize],
    #[doc = "0x98 - SPI_W0"]
    pub spi_w0: SPI_W0,
    #[doc = "0x9c - SPI_W1"]
    pub spi_w1: SPI_W1,
    #[doc = "0xa0 - SPI_W2"]
    pub spi_w2: SPI_W2,
    #[doc = "0xa4 - SPI_W3"]
    pub spi_w3: SPI_W3,
    #[doc = "0xa8 - SPI_W4"]
    pub spi_w4: SPI_W4,
    #[doc = "0xac - SPI_W5"]
    pub spi_w5: SPI_W5,
    #[doc = "0xb0 - SPI_W6"]
    pub spi_w6: SPI_W6,
    #[doc = "0xb4 - SPI_W7"]
    pub spi_w7: SPI_W7,
    #[doc = "0xb8 - SPI_W8"]
    pub spi_w8: SPI_W8,
    #[doc = "0xbc - SPI_W9"]
    pub spi_w9: SPI_W9,
    #[doc = "0xc0 - SPI_W10"]
    pub spi_w10: SPI_W10,
    #[doc = "0xc4 - SPI_W11"]
    pub spi_w11: SPI_W11,
    #[doc = "0xc8 - SPI_W12"]
    pub spi_w12: SPI_W12,
    #[doc = "0xcc - SPI_W13"]
    pub spi_w13: SPI_W13,
    #[doc = "0xd0 - SPI_W14"]
    pub spi_w14: SPI_W14,
    #[doc = "0xd4 - SPI_W15"]
    pub spi_w15: SPI_W15,
    _reserved33: [u8; 8usize],
    #[doc = "0xe0 - SPI_SLAVE"]
    pub spi_slave: SPI_SLAVE,
    #[doc = "0xe4 - SPI_SLAVE1"]
    pub spi_slave1: SPI_SLAVE1,
    #[doc = "0xe8 - SPI_CLK_GATE"]
    pub spi_clk_gate: SPI_CLK_GATE,
    _reserved36: [u8; 4usize],
    #[doc = "0xf0 - SPI_DATE"]
    pub spi_date: SPI_DATE,
}
#[doc = "SPI_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cmd](spi_cmd) module"]
pub type SPI_CMD = crate::Reg<u32, _SPI_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CMD;
#[doc = "`read()` method returns [spi_cmd::R](spi_cmd::R) reader structure"]
impl crate::Readable for SPI_CMD {}
#[doc = "`write(|w| ..)` method takes [spi_cmd::W](spi_cmd::W) writer structure"]
impl crate::Writable for SPI_CMD {}
#[doc = "SPI_CMD"]
pub mod spi_cmd;
#[doc = "SPI_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_addr](spi_addr) module"]
pub type SPI_ADDR = crate::Reg<u32, _SPI_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_ADDR;
#[doc = "`read()` method returns [spi_addr::R](spi_addr::R) reader structure"]
impl crate::Readable for SPI_ADDR {}
#[doc = "`write(|w| ..)` method takes [spi_addr::W](spi_addr::W) writer structure"]
impl crate::Writable for SPI_ADDR {}
#[doc = "SPI_ADDR"]
pub mod spi_addr;
#[doc = "SPI_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl](spi_ctrl) module"]
pub type SPI_CTRL = crate::Reg<u32, _SPI_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CTRL;
#[doc = "`read()` method returns [spi_ctrl::R](spi_ctrl::R) reader structure"]
impl crate::Readable for SPI_CTRL {}
#[doc = "`write(|w| ..)` method takes [spi_ctrl::W](spi_ctrl::W) writer structure"]
impl crate::Writable for SPI_CTRL {}
#[doc = "SPI_CTRL"]
pub mod spi_ctrl;
#[doc = "SPI_CLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_clock](spi_clock) module"]
pub type SPI_CLOCK = crate::Reg<u32, _SPI_CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CLOCK;
#[doc = "`read()` method returns [spi_clock::R](spi_clock::R) reader structure"]
impl crate::Readable for SPI_CLOCK {}
#[doc = "`write(|w| ..)` method takes [spi_clock::W](spi_clock::W) writer structure"]
impl crate::Writable for SPI_CLOCK {}
#[doc = "SPI_CLOCK"]
pub mod spi_clock;
#[doc = "SPI_USER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_user](spi_user) module"]
pub type SPI_USER = crate::Reg<u32, _SPI_USER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_USER;
#[doc = "`read()` method returns [spi_user::R](spi_user::R) reader structure"]
impl crate::Readable for SPI_USER {}
#[doc = "`write(|w| ..)` method takes [spi_user::W](spi_user::W) writer structure"]
impl crate::Writable for SPI_USER {}
#[doc = "SPI_USER"]
pub mod spi_user;
#[doc = "SPI_USER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_user1](spi_user1) module"]
pub type SPI_USER1 = crate::Reg<u32, _SPI_USER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_USER1;
#[doc = "`read()` method returns [spi_user1::R](spi_user1::R) reader structure"]
impl crate::Readable for SPI_USER1 {}
#[doc = "`write(|w| ..)` method takes [spi_user1::W](spi_user1::W) writer structure"]
impl crate::Writable for SPI_USER1 {}
#[doc = "SPI_USER1"]
pub mod spi_user1;
#[doc = "SPI_USER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_user2](spi_user2) module"]
pub type SPI_USER2 = crate::Reg<u32, _SPI_USER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_USER2;
#[doc = "`read()` method returns [spi_user2::R](spi_user2::R) reader structure"]
impl crate::Readable for SPI_USER2 {}
#[doc = "`write(|w| ..)` method takes [spi_user2::W](spi_user2::W) writer structure"]
impl crate::Writable for SPI_USER2 {}
#[doc = "SPI_USER2"]
pub mod spi_user2;
#[doc = "SPI_MS_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ms_dlen](spi_ms_dlen) module"]
pub type SPI_MS_DLEN = crate::Reg<u32, _SPI_MS_DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MS_DLEN;
#[doc = "`read()` method returns [spi_ms_dlen::R](spi_ms_dlen::R) reader structure"]
impl crate::Readable for SPI_MS_DLEN {}
#[doc = "`write(|w| ..)` method takes [spi_ms_dlen::W](spi_ms_dlen::W) writer structure"]
impl crate::Writable for SPI_MS_DLEN {}
#[doc = "SPI_MS_DLEN"]
pub mod spi_ms_dlen;
#[doc = "SPI_MISC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_misc](spi_misc) module"]
pub type SPI_MISC = crate::Reg<u32, _SPI_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MISC;
#[doc = "`read()` method returns [spi_misc::R](spi_misc::R) reader structure"]
impl crate::Readable for SPI_MISC {}
#[doc = "`write(|w| ..)` method takes [spi_misc::W](spi_misc::W) writer structure"]
impl crate::Writable for SPI_MISC {}
#[doc = "SPI_MISC"]
pub mod spi_misc;
#[doc = "SPI_DIN_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_din_mode](spi_din_mode) module"]
pub type SPI_DIN_MODE = crate::Reg<u32, _SPI_DIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DIN_MODE;
#[doc = "`read()` method returns [spi_din_mode::R](spi_din_mode::R) reader structure"]
impl crate::Readable for SPI_DIN_MODE {}
#[doc = "`write(|w| ..)` method takes [spi_din_mode::W](spi_din_mode::W) writer structure"]
impl crate::Writable for SPI_DIN_MODE {}
#[doc = "SPI_DIN_MODE"]
pub mod spi_din_mode;
#[doc = "SPI_DIN_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_din_num](spi_din_num) module"]
pub type SPI_DIN_NUM = crate::Reg<u32, _SPI_DIN_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DIN_NUM;
#[doc = "`read()` method returns [spi_din_num::R](spi_din_num::R) reader structure"]
impl crate::Readable for SPI_DIN_NUM {}
#[doc = "`write(|w| ..)` method takes [spi_din_num::W](spi_din_num::W) writer structure"]
impl crate::Writable for SPI_DIN_NUM {}
#[doc = "SPI_DIN_NUM"]
pub mod spi_din_num;
#[doc = "SPI_DOUT_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dout_mode](spi_dout_mode) module"]
pub type SPI_DOUT_MODE = crate::Reg<u32, _SPI_DOUT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DOUT_MODE;
#[doc = "`read()` method returns [spi_dout_mode::R](spi_dout_mode::R) reader structure"]
impl crate::Readable for SPI_DOUT_MODE {}
#[doc = "`write(|w| ..)` method takes [spi_dout_mode::W](spi_dout_mode::W) writer structure"]
impl crate::Writable for SPI_DOUT_MODE {}
#[doc = "SPI_DOUT_MODE"]
pub mod spi_dout_mode;
#[doc = "SPI_DMA_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_conf](spi_dma_conf) module"]
pub type SPI_DMA_CONF = crate::Reg<u32, _SPI_DMA_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_CONF;
#[doc = "`read()` method returns [spi_dma_conf::R](spi_dma_conf::R) reader structure"]
impl crate::Readable for SPI_DMA_CONF {}
#[doc = "`write(|w| ..)` method takes [spi_dma_conf::W](spi_dma_conf::W) writer structure"]
impl crate::Writable for SPI_DMA_CONF {}
#[doc = "SPI_DMA_CONF"]
pub mod spi_dma_conf;
#[doc = "SPI_DMA_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_int_ena](spi_dma_int_ena) module"]
pub type SPI_DMA_INT_ENA = crate::Reg<u32, _SPI_DMA_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_INT_ENA;
#[doc = "`read()` method returns [spi_dma_int_ena::R](spi_dma_int_ena::R) reader structure"]
impl crate::Readable for SPI_DMA_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [spi_dma_int_ena::W](spi_dma_int_ena::W) writer structure"]
impl crate::Writable for SPI_DMA_INT_ENA {}
#[doc = "SPI_DMA_INT_ENA"]
pub mod spi_dma_int_ena;
#[doc = "SPI_DMA_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_int_clr](spi_dma_int_clr) module"]
pub type SPI_DMA_INT_CLR = crate::Reg<u32, _SPI_DMA_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_INT_CLR;
#[doc = "`write(|w| ..)` method takes [spi_dma_int_clr::W](spi_dma_int_clr::W) writer structure"]
impl crate::Writable for SPI_DMA_INT_CLR {}
#[doc = "SPI_DMA_INT_CLR"]
pub mod spi_dma_int_clr;
#[doc = "SPI_DMA_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_int_raw](spi_dma_int_raw) module"]
pub type SPI_DMA_INT_RAW = crate::Reg<u32, _SPI_DMA_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_INT_RAW;
#[doc = "`read()` method returns [spi_dma_int_raw::R](spi_dma_int_raw::R) reader structure"]
impl crate::Readable for SPI_DMA_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [spi_dma_int_raw::W](spi_dma_int_raw::W) writer structure"]
impl crate::Writable for SPI_DMA_INT_RAW {}
#[doc = "SPI_DMA_INT_RAW"]
pub mod spi_dma_int_raw;
#[doc = "SPI_DMA_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_int_st](spi_dma_int_st) module"]
pub type SPI_DMA_INT_ST = crate::Reg<u32, _SPI_DMA_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DMA_INT_ST;
#[doc = "`read()` method returns [spi_dma_int_st::R](spi_dma_int_st::R) reader structure"]
impl crate::Readable for SPI_DMA_INT_ST {}
#[doc = "SPI_DMA_INT_ST"]
pub mod spi_dma_int_st;
#[doc = "SPI_W0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w0](spi_w0) module"]
pub type SPI_W0 = crate::Reg<u32, _SPI_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W0;
#[doc = "`read()` method returns [spi_w0::R](spi_w0::R) reader structure"]
impl crate::Readable for SPI_W0 {}
#[doc = "`write(|w| ..)` method takes [spi_w0::W](spi_w0::W) writer structure"]
impl crate::Writable for SPI_W0 {}
#[doc = "SPI_W0"]
pub mod spi_w0;
#[doc = "SPI_W1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w1](spi_w1) module"]
pub type SPI_W1 = crate::Reg<u32, _SPI_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W1;
#[doc = "`read()` method returns [spi_w1::R](spi_w1::R) reader structure"]
impl crate::Readable for SPI_W1 {}
#[doc = "`write(|w| ..)` method takes [spi_w1::W](spi_w1::W) writer structure"]
impl crate::Writable for SPI_W1 {}
#[doc = "SPI_W1"]
pub mod spi_w1;
#[doc = "SPI_W2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w2](spi_w2) module"]
pub type SPI_W2 = crate::Reg<u32, _SPI_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W2;
#[doc = "`read()` method returns [spi_w2::R](spi_w2::R) reader structure"]
impl crate::Readable for SPI_W2 {}
#[doc = "`write(|w| ..)` method takes [spi_w2::W](spi_w2::W) writer structure"]
impl crate::Writable for SPI_W2 {}
#[doc = "SPI_W2"]
pub mod spi_w2;
#[doc = "SPI_W3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w3](spi_w3) module"]
pub type SPI_W3 = crate::Reg<u32, _SPI_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W3;
#[doc = "`read()` method returns [spi_w3::R](spi_w3::R) reader structure"]
impl crate::Readable for SPI_W3 {}
#[doc = "`write(|w| ..)` method takes [spi_w3::W](spi_w3::W) writer structure"]
impl crate::Writable for SPI_W3 {}
#[doc = "SPI_W3"]
pub mod spi_w3;
#[doc = "SPI_W4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w4](spi_w4) module"]
pub type SPI_W4 = crate::Reg<u32, _SPI_W4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W4;
#[doc = "`read()` method returns [spi_w4::R](spi_w4::R) reader structure"]
impl crate::Readable for SPI_W4 {}
#[doc = "`write(|w| ..)` method takes [spi_w4::W](spi_w4::W) writer structure"]
impl crate::Writable for SPI_W4 {}
#[doc = "SPI_W4"]
pub mod spi_w4;
#[doc = "SPI_W5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w5](spi_w5) module"]
pub type SPI_W5 = crate::Reg<u32, _SPI_W5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W5;
#[doc = "`read()` method returns [spi_w5::R](spi_w5::R) reader structure"]
impl crate::Readable for SPI_W5 {}
#[doc = "`write(|w| ..)` method takes [spi_w5::W](spi_w5::W) writer structure"]
impl crate::Writable for SPI_W5 {}
#[doc = "SPI_W5"]
pub mod spi_w5;
#[doc = "SPI_W6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w6](spi_w6) module"]
pub type SPI_W6 = crate::Reg<u32, _SPI_W6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W6;
#[doc = "`read()` method returns [spi_w6::R](spi_w6::R) reader structure"]
impl crate::Readable for SPI_W6 {}
#[doc = "`write(|w| ..)` method takes [spi_w6::W](spi_w6::W) writer structure"]
impl crate::Writable for SPI_W6 {}
#[doc = "SPI_W6"]
pub mod spi_w6;
#[doc = "SPI_W7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w7](spi_w7) module"]
pub type SPI_W7 = crate::Reg<u32, _SPI_W7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W7;
#[doc = "`read()` method returns [spi_w7::R](spi_w7::R) reader structure"]
impl crate::Readable for SPI_W7 {}
#[doc = "`write(|w| ..)` method takes [spi_w7::W](spi_w7::W) writer structure"]
impl crate::Writable for SPI_W7 {}
#[doc = "SPI_W7"]
pub mod spi_w7;
#[doc = "SPI_W8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w8](spi_w8) module"]
pub type SPI_W8 = crate::Reg<u32, _SPI_W8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W8;
#[doc = "`read()` method returns [spi_w8::R](spi_w8::R) reader structure"]
impl crate::Readable for SPI_W8 {}
#[doc = "`write(|w| ..)` method takes [spi_w8::W](spi_w8::W) writer structure"]
impl crate::Writable for SPI_W8 {}
#[doc = "SPI_W8"]
pub mod spi_w8;
#[doc = "SPI_W9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w9](spi_w9) module"]
pub type SPI_W9 = crate::Reg<u32, _SPI_W9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W9;
#[doc = "`read()` method returns [spi_w9::R](spi_w9::R) reader structure"]
impl crate::Readable for SPI_W9 {}
#[doc = "`write(|w| ..)` method takes [spi_w9::W](spi_w9::W) writer structure"]
impl crate::Writable for SPI_W9 {}
#[doc = "SPI_W9"]
pub mod spi_w9;
#[doc = "SPI_W10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w10](spi_w10) module"]
pub type SPI_W10 = crate::Reg<u32, _SPI_W10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W10;
#[doc = "`read()` method returns [spi_w10::R](spi_w10::R) reader structure"]
impl crate::Readable for SPI_W10 {}
#[doc = "`write(|w| ..)` method takes [spi_w10::W](spi_w10::W) writer structure"]
impl crate::Writable for SPI_W10 {}
#[doc = "SPI_W10"]
pub mod spi_w10;
#[doc = "SPI_W11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w11](spi_w11) module"]
pub type SPI_W11 = crate::Reg<u32, _SPI_W11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W11;
#[doc = "`read()` method returns [spi_w11::R](spi_w11::R) reader structure"]
impl crate::Readable for SPI_W11 {}
#[doc = "`write(|w| ..)` method takes [spi_w11::W](spi_w11::W) writer structure"]
impl crate::Writable for SPI_W11 {}
#[doc = "SPI_W11"]
pub mod spi_w11;
#[doc = "SPI_W12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w12](spi_w12) module"]
pub type SPI_W12 = crate::Reg<u32, _SPI_W12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W12;
#[doc = "`read()` method returns [spi_w12::R](spi_w12::R) reader structure"]
impl crate::Readable for SPI_W12 {}
#[doc = "`write(|w| ..)` method takes [spi_w12::W](spi_w12::W) writer structure"]
impl crate::Writable for SPI_W12 {}
#[doc = "SPI_W12"]
pub mod spi_w12;
#[doc = "SPI_W13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w13](spi_w13) module"]
pub type SPI_W13 = crate::Reg<u32, _SPI_W13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W13;
#[doc = "`read()` method returns [spi_w13::R](spi_w13::R) reader structure"]
impl crate::Readable for SPI_W13 {}
#[doc = "`write(|w| ..)` method takes [spi_w13::W](spi_w13::W) writer structure"]
impl crate::Writable for SPI_W13 {}
#[doc = "SPI_W13"]
pub mod spi_w13;
#[doc = "SPI_W14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w14](spi_w14) module"]
pub type SPI_W14 = crate::Reg<u32, _SPI_W14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W14;
#[doc = "`read()` method returns [spi_w14::R](spi_w14::R) reader structure"]
impl crate::Readable for SPI_W14 {}
#[doc = "`write(|w| ..)` method takes [spi_w14::W](spi_w14::W) writer structure"]
impl crate::Writable for SPI_W14 {}
#[doc = "SPI_W14"]
pub mod spi_w14;
#[doc = "SPI_W15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w15](spi_w15) module"]
pub type SPI_W15 = crate::Reg<u32, _SPI_W15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_W15;
#[doc = "`read()` method returns [spi_w15::R](spi_w15::R) reader structure"]
impl crate::Readable for SPI_W15 {}
#[doc = "`write(|w| ..)` method takes [spi_w15::W](spi_w15::W) writer structure"]
impl crate::Writable for SPI_W15 {}
#[doc = "SPI_W15"]
pub mod spi_w15;
#[doc = "SPI_SLAVE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_slave](spi_slave) module"]
pub type SPI_SLAVE = crate::Reg<u32, _SPI_SLAVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLAVE;
#[doc = "`read()` method returns [spi_slave::R](spi_slave::R) reader structure"]
impl crate::Readable for SPI_SLAVE {}
#[doc = "`write(|w| ..)` method takes [spi_slave::W](spi_slave::W) writer structure"]
impl crate::Writable for SPI_SLAVE {}
#[doc = "SPI_SLAVE"]
pub mod spi_slave;
#[doc = "SPI_SLAVE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_slave1](spi_slave1) module"]
pub type SPI_SLAVE1 = crate::Reg<u32, _SPI_SLAVE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLAVE1;
#[doc = "`read()` method returns [spi_slave1::R](spi_slave1::R) reader structure"]
impl crate::Readable for SPI_SLAVE1 {}
#[doc = "`write(|w| ..)` method takes [spi_slave1::W](spi_slave1::W) writer structure"]
impl crate::Writable for SPI_SLAVE1 {}
#[doc = "SPI_SLAVE1"]
pub mod spi_slave1;
#[doc = "SPI_CLK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_clk_gate](spi_clk_gate) module"]
pub type SPI_CLK_GATE = crate::Reg<u32, _SPI_CLK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CLK_GATE;
#[doc = "`read()` method returns [spi_clk_gate::R](spi_clk_gate::R) reader structure"]
impl crate::Readable for SPI_CLK_GATE {}
#[doc = "`write(|w| ..)` method takes [spi_clk_gate::W](spi_clk_gate::W) writer structure"]
impl crate::Writable for SPI_CLK_GATE {}
#[doc = "SPI_CLK_GATE"]
pub mod spi_clk_gate;
#[doc = "SPI_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_date](spi_date) module"]
pub type SPI_DATE = crate::Reg<u32, _SPI_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_DATE;
#[doc = "`read()` method returns [spi_date::R](spi_date::R) reader structure"]
impl crate::Readable for SPI_DATE {}
#[doc = "`write(|w| ..)` method takes [spi_date::W](spi_date::W) writer structure"]
impl crate::Writable for SPI_DATE {}
#[doc = "SPI_DATE"]
pub mod spi_date;
