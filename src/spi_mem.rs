#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_MEM_CMD"]
    pub spi_mem_cmd: SPI_MEM_CMD,
    #[doc = "0x04 - SPI_MEM_ADDR"]
    pub spi_mem_addr: SPI_MEM_ADDR,
    #[doc = "0x08 - SPI_MEM_CTRL"]
    pub spi_mem_ctrl: SPI_MEM_CTRL,
    #[doc = "0x0c - SPI_MEM_CTRL1"]
    pub spi_mem_ctrl1: SPI_MEM_CTRL1,
    #[doc = "0x10 - SPI_MEM_CTRL2"]
    pub spi_mem_ctrl2: SPI_MEM_CTRL2,
    #[doc = "0x14 - SPI_MEM_CLOCK"]
    pub spi_mem_clock: SPI_MEM_CLOCK,
    #[doc = "0x18 - SPI_MEM_USER"]
    pub spi_mem_user: SPI_MEM_USER,
    #[doc = "0x1c - SPI_MEM_USER1"]
    pub spi_mem_user1: SPI_MEM_USER1,
    #[doc = "0x20 - SPI_MEM_USER2"]
    pub spi_mem_user2: SPI_MEM_USER2,
    #[doc = "0x24 - SPI_MEM_MOSI_DLEN"]
    pub spi_mem_mosi_dlen: SPI_MEM_MOSI_DLEN,
    #[doc = "0x28 - SPI_MEM_MISO_DLEN"]
    pub spi_mem_miso_dlen: SPI_MEM_MISO_DLEN,
    #[doc = "0x2c - SPI_MEM_RD_STATUS"]
    pub spi_mem_rd_status: SPI_MEM_RD_STATUS,
    _reserved12: [u8; 4usize],
    #[doc = "0x34 - SPI_MEM_MISC"]
    pub spi_mem_misc: SPI_MEM_MISC,
    #[doc = "0x38 - SPI_MEM_TX_CRC"]
    pub spi_mem_tx_crc: SPI_MEM_TX_CRC,
    #[doc = "0x3c - SPI_MEM_CACHE_FCTRL"]
    pub spi_mem_cache_fctrl: SPI_MEM_CACHE_FCTRL,
    _reserved15: [u8; 20usize],
    #[doc = "0x54 - SPI_MEM_FSM"]
    pub spi_mem_fsm: SPI_MEM_FSM,
    #[doc = "0x58 - SPI_MEM_W0"]
    pub spi_mem_w0: SPI_MEM_W0,
    #[doc = "0x5c - SPI_MEM_W1"]
    pub spi_mem_w1: SPI_MEM_W1,
    #[doc = "0x60 - SPI_MEM_W2"]
    pub spi_mem_w2: SPI_MEM_W2,
    #[doc = "0x64 - SPI_MEM_W3"]
    pub spi_mem_w3: SPI_MEM_W3,
    #[doc = "0x68 - SPI_MEM_W4"]
    pub spi_mem_w4: SPI_MEM_W4,
    #[doc = "0x6c - SPI_MEM_W5"]
    pub spi_mem_w5: SPI_MEM_W5,
    #[doc = "0x70 - SPI_MEM_W6"]
    pub spi_mem_w6: SPI_MEM_W6,
    #[doc = "0x74 - SPI_MEM_W7"]
    pub spi_mem_w7: SPI_MEM_W7,
    #[doc = "0x78 - SPI_MEM_W8"]
    pub spi_mem_w8: SPI_MEM_W8,
    #[doc = "0x7c - SPI_MEM_W9"]
    pub spi_mem_w9: SPI_MEM_W9,
    #[doc = "0x80 - SPI_MEM_W10"]
    pub spi_mem_w10: SPI_MEM_W10,
    #[doc = "0x84 - SPI_MEM_W11"]
    pub spi_mem_w11: SPI_MEM_W11,
    #[doc = "0x88 - SPI_MEM_W12"]
    pub spi_mem_w12: SPI_MEM_W12,
    #[doc = "0x8c - SPI_MEM_W13"]
    pub spi_mem_w13: SPI_MEM_W13,
    #[doc = "0x90 - SPI_MEM_W14"]
    pub spi_mem_w14: SPI_MEM_W14,
    #[doc = "0x94 - SPI_MEM_W15"]
    pub spi_mem_w15: SPI_MEM_W15,
    #[doc = "0x98 - SPI_MEM_FLASH_WAITI_CTRL"]
    pub spi_mem_flash_waiti_ctrl: SPI_MEM_FLASH_WAITI_CTRL,
    #[doc = "0x9c - SPI_MEM_FLASH_SUS_CTRL"]
    pub spi_mem_flash_sus_ctrl: SPI_MEM_FLASH_SUS_CTRL,
    #[doc = "0xa0 - SPI_MEM_FLASH_SUS_CMD"]
    pub spi_mem_flash_sus_cmd: SPI_MEM_FLASH_SUS_CMD,
    #[doc = "0xa4 - SPI_MEM_SUS_STATUS"]
    pub spi_mem_sus_status: SPI_MEM_SUS_STATUS,
    #[doc = "0xa8 - SPI_MEM_TIMING_CALI"]
    pub spi_mem_timing_cali: SPI_MEM_TIMING_CALI,
    #[doc = "0xac - SPI_MEM_DIN_MODE"]
    pub spi_mem_din_mode: SPI_MEM_DIN_MODE,
    #[doc = "0xb0 - SPI_MEM_DIN_NUM"]
    pub spi_mem_din_num: SPI_MEM_DIN_NUM,
    #[doc = "0xb4 - SPI_MEM_DOUT_MODE"]
    pub spi_mem_dout_mode: SPI_MEM_DOUT_MODE,
    _reserved40: [u8; 8usize],
    #[doc = "0xc0 - SPI_MEM_INT_ENA"]
    pub spi_mem_int_ena: SPI_MEM_INT_ENA,
    #[doc = "0xc4 - SPI_MEM_INT_CLR"]
    pub spi_mem_int_clr: SPI_MEM_INT_CLR,
    #[doc = "0xc8 - SPI_MEM_INT_RAW"]
    pub spi_mem_int_raw: SPI_MEM_INT_RAW,
    #[doc = "0xcc - SPI_MEM_INT_ST"]
    pub spi_mem_int_st: SPI_MEM_INT_ST,
    _reserved44: [u8; 12usize],
    #[doc = "0xdc - SPI_MEM_CLOCK_GATE"]
    pub spi_mem_clock_gate: SPI_MEM_CLOCK_GATE,
    #[doc = "0xe0 - SPI_MEM_CORE_CLK_SEL"]
    pub spi_mem_core_clk_sel: SPI_MEM_CORE_CLK_SEL,
    _reserved46: [u8; 792usize],
    #[doc = "0x3fc - SPI_MEM_DATE"]
    pub spi_mem_date: SPI_MEM_DATE,
}
#[doc = "SPI_MEM_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_cmd](spi_mem_cmd) module"]
pub type SPI_MEM_CMD = crate::Reg<u32, _SPI_MEM_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_CMD;
#[doc = "`read()` method returns [spi_mem_cmd::R](spi_mem_cmd::R) reader structure"]
impl crate::Readable for SPI_MEM_CMD {}
#[doc = "`write(|w| ..)` method takes [spi_mem_cmd::W](spi_mem_cmd::W) writer structure"]
impl crate::Writable for SPI_MEM_CMD {}
#[doc = "SPI_MEM_CMD"]
pub mod spi_mem_cmd;
#[doc = "SPI_MEM_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_addr](spi_mem_addr) module"]
pub type SPI_MEM_ADDR = crate::Reg<u32, _SPI_MEM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_ADDR;
#[doc = "`read()` method returns [spi_mem_addr::R](spi_mem_addr::R) reader structure"]
impl crate::Readable for SPI_MEM_ADDR {}
#[doc = "`write(|w| ..)` method takes [spi_mem_addr::W](spi_mem_addr::W) writer structure"]
impl crate::Writable for SPI_MEM_ADDR {}
#[doc = "SPI_MEM_ADDR"]
pub mod spi_mem_addr;
#[doc = "SPI_MEM_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ctrl](spi_mem_ctrl) module"]
pub type SPI_MEM_CTRL = crate::Reg<u32, _SPI_MEM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_CTRL;
#[doc = "`read()` method returns [spi_mem_ctrl::R](spi_mem_ctrl::R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL {}
#[doc = "`write(|w| ..)` method takes [spi_mem_ctrl::W](spi_mem_ctrl::W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL {}
#[doc = "SPI_MEM_CTRL"]
pub mod spi_mem_ctrl;
#[doc = "SPI_MEM_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ctrl1](spi_mem_ctrl1) module"]
pub type SPI_MEM_CTRL1 = crate::Reg<u32, _SPI_MEM_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_CTRL1;
#[doc = "`read()` method returns [spi_mem_ctrl1::R](spi_mem_ctrl1::R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_ctrl1::W](spi_mem_ctrl1::W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL1 {}
#[doc = "SPI_MEM_CTRL1"]
pub mod spi_mem_ctrl1;
#[doc = "SPI_MEM_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ctrl2](spi_mem_ctrl2) module"]
pub type SPI_MEM_CTRL2 = crate::Reg<u32, _SPI_MEM_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_CTRL2;
#[doc = "`read()` method returns [spi_mem_ctrl2::R](spi_mem_ctrl2::R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_ctrl2::W](spi_mem_ctrl2::W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL2 {}
#[doc = "SPI_MEM_CTRL2"]
pub mod spi_mem_ctrl2;
#[doc = "SPI_MEM_CLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_clock](spi_mem_clock) module"]
pub type SPI_MEM_CLOCK = crate::Reg<u32, _SPI_MEM_CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_CLOCK;
#[doc = "`read()` method returns [spi_mem_clock::R](spi_mem_clock::R) reader structure"]
impl crate::Readable for SPI_MEM_CLOCK {}
#[doc = "`write(|w| ..)` method takes [spi_mem_clock::W](spi_mem_clock::W) writer structure"]
impl crate::Writable for SPI_MEM_CLOCK {}
#[doc = "SPI_MEM_CLOCK"]
pub mod spi_mem_clock;
#[doc = "SPI_MEM_USER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_user](spi_mem_user) module"]
pub type SPI_MEM_USER = crate::Reg<u32, _SPI_MEM_USER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_USER;
#[doc = "`read()` method returns [spi_mem_user::R](spi_mem_user::R) reader structure"]
impl crate::Readable for SPI_MEM_USER {}
#[doc = "`write(|w| ..)` method takes [spi_mem_user::W](spi_mem_user::W) writer structure"]
impl crate::Writable for SPI_MEM_USER {}
#[doc = "SPI_MEM_USER"]
pub mod spi_mem_user;
#[doc = "SPI_MEM_USER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_user1](spi_mem_user1) module"]
pub type SPI_MEM_USER1 = crate::Reg<u32, _SPI_MEM_USER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_USER1;
#[doc = "`read()` method returns [spi_mem_user1::R](spi_mem_user1::R) reader structure"]
impl crate::Readable for SPI_MEM_USER1 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_user1::W](spi_mem_user1::W) writer structure"]
impl crate::Writable for SPI_MEM_USER1 {}
#[doc = "SPI_MEM_USER1"]
pub mod spi_mem_user1;
#[doc = "SPI_MEM_USER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_user2](spi_mem_user2) module"]
pub type SPI_MEM_USER2 = crate::Reg<u32, _SPI_MEM_USER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_USER2;
#[doc = "`read()` method returns [spi_mem_user2::R](spi_mem_user2::R) reader structure"]
impl crate::Readable for SPI_MEM_USER2 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_user2::W](spi_mem_user2::W) writer structure"]
impl crate::Writable for SPI_MEM_USER2 {}
#[doc = "SPI_MEM_USER2"]
pub mod spi_mem_user2;
#[doc = "SPI_MEM_MOSI_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_mosi_dlen](spi_mem_mosi_dlen) module"]
pub type SPI_MEM_MOSI_DLEN = crate::Reg<u32, _SPI_MEM_MOSI_DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_MOSI_DLEN;
#[doc = "`read()` method returns [spi_mem_mosi_dlen::R](spi_mem_mosi_dlen::R) reader structure"]
impl crate::Readable for SPI_MEM_MOSI_DLEN {}
#[doc = "`write(|w| ..)` method takes [spi_mem_mosi_dlen::W](spi_mem_mosi_dlen::W) writer structure"]
impl crate::Writable for SPI_MEM_MOSI_DLEN {}
#[doc = "SPI_MEM_MOSI_DLEN"]
pub mod spi_mem_mosi_dlen;
#[doc = "SPI_MEM_MISO_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_miso_dlen](spi_mem_miso_dlen) module"]
pub type SPI_MEM_MISO_DLEN = crate::Reg<u32, _SPI_MEM_MISO_DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_MISO_DLEN;
#[doc = "`read()` method returns [spi_mem_miso_dlen::R](spi_mem_miso_dlen::R) reader structure"]
impl crate::Readable for SPI_MEM_MISO_DLEN {}
#[doc = "`write(|w| ..)` method takes [spi_mem_miso_dlen::W](spi_mem_miso_dlen::W) writer structure"]
impl crate::Writable for SPI_MEM_MISO_DLEN {}
#[doc = "SPI_MEM_MISO_DLEN"]
pub mod spi_mem_miso_dlen;
#[doc = "SPI_MEM_RD_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_rd_status](spi_mem_rd_status) module"]
pub type SPI_MEM_RD_STATUS = crate::Reg<u32, _SPI_MEM_RD_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_RD_STATUS;
#[doc = "`read()` method returns [spi_mem_rd_status::R](spi_mem_rd_status::R) reader structure"]
impl crate::Readable for SPI_MEM_RD_STATUS {}
#[doc = "`write(|w| ..)` method takes [spi_mem_rd_status::W](spi_mem_rd_status::W) writer structure"]
impl crate::Writable for SPI_MEM_RD_STATUS {}
#[doc = "SPI_MEM_RD_STATUS"]
pub mod spi_mem_rd_status;
#[doc = "SPI_MEM_MISC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_misc](spi_mem_misc) module"]
pub type SPI_MEM_MISC = crate::Reg<u32, _SPI_MEM_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_MISC;
#[doc = "`read()` method returns [spi_mem_misc::R](spi_mem_misc::R) reader structure"]
impl crate::Readable for SPI_MEM_MISC {}
#[doc = "`write(|w| ..)` method takes [spi_mem_misc::W](spi_mem_misc::W) writer structure"]
impl crate::Writable for SPI_MEM_MISC {}
#[doc = "SPI_MEM_MISC"]
pub mod spi_mem_misc;
#[doc = "SPI_MEM_TX_CRC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_tx_crc](spi_mem_tx_crc) module"]
pub type SPI_MEM_TX_CRC = crate::Reg<u32, _SPI_MEM_TX_CRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_TX_CRC;
#[doc = "`read()` method returns [spi_mem_tx_crc::R](spi_mem_tx_crc::R) reader structure"]
impl crate::Readable for SPI_MEM_TX_CRC {}
#[doc = "SPI_MEM_TX_CRC"]
pub mod spi_mem_tx_crc;
#[doc = "SPI_MEM_CACHE_FCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_cache_fctrl](spi_mem_cache_fctrl) module"]
pub type SPI_MEM_CACHE_FCTRL = crate::Reg<u32, _SPI_MEM_CACHE_FCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_CACHE_FCTRL;
#[doc = "`read()` method returns [spi_mem_cache_fctrl::R](spi_mem_cache_fctrl::R) reader structure"]
impl crate::Readable for SPI_MEM_CACHE_FCTRL {}
#[doc = "`write(|w| ..)` method takes [spi_mem_cache_fctrl::W](spi_mem_cache_fctrl::W) writer structure"]
impl crate::Writable for SPI_MEM_CACHE_FCTRL {}
#[doc = "SPI_MEM_CACHE_FCTRL"]
pub mod spi_mem_cache_fctrl;
#[doc = "SPI_MEM_FSM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_fsm](spi_mem_fsm) module"]
pub type SPI_MEM_FSM = crate::Reg<u32, _SPI_MEM_FSM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_FSM;
#[doc = "`read()` method returns [spi_mem_fsm::R](spi_mem_fsm::R) reader structure"]
impl crate::Readable for SPI_MEM_FSM {}
#[doc = "`write(|w| ..)` method takes [spi_mem_fsm::W](spi_mem_fsm::W) writer structure"]
impl crate::Writable for SPI_MEM_FSM {}
#[doc = "SPI_MEM_FSM"]
pub mod spi_mem_fsm;
#[doc = "SPI_MEM_W0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w0](spi_mem_w0) module"]
pub type SPI_MEM_W0 = crate::Reg<u32, _SPI_MEM_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W0;
#[doc = "`read()` method returns [spi_mem_w0::R](spi_mem_w0::R) reader structure"]
impl crate::Readable for SPI_MEM_W0 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w0::W](spi_mem_w0::W) writer structure"]
impl crate::Writable for SPI_MEM_W0 {}
#[doc = "SPI_MEM_W0"]
pub mod spi_mem_w0;
#[doc = "SPI_MEM_W1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w1](spi_mem_w1) module"]
pub type SPI_MEM_W1 = crate::Reg<u32, _SPI_MEM_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W1;
#[doc = "`read()` method returns [spi_mem_w1::R](spi_mem_w1::R) reader structure"]
impl crate::Readable for SPI_MEM_W1 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w1::W](spi_mem_w1::W) writer structure"]
impl crate::Writable for SPI_MEM_W1 {}
#[doc = "SPI_MEM_W1"]
pub mod spi_mem_w1;
#[doc = "SPI_MEM_W2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w2](spi_mem_w2) module"]
pub type SPI_MEM_W2 = crate::Reg<u32, _SPI_MEM_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W2;
#[doc = "`read()` method returns [spi_mem_w2::R](spi_mem_w2::R) reader structure"]
impl crate::Readable for SPI_MEM_W2 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w2::W](spi_mem_w2::W) writer structure"]
impl crate::Writable for SPI_MEM_W2 {}
#[doc = "SPI_MEM_W2"]
pub mod spi_mem_w2;
#[doc = "SPI_MEM_W3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w3](spi_mem_w3) module"]
pub type SPI_MEM_W3 = crate::Reg<u32, _SPI_MEM_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W3;
#[doc = "`read()` method returns [spi_mem_w3::R](spi_mem_w3::R) reader structure"]
impl crate::Readable for SPI_MEM_W3 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w3::W](spi_mem_w3::W) writer structure"]
impl crate::Writable for SPI_MEM_W3 {}
#[doc = "SPI_MEM_W3"]
pub mod spi_mem_w3;
#[doc = "SPI_MEM_W4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w4](spi_mem_w4) module"]
pub type SPI_MEM_W4 = crate::Reg<u32, _SPI_MEM_W4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W4;
#[doc = "`read()` method returns [spi_mem_w4::R](spi_mem_w4::R) reader structure"]
impl crate::Readable for SPI_MEM_W4 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w4::W](spi_mem_w4::W) writer structure"]
impl crate::Writable for SPI_MEM_W4 {}
#[doc = "SPI_MEM_W4"]
pub mod spi_mem_w4;
#[doc = "SPI_MEM_W5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w5](spi_mem_w5) module"]
pub type SPI_MEM_W5 = crate::Reg<u32, _SPI_MEM_W5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W5;
#[doc = "`read()` method returns [spi_mem_w5::R](spi_mem_w5::R) reader structure"]
impl crate::Readable for SPI_MEM_W5 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w5::W](spi_mem_w5::W) writer structure"]
impl crate::Writable for SPI_MEM_W5 {}
#[doc = "SPI_MEM_W5"]
pub mod spi_mem_w5;
#[doc = "SPI_MEM_W6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w6](spi_mem_w6) module"]
pub type SPI_MEM_W6 = crate::Reg<u32, _SPI_MEM_W6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W6;
#[doc = "`read()` method returns [spi_mem_w6::R](spi_mem_w6::R) reader structure"]
impl crate::Readable for SPI_MEM_W6 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w6::W](spi_mem_w6::W) writer structure"]
impl crate::Writable for SPI_MEM_W6 {}
#[doc = "SPI_MEM_W6"]
pub mod spi_mem_w6;
#[doc = "SPI_MEM_W7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w7](spi_mem_w7) module"]
pub type SPI_MEM_W7 = crate::Reg<u32, _SPI_MEM_W7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W7;
#[doc = "`read()` method returns [spi_mem_w7::R](spi_mem_w7::R) reader structure"]
impl crate::Readable for SPI_MEM_W7 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w7::W](spi_mem_w7::W) writer structure"]
impl crate::Writable for SPI_MEM_W7 {}
#[doc = "SPI_MEM_W7"]
pub mod spi_mem_w7;
#[doc = "SPI_MEM_W8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w8](spi_mem_w8) module"]
pub type SPI_MEM_W8 = crate::Reg<u32, _SPI_MEM_W8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W8;
#[doc = "`read()` method returns [spi_mem_w8::R](spi_mem_w8::R) reader structure"]
impl crate::Readable for SPI_MEM_W8 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w8::W](spi_mem_w8::W) writer structure"]
impl crate::Writable for SPI_MEM_W8 {}
#[doc = "SPI_MEM_W8"]
pub mod spi_mem_w8;
#[doc = "SPI_MEM_W9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w9](spi_mem_w9) module"]
pub type SPI_MEM_W9 = crate::Reg<u32, _SPI_MEM_W9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W9;
#[doc = "`read()` method returns [spi_mem_w9::R](spi_mem_w9::R) reader structure"]
impl crate::Readable for SPI_MEM_W9 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w9::W](spi_mem_w9::W) writer structure"]
impl crate::Writable for SPI_MEM_W9 {}
#[doc = "SPI_MEM_W9"]
pub mod spi_mem_w9;
#[doc = "SPI_MEM_W10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w10](spi_mem_w10) module"]
pub type SPI_MEM_W10 = crate::Reg<u32, _SPI_MEM_W10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W10;
#[doc = "`read()` method returns [spi_mem_w10::R](spi_mem_w10::R) reader structure"]
impl crate::Readable for SPI_MEM_W10 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w10::W](spi_mem_w10::W) writer structure"]
impl crate::Writable for SPI_MEM_W10 {}
#[doc = "SPI_MEM_W10"]
pub mod spi_mem_w10;
#[doc = "SPI_MEM_W11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w11](spi_mem_w11) module"]
pub type SPI_MEM_W11 = crate::Reg<u32, _SPI_MEM_W11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W11;
#[doc = "`read()` method returns [spi_mem_w11::R](spi_mem_w11::R) reader structure"]
impl crate::Readable for SPI_MEM_W11 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w11::W](spi_mem_w11::W) writer structure"]
impl crate::Writable for SPI_MEM_W11 {}
#[doc = "SPI_MEM_W11"]
pub mod spi_mem_w11;
#[doc = "SPI_MEM_W12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w12](spi_mem_w12) module"]
pub type SPI_MEM_W12 = crate::Reg<u32, _SPI_MEM_W12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W12;
#[doc = "`read()` method returns [spi_mem_w12::R](spi_mem_w12::R) reader structure"]
impl crate::Readable for SPI_MEM_W12 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w12::W](spi_mem_w12::W) writer structure"]
impl crate::Writable for SPI_MEM_W12 {}
#[doc = "SPI_MEM_W12"]
pub mod spi_mem_w12;
#[doc = "SPI_MEM_W13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w13](spi_mem_w13) module"]
pub type SPI_MEM_W13 = crate::Reg<u32, _SPI_MEM_W13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W13;
#[doc = "`read()` method returns [spi_mem_w13::R](spi_mem_w13::R) reader structure"]
impl crate::Readable for SPI_MEM_W13 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w13::W](spi_mem_w13::W) writer structure"]
impl crate::Writable for SPI_MEM_W13 {}
#[doc = "SPI_MEM_W13"]
pub mod spi_mem_w13;
#[doc = "SPI_MEM_W14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w14](spi_mem_w14) module"]
pub type SPI_MEM_W14 = crate::Reg<u32, _SPI_MEM_W14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W14;
#[doc = "`read()` method returns [spi_mem_w14::R](spi_mem_w14::R) reader structure"]
impl crate::Readable for SPI_MEM_W14 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w14::W](spi_mem_w14::W) writer structure"]
impl crate::Writable for SPI_MEM_W14 {}
#[doc = "SPI_MEM_W14"]
pub mod spi_mem_w14;
#[doc = "SPI_MEM_W15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w15](spi_mem_w15) module"]
pub type SPI_MEM_W15 = crate::Reg<u32, _SPI_MEM_W15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_W15;
#[doc = "`read()` method returns [spi_mem_w15::R](spi_mem_w15::R) reader structure"]
impl crate::Readable for SPI_MEM_W15 {}
#[doc = "`write(|w| ..)` method takes [spi_mem_w15::W](spi_mem_w15::W) writer structure"]
impl crate::Writable for SPI_MEM_W15 {}
#[doc = "SPI_MEM_W15"]
pub mod spi_mem_w15;
#[doc = "SPI_MEM_FLASH_WAITI_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_flash_waiti_ctrl](spi_mem_flash_waiti_ctrl) module"]
pub type SPI_MEM_FLASH_WAITI_CTRL = crate::Reg<u32, _SPI_MEM_FLASH_WAITI_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_FLASH_WAITI_CTRL;
#[doc = "`read()` method returns [spi_mem_flash_waiti_ctrl::R](spi_mem_flash_waiti_ctrl::R) reader structure"]
impl crate::Readable for SPI_MEM_FLASH_WAITI_CTRL {}
#[doc = "`write(|w| ..)` method takes [spi_mem_flash_waiti_ctrl::W](spi_mem_flash_waiti_ctrl::W) writer structure"]
impl crate::Writable for SPI_MEM_FLASH_WAITI_CTRL {}
#[doc = "SPI_MEM_FLASH_WAITI_CTRL"]
pub mod spi_mem_flash_waiti_ctrl;
#[doc = "SPI_MEM_FLASH_SUS_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_flash_sus_ctrl](spi_mem_flash_sus_ctrl) module"]
pub type SPI_MEM_FLASH_SUS_CTRL = crate::Reg<u32, _SPI_MEM_FLASH_SUS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_FLASH_SUS_CTRL;
#[doc = "`read()` method returns [spi_mem_flash_sus_ctrl::R](spi_mem_flash_sus_ctrl::R) reader structure"]
impl crate::Readable for SPI_MEM_FLASH_SUS_CTRL {}
#[doc = "`write(|w| ..)` method takes [spi_mem_flash_sus_ctrl::W](spi_mem_flash_sus_ctrl::W) writer structure"]
impl crate::Writable for SPI_MEM_FLASH_SUS_CTRL {}
#[doc = "SPI_MEM_FLASH_SUS_CTRL"]
pub mod spi_mem_flash_sus_ctrl;
#[doc = "SPI_MEM_FLASH_SUS_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_flash_sus_cmd](spi_mem_flash_sus_cmd) module"]
pub type SPI_MEM_FLASH_SUS_CMD = crate::Reg<u32, _SPI_MEM_FLASH_SUS_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_FLASH_SUS_CMD;
#[doc = "`read()` method returns [spi_mem_flash_sus_cmd::R](spi_mem_flash_sus_cmd::R) reader structure"]
impl crate::Readable for SPI_MEM_FLASH_SUS_CMD {}
#[doc = "`write(|w| ..)` method takes [spi_mem_flash_sus_cmd::W](spi_mem_flash_sus_cmd::W) writer structure"]
impl crate::Writable for SPI_MEM_FLASH_SUS_CMD {}
#[doc = "SPI_MEM_FLASH_SUS_CMD"]
pub mod spi_mem_flash_sus_cmd;
#[doc = "SPI_MEM_SUS_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_sus_status](spi_mem_sus_status) module"]
pub type SPI_MEM_SUS_STATUS = crate::Reg<u32, _SPI_MEM_SUS_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_SUS_STATUS;
#[doc = "`read()` method returns [spi_mem_sus_status::R](spi_mem_sus_status::R) reader structure"]
impl crate::Readable for SPI_MEM_SUS_STATUS {}
#[doc = "`write(|w| ..)` method takes [spi_mem_sus_status::W](spi_mem_sus_status::W) writer structure"]
impl crate::Writable for SPI_MEM_SUS_STATUS {}
#[doc = "SPI_MEM_SUS_STATUS"]
pub mod spi_mem_sus_status;
#[doc = "SPI_MEM_TIMING_CALI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_timing_cali](spi_mem_timing_cali) module"]
pub type SPI_MEM_TIMING_CALI = crate::Reg<u32, _SPI_MEM_TIMING_CALI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_TIMING_CALI;
#[doc = "`read()` method returns [spi_mem_timing_cali::R](spi_mem_timing_cali::R) reader structure"]
impl crate::Readable for SPI_MEM_TIMING_CALI {}
#[doc = "`write(|w| ..)` method takes [spi_mem_timing_cali::W](spi_mem_timing_cali::W) writer structure"]
impl crate::Writable for SPI_MEM_TIMING_CALI {}
#[doc = "SPI_MEM_TIMING_CALI"]
pub mod spi_mem_timing_cali;
#[doc = "SPI_MEM_DIN_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_din_mode](spi_mem_din_mode) module"]
pub type SPI_MEM_DIN_MODE = crate::Reg<u32, _SPI_MEM_DIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_DIN_MODE;
#[doc = "`read()` method returns [spi_mem_din_mode::R](spi_mem_din_mode::R) reader structure"]
impl crate::Readable for SPI_MEM_DIN_MODE {}
#[doc = "`write(|w| ..)` method takes [spi_mem_din_mode::W](spi_mem_din_mode::W) writer structure"]
impl crate::Writable for SPI_MEM_DIN_MODE {}
#[doc = "SPI_MEM_DIN_MODE"]
pub mod spi_mem_din_mode;
#[doc = "SPI_MEM_DIN_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_din_num](spi_mem_din_num) module"]
pub type SPI_MEM_DIN_NUM = crate::Reg<u32, _SPI_MEM_DIN_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_DIN_NUM;
#[doc = "`read()` method returns [spi_mem_din_num::R](spi_mem_din_num::R) reader structure"]
impl crate::Readable for SPI_MEM_DIN_NUM {}
#[doc = "`write(|w| ..)` method takes [spi_mem_din_num::W](spi_mem_din_num::W) writer structure"]
impl crate::Writable for SPI_MEM_DIN_NUM {}
#[doc = "SPI_MEM_DIN_NUM"]
pub mod spi_mem_din_num;
#[doc = "SPI_MEM_DOUT_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_dout_mode](spi_mem_dout_mode) module"]
pub type SPI_MEM_DOUT_MODE = crate::Reg<u32, _SPI_MEM_DOUT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_DOUT_MODE;
#[doc = "`read()` method returns [spi_mem_dout_mode::R](spi_mem_dout_mode::R) reader structure"]
impl crate::Readable for SPI_MEM_DOUT_MODE {}
#[doc = "`write(|w| ..)` method takes [spi_mem_dout_mode::W](spi_mem_dout_mode::W) writer structure"]
impl crate::Writable for SPI_MEM_DOUT_MODE {}
#[doc = "SPI_MEM_DOUT_MODE"]
pub mod spi_mem_dout_mode;
#[doc = "SPI_MEM_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_ena](spi_mem_int_ena) module"]
pub type SPI_MEM_INT_ENA = crate::Reg<u32, _SPI_MEM_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_INT_ENA;
#[doc = "`read()` method returns [spi_mem_int_ena::R](spi_mem_int_ena::R) reader structure"]
impl crate::Readable for SPI_MEM_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [spi_mem_int_ena::W](spi_mem_int_ena::W) writer structure"]
impl crate::Writable for SPI_MEM_INT_ENA {}
#[doc = "SPI_MEM_INT_ENA"]
pub mod spi_mem_int_ena;
#[doc = "SPI_MEM_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_clr](spi_mem_int_clr) module"]
pub type SPI_MEM_INT_CLR = crate::Reg<u32, _SPI_MEM_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_INT_CLR;
#[doc = "`write(|w| ..)` method takes [spi_mem_int_clr::W](spi_mem_int_clr::W) writer structure"]
impl crate::Writable for SPI_MEM_INT_CLR {}
#[doc = "SPI_MEM_INT_CLR"]
pub mod spi_mem_int_clr;
#[doc = "SPI_MEM_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_raw](spi_mem_int_raw) module"]
pub type SPI_MEM_INT_RAW = crate::Reg<u32, _SPI_MEM_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_INT_RAW;
#[doc = "`read()` method returns [spi_mem_int_raw::R](spi_mem_int_raw::R) reader structure"]
impl crate::Readable for SPI_MEM_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [spi_mem_int_raw::W](spi_mem_int_raw::W) writer structure"]
impl crate::Writable for SPI_MEM_INT_RAW {}
#[doc = "SPI_MEM_INT_RAW"]
pub mod spi_mem_int_raw;
#[doc = "SPI_MEM_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_st](spi_mem_int_st) module"]
pub type SPI_MEM_INT_ST = crate::Reg<u32, _SPI_MEM_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_INT_ST;
#[doc = "`read()` method returns [spi_mem_int_st::R](spi_mem_int_st::R) reader structure"]
impl crate::Readable for SPI_MEM_INT_ST {}
#[doc = "SPI_MEM_INT_ST"]
pub mod spi_mem_int_st;
#[doc = "SPI_MEM_CLOCK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_clock_gate](spi_mem_clock_gate) module"]
pub type SPI_MEM_CLOCK_GATE = crate::Reg<u32, _SPI_MEM_CLOCK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_CLOCK_GATE;
#[doc = "`read()` method returns [spi_mem_clock_gate::R](spi_mem_clock_gate::R) reader structure"]
impl crate::Readable for SPI_MEM_CLOCK_GATE {}
#[doc = "`write(|w| ..)` method takes [spi_mem_clock_gate::W](spi_mem_clock_gate::W) writer structure"]
impl crate::Writable for SPI_MEM_CLOCK_GATE {}
#[doc = "SPI_MEM_CLOCK_GATE"]
pub mod spi_mem_clock_gate;
#[doc = "SPI_MEM_CORE_CLK_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_core_clk_sel](spi_mem_core_clk_sel) module"]
pub type SPI_MEM_CORE_CLK_SEL = crate::Reg<u32, _SPI_MEM_CORE_CLK_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_CORE_CLK_SEL;
#[doc = "`read()` method returns [spi_mem_core_clk_sel::R](spi_mem_core_clk_sel::R) reader structure"]
impl crate::Readable for SPI_MEM_CORE_CLK_SEL {}
#[doc = "`write(|w| ..)` method takes [spi_mem_core_clk_sel::W](spi_mem_core_clk_sel::W) writer structure"]
impl crate::Writable for SPI_MEM_CORE_CLK_SEL {}
#[doc = "SPI_MEM_CORE_CLK_SEL"]
pub mod spi_mem_core_clk_sel;
#[doc = "SPI_MEM_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_date](spi_mem_date) module"]
pub type SPI_MEM_DATE = crate::Reg<u32, _SPI_MEM_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_DATE;
#[doc = "`read()` method returns [spi_mem_date::R](spi_mem_date::R) reader structure"]
impl crate::Readable for SPI_MEM_DATE {}
#[doc = "`write(|w| ..)` method takes [spi_mem_date::W](spi_mem_date::W) writer structure"]
impl crate::Writable for SPI_MEM_DATE {}
#[doc = "SPI_MEM_DATE"]
pub mod spi_mem_date;
