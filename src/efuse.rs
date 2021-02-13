#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EFUSE_PGM_DATA0"]
    pub efuse_pgm_data0: EFUSE_PGM_DATA0,
    #[doc = "0x04 - EFUSE_PGM_DATA1"]
    pub efuse_pgm_data1: EFUSE_PGM_DATA1,
    #[doc = "0x08 - EFUSE_PGM_DATA2"]
    pub efuse_pgm_data2: EFUSE_PGM_DATA2,
    #[doc = "0x0c - EFUSE_PGM_DATA3"]
    pub efuse_pgm_data3: EFUSE_PGM_DATA3,
    #[doc = "0x10 - EFUSE_PGM_DATA4"]
    pub efuse_pgm_data4: EFUSE_PGM_DATA4,
    #[doc = "0x14 - EFUSE_PGM_DATA5"]
    pub efuse_pgm_data5: EFUSE_PGM_DATA5,
    #[doc = "0x18 - EFUSE_PGM_DATA6"]
    pub efuse_pgm_data6: EFUSE_PGM_DATA6,
    #[doc = "0x1c - EFUSE_PGM_DATA7"]
    pub efuse_pgm_data7: EFUSE_PGM_DATA7,
    #[doc = "0x20 - EFUSE_PGM_CHECK_VALUE0"]
    pub efuse_pgm_check_value0: EFUSE_PGM_CHECK_VALUE0,
    #[doc = "0x24 - EFUSE_PGM_CHECK_VALUE1"]
    pub efuse_pgm_check_value1: EFUSE_PGM_CHECK_VALUE1,
    #[doc = "0x28 - EFUSE_PGM_CHECK_VALUE2"]
    pub efuse_pgm_check_value2: EFUSE_PGM_CHECK_VALUE2,
    #[doc = "0x2c - EFUSE_RD_WR_DIS"]
    pub efuse_rd_wr_dis: EFUSE_RD_WR_DIS,
    #[doc = "0x30 - EFUSE_RD_REPEAT_DATA0"]
    pub efuse_rd_repeat_data0: EFUSE_RD_REPEAT_DATA0,
    #[doc = "0x34 - EFUSE_RD_REPEAT_DATA1"]
    pub efuse_rd_repeat_data1: EFUSE_RD_REPEAT_DATA1,
    #[doc = "0x38 - EFUSE_RD_REPEAT_DATA2"]
    pub efuse_rd_repeat_data2: EFUSE_RD_REPEAT_DATA2,
    #[doc = "0x3c - EFUSE_RD_REPEAT_DATA3"]
    pub efuse_rd_repeat_data3: EFUSE_RD_REPEAT_DATA3,
    #[doc = "0x40 - EFUSE_RD_REPEAT_DATA4"]
    pub efuse_rd_repeat_data4: EFUSE_RD_REPEAT_DATA4,
    #[doc = "0x44 - EFUSE_RD_MAC_SPI_SYS_0"]
    pub efuse_rd_mac_spi_sys_0: EFUSE_RD_MAC_SPI_SYS_0,
    #[doc = "0x48 - EFUSE_RD_MAC_SPI_SYS_1"]
    pub efuse_rd_mac_spi_sys_1: EFUSE_RD_MAC_SPI_SYS_1,
    #[doc = "0x4c - EFUSE_RD_MAC_SPI_SYS_2"]
    pub efuse_rd_mac_spi_sys_2: EFUSE_RD_MAC_SPI_SYS_2,
    #[doc = "0x50 - EFUSE_RD_MAC_SPI_SYS_3"]
    pub efuse_rd_mac_spi_sys_3: EFUSE_RD_MAC_SPI_SYS_3,
    #[doc = "0x54 - EFUSE_RD_MAC_SPI_SYS_4"]
    pub efuse_rd_mac_spi_sys_4: EFUSE_RD_MAC_SPI_SYS_4,
    #[doc = "0x58 - EFUSE_RD_MAC_SPI_SYS_5"]
    pub efuse_rd_mac_spi_sys_5: EFUSE_RD_MAC_SPI_SYS_5,
    #[doc = "0x5c - EFUSE_RD_SYS_PART1_DATA0"]
    pub efuse_rd_sys_part1_data0: EFUSE_RD_SYS_PART1_DATA0,
    #[doc = "0x60 - EFUSE_RD_SYS_PART1_DATA1"]
    pub efuse_rd_sys_part1_data1: EFUSE_RD_SYS_PART1_DATA1,
    #[doc = "0x64 - EFUSE_RD_SYS_PART1_DATA2"]
    pub efuse_rd_sys_part1_data2: EFUSE_RD_SYS_PART1_DATA2,
    #[doc = "0x68 - EFUSE_RD_SYS_PART1_DATA3"]
    pub efuse_rd_sys_part1_data3: EFUSE_RD_SYS_PART1_DATA3,
    #[doc = "0x6c - EFUSE_RD_SYS_PART1_DATA4"]
    pub efuse_rd_sys_part1_data4: EFUSE_RD_SYS_PART1_DATA4,
    #[doc = "0x70 - EFUSE_RD_SYS_PART1_DATA5"]
    pub efuse_rd_sys_part1_data5: EFUSE_RD_SYS_PART1_DATA5,
    #[doc = "0x74 - EFUSE_RD_SYS_PART1_DATA6"]
    pub efuse_rd_sys_part1_data6: EFUSE_RD_SYS_PART1_DATA6,
    #[doc = "0x78 - EFUSE_RD_SYS_PART1_DATA7"]
    pub efuse_rd_sys_part1_data7: EFUSE_RD_SYS_PART1_DATA7,
    #[doc = "0x7c - EFUSE_RD_USR_DATA0"]
    pub efuse_rd_usr_data0: EFUSE_RD_USR_DATA0,
    #[doc = "0x80 - EFUSE_RD_USR_DATA1"]
    pub efuse_rd_usr_data1: EFUSE_RD_USR_DATA1,
    #[doc = "0x84 - EFUSE_RD_USR_DATA2"]
    pub efuse_rd_usr_data2: EFUSE_RD_USR_DATA2,
    #[doc = "0x88 - EFUSE_RD_USR_DATA3"]
    pub efuse_rd_usr_data3: EFUSE_RD_USR_DATA3,
    #[doc = "0x8c - EFUSE_RD_USR_DATA4"]
    pub efuse_rd_usr_data4: EFUSE_RD_USR_DATA4,
    #[doc = "0x90 - EFUSE_RD_USR_DATA5"]
    pub efuse_rd_usr_data5: EFUSE_RD_USR_DATA5,
    #[doc = "0x94 - EFUSE_RD_USR_DATA6"]
    pub efuse_rd_usr_data6: EFUSE_RD_USR_DATA6,
    #[doc = "0x98 - EFUSE_RD_USR_DATA7"]
    pub efuse_rd_usr_data7: EFUSE_RD_USR_DATA7,
    #[doc = "0x9c - EFUSE_RD_KEY0_DATA0"]
    pub efuse_rd_key0_data0: EFUSE_RD_KEY0_DATA0,
    #[doc = "0xa0 - EFUSE_RD_KEY0_DATA1"]
    pub efuse_rd_key0_data1: EFUSE_RD_KEY0_DATA1,
    #[doc = "0xa4 - EFUSE_RD_KEY0_DATA2"]
    pub efuse_rd_key0_data2: EFUSE_RD_KEY0_DATA2,
    #[doc = "0xa8 - EFUSE_RD_KEY0_DATA3"]
    pub efuse_rd_key0_data3: EFUSE_RD_KEY0_DATA3,
    #[doc = "0xac - EFUSE_RD_KEY0_DATA4"]
    pub efuse_rd_key0_data4: EFUSE_RD_KEY0_DATA4,
    #[doc = "0xb0 - EFUSE_RD_KEY0_DATA5"]
    pub efuse_rd_key0_data5: EFUSE_RD_KEY0_DATA5,
    #[doc = "0xb4 - EFUSE_RD_KEY0_DATA6"]
    pub efuse_rd_key0_data6: EFUSE_RD_KEY0_DATA6,
    #[doc = "0xb8 - EFUSE_RD_KEY0_DATA7"]
    pub efuse_rd_key0_data7: EFUSE_RD_KEY0_DATA7,
    #[doc = "0xbc - EFUSE_RD_KEY1_DATA0"]
    pub efuse_rd_key1_data0: EFUSE_RD_KEY1_DATA0,
    #[doc = "0xc0 - EFUSE_RD_KEY1_DATA1"]
    pub efuse_rd_key1_data1: EFUSE_RD_KEY1_DATA1,
    #[doc = "0xc4 - EFUSE_RD_KEY1_DATA2"]
    pub efuse_rd_key1_data2: EFUSE_RD_KEY1_DATA2,
    #[doc = "0xc8 - EFUSE_RD_KEY1_DATA3"]
    pub efuse_rd_key1_data3: EFUSE_RD_KEY1_DATA3,
    #[doc = "0xcc - EFUSE_RD_KEY1_DATA4"]
    pub efuse_rd_key1_data4: EFUSE_RD_KEY1_DATA4,
    #[doc = "0xd0 - EFUSE_RD_KEY1_DATA5"]
    pub efuse_rd_key1_data5: EFUSE_RD_KEY1_DATA5,
    #[doc = "0xd4 - EFUSE_RD_KEY1_DATA6"]
    pub efuse_rd_key1_data6: EFUSE_RD_KEY1_DATA6,
    #[doc = "0xd8 - EFUSE_RD_KEY1_DATA7"]
    pub efuse_rd_key1_data7: EFUSE_RD_KEY1_DATA7,
    #[doc = "0xdc - EFUSE_RD_KEY2_DATA0"]
    pub efuse_rd_key2_data0: EFUSE_RD_KEY2_DATA0,
    #[doc = "0xe0 - EFUSE_RD_KEY2_DATA1"]
    pub efuse_rd_key2_data1: EFUSE_RD_KEY2_DATA1,
    #[doc = "0xe4 - EFUSE_RD_KEY2_DATA2"]
    pub efuse_rd_key2_data2: EFUSE_RD_KEY2_DATA2,
    #[doc = "0xe8 - EFUSE_RD_KEY2_DATA3"]
    pub efuse_rd_key2_data3: EFUSE_RD_KEY2_DATA3,
    #[doc = "0xec - EFUSE_RD_KEY2_DATA4"]
    pub efuse_rd_key2_data4: EFUSE_RD_KEY2_DATA4,
    #[doc = "0xf0 - EFUSE_RD_KEY2_DATA5"]
    pub efuse_rd_key2_data5: EFUSE_RD_KEY2_DATA5,
    #[doc = "0xf4 - EFUSE_RD_KEY2_DATA6"]
    pub efuse_rd_key2_data6: EFUSE_RD_KEY2_DATA6,
    #[doc = "0xf8 - EFUSE_RD_KEY2_DATA7"]
    pub efuse_rd_key2_data7: EFUSE_RD_KEY2_DATA7,
    #[doc = "0xfc - EFUSE_RD_KEY3_DATA0"]
    pub efuse_rd_key3_data0: EFUSE_RD_KEY3_DATA0,
    #[doc = "0x100 - EFUSE_RD_KEY3_DATA1"]
    pub efuse_rd_key3_data1: EFUSE_RD_KEY3_DATA1,
    #[doc = "0x104 - EFUSE_RD_KEY3_DATA2"]
    pub efuse_rd_key3_data2: EFUSE_RD_KEY3_DATA2,
    #[doc = "0x108 - EFUSE_RD_KEY3_DATA3"]
    pub efuse_rd_key3_data3: EFUSE_RD_KEY3_DATA3,
    #[doc = "0x10c - EFUSE_RD_KEY3_DATA4"]
    pub efuse_rd_key3_data4: EFUSE_RD_KEY3_DATA4,
    #[doc = "0x110 - EFUSE_RD_KEY3_DATA5"]
    pub efuse_rd_key3_data5: EFUSE_RD_KEY3_DATA5,
    #[doc = "0x114 - EFUSE_RD_KEY3_DATA6"]
    pub efuse_rd_key3_data6: EFUSE_RD_KEY3_DATA6,
    #[doc = "0x118 - EFUSE_RD_KEY3_DATA7"]
    pub efuse_rd_key3_data7: EFUSE_RD_KEY3_DATA7,
    #[doc = "0x11c - EFUSE_RD_KEY4_DATA0"]
    pub efuse_rd_key4_data0: EFUSE_RD_KEY4_DATA0,
    #[doc = "0x120 - EFUSE_RD_KEY4_DATA1"]
    pub efuse_rd_key4_data1: EFUSE_RD_KEY4_DATA1,
    #[doc = "0x124 - EFUSE_RD_KEY4_DATA2"]
    pub efuse_rd_key4_data2: EFUSE_RD_KEY4_DATA2,
    #[doc = "0x128 - EFUSE_RD_KEY4_DATA3"]
    pub efuse_rd_key4_data3: EFUSE_RD_KEY4_DATA3,
    #[doc = "0x12c - EFUSE_RD_KEY4_DATA4"]
    pub efuse_rd_key4_data4: EFUSE_RD_KEY4_DATA4,
    #[doc = "0x130 - EFUSE_RD_KEY4_DATA5"]
    pub efuse_rd_key4_data5: EFUSE_RD_KEY4_DATA5,
    #[doc = "0x134 - EFUSE_RD_KEY4_DATA6"]
    pub efuse_rd_key4_data6: EFUSE_RD_KEY4_DATA6,
    #[doc = "0x138 - EFUSE_RD_KEY4_DATA7"]
    pub efuse_rd_key4_data7: EFUSE_RD_KEY4_DATA7,
    #[doc = "0x13c - EFUSE_RD_KEY5_DATA0"]
    pub efuse_rd_key5_data0: EFUSE_RD_KEY5_DATA0,
    #[doc = "0x140 - EFUSE_RD_KEY5_DATA1"]
    pub efuse_rd_key5_data1: EFUSE_RD_KEY5_DATA1,
    #[doc = "0x144 - EFUSE_RD_KEY5_DATA2"]
    pub efuse_rd_key5_data2: EFUSE_RD_KEY5_DATA2,
    #[doc = "0x148 - EFUSE_RD_KEY5_DATA3"]
    pub efuse_rd_key5_data3: EFUSE_RD_KEY5_DATA3,
    #[doc = "0x14c - EFUSE_RD_KEY5_DATA4"]
    pub efuse_rd_key5_data4: EFUSE_RD_KEY5_DATA4,
    #[doc = "0x150 - EFUSE_RD_KEY5_DATA5"]
    pub efuse_rd_key5_data5: EFUSE_RD_KEY5_DATA5,
    #[doc = "0x154 - EFUSE_RD_KEY5_DATA6"]
    pub efuse_rd_key5_data6: EFUSE_RD_KEY5_DATA6,
    #[doc = "0x158 - EFUSE_RD_KEY5_DATA7"]
    pub efuse_rd_key5_data7: EFUSE_RD_KEY5_DATA7,
    #[doc = "0x15c - EFUSE_RD_SYS_PART2_DATA0"]
    pub efuse_rd_sys_part2_data0: EFUSE_RD_SYS_PART2_DATA0,
    #[doc = "0x160 - EFUSE_RD_SYS_PART2_DATA1"]
    pub efuse_rd_sys_part2_data1: EFUSE_RD_SYS_PART2_DATA1,
    #[doc = "0x164 - EFUSE_RD_SYS_PART2_DATA2"]
    pub efuse_rd_sys_part2_data2: EFUSE_RD_SYS_PART2_DATA2,
    #[doc = "0x168 - EFUSE_RD_SYS_PART2_DATA3"]
    pub efuse_rd_sys_part2_data3: EFUSE_RD_SYS_PART2_DATA3,
    #[doc = "0x16c - EFUSE_RD_SYS_PART2_DATA4"]
    pub efuse_rd_sys_part2_data4: EFUSE_RD_SYS_PART2_DATA4,
    #[doc = "0x170 - EFUSE_RD_SYS_PART2_DATA5"]
    pub efuse_rd_sys_part2_data5: EFUSE_RD_SYS_PART2_DATA5,
    #[doc = "0x174 - EFUSE_RD_SYS_PART2_DATA6"]
    pub efuse_rd_sys_part2_data6: EFUSE_RD_SYS_PART2_DATA6,
    #[doc = "0x178 - EFUSE_RD_SYS_PART2_DATA7"]
    pub efuse_rd_sys_part2_data7: EFUSE_RD_SYS_PART2_DATA7,
    #[doc = "0x17c - EFUSE_RD_REPEAT_ERR0"]
    pub efuse_rd_repeat_err0: EFUSE_RD_REPEAT_ERR0,
    #[doc = "0x180 - EFUSE_RD_REPEAT_ERR1"]
    pub efuse_rd_repeat_err1: EFUSE_RD_REPEAT_ERR1,
    #[doc = "0x184 - EFUSE_RD_REPEAT_ERR2"]
    pub efuse_rd_repeat_err2: EFUSE_RD_REPEAT_ERR2,
    #[doc = "0x188 - EFUSE_RD_REPEAT_ERR3"]
    pub efuse_rd_repeat_err3: EFUSE_RD_REPEAT_ERR3,
    _reserved99: [u8; 4usize],
    #[doc = "0x190 - EFUSE_RD_REPEAT_ERR4"]
    pub efuse_rd_repeat_err4: EFUSE_RD_REPEAT_ERR4,
    _reserved100: [u8; 44usize],
    #[doc = "0x1c0 - EFUSE_RD_RS_ERR0"]
    pub efuse_rd_rs_err0: EFUSE_RD_RS_ERR0,
    #[doc = "0x1c4 - EFUSE_RD_RS_ERR1"]
    pub efuse_rd_rs_err1: EFUSE_RD_RS_ERR1,
    #[doc = "0x1c8 - EFUSE_CLK"]
    pub efuse_clk: EFUSE_CLK,
    #[doc = "0x1cc - EFUSE_CONF"]
    pub efuse_conf: EFUSE_CONF,
    #[doc = "0x1d0 - EFUSE_STATUS"]
    pub efuse_status: EFUSE_STATUS,
    #[doc = "0x1d4 - EFUSE_CMD"]
    pub efuse_cmd: EFUSE_CMD,
    #[doc = "0x1d8 - EFUSE_INT_RAW"]
    pub efuse_int_raw: EFUSE_INT_RAW,
    #[doc = "0x1dc - EFUSE_INT_ST"]
    pub efuse_int_st: EFUSE_INT_ST,
    #[doc = "0x1e0 - EFUSE_INT_ENA"]
    pub efuse_int_ena: EFUSE_INT_ENA,
    #[doc = "0x1e4 - EFUSE_INT_CLR"]
    pub efuse_int_clr: EFUSE_INT_CLR,
    #[doc = "0x1e8 - EFUSE_DAC_CONF"]
    pub efuse_dac_conf: EFUSE_DAC_CONF,
    #[doc = "0x1ec - EFUSE_RD_TIM_CONF"]
    pub efuse_rd_tim_conf: EFUSE_RD_TIM_CONF,
    #[doc = "0x1f0 - EFUSE_WR_TIM_CONF1"]
    pub efuse_wr_tim_conf1: EFUSE_WR_TIM_CONF1,
    #[doc = "0x1f4 - EFUSE_WR_TIM_CONF2"]
    pub efuse_wr_tim_conf2: EFUSE_WR_TIM_CONF2,
    _reserved114: [u8; 4usize],
    #[doc = "0x1fc - EFUSE_DATE"]
    pub efuse_date: EFUSE_DATE,
}
#[doc = "EFUSE_PGM_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_data0](efuse_pgm_data0) module"]
pub type EFUSE_PGM_DATA0 = crate::Reg<u32, _EFUSE_PGM_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_DATA0;
#[doc = "`read()` method returns [efuse_pgm_data0::R](efuse_pgm_data0::R) reader structure"]
impl crate::Readable for EFUSE_PGM_DATA0 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_data0::W](efuse_pgm_data0::W) writer structure"]
impl crate::Writable for EFUSE_PGM_DATA0 {}
#[doc = "EFUSE_PGM_DATA0"]
pub mod efuse_pgm_data0;
#[doc = "EFUSE_PGM_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_data1](efuse_pgm_data1) module"]
pub type EFUSE_PGM_DATA1 = crate::Reg<u32, _EFUSE_PGM_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_DATA1;
#[doc = "`read()` method returns [efuse_pgm_data1::R](efuse_pgm_data1::R) reader structure"]
impl crate::Readable for EFUSE_PGM_DATA1 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_data1::W](efuse_pgm_data1::W) writer structure"]
impl crate::Writable for EFUSE_PGM_DATA1 {}
#[doc = "EFUSE_PGM_DATA1"]
pub mod efuse_pgm_data1;
#[doc = "EFUSE_PGM_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_data2](efuse_pgm_data2) module"]
pub type EFUSE_PGM_DATA2 = crate::Reg<u32, _EFUSE_PGM_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_DATA2;
#[doc = "`read()` method returns [efuse_pgm_data2::R](efuse_pgm_data2::R) reader structure"]
impl crate::Readable for EFUSE_PGM_DATA2 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_data2::W](efuse_pgm_data2::W) writer structure"]
impl crate::Writable for EFUSE_PGM_DATA2 {}
#[doc = "EFUSE_PGM_DATA2"]
pub mod efuse_pgm_data2;
#[doc = "EFUSE_PGM_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_data3](efuse_pgm_data3) module"]
pub type EFUSE_PGM_DATA3 = crate::Reg<u32, _EFUSE_PGM_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_DATA3;
#[doc = "`read()` method returns [efuse_pgm_data3::R](efuse_pgm_data3::R) reader structure"]
impl crate::Readable for EFUSE_PGM_DATA3 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_data3::W](efuse_pgm_data3::W) writer structure"]
impl crate::Writable for EFUSE_PGM_DATA3 {}
#[doc = "EFUSE_PGM_DATA3"]
pub mod efuse_pgm_data3;
#[doc = "EFUSE_PGM_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_data4](efuse_pgm_data4) module"]
pub type EFUSE_PGM_DATA4 = crate::Reg<u32, _EFUSE_PGM_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_DATA4;
#[doc = "`read()` method returns [efuse_pgm_data4::R](efuse_pgm_data4::R) reader structure"]
impl crate::Readable for EFUSE_PGM_DATA4 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_data4::W](efuse_pgm_data4::W) writer structure"]
impl crate::Writable for EFUSE_PGM_DATA4 {}
#[doc = "EFUSE_PGM_DATA4"]
pub mod efuse_pgm_data4;
#[doc = "EFUSE_PGM_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_data5](efuse_pgm_data5) module"]
pub type EFUSE_PGM_DATA5 = crate::Reg<u32, _EFUSE_PGM_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_DATA5;
#[doc = "`read()` method returns [efuse_pgm_data5::R](efuse_pgm_data5::R) reader structure"]
impl crate::Readable for EFUSE_PGM_DATA5 {}
#[doc = "EFUSE_PGM_DATA5"]
pub mod efuse_pgm_data5;
#[doc = "EFUSE_PGM_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_data6](efuse_pgm_data6) module"]
pub type EFUSE_PGM_DATA6 = crate::Reg<u32, _EFUSE_PGM_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_DATA6;
#[doc = "`read()` method returns [efuse_pgm_data6::R](efuse_pgm_data6::R) reader structure"]
impl crate::Readable for EFUSE_PGM_DATA6 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_data6::W](efuse_pgm_data6::W) writer structure"]
impl crate::Writable for EFUSE_PGM_DATA6 {}
#[doc = "EFUSE_PGM_DATA6"]
pub mod efuse_pgm_data6;
#[doc = "EFUSE_PGM_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_data7](efuse_pgm_data7) module"]
pub type EFUSE_PGM_DATA7 = crate::Reg<u32, _EFUSE_PGM_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_DATA7;
#[doc = "`read()` method returns [efuse_pgm_data7::R](efuse_pgm_data7::R) reader structure"]
impl crate::Readable for EFUSE_PGM_DATA7 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_data7::W](efuse_pgm_data7::W) writer structure"]
impl crate::Writable for EFUSE_PGM_DATA7 {}
#[doc = "EFUSE_PGM_DATA7"]
pub mod efuse_pgm_data7;
#[doc = "EFUSE_PGM_CHECK_VALUE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_check_value0](efuse_pgm_check_value0) module"]
pub type EFUSE_PGM_CHECK_VALUE0 = crate::Reg<u32, _EFUSE_PGM_CHECK_VALUE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_CHECK_VALUE0;
#[doc = "`read()` method returns [efuse_pgm_check_value0::R](efuse_pgm_check_value0::R) reader structure"]
impl crate::Readable for EFUSE_PGM_CHECK_VALUE0 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_check_value0::W](efuse_pgm_check_value0::W) writer structure"]
impl crate::Writable for EFUSE_PGM_CHECK_VALUE0 {}
#[doc = "EFUSE_PGM_CHECK_VALUE0"]
pub mod efuse_pgm_check_value0;
#[doc = "EFUSE_PGM_CHECK_VALUE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_check_value1](efuse_pgm_check_value1) module"]
pub type EFUSE_PGM_CHECK_VALUE1 = crate::Reg<u32, _EFUSE_PGM_CHECK_VALUE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_CHECK_VALUE1;
#[doc = "`read()` method returns [efuse_pgm_check_value1::R](efuse_pgm_check_value1::R) reader structure"]
impl crate::Readable for EFUSE_PGM_CHECK_VALUE1 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_check_value1::W](efuse_pgm_check_value1::W) writer structure"]
impl crate::Writable for EFUSE_PGM_CHECK_VALUE1 {}
#[doc = "EFUSE_PGM_CHECK_VALUE1"]
pub mod efuse_pgm_check_value1;
#[doc = "EFUSE_PGM_CHECK_VALUE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_pgm_check_value2](efuse_pgm_check_value2) module"]
pub type EFUSE_PGM_CHECK_VALUE2 = crate::Reg<u32, _EFUSE_PGM_CHECK_VALUE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_PGM_CHECK_VALUE2;
#[doc = "`read()` method returns [efuse_pgm_check_value2::R](efuse_pgm_check_value2::R) reader structure"]
impl crate::Readable for EFUSE_PGM_CHECK_VALUE2 {}
#[doc = "`write(|w| ..)` method takes [efuse_pgm_check_value2::W](efuse_pgm_check_value2::W) writer structure"]
impl crate::Writable for EFUSE_PGM_CHECK_VALUE2 {}
#[doc = "EFUSE_PGM_CHECK_VALUE2"]
pub mod efuse_pgm_check_value2;
#[doc = "EFUSE_RD_WR_DIS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_wr_dis](efuse_rd_wr_dis) module"]
pub type EFUSE_RD_WR_DIS = crate::Reg<u32, _EFUSE_RD_WR_DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_WR_DIS;
#[doc = "`read()` method returns [efuse_rd_wr_dis::R](efuse_rd_wr_dis::R) reader structure"]
impl crate::Readable for EFUSE_RD_WR_DIS {}
#[doc = "EFUSE_RD_WR_DIS"]
pub mod efuse_rd_wr_dis;
#[doc = "EFUSE_RD_REPEAT_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_data0](efuse_rd_repeat_data0) module"]
pub type EFUSE_RD_REPEAT_DATA0 = crate::Reg<u32, _EFUSE_RD_REPEAT_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_DATA0;
#[doc = "`read()` method returns [efuse_rd_repeat_data0::R](efuse_rd_repeat_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_DATA0 {}
#[doc = "EFUSE_RD_REPEAT_DATA0"]
pub mod efuse_rd_repeat_data0;
#[doc = "EFUSE_RD_REPEAT_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_data1](efuse_rd_repeat_data1) module"]
pub type EFUSE_RD_REPEAT_DATA1 = crate::Reg<u32, _EFUSE_RD_REPEAT_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_DATA1;
#[doc = "`read()` method returns [efuse_rd_repeat_data1::R](efuse_rd_repeat_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_DATA1 {}
#[doc = "EFUSE_RD_REPEAT_DATA1"]
pub mod efuse_rd_repeat_data1;
#[doc = "EFUSE_RD_REPEAT_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_data2](efuse_rd_repeat_data2) module"]
pub type EFUSE_RD_REPEAT_DATA2 = crate::Reg<u32, _EFUSE_RD_REPEAT_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_DATA2;
#[doc = "`read()` method returns [efuse_rd_repeat_data2::R](efuse_rd_repeat_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_DATA2 {}
#[doc = "EFUSE_RD_REPEAT_DATA2"]
pub mod efuse_rd_repeat_data2;
#[doc = "EFUSE_RD_REPEAT_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_data3](efuse_rd_repeat_data3) module"]
pub type EFUSE_RD_REPEAT_DATA3 = crate::Reg<u32, _EFUSE_RD_REPEAT_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_DATA3;
#[doc = "`read()` method returns [efuse_rd_repeat_data3::R](efuse_rd_repeat_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_DATA3 {}
#[doc = "EFUSE_RD_REPEAT_DATA3"]
pub mod efuse_rd_repeat_data3;
#[doc = "EFUSE_RD_REPEAT_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_data4](efuse_rd_repeat_data4) module"]
pub type EFUSE_RD_REPEAT_DATA4 = crate::Reg<u32, _EFUSE_RD_REPEAT_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_DATA4;
#[doc = "`read()` method returns [efuse_rd_repeat_data4::R](efuse_rd_repeat_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_DATA4 {}
#[doc = "EFUSE_RD_REPEAT_DATA4"]
pub mod efuse_rd_repeat_data4;
#[doc = "EFUSE_RD_MAC_SPI_SYS_0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_mac_spi_sys_0](efuse_rd_mac_spi_sys_0) module"]
pub type EFUSE_RD_MAC_SPI_SYS_0 = crate::Reg<u32, _EFUSE_RD_MAC_SPI_SYS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_MAC_SPI_SYS_0;
#[doc = "`read()` method returns [efuse_rd_mac_spi_sys_0::R](efuse_rd_mac_spi_sys_0::R) reader structure"]
impl crate::Readable for EFUSE_RD_MAC_SPI_SYS_0 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_0"]
pub mod efuse_rd_mac_spi_sys_0;
#[doc = "EFUSE_RD_MAC_SPI_SYS_1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_mac_spi_sys_1](efuse_rd_mac_spi_sys_1) module"]
pub type EFUSE_RD_MAC_SPI_SYS_1 = crate::Reg<u32, _EFUSE_RD_MAC_SPI_SYS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_MAC_SPI_SYS_1;
#[doc = "`read()` method returns [efuse_rd_mac_spi_sys_1::R](efuse_rd_mac_spi_sys_1::R) reader structure"]
impl crate::Readable for EFUSE_RD_MAC_SPI_SYS_1 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_1"]
pub mod efuse_rd_mac_spi_sys_1;
#[doc = "EFUSE_RD_MAC_SPI_SYS_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_mac_spi_sys_2](efuse_rd_mac_spi_sys_2) module"]
pub type EFUSE_RD_MAC_SPI_SYS_2 = crate::Reg<u32, _EFUSE_RD_MAC_SPI_SYS_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_MAC_SPI_SYS_2;
#[doc = "`read()` method returns [efuse_rd_mac_spi_sys_2::R](efuse_rd_mac_spi_sys_2::R) reader structure"]
impl crate::Readable for EFUSE_RD_MAC_SPI_SYS_2 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_2"]
pub mod efuse_rd_mac_spi_sys_2;
#[doc = "EFUSE_RD_MAC_SPI_SYS_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_mac_spi_sys_3](efuse_rd_mac_spi_sys_3) module"]
pub type EFUSE_RD_MAC_SPI_SYS_3 = crate::Reg<u32, _EFUSE_RD_MAC_SPI_SYS_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_MAC_SPI_SYS_3;
#[doc = "`read()` method returns [efuse_rd_mac_spi_sys_3::R](efuse_rd_mac_spi_sys_3::R) reader structure"]
impl crate::Readable for EFUSE_RD_MAC_SPI_SYS_3 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_3"]
pub mod efuse_rd_mac_spi_sys_3;
#[doc = "EFUSE_RD_MAC_SPI_SYS_4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_mac_spi_sys_4](efuse_rd_mac_spi_sys_4) module"]
pub type EFUSE_RD_MAC_SPI_SYS_4 = crate::Reg<u32, _EFUSE_RD_MAC_SPI_SYS_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_MAC_SPI_SYS_4;
#[doc = "`read()` method returns [efuse_rd_mac_spi_sys_4::R](efuse_rd_mac_spi_sys_4::R) reader structure"]
impl crate::Readable for EFUSE_RD_MAC_SPI_SYS_4 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_4"]
pub mod efuse_rd_mac_spi_sys_4;
#[doc = "EFUSE_RD_MAC_SPI_SYS_5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_mac_spi_sys_5](efuse_rd_mac_spi_sys_5) module"]
pub type EFUSE_RD_MAC_SPI_SYS_5 = crate::Reg<u32, _EFUSE_RD_MAC_SPI_SYS_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_MAC_SPI_SYS_5;
#[doc = "`read()` method returns [efuse_rd_mac_spi_sys_5::R](efuse_rd_mac_spi_sys_5::R) reader structure"]
impl crate::Readable for EFUSE_RD_MAC_SPI_SYS_5 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_5"]
pub mod efuse_rd_mac_spi_sys_5;
#[doc = "EFUSE_RD_SYS_PART1_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part1_data0](efuse_rd_sys_part1_data0) module"]
pub type EFUSE_RD_SYS_PART1_DATA0 = crate::Reg<u32, _EFUSE_RD_SYS_PART1_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART1_DATA0;
#[doc = "`read()` method returns [efuse_rd_sys_part1_data0::R](efuse_rd_sys_part1_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART1_DATA0 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA0"]
pub mod efuse_rd_sys_part1_data0;
#[doc = "EFUSE_RD_SYS_PART1_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part1_data1](efuse_rd_sys_part1_data1) module"]
pub type EFUSE_RD_SYS_PART1_DATA1 = crate::Reg<u32, _EFUSE_RD_SYS_PART1_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART1_DATA1;
#[doc = "`read()` method returns [efuse_rd_sys_part1_data1::R](efuse_rd_sys_part1_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART1_DATA1 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA1"]
pub mod efuse_rd_sys_part1_data1;
#[doc = "EFUSE_RD_SYS_PART1_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part1_data2](efuse_rd_sys_part1_data2) module"]
pub type EFUSE_RD_SYS_PART1_DATA2 = crate::Reg<u32, _EFUSE_RD_SYS_PART1_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART1_DATA2;
#[doc = "`read()` method returns [efuse_rd_sys_part1_data2::R](efuse_rd_sys_part1_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART1_DATA2 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA2"]
pub mod efuse_rd_sys_part1_data2;
#[doc = "EFUSE_RD_SYS_PART1_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part1_data3](efuse_rd_sys_part1_data3) module"]
pub type EFUSE_RD_SYS_PART1_DATA3 = crate::Reg<u32, _EFUSE_RD_SYS_PART1_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART1_DATA3;
#[doc = "`read()` method returns [efuse_rd_sys_part1_data3::R](efuse_rd_sys_part1_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART1_DATA3 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA3"]
pub mod efuse_rd_sys_part1_data3;
#[doc = "EFUSE_RD_SYS_PART1_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part1_data4](efuse_rd_sys_part1_data4) module"]
pub type EFUSE_RD_SYS_PART1_DATA4 = crate::Reg<u32, _EFUSE_RD_SYS_PART1_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART1_DATA4;
#[doc = "`read()` method returns [efuse_rd_sys_part1_data4::R](efuse_rd_sys_part1_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART1_DATA4 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA4"]
pub mod efuse_rd_sys_part1_data4;
#[doc = "EFUSE_RD_SYS_PART1_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part1_data5](efuse_rd_sys_part1_data5) module"]
pub type EFUSE_RD_SYS_PART1_DATA5 = crate::Reg<u32, _EFUSE_RD_SYS_PART1_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART1_DATA5;
#[doc = "`read()` method returns [efuse_rd_sys_part1_data5::R](efuse_rd_sys_part1_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART1_DATA5 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA5"]
pub mod efuse_rd_sys_part1_data5;
#[doc = "EFUSE_RD_SYS_PART1_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part1_data6](efuse_rd_sys_part1_data6) module"]
pub type EFUSE_RD_SYS_PART1_DATA6 = crate::Reg<u32, _EFUSE_RD_SYS_PART1_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART1_DATA6;
#[doc = "`read()` method returns [efuse_rd_sys_part1_data6::R](efuse_rd_sys_part1_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART1_DATA6 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA6"]
pub mod efuse_rd_sys_part1_data6;
#[doc = "EFUSE_RD_SYS_PART1_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part1_data7](efuse_rd_sys_part1_data7) module"]
pub type EFUSE_RD_SYS_PART1_DATA7 = crate::Reg<u32, _EFUSE_RD_SYS_PART1_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART1_DATA7;
#[doc = "`read()` method returns [efuse_rd_sys_part1_data7::R](efuse_rd_sys_part1_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART1_DATA7 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA7"]
pub mod efuse_rd_sys_part1_data7;
#[doc = "EFUSE_RD_USR_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_usr_data0](efuse_rd_usr_data0) module"]
pub type EFUSE_RD_USR_DATA0 = crate::Reg<u32, _EFUSE_RD_USR_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_USR_DATA0;
#[doc = "`read()` method returns [efuse_rd_usr_data0::R](efuse_rd_usr_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_USR_DATA0 {}
#[doc = "EFUSE_RD_USR_DATA0"]
pub mod efuse_rd_usr_data0;
#[doc = "EFUSE_RD_USR_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_usr_data1](efuse_rd_usr_data1) module"]
pub type EFUSE_RD_USR_DATA1 = crate::Reg<u32, _EFUSE_RD_USR_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_USR_DATA1;
#[doc = "`read()` method returns [efuse_rd_usr_data1::R](efuse_rd_usr_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_USR_DATA1 {}
#[doc = "EFUSE_RD_USR_DATA1"]
pub mod efuse_rd_usr_data1;
#[doc = "EFUSE_RD_USR_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_usr_data2](efuse_rd_usr_data2) module"]
pub type EFUSE_RD_USR_DATA2 = crate::Reg<u32, _EFUSE_RD_USR_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_USR_DATA2;
#[doc = "`read()` method returns [efuse_rd_usr_data2::R](efuse_rd_usr_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_USR_DATA2 {}
#[doc = "EFUSE_RD_USR_DATA2"]
pub mod efuse_rd_usr_data2;
#[doc = "EFUSE_RD_USR_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_usr_data3](efuse_rd_usr_data3) module"]
pub type EFUSE_RD_USR_DATA3 = crate::Reg<u32, _EFUSE_RD_USR_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_USR_DATA3;
#[doc = "`read()` method returns [efuse_rd_usr_data3::R](efuse_rd_usr_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_USR_DATA3 {}
#[doc = "EFUSE_RD_USR_DATA3"]
pub mod efuse_rd_usr_data3;
#[doc = "EFUSE_RD_USR_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_usr_data4](efuse_rd_usr_data4) module"]
pub type EFUSE_RD_USR_DATA4 = crate::Reg<u32, _EFUSE_RD_USR_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_USR_DATA4;
#[doc = "`read()` method returns [efuse_rd_usr_data4::R](efuse_rd_usr_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_USR_DATA4 {}
#[doc = "EFUSE_RD_USR_DATA4"]
pub mod efuse_rd_usr_data4;
#[doc = "EFUSE_RD_USR_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_usr_data5](efuse_rd_usr_data5) module"]
pub type EFUSE_RD_USR_DATA5 = crate::Reg<u32, _EFUSE_RD_USR_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_USR_DATA5;
#[doc = "`read()` method returns [efuse_rd_usr_data5::R](efuse_rd_usr_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_USR_DATA5 {}
#[doc = "EFUSE_RD_USR_DATA5"]
pub mod efuse_rd_usr_data5;
#[doc = "EFUSE_RD_USR_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_usr_data6](efuse_rd_usr_data6) module"]
pub type EFUSE_RD_USR_DATA6 = crate::Reg<u32, _EFUSE_RD_USR_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_USR_DATA6;
#[doc = "`read()` method returns [efuse_rd_usr_data6::R](efuse_rd_usr_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_USR_DATA6 {}
#[doc = "EFUSE_RD_USR_DATA6"]
pub mod efuse_rd_usr_data6;
#[doc = "EFUSE_RD_USR_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_usr_data7](efuse_rd_usr_data7) module"]
pub type EFUSE_RD_USR_DATA7 = crate::Reg<u32, _EFUSE_RD_USR_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_USR_DATA7;
#[doc = "`read()` method returns [efuse_rd_usr_data7::R](efuse_rd_usr_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_USR_DATA7 {}
#[doc = "EFUSE_RD_USR_DATA7"]
pub mod efuse_rd_usr_data7;
#[doc = "EFUSE_RD_KEY0_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key0_data0](efuse_rd_key0_data0) module"]
pub type EFUSE_RD_KEY0_DATA0 = crate::Reg<u32, _EFUSE_RD_KEY0_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY0_DATA0;
#[doc = "`read()` method returns [efuse_rd_key0_data0::R](efuse_rd_key0_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY0_DATA0 {}
#[doc = "EFUSE_RD_KEY0_DATA0"]
pub mod efuse_rd_key0_data0;
#[doc = "EFUSE_RD_KEY0_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key0_data1](efuse_rd_key0_data1) module"]
pub type EFUSE_RD_KEY0_DATA1 = crate::Reg<u32, _EFUSE_RD_KEY0_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY0_DATA1;
#[doc = "`read()` method returns [efuse_rd_key0_data1::R](efuse_rd_key0_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY0_DATA1 {}
#[doc = "EFUSE_RD_KEY0_DATA1"]
pub mod efuse_rd_key0_data1;
#[doc = "EFUSE_RD_KEY0_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key0_data2](efuse_rd_key0_data2) module"]
pub type EFUSE_RD_KEY0_DATA2 = crate::Reg<u32, _EFUSE_RD_KEY0_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY0_DATA2;
#[doc = "`read()` method returns [efuse_rd_key0_data2::R](efuse_rd_key0_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY0_DATA2 {}
#[doc = "EFUSE_RD_KEY0_DATA2"]
pub mod efuse_rd_key0_data2;
#[doc = "EFUSE_RD_KEY0_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key0_data3](efuse_rd_key0_data3) module"]
pub type EFUSE_RD_KEY0_DATA3 = crate::Reg<u32, _EFUSE_RD_KEY0_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY0_DATA3;
#[doc = "`read()` method returns [efuse_rd_key0_data3::R](efuse_rd_key0_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY0_DATA3 {}
#[doc = "EFUSE_RD_KEY0_DATA3"]
pub mod efuse_rd_key0_data3;
#[doc = "EFUSE_RD_KEY0_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key0_data4](efuse_rd_key0_data4) module"]
pub type EFUSE_RD_KEY0_DATA4 = crate::Reg<u32, _EFUSE_RD_KEY0_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY0_DATA4;
#[doc = "`read()` method returns [efuse_rd_key0_data4::R](efuse_rd_key0_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY0_DATA4 {}
#[doc = "EFUSE_RD_KEY0_DATA4"]
pub mod efuse_rd_key0_data4;
#[doc = "EFUSE_RD_KEY0_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key0_data5](efuse_rd_key0_data5) module"]
pub type EFUSE_RD_KEY0_DATA5 = crate::Reg<u32, _EFUSE_RD_KEY0_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY0_DATA5;
#[doc = "`read()` method returns [efuse_rd_key0_data5::R](efuse_rd_key0_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY0_DATA5 {}
#[doc = "EFUSE_RD_KEY0_DATA5"]
pub mod efuse_rd_key0_data5;
#[doc = "EFUSE_RD_KEY0_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key0_data6](efuse_rd_key0_data6) module"]
pub type EFUSE_RD_KEY0_DATA6 = crate::Reg<u32, _EFUSE_RD_KEY0_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY0_DATA6;
#[doc = "`read()` method returns [efuse_rd_key0_data6::R](efuse_rd_key0_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY0_DATA6 {}
#[doc = "EFUSE_RD_KEY0_DATA6"]
pub mod efuse_rd_key0_data6;
#[doc = "EFUSE_RD_KEY0_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key0_data7](efuse_rd_key0_data7) module"]
pub type EFUSE_RD_KEY0_DATA7 = crate::Reg<u32, _EFUSE_RD_KEY0_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY0_DATA7;
#[doc = "`read()` method returns [efuse_rd_key0_data7::R](efuse_rd_key0_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY0_DATA7 {}
#[doc = "EFUSE_RD_KEY0_DATA7"]
pub mod efuse_rd_key0_data7;
#[doc = "EFUSE_RD_KEY1_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key1_data0](efuse_rd_key1_data0) module"]
pub type EFUSE_RD_KEY1_DATA0 = crate::Reg<u32, _EFUSE_RD_KEY1_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY1_DATA0;
#[doc = "`read()` method returns [efuse_rd_key1_data0::R](efuse_rd_key1_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY1_DATA0 {}
#[doc = "EFUSE_RD_KEY1_DATA0"]
pub mod efuse_rd_key1_data0;
#[doc = "EFUSE_RD_KEY1_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key1_data1](efuse_rd_key1_data1) module"]
pub type EFUSE_RD_KEY1_DATA1 = crate::Reg<u32, _EFUSE_RD_KEY1_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY1_DATA1;
#[doc = "`read()` method returns [efuse_rd_key1_data1::R](efuse_rd_key1_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY1_DATA1 {}
#[doc = "EFUSE_RD_KEY1_DATA1"]
pub mod efuse_rd_key1_data1;
#[doc = "EFUSE_RD_KEY1_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key1_data2](efuse_rd_key1_data2) module"]
pub type EFUSE_RD_KEY1_DATA2 = crate::Reg<u32, _EFUSE_RD_KEY1_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY1_DATA2;
#[doc = "`read()` method returns [efuse_rd_key1_data2::R](efuse_rd_key1_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY1_DATA2 {}
#[doc = "EFUSE_RD_KEY1_DATA2"]
pub mod efuse_rd_key1_data2;
#[doc = "EFUSE_RD_KEY1_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key1_data3](efuse_rd_key1_data3) module"]
pub type EFUSE_RD_KEY1_DATA3 = crate::Reg<u32, _EFUSE_RD_KEY1_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY1_DATA3;
#[doc = "`read()` method returns [efuse_rd_key1_data3::R](efuse_rd_key1_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY1_DATA3 {}
#[doc = "EFUSE_RD_KEY1_DATA3"]
pub mod efuse_rd_key1_data3;
#[doc = "EFUSE_RD_KEY1_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key1_data4](efuse_rd_key1_data4) module"]
pub type EFUSE_RD_KEY1_DATA4 = crate::Reg<u32, _EFUSE_RD_KEY1_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY1_DATA4;
#[doc = "`read()` method returns [efuse_rd_key1_data4::R](efuse_rd_key1_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY1_DATA4 {}
#[doc = "EFUSE_RD_KEY1_DATA4"]
pub mod efuse_rd_key1_data4;
#[doc = "EFUSE_RD_KEY1_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key1_data5](efuse_rd_key1_data5) module"]
pub type EFUSE_RD_KEY1_DATA5 = crate::Reg<u32, _EFUSE_RD_KEY1_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY1_DATA5;
#[doc = "`read()` method returns [efuse_rd_key1_data5::R](efuse_rd_key1_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY1_DATA5 {}
#[doc = "EFUSE_RD_KEY1_DATA5"]
pub mod efuse_rd_key1_data5;
#[doc = "EFUSE_RD_KEY1_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key1_data6](efuse_rd_key1_data6) module"]
pub type EFUSE_RD_KEY1_DATA6 = crate::Reg<u32, _EFUSE_RD_KEY1_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY1_DATA6;
#[doc = "`read()` method returns [efuse_rd_key1_data6::R](efuse_rd_key1_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY1_DATA6 {}
#[doc = "EFUSE_RD_KEY1_DATA6"]
pub mod efuse_rd_key1_data6;
#[doc = "EFUSE_RD_KEY1_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key1_data7](efuse_rd_key1_data7) module"]
pub type EFUSE_RD_KEY1_DATA7 = crate::Reg<u32, _EFUSE_RD_KEY1_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY1_DATA7;
#[doc = "`read()` method returns [efuse_rd_key1_data7::R](efuse_rd_key1_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY1_DATA7 {}
#[doc = "EFUSE_RD_KEY1_DATA7"]
pub mod efuse_rd_key1_data7;
#[doc = "EFUSE_RD_KEY2_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key2_data0](efuse_rd_key2_data0) module"]
pub type EFUSE_RD_KEY2_DATA0 = crate::Reg<u32, _EFUSE_RD_KEY2_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY2_DATA0;
#[doc = "`read()` method returns [efuse_rd_key2_data0::R](efuse_rd_key2_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY2_DATA0 {}
#[doc = "EFUSE_RD_KEY2_DATA0"]
pub mod efuse_rd_key2_data0;
#[doc = "EFUSE_RD_KEY2_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key2_data1](efuse_rd_key2_data1) module"]
pub type EFUSE_RD_KEY2_DATA1 = crate::Reg<u32, _EFUSE_RD_KEY2_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY2_DATA1;
#[doc = "`read()` method returns [efuse_rd_key2_data1::R](efuse_rd_key2_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY2_DATA1 {}
#[doc = "EFUSE_RD_KEY2_DATA1"]
pub mod efuse_rd_key2_data1;
#[doc = "EFUSE_RD_KEY2_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key2_data2](efuse_rd_key2_data2) module"]
pub type EFUSE_RD_KEY2_DATA2 = crate::Reg<u32, _EFUSE_RD_KEY2_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY2_DATA2;
#[doc = "`read()` method returns [efuse_rd_key2_data2::R](efuse_rd_key2_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY2_DATA2 {}
#[doc = "EFUSE_RD_KEY2_DATA2"]
pub mod efuse_rd_key2_data2;
#[doc = "EFUSE_RD_KEY2_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key2_data3](efuse_rd_key2_data3) module"]
pub type EFUSE_RD_KEY2_DATA3 = crate::Reg<u32, _EFUSE_RD_KEY2_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY2_DATA3;
#[doc = "`read()` method returns [efuse_rd_key2_data3::R](efuse_rd_key2_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY2_DATA3 {}
#[doc = "EFUSE_RD_KEY2_DATA3"]
pub mod efuse_rd_key2_data3;
#[doc = "EFUSE_RD_KEY2_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key2_data4](efuse_rd_key2_data4) module"]
pub type EFUSE_RD_KEY2_DATA4 = crate::Reg<u32, _EFUSE_RD_KEY2_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY2_DATA4;
#[doc = "`read()` method returns [efuse_rd_key2_data4::R](efuse_rd_key2_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY2_DATA4 {}
#[doc = "EFUSE_RD_KEY2_DATA4"]
pub mod efuse_rd_key2_data4;
#[doc = "EFUSE_RD_KEY2_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key2_data5](efuse_rd_key2_data5) module"]
pub type EFUSE_RD_KEY2_DATA5 = crate::Reg<u32, _EFUSE_RD_KEY2_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY2_DATA5;
#[doc = "`read()` method returns [efuse_rd_key2_data5::R](efuse_rd_key2_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY2_DATA5 {}
#[doc = "EFUSE_RD_KEY2_DATA5"]
pub mod efuse_rd_key2_data5;
#[doc = "EFUSE_RD_KEY2_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key2_data6](efuse_rd_key2_data6) module"]
pub type EFUSE_RD_KEY2_DATA6 = crate::Reg<u32, _EFUSE_RD_KEY2_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY2_DATA6;
#[doc = "`read()` method returns [efuse_rd_key2_data6::R](efuse_rd_key2_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY2_DATA6 {}
#[doc = "EFUSE_RD_KEY2_DATA6"]
pub mod efuse_rd_key2_data6;
#[doc = "EFUSE_RD_KEY2_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key2_data7](efuse_rd_key2_data7) module"]
pub type EFUSE_RD_KEY2_DATA7 = crate::Reg<u32, _EFUSE_RD_KEY2_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY2_DATA7;
#[doc = "`read()` method returns [efuse_rd_key2_data7::R](efuse_rd_key2_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY2_DATA7 {}
#[doc = "EFUSE_RD_KEY2_DATA7"]
pub mod efuse_rd_key2_data7;
#[doc = "EFUSE_RD_KEY3_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key3_data0](efuse_rd_key3_data0) module"]
pub type EFUSE_RD_KEY3_DATA0 = crate::Reg<u32, _EFUSE_RD_KEY3_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY3_DATA0;
#[doc = "`read()` method returns [efuse_rd_key3_data0::R](efuse_rd_key3_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY3_DATA0 {}
#[doc = "EFUSE_RD_KEY3_DATA0"]
pub mod efuse_rd_key3_data0;
#[doc = "EFUSE_RD_KEY3_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key3_data1](efuse_rd_key3_data1) module"]
pub type EFUSE_RD_KEY3_DATA1 = crate::Reg<u32, _EFUSE_RD_KEY3_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY3_DATA1;
#[doc = "`read()` method returns [efuse_rd_key3_data1::R](efuse_rd_key3_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY3_DATA1 {}
#[doc = "EFUSE_RD_KEY3_DATA1"]
pub mod efuse_rd_key3_data1;
#[doc = "EFUSE_RD_KEY3_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key3_data2](efuse_rd_key3_data2) module"]
pub type EFUSE_RD_KEY3_DATA2 = crate::Reg<u32, _EFUSE_RD_KEY3_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY3_DATA2;
#[doc = "`read()` method returns [efuse_rd_key3_data2::R](efuse_rd_key3_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY3_DATA2 {}
#[doc = "EFUSE_RD_KEY3_DATA2"]
pub mod efuse_rd_key3_data2;
#[doc = "EFUSE_RD_KEY3_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key3_data3](efuse_rd_key3_data3) module"]
pub type EFUSE_RD_KEY3_DATA3 = crate::Reg<u32, _EFUSE_RD_KEY3_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY3_DATA3;
#[doc = "`read()` method returns [efuse_rd_key3_data3::R](efuse_rd_key3_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY3_DATA3 {}
#[doc = "EFUSE_RD_KEY3_DATA3"]
pub mod efuse_rd_key3_data3;
#[doc = "EFUSE_RD_KEY3_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key3_data4](efuse_rd_key3_data4) module"]
pub type EFUSE_RD_KEY3_DATA4 = crate::Reg<u32, _EFUSE_RD_KEY3_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY3_DATA4;
#[doc = "`read()` method returns [efuse_rd_key3_data4::R](efuse_rd_key3_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY3_DATA4 {}
#[doc = "EFUSE_RD_KEY3_DATA4"]
pub mod efuse_rd_key3_data4;
#[doc = "EFUSE_RD_KEY3_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key3_data5](efuse_rd_key3_data5) module"]
pub type EFUSE_RD_KEY3_DATA5 = crate::Reg<u32, _EFUSE_RD_KEY3_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY3_DATA5;
#[doc = "`read()` method returns [efuse_rd_key3_data5::R](efuse_rd_key3_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY3_DATA5 {}
#[doc = "EFUSE_RD_KEY3_DATA5"]
pub mod efuse_rd_key3_data5;
#[doc = "EFUSE_RD_KEY3_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key3_data6](efuse_rd_key3_data6) module"]
pub type EFUSE_RD_KEY3_DATA6 = crate::Reg<u32, _EFUSE_RD_KEY3_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY3_DATA6;
#[doc = "`read()` method returns [efuse_rd_key3_data6::R](efuse_rd_key3_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY3_DATA6 {}
#[doc = "EFUSE_RD_KEY3_DATA6"]
pub mod efuse_rd_key3_data6;
#[doc = "EFUSE_RD_KEY3_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key3_data7](efuse_rd_key3_data7) module"]
pub type EFUSE_RD_KEY3_DATA7 = crate::Reg<u32, _EFUSE_RD_KEY3_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY3_DATA7;
#[doc = "`read()` method returns [efuse_rd_key3_data7::R](efuse_rd_key3_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY3_DATA7 {}
#[doc = "EFUSE_RD_KEY3_DATA7"]
pub mod efuse_rd_key3_data7;
#[doc = "EFUSE_RD_KEY4_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key4_data0](efuse_rd_key4_data0) module"]
pub type EFUSE_RD_KEY4_DATA0 = crate::Reg<u32, _EFUSE_RD_KEY4_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY4_DATA0;
#[doc = "`read()` method returns [efuse_rd_key4_data0::R](efuse_rd_key4_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY4_DATA0 {}
#[doc = "EFUSE_RD_KEY4_DATA0"]
pub mod efuse_rd_key4_data0;
#[doc = "EFUSE_RD_KEY4_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key4_data1](efuse_rd_key4_data1) module"]
pub type EFUSE_RD_KEY4_DATA1 = crate::Reg<u32, _EFUSE_RD_KEY4_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY4_DATA1;
#[doc = "`read()` method returns [efuse_rd_key4_data1::R](efuse_rd_key4_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY4_DATA1 {}
#[doc = "EFUSE_RD_KEY4_DATA1"]
pub mod efuse_rd_key4_data1;
#[doc = "EFUSE_RD_KEY4_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key4_data2](efuse_rd_key4_data2) module"]
pub type EFUSE_RD_KEY4_DATA2 = crate::Reg<u32, _EFUSE_RD_KEY4_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY4_DATA2;
#[doc = "`read()` method returns [efuse_rd_key4_data2::R](efuse_rd_key4_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY4_DATA2 {}
#[doc = "EFUSE_RD_KEY4_DATA2"]
pub mod efuse_rd_key4_data2;
#[doc = "EFUSE_RD_KEY4_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key4_data3](efuse_rd_key4_data3) module"]
pub type EFUSE_RD_KEY4_DATA3 = crate::Reg<u32, _EFUSE_RD_KEY4_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY4_DATA3;
#[doc = "`read()` method returns [efuse_rd_key4_data3::R](efuse_rd_key4_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY4_DATA3 {}
#[doc = "EFUSE_RD_KEY4_DATA3"]
pub mod efuse_rd_key4_data3;
#[doc = "EFUSE_RD_KEY4_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key4_data4](efuse_rd_key4_data4) module"]
pub type EFUSE_RD_KEY4_DATA4 = crate::Reg<u32, _EFUSE_RD_KEY4_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY4_DATA4;
#[doc = "`read()` method returns [efuse_rd_key4_data4::R](efuse_rd_key4_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY4_DATA4 {}
#[doc = "EFUSE_RD_KEY4_DATA4"]
pub mod efuse_rd_key4_data4;
#[doc = "EFUSE_RD_KEY4_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key4_data5](efuse_rd_key4_data5) module"]
pub type EFUSE_RD_KEY4_DATA5 = crate::Reg<u32, _EFUSE_RD_KEY4_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY4_DATA5;
#[doc = "`read()` method returns [efuse_rd_key4_data5::R](efuse_rd_key4_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY4_DATA5 {}
#[doc = "EFUSE_RD_KEY4_DATA5"]
pub mod efuse_rd_key4_data5;
#[doc = "EFUSE_RD_KEY4_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key4_data6](efuse_rd_key4_data6) module"]
pub type EFUSE_RD_KEY4_DATA6 = crate::Reg<u32, _EFUSE_RD_KEY4_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY4_DATA6;
#[doc = "`read()` method returns [efuse_rd_key4_data6::R](efuse_rd_key4_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY4_DATA6 {}
#[doc = "EFUSE_RD_KEY4_DATA6"]
pub mod efuse_rd_key4_data6;
#[doc = "EFUSE_RD_KEY4_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key4_data7](efuse_rd_key4_data7) module"]
pub type EFUSE_RD_KEY4_DATA7 = crate::Reg<u32, _EFUSE_RD_KEY4_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY4_DATA7;
#[doc = "`read()` method returns [efuse_rd_key4_data7::R](efuse_rd_key4_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY4_DATA7 {}
#[doc = "EFUSE_RD_KEY4_DATA7"]
pub mod efuse_rd_key4_data7;
#[doc = "EFUSE_RD_KEY5_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key5_data0](efuse_rd_key5_data0) module"]
pub type EFUSE_RD_KEY5_DATA0 = crate::Reg<u32, _EFUSE_RD_KEY5_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY5_DATA0;
#[doc = "`read()` method returns [efuse_rd_key5_data0::R](efuse_rd_key5_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY5_DATA0 {}
#[doc = "EFUSE_RD_KEY5_DATA0"]
pub mod efuse_rd_key5_data0;
#[doc = "EFUSE_RD_KEY5_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key5_data1](efuse_rd_key5_data1) module"]
pub type EFUSE_RD_KEY5_DATA1 = crate::Reg<u32, _EFUSE_RD_KEY5_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY5_DATA1;
#[doc = "`read()` method returns [efuse_rd_key5_data1::R](efuse_rd_key5_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY5_DATA1 {}
#[doc = "EFUSE_RD_KEY5_DATA1"]
pub mod efuse_rd_key5_data1;
#[doc = "EFUSE_RD_KEY5_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key5_data2](efuse_rd_key5_data2) module"]
pub type EFUSE_RD_KEY5_DATA2 = crate::Reg<u32, _EFUSE_RD_KEY5_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY5_DATA2;
#[doc = "`read()` method returns [efuse_rd_key5_data2::R](efuse_rd_key5_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY5_DATA2 {}
#[doc = "EFUSE_RD_KEY5_DATA2"]
pub mod efuse_rd_key5_data2;
#[doc = "EFUSE_RD_KEY5_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key5_data3](efuse_rd_key5_data3) module"]
pub type EFUSE_RD_KEY5_DATA3 = crate::Reg<u32, _EFUSE_RD_KEY5_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY5_DATA3;
#[doc = "`read()` method returns [efuse_rd_key5_data3::R](efuse_rd_key5_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY5_DATA3 {}
#[doc = "EFUSE_RD_KEY5_DATA3"]
pub mod efuse_rd_key5_data3;
#[doc = "EFUSE_RD_KEY5_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key5_data4](efuse_rd_key5_data4) module"]
pub type EFUSE_RD_KEY5_DATA4 = crate::Reg<u32, _EFUSE_RD_KEY5_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY5_DATA4;
#[doc = "`read()` method returns [efuse_rd_key5_data4::R](efuse_rd_key5_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY5_DATA4 {}
#[doc = "EFUSE_RD_KEY5_DATA4"]
pub mod efuse_rd_key5_data4;
#[doc = "EFUSE_RD_KEY5_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key5_data5](efuse_rd_key5_data5) module"]
pub type EFUSE_RD_KEY5_DATA5 = crate::Reg<u32, _EFUSE_RD_KEY5_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY5_DATA5;
#[doc = "`read()` method returns [efuse_rd_key5_data5::R](efuse_rd_key5_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY5_DATA5 {}
#[doc = "EFUSE_RD_KEY5_DATA5"]
pub mod efuse_rd_key5_data5;
#[doc = "EFUSE_RD_KEY5_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key5_data6](efuse_rd_key5_data6) module"]
pub type EFUSE_RD_KEY5_DATA6 = crate::Reg<u32, _EFUSE_RD_KEY5_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY5_DATA6;
#[doc = "`read()` method returns [efuse_rd_key5_data6::R](efuse_rd_key5_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY5_DATA6 {}
#[doc = "EFUSE_RD_KEY5_DATA6"]
pub mod efuse_rd_key5_data6;
#[doc = "EFUSE_RD_KEY5_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_key5_data7](efuse_rd_key5_data7) module"]
pub type EFUSE_RD_KEY5_DATA7 = crate::Reg<u32, _EFUSE_RD_KEY5_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_KEY5_DATA7;
#[doc = "`read()` method returns [efuse_rd_key5_data7::R](efuse_rd_key5_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_KEY5_DATA7 {}
#[doc = "EFUSE_RD_KEY5_DATA7"]
pub mod efuse_rd_key5_data7;
#[doc = "EFUSE_RD_SYS_PART2_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part2_data0](efuse_rd_sys_part2_data0) module"]
pub type EFUSE_RD_SYS_PART2_DATA0 = crate::Reg<u32, _EFUSE_RD_SYS_PART2_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART2_DATA0;
#[doc = "`read()` method returns [efuse_rd_sys_part2_data0::R](efuse_rd_sys_part2_data0::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART2_DATA0 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA0"]
pub mod efuse_rd_sys_part2_data0;
#[doc = "EFUSE_RD_SYS_PART2_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part2_data1](efuse_rd_sys_part2_data1) module"]
pub type EFUSE_RD_SYS_PART2_DATA1 = crate::Reg<u32, _EFUSE_RD_SYS_PART2_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART2_DATA1;
#[doc = "`read()` method returns [efuse_rd_sys_part2_data1::R](efuse_rd_sys_part2_data1::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART2_DATA1 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA1"]
pub mod efuse_rd_sys_part2_data1;
#[doc = "EFUSE_RD_SYS_PART2_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part2_data2](efuse_rd_sys_part2_data2) module"]
pub type EFUSE_RD_SYS_PART2_DATA2 = crate::Reg<u32, _EFUSE_RD_SYS_PART2_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART2_DATA2;
#[doc = "`read()` method returns [efuse_rd_sys_part2_data2::R](efuse_rd_sys_part2_data2::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART2_DATA2 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA2"]
pub mod efuse_rd_sys_part2_data2;
#[doc = "EFUSE_RD_SYS_PART2_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part2_data3](efuse_rd_sys_part2_data3) module"]
pub type EFUSE_RD_SYS_PART2_DATA3 = crate::Reg<u32, _EFUSE_RD_SYS_PART2_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART2_DATA3;
#[doc = "`read()` method returns [efuse_rd_sys_part2_data3::R](efuse_rd_sys_part2_data3::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART2_DATA3 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA3"]
pub mod efuse_rd_sys_part2_data3;
#[doc = "EFUSE_RD_SYS_PART2_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part2_data4](efuse_rd_sys_part2_data4) module"]
pub type EFUSE_RD_SYS_PART2_DATA4 = crate::Reg<u32, _EFUSE_RD_SYS_PART2_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART2_DATA4;
#[doc = "`read()` method returns [efuse_rd_sys_part2_data4::R](efuse_rd_sys_part2_data4::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART2_DATA4 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA4"]
pub mod efuse_rd_sys_part2_data4;
#[doc = "EFUSE_RD_SYS_PART2_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part2_data5](efuse_rd_sys_part2_data5) module"]
pub type EFUSE_RD_SYS_PART2_DATA5 = crate::Reg<u32, _EFUSE_RD_SYS_PART2_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART2_DATA5;
#[doc = "`read()` method returns [efuse_rd_sys_part2_data5::R](efuse_rd_sys_part2_data5::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART2_DATA5 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA5"]
pub mod efuse_rd_sys_part2_data5;
#[doc = "EFUSE_RD_SYS_PART2_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part2_data6](efuse_rd_sys_part2_data6) module"]
pub type EFUSE_RD_SYS_PART2_DATA6 = crate::Reg<u32, _EFUSE_RD_SYS_PART2_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART2_DATA6;
#[doc = "`read()` method returns [efuse_rd_sys_part2_data6::R](efuse_rd_sys_part2_data6::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART2_DATA6 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA6"]
pub mod efuse_rd_sys_part2_data6;
#[doc = "EFUSE_RD_SYS_PART2_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_sys_part2_data7](efuse_rd_sys_part2_data7) module"]
pub type EFUSE_RD_SYS_PART2_DATA7 = crate::Reg<u32, _EFUSE_RD_SYS_PART2_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_SYS_PART2_DATA7;
#[doc = "`read()` method returns [efuse_rd_sys_part2_data7::R](efuse_rd_sys_part2_data7::R) reader structure"]
impl crate::Readable for EFUSE_RD_SYS_PART2_DATA7 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA7"]
pub mod efuse_rd_sys_part2_data7;
#[doc = "EFUSE_RD_REPEAT_ERR0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_err0](efuse_rd_repeat_err0) module"]
pub type EFUSE_RD_REPEAT_ERR0 = crate::Reg<u32, _EFUSE_RD_REPEAT_ERR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_ERR0;
#[doc = "`read()` method returns [efuse_rd_repeat_err0::R](efuse_rd_repeat_err0::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_ERR0 {}
#[doc = "EFUSE_RD_REPEAT_ERR0"]
pub mod efuse_rd_repeat_err0;
#[doc = "EFUSE_RD_REPEAT_ERR1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_err1](efuse_rd_repeat_err1) module"]
pub type EFUSE_RD_REPEAT_ERR1 = crate::Reg<u32, _EFUSE_RD_REPEAT_ERR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_ERR1;
#[doc = "`read()` method returns [efuse_rd_repeat_err1::R](efuse_rd_repeat_err1::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_ERR1 {}
#[doc = "EFUSE_RD_REPEAT_ERR1"]
pub mod efuse_rd_repeat_err1;
#[doc = "EFUSE_RD_REPEAT_ERR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_err2](efuse_rd_repeat_err2) module"]
pub type EFUSE_RD_REPEAT_ERR2 = crate::Reg<u32, _EFUSE_RD_REPEAT_ERR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_ERR2;
#[doc = "`read()` method returns [efuse_rd_repeat_err2::R](efuse_rd_repeat_err2::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_ERR2 {}
#[doc = "EFUSE_RD_REPEAT_ERR2"]
pub mod efuse_rd_repeat_err2;
#[doc = "EFUSE_RD_REPEAT_ERR3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_err3](efuse_rd_repeat_err3) module"]
pub type EFUSE_RD_REPEAT_ERR3 = crate::Reg<u32, _EFUSE_RD_REPEAT_ERR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_ERR3;
#[doc = "`read()` method returns [efuse_rd_repeat_err3::R](efuse_rd_repeat_err3::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_ERR3 {}
#[doc = "EFUSE_RD_REPEAT_ERR3"]
pub mod efuse_rd_repeat_err3;
#[doc = "EFUSE_RD_REPEAT_ERR4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_repeat_err4](efuse_rd_repeat_err4) module"]
pub type EFUSE_RD_REPEAT_ERR4 = crate::Reg<u32, _EFUSE_RD_REPEAT_ERR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_REPEAT_ERR4;
#[doc = "`read()` method returns [efuse_rd_repeat_err4::R](efuse_rd_repeat_err4::R) reader structure"]
impl crate::Readable for EFUSE_RD_REPEAT_ERR4 {}
#[doc = "EFUSE_RD_REPEAT_ERR4"]
pub mod efuse_rd_repeat_err4;
#[doc = "EFUSE_RD_RS_ERR0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_rs_err0](efuse_rd_rs_err0) module"]
pub type EFUSE_RD_RS_ERR0 = crate::Reg<u32, _EFUSE_RD_RS_ERR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_RS_ERR0;
#[doc = "`read()` method returns [efuse_rd_rs_err0::R](efuse_rd_rs_err0::R) reader structure"]
impl crate::Readable for EFUSE_RD_RS_ERR0 {}
#[doc = "EFUSE_RD_RS_ERR0"]
pub mod efuse_rd_rs_err0;
#[doc = "EFUSE_RD_RS_ERR1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_rs_err1](efuse_rd_rs_err1) module"]
pub type EFUSE_RD_RS_ERR1 = crate::Reg<u32, _EFUSE_RD_RS_ERR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_RS_ERR1;
#[doc = "`read()` method returns [efuse_rd_rs_err1::R](efuse_rd_rs_err1::R) reader structure"]
impl crate::Readable for EFUSE_RD_RS_ERR1 {}
#[doc = "EFUSE_RD_RS_ERR1"]
pub mod efuse_rd_rs_err1;
#[doc = "EFUSE_CLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_clk](efuse_clk) module"]
pub type EFUSE_CLK = crate::Reg<u32, _EFUSE_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_CLK;
#[doc = "`read()` method returns [efuse_clk::R](efuse_clk::R) reader structure"]
impl crate::Readable for EFUSE_CLK {}
#[doc = "`write(|w| ..)` method takes [efuse_clk::W](efuse_clk::W) writer structure"]
impl crate::Writable for EFUSE_CLK {}
#[doc = "EFUSE_CLK"]
pub mod efuse_clk;
#[doc = "EFUSE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_conf](efuse_conf) module"]
pub type EFUSE_CONF = crate::Reg<u32, _EFUSE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_CONF;
#[doc = "`read()` method returns [efuse_conf::R](efuse_conf::R) reader structure"]
impl crate::Readable for EFUSE_CONF {}
#[doc = "`write(|w| ..)` method takes [efuse_conf::W](efuse_conf::W) writer structure"]
impl crate::Writable for EFUSE_CONF {}
#[doc = "EFUSE_CONF"]
pub mod efuse_conf;
#[doc = "EFUSE_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_status](efuse_status) module"]
pub type EFUSE_STATUS = crate::Reg<u32, _EFUSE_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_STATUS;
#[doc = "`read()` method returns [efuse_status::R](efuse_status::R) reader structure"]
impl crate::Readable for EFUSE_STATUS {}
#[doc = "EFUSE_STATUS"]
pub mod efuse_status;
#[doc = "EFUSE_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_cmd](efuse_cmd) module"]
pub type EFUSE_CMD = crate::Reg<u32, _EFUSE_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_CMD;
#[doc = "`read()` method returns [efuse_cmd::R](efuse_cmd::R) reader structure"]
impl crate::Readable for EFUSE_CMD {}
#[doc = "`write(|w| ..)` method takes [efuse_cmd::W](efuse_cmd::W) writer structure"]
impl crate::Writable for EFUSE_CMD {}
#[doc = "EFUSE_CMD"]
pub mod efuse_cmd;
#[doc = "EFUSE_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_int_raw](efuse_int_raw) module"]
pub type EFUSE_INT_RAW = crate::Reg<u32, _EFUSE_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_INT_RAW;
#[doc = "`read()` method returns [efuse_int_raw::R](efuse_int_raw::R) reader structure"]
impl crate::Readable for EFUSE_INT_RAW {}
#[doc = "EFUSE_INT_RAW"]
pub mod efuse_int_raw;
#[doc = "EFUSE_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_int_st](efuse_int_st) module"]
pub type EFUSE_INT_ST = crate::Reg<u32, _EFUSE_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_INT_ST;
#[doc = "`read()` method returns [efuse_int_st::R](efuse_int_st::R) reader structure"]
impl crate::Readable for EFUSE_INT_ST {}
#[doc = "EFUSE_INT_ST"]
pub mod efuse_int_st;
#[doc = "EFUSE_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_int_ena](efuse_int_ena) module"]
pub type EFUSE_INT_ENA = crate::Reg<u32, _EFUSE_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_INT_ENA;
#[doc = "`read()` method returns [efuse_int_ena::R](efuse_int_ena::R) reader structure"]
impl crate::Readable for EFUSE_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [efuse_int_ena::W](efuse_int_ena::W) writer structure"]
impl crate::Writable for EFUSE_INT_ENA {}
#[doc = "EFUSE_INT_ENA"]
pub mod efuse_int_ena;
#[doc = "EFUSE_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_int_clr](efuse_int_clr) module"]
pub type EFUSE_INT_CLR = crate::Reg<u32, _EFUSE_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_INT_CLR;
#[doc = "`write(|w| ..)` method takes [efuse_int_clr::W](efuse_int_clr::W) writer structure"]
impl crate::Writable for EFUSE_INT_CLR {}
#[doc = "EFUSE_INT_CLR"]
pub mod efuse_int_clr;
#[doc = "EFUSE_DAC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_dac_conf](efuse_dac_conf) module"]
pub type EFUSE_DAC_CONF = crate::Reg<u32, _EFUSE_DAC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_DAC_CONF;
#[doc = "`read()` method returns [efuse_dac_conf::R](efuse_dac_conf::R) reader structure"]
impl crate::Readable for EFUSE_DAC_CONF {}
#[doc = "`write(|w| ..)` method takes [efuse_dac_conf::W](efuse_dac_conf::W) writer structure"]
impl crate::Writable for EFUSE_DAC_CONF {}
#[doc = "EFUSE_DAC_CONF"]
pub mod efuse_dac_conf;
#[doc = "EFUSE_RD_TIM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rd_tim_conf](efuse_rd_tim_conf) module"]
pub type EFUSE_RD_TIM_CONF = crate::Reg<u32, _EFUSE_RD_TIM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RD_TIM_CONF;
#[doc = "`read()` method returns [efuse_rd_tim_conf::R](efuse_rd_tim_conf::R) reader structure"]
impl crate::Readable for EFUSE_RD_TIM_CONF {}
#[doc = "`write(|w| ..)` method takes [efuse_rd_tim_conf::W](efuse_rd_tim_conf::W) writer structure"]
impl crate::Writable for EFUSE_RD_TIM_CONF {}
#[doc = "EFUSE_RD_TIM_CONF"]
pub mod efuse_rd_tim_conf;
#[doc = "EFUSE_WR_TIM_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_wr_tim_conf1](efuse_wr_tim_conf1) module"]
pub type EFUSE_WR_TIM_CONF1 = crate::Reg<u32, _EFUSE_WR_TIM_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_WR_TIM_CONF1;
#[doc = "`read()` method returns [efuse_wr_tim_conf1::R](efuse_wr_tim_conf1::R) reader structure"]
impl crate::Readable for EFUSE_WR_TIM_CONF1 {}
#[doc = "`write(|w| ..)` method takes [efuse_wr_tim_conf1::W](efuse_wr_tim_conf1::W) writer structure"]
impl crate::Writable for EFUSE_WR_TIM_CONF1 {}
#[doc = "EFUSE_WR_TIM_CONF1"]
pub mod efuse_wr_tim_conf1;
#[doc = "EFUSE_WR_TIM_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_wr_tim_conf2](efuse_wr_tim_conf2) module"]
pub type EFUSE_WR_TIM_CONF2 = crate::Reg<u32, _EFUSE_WR_TIM_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_WR_TIM_CONF2;
#[doc = "`read()` method returns [efuse_wr_tim_conf2::R](efuse_wr_tim_conf2::R) reader structure"]
impl crate::Readable for EFUSE_WR_TIM_CONF2 {}
#[doc = "`write(|w| ..)` method takes [efuse_wr_tim_conf2::W](efuse_wr_tim_conf2::W) writer structure"]
impl crate::Writable for EFUSE_WR_TIM_CONF2 {}
#[doc = "EFUSE_WR_TIM_CONF2"]
pub mod efuse_wr_tim_conf2;
#[doc = "EFUSE_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_date](efuse_date) module"]
pub type EFUSE_DATE = crate::Reg<u32, _EFUSE_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_DATE;
#[doc = "`read()` method returns [efuse_date::R](efuse_date::R) reader structure"]
impl crate::Readable for EFUSE_DATE {}
#[doc = "`write(|w| ..)` method takes [efuse_date::W](efuse_date::W) writer structure"]
impl crate::Writable for EFUSE_DATE {}
#[doc = "EFUSE_DATE"]
pub mod efuse_date;
