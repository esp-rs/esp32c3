#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_INT_RAW_CH0"]
    pub dma_int_raw_ch0: DMA_INT_RAW_CH0,
    #[doc = "0x04 - DMA_INT_ST_CH0"]
    pub dma_int_st_ch0: DMA_INT_ST_CH0,
    #[doc = "0x08 - DMA_INT_ENA_CH0"]
    pub dma_int_ena_ch0: DMA_INT_ENA_CH0,
    #[doc = "0x0c - DMA_INT_CLR_CH0"]
    pub dma_int_clr_ch0: DMA_INT_CLR_CH0,
    #[doc = "0x10 - DMA_INT_RAW_CH1"]
    pub dma_int_raw_ch1: DMA_INT_RAW_CH1,
    #[doc = "0x14 - DMA_INT_ST_CH1"]
    pub dma_int_st_ch1: DMA_INT_ST_CH1,
    #[doc = "0x18 - DMA_INT_ENA_CH1"]
    pub dma_int_ena_ch1: DMA_INT_ENA_CH1,
    #[doc = "0x1c - DMA_INT_CLR_CH1"]
    pub dma_int_clr_ch1: DMA_INT_CLR_CH1,
    #[doc = "0x20 - DMA_INT_RAW_CH2"]
    pub dma_int_raw_ch2: DMA_INT_RAW_CH2,
    #[doc = "0x24 - DMA_INT_ST_CH2"]
    pub dma_int_st_ch2: DMA_INT_ST_CH2,
    #[doc = "0x28 - DMA_INT_ENA_CH2"]
    pub dma_int_ena_ch2: DMA_INT_ENA_CH2,
    #[doc = "0x2c - DMA_INT_CLR_CH2"]
    pub dma_int_clr_ch2: DMA_INT_CLR_CH2,
    _reserved12: [u8; 16usize],
    #[doc = "0x40 - DMA_AHB_TEST"]
    pub dma_ahb_test: DMA_AHB_TEST,
    #[doc = "0x44 - DMA_MISC_CONF"]
    pub dma_misc_conf: DMA_MISC_CONF,
    #[doc = "0x48 - DMA_DATE"]
    pub dma_date: DMA_DATE,
    _reserved15: [u8; 36usize],
    #[doc = "0x70 - DMA_IN_CONF0_CH0"]
    pub dma_in_conf0_ch0: DMA_IN_CONF0_CH0,
    #[doc = "0x74 - DMA_IN_CONF1_CH0"]
    pub dma_in_conf1_ch0: DMA_IN_CONF1_CH0,
    #[doc = "0x78 - DMA_INFIFO_STATUS_CH0"]
    pub dma_infifo_status_ch0: DMA_INFIFO_STATUS_CH0,
    #[doc = "0x7c - DMA_IN_POP_CH0"]
    pub dma_in_pop_ch0: DMA_IN_POP_CH0,
    #[doc = "0x80 - DMA_IN_LINK_CH0"]
    pub dma_in_link_ch0: DMA_IN_LINK_CH0,
    #[doc = "0x84 - DMA_IN_STATE_CH0"]
    pub dma_in_state_ch0: DMA_IN_STATE_CH0,
    #[doc = "0x88 - DMA_IN_SUC_EOF_DES_ADDR_CH0"]
    pub dma_in_suc_eof_des_addr_ch0: DMA_IN_SUC_EOF_DES_ADDR_CH0,
    #[doc = "0x8c - DMA_IN_ERR_EOF_DES_ADDR_CH0"]
    pub dma_in_err_eof_des_addr_ch0: DMA_IN_ERR_EOF_DES_ADDR_CH0,
    #[doc = "0x90 - DMA_IN_DSCR_CH0"]
    pub dma_in_dscr_ch0: DMA_IN_DSCR_CH0,
    #[doc = "0x94 - DMA_IN_DSCR_BF0_CH0"]
    pub dma_in_dscr_bf0_ch0: DMA_IN_DSCR_BF0_CH0,
    #[doc = "0x98 - DMA_IN_DSCR_BF1_CH0"]
    pub dma_in_dscr_bf1_ch0: DMA_IN_DSCR_BF1_CH0,
    #[doc = "0x9c - DMA_IN_PRI_CH0"]
    pub dma_in_pri_ch0: DMA_IN_PRI_CH0,
    #[doc = "0xa0 - DMA_IN_PERI_SEL_CH0"]
    pub dma_in_peri_sel_ch0: DMA_IN_PERI_SEL_CH0,
    _reserved28: [u8; 44usize],
    #[doc = "0xd0 - DMA_OUT_CONF0_CH0"]
    pub dma_out_conf0_ch0: DMA_OUT_CONF0_CH0,
    #[doc = "0xd4 - DMA_OUT_CONF1_CH0"]
    pub dma_out_conf1_ch0: DMA_OUT_CONF1_CH0,
    #[doc = "0xd8 - DMA_OUTFIFO_STATUS_CH0"]
    pub dma_outfifo_status_ch0: DMA_OUTFIFO_STATUS_CH0,
    #[doc = "0xdc - DMA_OUT_PUSH_CH0"]
    pub dma_out_push_ch0: DMA_OUT_PUSH_CH0,
    #[doc = "0xe0 - DMA_OUT_LINK_CH0"]
    pub dma_out_link_ch0: DMA_OUT_LINK_CH0,
    #[doc = "0xe4 - DMA_OUT_STATE_CH0"]
    pub dma_out_state_ch0: DMA_OUT_STATE_CH0,
    #[doc = "0xe8 - DMA_OUT_EOF_DES_ADDR_CH0"]
    pub dma_out_eof_des_addr_ch0: DMA_OUT_EOF_DES_ADDR_CH0,
    #[doc = "0xec - DMA_OUT_EOF_BFR_DES_ADDR_CH0"]
    pub dma_out_eof_bfr_des_addr_ch0: DMA_OUT_EOF_BFR_DES_ADDR_CH0,
    #[doc = "0xf0 - DMA_OUT_DSCR_CH0"]
    pub dma_out_dscr_ch0: DMA_OUT_DSCR_CH0,
    #[doc = "0xf4 - DMA_OUT_DSCR_BF0_CH0"]
    pub dma_out_dscr_bf0_ch0: DMA_OUT_DSCR_BF0_CH0,
    #[doc = "0xf8 - DMA_OUT_DSCR_BF1_CH0"]
    pub dma_out_dscr_bf1_ch0: DMA_OUT_DSCR_BF1_CH0,
    #[doc = "0xfc - DMA_OUT_PRI_CH0"]
    pub dma_out_pri_ch0: DMA_OUT_PRI_CH0,
    #[doc = "0x100 - DMA_OUT_PERI_SEL_CH0"]
    pub dma_out_peri_sel_ch0: DMA_OUT_PERI_SEL_CH0,
    _reserved41: [u8; 44usize],
    #[doc = "0x130 - DMA_IN_CONF0_CH1"]
    pub dma_in_conf0_ch1: DMA_IN_CONF0_CH1,
    #[doc = "0x134 - DMA_IN_CONF1_CH1"]
    pub dma_in_conf1_ch1: DMA_IN_CONF1_CH1,
    #[doc = "0x138 - DMA_INFIFO_STATUS_CH1"]
    pub dma_infifo_status_ch1: DMA_INFIFO_STATUS_CH1,
    #[doc = "0x13c - DMA_IN_POP_CH1"]
    pub dma_in_pop_ch1: DMA_IN_POP_CH1,
    #[doc = "0x140 - DMA_IN_LINK_CH1"]
    pub dma_in_link_ch1: DMA_IN_LINK_CH1,
    #[doc = "0x144 - DMA_IN_STATE_CH1"]
    pub dma_in_state_ch1: DMA_IN_STATE_CH1,
    #[doc = "0x148 - DMA_IN_SUC_EOF_DES_ADDR_CH1"]
    pub dma_in_suc_eof_des_addr_ch1: DMA_IN_SUC_EOF_DES_ADDR_CH1,
    #[doc = "0x14c - DMA_IN_ERR_EOF_DES_ADDR_CH1"]
    pub dma_in_err_eof_des_addr_ch1: DMA_IN_ERR_EOF_DES_ADDR_CH1,
    #[doc = "0x150 - DMA_IN_DSCR_CH1"]
    pub dma_in_dscr_ch1: DMA_IN_DSCR_CH1,
    #[doc = "0x154 - DMA_IN_DSCR_BF0_CH1"]
    pub dma_in_dscr_bf0_ch1: DMA_IN_DSCR_BF0_CH1,
    #[doc = "0x158 - DMA_IN_DSCR_BF1_CH1"]
    pub dma_in_dscr_bf1_ch1: DMA_IN_DSCR_BF1_CH1,
    #[doc = "0x15c - DMA_IN_PRI_CH1"]
    pub dma_in_pri_ch1: DMA_IN_PRI_CH1,
    #[doc = "0x160 - DMA_IN_PERI_SEL_CH1"]
    pub dma_in_peri_sel_ch1: DMA_IN_PERI_SEL_CH1,
    _reserved54: [u8; 44usize],
    #[doc = "0x190 - DMA_OUT_CONF0_CH1"]
    pub dma_out_conf0_ch1: DMA_OUT_CONF0_CH1,
    #[doc = "0x194 - DMA_OUT_CONF1_CH1"]
    pub dma_out_conf1_ch1: DMA_OUT_CONF1_CH1,
    #[doc = "0x198 - DMA_OUTFIFO_STATUS_CH1"]
    pub dma_outfifo_status_ch1: DMA_OUTFIFO_STATUS_CH1,
    #[doc = "0x19c - DMA_OUT_PUSH_CH1"]
    pub dma_out_push_ch1: DMA_OUT_PUSH_CH1,
    #[doc = "0x1a0 - DMA_OUT_LINK_CH1"]
    pub dma_out_link_ch1: DMA_OUT_LINK_CH1,
    #[doc = "0x1a4 - DMA_OUT_STATE_CH1"]
    pub dma_out_state_ch1: DMA_OUT_STATE_CH1,
    #[doc = "0x1a8 - DMA_OUT_EOF_DES_ADDR_CH1"]
    pub dma_out_eof_des_addr_ch1: DMA_OUT_EOF_DES_ADDR_CH1,
    #[doc = "0x1ac - DMA_OUT_EOF_BFR_DES_ADDR_CH1"]
    pub dma_out_eof_bfr_des_addr_ch1: DMA_OUT_EOF_BFR_DES_ADDR_CH1,
    #[doc = "0x1b0 - DMA_OUT_DSCR_CH1"]
    pub dma_out_dscr_ch1: DMA_OUT_DSCR_CH1,
    #[doc = "0x1b4 - DMA_OUT_DSCR_BF0_CH1"]
    pub dma_out_dscr_bf0_ch1: DMA_OUT_DSCR_BF0_CH1,
    #[doc = "0x1b8 - DMA_OUT_DSCR_BF1_CH1"]
    pub dma_out_dscr_bf1_ch1: DMA_OUT_DSCR_BF1_CH1,
    #[doc = "0x1bc - DMA_OUT_PRI_CH1"]
    pub dma_out_pri_ch1: DMA_OUT_PRI_CH1,
    #[doc = "0x1c0 - DMA_OUT_PERI_SEL_CH1"]
    pub dma_out_peri_sel_ch1: DMA_OUT_PERI_SEL_CH1,
    _reserved67: [u8; 44usize],
    #[doc = "0x1f0 - DMA_IN_CONF0_CH2"]
    pub dma_in_conf0_ch2: DMA_IN_CONF0_CH2,
    #[doc = "0x1f4 - DMA_IN_CONF1_CH2"]
    pub dma_in_conf1_ch2: DMA_IN_CONF1_CH2,
    #[doc = "0x1f8 - DMA_INFIFO_STATUS_CH2"]
    pub dma_infifo_status_ch2: DMA_INFIFO_STATUS_CH2,
    #[doc = "0x1fc - DMA_IN_POP_CH2"]
    pub dma_in_pop_ch2: DMA_IN_POP_CH2,
    #[doc = "0x200 - DMA_IN_LINK_CH2"]
    pub dma_in_link_ch2: DMA_IN_LINK_CH2,
    #[doc = "0x204 - DMA_IN_STATE_CH2"]
    pub dma_in_state_ch2: DMA_IN_STATE_CH2,
    #[doc = "0x208 - DMA_IN_SUC_EOF_DES_ADDR_CH2"]
    pub dma_in_suc_eof_des_addr_ch2: DMA_IN_SUC_EOF_DES_ADDR_CH2,
    #[doc = "0x20c - DMA_IN_ERR_EOF_DES_ADDR_CH2"]
    pub dma_in_err_eof_des_addr_ch2: DMA_IN_ERR_EOF_DES_ADDR_CH2,
    #[doc = "0x210 - DMA_IN_DSCR_CH2"]
    pub dma_in_dscr_ch2: DMA_IN_DSCR_CH2,
    #[doc = "0x214 - DMA_IN_DSCR_BF0_CH2"]
    pub dma_in_dscr_bf0_ch2: DMA_IN_DSCR_BF0_CH2,
    #[doc = "0x218 - DMA_IN_DSCR_BF1_CH2"]
    pub dma_in_dscr_bf1_ch2: DMA_IN_DSCR_BF1_CH2,
    #[doc = "0x21c - DMA_IN_PRI_CH2"]
    pub dma_in_pri_ch2: DMA_IN_PRI_CH2,
    #[doc = "0x220 - DMA_IN_PERI_SEL_CH2"]
    pub dma_in_peri_sel_ch2: DMA_IN_PERI_SEL_CH2,
    _reserved80: [u8; 44usize],
    #[doc = "0x250 - DMA_OUT_CONF0_CH2"]
    pub dma_out_conf0_ch2: DMA_OUT_CONF0_CH2,
    #[doc = "0x254 - DMA_OUT_CONF1_CH2"]
    pub dma_out_conf1_ch2: DMA_OUT_CONF1_CH2,
    #[doc = "0x258 - DMA_OUTFIFO_STATUS_CH2"]
    pub dma_outfifo_status_ch2: DMA_OUTFIFO_STATUS_CH2,
    #[doc = "0x25c - DMA_OUT_PUSH_CH2"]
    pub dma_out_push_ch2: DMA_OUT_PUSH_CH2,
    #[doc = "0x260 - DMA_OUT_LINK_CH2"]
    pub dma_out_link_ch2: DMA_OUT_LINK_CH2,
    #[doc = "0x264 - DMA_OUT_STATE_CH2"]
    pub dma_out_state_ch2: DMA_OUT_STATE_CH2,
    #[doc = "0x268 - DMA_OUT_EOF_DES_ADDR_CH2"]
    pub dma_out_eof_des_addr_ch2: DMA_OUT_EOF_DES_ADDR_CH2,
    #[doc = "0x26c - DMA_OUT_EOF_BFR_DES_ADDR_CH2"]
    pub dma_out_eof_bfr_des_addr_ch2: DMA_OUT_EOF_BFR_DES_ADDR_CH2,
    #[doc = "0x270 - DMA_OUT_DSCR_CH2"]
    pub dma_out_dscr_ch2: DMA_OUT_DSCR_CH2,
    #[doc = "0x274 - DMA_OUT_DSCR_BF0_CH2"]
    pub dma_out_dscr_bf0_ch2: DMA_OUT_DSCR_BF0_CH2,
    #[doc = "0x278 - DMA_OUT_DSCR_BF1_CH2"]
    pub dma_out_dscr_bf1_ch2: DMA_OUT_DSCR_BF1_CH2,
    #[doc = "0x27c - DMA_OUT_PRI_CH2"]
    pub dma_out_pri_ch2: DMA_OUT_PRI_CH2,
    #[doc = "0x280 - DMA_OUT_PERI_SEL_CH2"]
    pub dma_out_peri_sel_ch2: DMA_OUT_PERI_SEL_CH2,
}
#[doc = "DMA_INT_RAW_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_raw_ch0](dma_int_raw_ch0) module"]
pub type DMA_INT_RAW_CH0 = crate::Reg<u32, _DMA_INT_RAW_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_RAW_CH0;
#[doc = "`read()` method returns [dma_int_raw_ch0::R](dma_int_raw_ch0::R) reader structure"]
impl crate::Readable for DMA_INT_RAW_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_int_raw_ch0::W](dma_int_raw_ch0::W) writer structure"]
impl crate::Writable for DMA_INT_RAW_CH0 {}
#[doc = "DMA_INT_RAW_CH0"]
pub mod dma_int_raw_ch0;
#[doc = "DMA_INT_ST_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st_ch0](dma_int_st_ch0) module"]
pub type DMA_INT_ST_CH0 = crate::Reg<u32, _DMA_INT_ST_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ST_CH0;
#[doc = "`read()` method returns [dma_int_st_ch0::R](dma_int_st_ch0::R) reader structure"]
impl crate::Readable for DMA_INT_ST_CH0 {}
#[doc = "DMA_INT_ST_CH0"]
pub mod dma_int_st_ch0;
#[doc = "DMA_INT_ENA_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_ena_ch0](dma_int_ena_ch0) module"]
pub type DMA_INT_ENA_CH0 = crate::Reg<u32, _DMA_INT_ENA_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ENA_CH0;
#[doc = "`read()` method returns [dma_int_ena_ch0::R](dma_int_ena_ch0::R) reader structure"]
impl crate::Readable for DMA_INT_ENA_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_int_ena_ch0::W](dma_int_ena_ch0::W) writer structure"]
impl crate::Writable for DMA_INT_ENA_CH0 {}
#[doc = "DMA_INT_ENA_CH0"]
pub mod dma_int_ena_ch0;
#[doc = "DMA_INT_CLR_CH0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_clr_ch0](dma_int_clr_ch0) module"]
pub type DMA_INT_CLR_CH0 = crate::Reg<u32, _DMA_INT_CLR_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_CLR_CH0;
#[doc = "`write(|w| ..)` method takes [dma_int_clr_ch0::W](dma_int_clr_ch0::W) writer structure"]
impl crate::Writable for DMA_INT_CLR_CH0 {}
#[doc = "DMA_INT_CLR_CH0"]
pub mod dma_int_clr_ch0;
#[doc = "DMA_INT_RAW_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_raw_ch1](dma_int_raw_ch1) module"]
pub type DMA_INT_RAW_CH1 = crate::Reg<u32, _DMA_INT_RAW_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_RAW_CH1;
#[doc = "`read()` method returns [dma_int_raw_ch1::R](dma_int_raw_ch1::R) reader structure"]
impl crate::Readable for DMA_INT_RAW_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_int_raw_ch1::W](dma_int_raw_ch1::W) writer structure"]
impl crate::Writable for DMA_INT_RAW_CH1 {}
#[doc = "DMA_INT_RAW_CH1"]
pub mod dma_int_raw_ch1;
#[doc = "DMA_INT_ST_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st_ch1](dma_int_st_ch1) module"]
pub type DMA_INT_ST_CH1 = crate::Reg<u32, _DMA_INT_ST_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ST_CH1;
#[doc = "`read()` method returns [dma_int_st_ch1::R](dma_int_st_ch1::R) reader structure"]
impl crate::Readable for DMA_INT_ST_CH1 {}
#[doc = "DMA_INT_ST_CH1"]
pub mod dma_int_st_ch1;
#[doc = "DMA_INT_ENA_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_ena_ch1](dma_int_ena_ch1) module"]
pub type DMA_INT_ENA_CH1 = crate::Reg<u32, _DMA_INT_ENA_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ENA_CH1;
#[doc = "`read()` method returns [dma_int_ena_ch1::R](dma_int_ena_ch1::R) reader structure"]
impl crate::Readable for DMA_INT_ENA_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_int_ena_ch1::W](dma_int_ena_ch1::W) writer structure"]
impl crate::Writable for DMA_INT_ENA_CH1 {}
#[doc = "DMA_INT_ENA_CH1"]
pub mod dma_int_ena_ch1;
#[doc = "DMA_INT_CLR_CH1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_clr_ch1](dma_int_clr_ch1) module"]
pub type DMA_INT_CLR_CH1 = crate::Reg<u32, _DMA_INT_CLR_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_CLR_CH1;
#[doc = "`write(|w| ..)` method takes [dma_int_clr_ch1::W](dma_int_clr_ch1::W) writer structure"]
impl crate::Writable for DMA_INT_CLR_CH1 {}
#[doc = "DMA_INT_CLR_CH1"]
pub mod dma_int_clr_ch1;
#[doc = "DMA_INT_RAW_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_raw_ch2](dma_int_raw_ch2) module"]
pub type DMA_INT_RAW_CH2 = crate::Reg<u32, _DMA_INT_RAW_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_RAW_CH2;
#[doc = "`read()` method returns [dma_int_raw_ch2::R](dma_int_raw_ch2::R) reader structure"]
impl crate::Readable for DMA_INT_RAW_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_int_raw_ch2::W](dma_int_raw_ch2::W) writer structure"]
impl crate::Writable for DMA_INT_RAW_CH2 {}
#[doc = "DMA_INT_RAW_CH2"]
pub mod dma_int_raw_ch2;
#[doc = "DMA_INT_ST_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st_ch2](dma_int_st_ch2) module"]
pub type DMA_INT_ST_CH2 = crate::Reg<u32, _DMA_INT_ST_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ST_CH2;
#[doc = "`read()` method returns [dma_int_st_ch2::R](dma_int_st_ch2::R) reader structure"]
impl crate::Readable for DMA_INT_ST_CH2 {}
#[doc = "DMA_INT_ST_CH2"]
pub mod dma_int_st_ch2;
#[doc = "DMA_INT_ENA_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_ena_ch2](dma_int_ena_ch2) module"]
pub type DMA_INT_ENA_CH2 = crate::Reg<u32, _DMA_INT_ENA_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ENA_CH2;
#[doc = "`read()` method returns [dma_int_ena_ch2::R](dma_int_ena_ch2::R) reader structure"]
impl crate::Readable for DMA_INT_ENA_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_int_ena_ch2::W](dma_int_ena_ch2::W) writer structure"]
impl crate::Writable for DMA_INT_ENA_CH2 {}
#[doc = "DMA_INT_ENA_CH2"]
pub mod dma_int_ena_ch2;
#[doc = "DMA_INT_CLR_CH2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_clr_ch2](dma_int_clr_ch2) module"]
pub type DMA_INT_CLR_CH2 = crate::Reg<u32, _DMA_INT_CLR_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_CLR_CH2;
#[doc = "`write(|w| ..)` method takes [dma_int_clr_ch2::W](dma_int_clr_ch2::W) writer structure"]
impl crate::Writable for DMA_INT_CLR_CH2 {}
#[doc = "DMA_INT_CLR_CH2"]
pub mod dma_int_clr_ch2;
#[doc = "DMA_AHB_TEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ahb_test](dma_ahb_test) module"]
pub type DMA_AHB_TEST = crate::Reg<u32, _DMA_AHB_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_AHB_TEST;
#[doc = "`read()` method returns [dma_ahb_test::R](dma_ahb_test::R) reader structure"]
impl crate::Readable for DMA_AHB_TEST {}
#[doc = "`write(|w| ..)` method takes [dma_ahb_test::W](dma_ahb_test::W) writer structure"]
impl crate::Writable for DMA_AHB_TEST {}
#[doc = "DMA_AHB_TEST"]
pub mod dma_ahb_test;
#[doc = "DMA_MISC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_misc_conf](dma_misc_conf) module"]
pub type DMA_MISC_CONF = crate::Reg<u32, _DMA_MISC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_MISC_CONF;
#[doc = "`read()` method returns [dma_misc_conf::R](dma_misc_conf::R) reader structure"]
impl crate::Readable for DMA_MISC_CONF {}
#[doc = "`write(|w| ..)` method takes [dma_misc_conf::W](dma_misc_conf::W) writer structure"]
impl crate::Writable for DMA_MISC_CONF {}
#[doc = "DMA_MISC_CONF"]
pub mod dma_misc_conf;
#[doc = "DMA_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_date](dma_date) module"]
pub type DMA_DATE = crate::Reg<u32, _DMA_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DATE;
#[doc = "`read()` method returns [dma_date::R](dma_date::R) reader structure"]
impl crate::Readable for DMA_DATE {}
#[doc = "`write(|w| ..)` method takes [dma_date::W](dma_date::W) writer structure"]
impl crate::Writable for DMA_DATE {}
#[doc = "DMA_DATE"]
pub mod dma_date;
#[doc = "DMA_IN_CONF0_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_conf0_ch0](dma_in_conf0_ch0) module"]
pub type DMA_IN_CONF0_CH0 = crate::Reg<u32, _DMA_IN_CONF0_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_CONF0_CH0;
#[doc = "`read()` method returns [dma_in_conf0_ch0::R](dma_in_conf0_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_CONF0_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_in_conf0_ch0::W](dma_in_conf0_ch0::W) writer structure"]
impl crate::Writable for DMA_IN_CONF0_CH0 {}
#[doc = "DMA_IN_CONF0_CH0"]
pub mod dma_in_conf0_ch0;
#[doc = "DMA_IN_CONF1_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_conf1_ch0](dma_in_conf1_ch0) module"]
pub type DMA_IN_CONF1_CH0 = crate::Reg<u32, _DMA_IN_CONF1_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_CONF1_CH0;
#[doc = "`read()` method returns [dma_in_conf1_ch0::R](dma_in_conf1_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_CONF1_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_in_conf1_ch0::W](dma_in_conf1_ch0::W) writer structure"]
impl crate::Writable for DMA_IN_CONF1_CH0 {}
#[doc = "DMA_IN_CONF1_CH0"]
pub mod dma_in_conf1_ch0;
#[doc = "DMA_INFIFO_STATUS_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_infifo_status_ch0](dma_infifo_status_ch0) module"]
pub type DMA_INFIFO_STATUS_CH0 = crate::Reg<u32, _DMA_INFIFO_STATUS_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INFIFO_STATUS_CH0;
#[doc = "`read()` method returns [dma_infifo_status_ch0::R](dma_infifo_status_ch0::R) reader structure"]
impl crate::Readable for DMA_INFIFO_STATUS_CH0 {}
#[doc = "DMA_INFIFO_STATUS_CH0"]
pub mod dma_infifo_status_ch0;
#[doc = "DMA_IN_POP_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_pop_ch0](dma_in_pop_ch0) module"]
pub type DMA_IN_POP_CH0 = crate::Reg<u32, _DMA_IN_POP_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_POP_CH0;
#[doc = "`read()` method returns [dma_in_pop_ch0::R](dma_in_pop_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_POP_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_in_pop_ch0::W](dma_in_pop_ch0::W) writer structure"]
impl crate::Writable for DMA_IN_POP_CH0 {}
#[doc = "DMA_IN_POP_CH0"]
pub mod dma_in_pop_ch0;
#[doc = "DMA_IN_LINK_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_link_ch0](dma_in_link_ch0) module"]
pub type DMA_IN_LINK_CH0 = crate::Reg<u32, _DMA_IN_LINK_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_LINK_CH0;
#[doc = "`read()` method returns [dma_in_link_ch0::R](dma_in_link_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_LINK_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_in_link_ch0::W](dma_in_link_ch0::W) writer structure"]
impl crate::Writable for DMA_IN_LINK_CH0 {}
#[doc = "DMA_IN_LINK_CH0"]
pub mod dma_in_link_ch0;
#[doc = "DMA_IN_STATE_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_state_ch0](dma_in_state_ch0) module"]
pub type DMA_IN_STATE_CH0 = crate::Reg<u32, _DMA_IN_STATE_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_STATE_CH0;
#[doc = "`read()` method returns [dma_in_state_ch0::R](dma_in_state_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_STATE_CH0 {}
#[doc = "DMA_IN_STATE_CH0"]
pub mod dma_in_state_ch0;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_suc_eof_des_addr_ch0](dma_in_suc_eof_des_addr_ch0) module"]
pub type DMA_IN_SUC_EOF_DES_ADDR_CH0 = crate::Reg<u32, _DMA_IN_SUC_EOF_DES_ADDR_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_SUC_EOF_DES_ADDR_CH0;
#[doc = "`read()` method returns [dma_in_suc_eof_des_addr_ch0::R](dma_in_suc_eof_des_addr_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_SUC_EOF_DES_ADDR_CH0 {}
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH0"]
pub mod dma_in_suc_eof_des_addr_ch0;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_err_eof_des_addr_ch0](dma_in_err_eof_des_addr_ch0) module"]
pub type DMA_IN_ERR_EOF_DES_ADDR_CH0 = crate::Reg<u32, _DMA_IN_ERR_EOF_DES_ADDR_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_ERR_EOF_DES_ADDR_CH0;
#[doc = "`read()` method returns [dma_in_err_eof_des_addr_ch0::R](dma_in_err_eof_des_addr_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_ERR_EOF_DES_ADDR_CH0 {}
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0"]
pub mod dma_in_err_eof_des_addr_ch0;
#[doc = "DMA_IN_DSCR_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_ch0](dma_in_dscr_ch0) module"]
pub type DMA_IN_DSCR_CH0 = crate::Reg<u32, _DMA_IN_DSCR_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_CH0;
#[doc = "`read()` method returns [dma_in_dscr_ch0::R](dma_in_dscr_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_CH0 {}
#[doc = "DMA_IN_DSCR_CH0"]
pub mod dma_in_dscr_ch0;
#[doc = "DMA_IN_DSCR_BF0_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_bf0_ch0](dma_in_dscr_bf0_ch0) module"]
pub type DMA_IN_DSCR_BF0_CH0 = crate::Reg<u32, _DMA_IN_DSCR_BF0_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_BF0_CH0;
#[doc = "`read()` method returns [dma_in_dscr_bf0_ch0::R](dma_in_dscr_bf0_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF0_CH0 {}
#[doc = "DMA_IN_DSCR_BF0_CH0"]
pub mod dma_in_dscr_bf0_ch0;
#[doc = "DMA_IN_DSCR_BF1_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_bf1_ch0](dma_in_dscr_bf1_ch0) module"]
pub type DMA_IN_DSCR_BF1_CH0 = crate::Reg<u32, _DMA_IN_DSCR_BF1_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_BF1_CH0;
#[doc = "`read()` method returns [dma_in_dscr_bf1_ch0::R](dma_in_dscr_bf1_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF1_CH0 {}
#[doc = "DMA_IN_DSCR_BF1_CH0"]
pub mod dma_in_dscr_bf1_ch0;
#[doc = "DMA_IN_PRI_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_pri_ch0](dma_in_pri_ch0) module"]
pub type DMA_IN_PRI_CH0 = crate::Reg<u32, _DMA_IN_PRI_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_PRI_CH0;
#[doc = "`read()` method returns [dma_in_pri_ch0::R](dma_in_pri_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_PRI_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_in_pri_ch0::W](dma_in_pri_ch0::W) writer structure"]
impl crate::Writable for DMA_IN_PRI_CH0 {}
#[doc = "DMA_IN_PRI_CH0"]
pub mod dma_in_pri_ch0;
#[doc = "DMA_IN_PERI_SEL_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_peri_sel_ch0](dma_in_peri_sel_ch0) module"]
pub type DMA_IN_PERI_SEL_CH0 = crate::Reg<u32, _DMA_IN_PERI_SEL_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_PERI_SEL_CH0;
#[doc = "`read()` method returns [dma_in_peri_sel_ch0::R](dma_in_peri_sel_ch0::R) reader structure"]
impl crate::Readable for DMA_IN_PERI_SEL_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_in_peri_sel_ch0::W](dma_in_peri_sel_ch0::W) writer structure"]
impl crate::Writable for DMA_IN_PERI_SEL_CH0 {}
#[doc = "DMA_IN_PERI_SEL_CH0"]
pub mod dma_in_peri_sel_ch0;
#[doc = "DMA_OUT_CONF0_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_conf0_ch0](dma_out_conf0_ch0) module"]
pub type DMA_OUT_CONF0_CH0 = crate::Reg<u32, _DMA_OUT_CONF0_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_CONF0_CH0;
#[doc = "`read()` method returns [dma_out_conf0_ch0::R](dma_out_conf0_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_CONF0_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_out_conf0_ch0::W](dma_out_conf0_ch0::W) writer structure"]
impl crate::Writable for DMA_OUT_CONF0_CH0 {}
#[doc = "DMA_OUT_CONF0_CH0"]
pub mod dma_out_conf0_ch0;
#[doc = "DMA_OUT_CONF1_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_conf1_ch0](dma_out_conf1_ch0) module"]
pub type DMA_OUT_CONF1_CH0 = crate::Reg<u32, _DMA_OUT_CONF1_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_CONF1_CH0;
#[doc = "`read()` method returns [dma_out_conf1_ch0::R](dma_out_conf1_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_CONF1_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_out_conf1_ch0::W](dma_out_conf1_ch0::W) writer structure"]
impl crate::Writable for DMA_OUT_CONF1_CH0 {}
#[doc = "DMA_OUT_CONF1_CH0"]
pub mod dma_out_conf1_ch0;
#[doc = "DMA_OUTFIFO_STATUS_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_outfifo_status_ch0](dma_outfifo_status_ch0) module"]
pub type DMA_OUTFIFO_STATUS_CH0 = crate::Reg<u32, _DMA_OUTFIFO_STATUS_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUTFIFO_STATUS_CH0;
#[doc = "`read()` method returns [dma_outfifo_status_ch0::R](dma_outfifo_status_ch0::R) reader structure"]
impl crate::Readable for DMA_OUTFIFO_STATUS_CH0 {}
#[doc = "DMA_OUTFIFO_STATUS_CH0"]
pub mod dma_outfifo_status_ch0;
#[doc = "DMA_OUT_PUSH_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_push_ch0](dma_out_push_ch0) module"]
pub type DMA_OUT_PUSH_CH0 = crate::Reg<u32, _DMA_OUT_PUSH_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PUSH_CH0;
#[doc = "`read()` method returns [dma_out_push_ch0::R](dma_out_push_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_PUSH_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_out_push_ch0::W](dma_out_push_ch0::W) writer structure"]
impl crate::Writable for DMA_OUT_PUSH_CH0 {}
#[doc = "DMA_OUT_PUSH_CH0"]
pub mod dma_out_push_ch0;
#[doc = "DMA_OUT_LINK_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_link_ch0](dma_out_link_ch0) module"]
pub type DMA_OUT_LINK_CH0 = crate::Reg<u32, _DMA_OUT_LINK_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_LINK_CH0;
#[doc = "`read()` method returns [dma_out_link_ch0::R](dma_out_link_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_LINK_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_out_link_ch0::W](dma_out_link_ch0::W) writer structure"]
impl crate::Writable for DMA_OUT_LINK_CH0 {}
#[doc = "DMA_OUT_LINK_CH0"]
pub mod dma_out_link_ch0;
#[doc = "DMA_OUT_STATE_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_state_ch0](dma_out_state_ch0) module"]
pub type DMA_OUT_STATE_CH0 = crate::Reg<u32, _DMA_OUT_STATE_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_STATE_CH0;
#[doc = "`read()` method returns [dma_out_state_ch0::R](dma_out_state_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_STATE_CH0 {}
#[doc = "DMA_OUT_STATE_CH0"]
pub mod dma_out_state_ch0;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_eof_des_addr_ch0](dma_out_eof_des_addr_ch0) module"]
pub type DMA_OUT_EOF_DES_ADDR_CH0 = crate::Reg<u32, _DMA_OUT_EOF_DES_ADDR_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_EOF_DES_ADDR_CH0;
#[doc = "`read()` method returns [dma_out_eof_des_addr_ch0::R](dma_out_eof_des_addr_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_DES_ADDR_CH0 {}
#[doc = "DMA_OUT_EOF_DES_ADDR_CH0"]
pub mod dma_out_eof_des_addr_ch0;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_eof_bfr_des_addr_ch0](dma_out_eof_bfr_des_addr_ch0) module"]
pub type DMA_OUT_EOF_BFR_DES_ADDR_CH0 = crate::Reg<u32, _DMA_OUT_EOF_BFR_DES_ADDR_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_EOF_BFR_DES_ADDR_CH0;
#[doc = "`read()` method returns [dma_out_eof_bfr_des_addr_ch0::R](dma_out_eof_bfr_des_addr_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_BFR_DES_ADDR_CH0 {}
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH0"]
pub mod dma_out_eof_bfr_des_addr_ch0;
#[doc = "DMA_OUT_DSCR_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_ch0](dma_out_dscr_ch0) module"]
pub type DMA_OUT_DSCR_CH0 = crate::Reg<u32, _DMA_OUT_DSCR_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_CH0;
#[doc = "`read()` method returns [dma_out_dscr_ch0::R](dma_out_dscr_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_CH0 {}
#[doc = "DMA_OUT_DSCR_CH0"]
pub mod dma_out_dscr_ch0;
#[doc = "DMA_OUT_DSCR_BF0_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf0_ch0](dma_out_dscr_bf0_ch0) module"]
pub type DMA_OUT_DSCR_BF0_CH0 = crate::Reg<u32, _DMA_OUT_DSCR_BF0_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_BF0_CH0;
#[doc = "`read()` method returns [dma_out_dscr_bf0_ch0::R](dma_out_dscr_bf0_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF0_CH0 {}
#[doc = "DMA_OUT_DSCR_BF0_CH0"]
pub mod dma_out_dscr_bf0_ch0;
#[doc = "DMA_OUT_DSCR_BF1_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf1_ch0](dma_out_dscr_bf1_ch0) module"]
pub type DMA_OUT_DSCR_BF1_CH0 = crate::Reg<u32, _DMA_OUT_DSCR_BF1_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_BF1_CH0;
#[doc = "`read()` method returns [dma_out_dscr_bf1_ch0::R](dma_out_dscr_bf1_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF1_CH0 {}
#[doc = "DMA_OUT_DSCR_BF1_CH0"]
pub mod dma_out_dscr_bf1_ch0;
#[doc = "DMA_OUT_PRI_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_pri_ch0](dma_out_pri_ch0) module"]
pub type DMA_OUT_PRI_CH0 = crate::Reg<u32, _DMA_OUT_PRI_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PRI_CH0;
#[doc = "`read()` method returns [dma_out_pri_ch0::R](dma_out_pri_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_PRI_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_out_pri_ch0::W](dma_out_pri_ch0::W) writer structure"]
impl crate::Writable for DMA_OUT_PRI_CH0 {}
#[doc = "DMA_OUT_PRI_CH0"]
pub mod dma_out_pri_ch0;
#[doc = "DMA_OUT_PERI_SEL_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_peri_sel_ch0](dma_out_peri_sel_ch0) module"]
pub type DMA_OUT_PERI_SEL_CH0 = crate::Reg<u32, _DMA_OUT_PERI_SEL_CH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PERI_SEL_CH0;
#[doc = "`read()` method returns [dma_out_peri_sel_ch0::R](dma_out_peri_sel_ch0::R) reader structure"]
impl crate::Readable for DMA_OUT_PERI_SEL_CH0 {}
#[doc = "`write(|w| ..)` method takes [dma_out_peri_sel_ch0::W](dma_out_peri_sel_ch0::W) writer structure"]
impl crate::Writable for DMA_OUT_PERI_SEL_CH0 {}
#[doc = "DMA_OUT_PERI_SEL_CH0"]
pub mod dma_out_peri_sel_ch0;
#[doc = "DMA_IN_CONF0_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_conf0_ch1](dma_in_conf0_ch1) module"]
pub type DMA_IN_CONF0_CH1 = crate::Reg<u32, _DMA_IN_CONF0_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_CONF0_CH1;
#[doc = "`read()` method returns [dma_in_conf0_ch1::R](dma_in_conf0_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_CONF0_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_in_conf0_ch1::W](dma_in_conf0_ch1::W) writer structure"]
impl crate::Writable for DMA_IN_CONF0_CH1 {}
#[doc = "DMA_IN_CONF0_CH1"]
pub mod dma_in_conf0_ch1;
#[doc = "DMA_IN_CONF1_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_conf1_ch1](dma_in_conf1_ch1) module"]
pub type DMA_IN_CONF1_CH1 = crate::Reg<u32, _DMA_IN_CONF1_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_CONF1_CH1;
#[doc = "`read()` method returns [dma_in_conf1_ch1::R](dma_in_conf1_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_CONF1_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_in_conf1_ch1::W](dma_in_conf1_ch1::W) writer structure"]
impl crate::Writable for DMA_IN_CONF1_CH1 {}
#[doc = "DMA_IN_CONF1_CH1"]
pub mod dma_in_conf1_ch1;
#[doc = "DMA_INFIFO_STATUS_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_infifo_status_ch1](dma_infifo_status_ch1) module"]
pub type DMA_INFIFO_STATUS_CH1 = crate::Reg<u32, _DMA_INFIFO_STATUS_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INFIFO_STATUS_CH1;
#[doc = "`read()` method returns [dma_infifo_status_ch1::R](dma_infifo_status_ch1::R) reader structure"]
impl crate::Readable for DMA_INFIFO_STATUS_CH1 {}
#[doc = "DMA_INFIFO_STATUS_CH1"]
pub mod dma_infifo_status_ch1;
#[doc = "DMA_IN_POP_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_pop_ch1](dma_in_pop_ch1) module"]
pub type DMA_IN_POP_CH1 = crate::Reg<u32, _DMA_IN_POP_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_POP_CH1;
#[doc = "`read()` method returns [dma_in_pop_ch1::R](dma_in_pop_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_POP_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_in_pop_ch1::W](dma_in_pop_ch1::W) writer structure"]
impl crate::Writable for DMA_IN_POP_CH1 {}
#[doc = "DMA_IN_POP_CH1"]
pub mod dma_in_pop_ch1;
#[doc = "DMA_IN_LINK_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_link_ch1](dma_in_link_ch1) module"]
pub type DMA_IN_LINK_CH1 = crate::Reg<u32, _DMA_IN_LINK_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_LINK_CH1;
#[doc = "`read()` method returns [dma_in_link_ch1::R](dma_in_link_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_LINK_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_in_link_ch1::W](dma_in_link_ch1::W) writer structure"]
impl crate::Writable for DMA_IN_LINK_CH1 {}
#[doc = "DMA_IN_LINK_CH1"]
pub mod dma_in_link_ch1;
#[doc = "DMA_IN_STATE_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_state_ch1](dma_in_state_ch1) module"]
pub type DMA_IN_STATE_CH1 = crate::Reg<u32, _DMA_IN_STATE_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_STATE_CH1;
#[doc = "`read()` method returns [dma_in_state_ch1::R](dma_in_state_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_STATE_CH1 {}
#[doc = "DMA_IN_STATE_CH1"]
pub mod dma_in_state_ch1;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_suc_eof_des_addr_ch1](dma_in_suc_eof_des_addr_ch1) module"]
pub type DMA_IN_SUC_EOF_DES_ADDR_CH1 = crate::Reg<u32, _DMA_IN_SUC_EOF_DES_ADDR_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_SUC_EOF_DES_ADDR_CH1;
#[doc = "`read()` method returns [dma_in_suc_eof_des_addr_ch1::R](dma_in_suc_eof_des_addr_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_SUC_EOF_DES_ADDR_CH1 {}
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH1"]
pub mod dma_in_suc_eof_des_addr_ch1;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_err_eof_des_addr_ch1](dma_in_err_eof_des_addr_ch1) module"]
pub type DMA_IN_ERR_EOF_DES_ADDR_CH1 = crate::Reg<u32, _DMA_IN_ERR_EOF_DES_ADDR_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_ERR_EOF_DES_ADDR_CH1;
#[doc = "`read()` method returns [dma_in_err_eof_des_addr_ch1::R](dma_in_err_eof_des_addr_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_ERR_EOF_DES_ADDR_CH1 {}
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH1"]
pub mod dma_in_err_eof_des_addr_ch1;
#[doc = "DMA_IN_DSCR_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_ch1](dma_in_dscr_ch1) module"]
pub type DMA_IN_DSCR_CH1 = crate::Reg<u32, _DMA_IN_DSCR_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_CH1;
#[doc = "`read()` method returns [dma_in_dscr_ch1::R](dma_in_dscr_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_CH1 {}
#[doc = "DMA_IN_DSCR_CH1"]
pub mod dma_in_dscr_ch1;
#[doc = "DMA_IN_DSCR_BF0_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_bf0_ch1](dma_in_dscr_bf0_ch1) module"]
pub type DMA_IN_DSCR_BF0_CH1 = crate::Reg<u32, _DMA_IN_DSCR_BF0_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_BF0_CH1;
#[doc = "`read()` method returns [dma_in_dscr_bf0_ch1::R](dma_in_dscr_bf0_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF0_CH1 {}
#[doc = "DMA_IN_DSCR_BF0_CH1"]
pub mod dma_in_dscr_bf0_ch1;
#[doc = "DMA_IN_DSCR_BF1_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_bf1_ch1](dma_in_dscr_bf1_ch1) module"]
pub type DMA_IN_DSCR_BF1_CH1 = crate::Reg<u32, _DMA_IN_DSCR_BF1_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_BF1_CH1;
#[doc = "`read()` method returns [dma_in_dscr_bf1_ch1::R](dma_in_dscr_bf1_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF1_CH1 {}
#[doc = "DMA_IN_DSCR_BF1_CH1"]
pub mod dma_in_dscr_bf1_ch1;
#[doc = "DMA_IN_PRI_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_pri_ch1](dma_in_pri_ch1) module"]
pub type DMA_IN_PRI_CH1 = crate::Reg<u32, _DMA_IN_PRI_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_PRI_CH1;
#[doc = "`read()` method returns [dma_in_pri_ch1::R](dma_in_pri_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_PRI_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_in_pri_ch1::W](dma_in_pri_ch1::W) writer structure"]
impl crate::Writable for DMA_IN_PRI_CH1 {}
#[doc = "DMA_IN_PRI_CH1"]
pub mod dma_in_pri_ch1;
#[doc = "DMA_IN_PERI_SEL_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_peri_sel_ch1](dma_in_peri_sel_ch1) module"]
pub type DMA_IN_PERI_SEL_CH1 = crate::Reg<u32, _DMA_IN_PERI_SEL_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_PERI_SEL_CH1;
#[doc = "`read()` method returns [dma_in_peri_sel_ch1::R](dma_in_peri_sel_ch1::R) reader structure"]
impl crate::Readable for DMA_IN_PERI_SEL_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_in_peri_sel_ch1::W](dma_in_peri_sel_ch1::W) writer structure"]
impl crate::Writable for DMA_IN_PERI_SEL_CH1 {}
#[doc = "DMA_IN_PERI_SEL_CH1"]
pub mod dma_in_peri_sel_ch1;
#[doc = "DMA_OUT_CONF0_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_conf0_ch1](dma_out_conf0_ch1) module"]
pub type DMA_OUT_CONF0_CH1 = crate::Reg<u32, _DMA_OUT_CONF0_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_CONF0_CH1;
#[doc = "`read()` method returns [dma_out_conf0_ch1::R](dma_out_conf0_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_CONF0_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_out_conf0_ch1::W](dma_out_conf0_ch1::W) writer structure"]
impl crate::Writable for DMA_OUT_CONF0_CH1 {}
#[doc = "DMA_OUT_CONF0_CH1"]
pub mod dma_out_conf0_ch1;
#[doc = "DMA_OUT_CONF1_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_conf1_ch1](dma_out_conf1_ch1) module"]
pub type DMA_OUT_CONF1_CH1 = crate::Reg<u32, _DMA_OUT_CONF1_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_CONF1_CH1;
#[doc = "`read()` method returns [dma_out_conf1_ch1::R](dma_out_conf1_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_CONF1_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_out_conf1_ch1::W](dma_out_conf1_ch1::W) writer structure"]
impl crate::Writable for DMA_OUT_CONF1_CH1 {}
#[doc = "DMA_OUT_CONF1_CH1"]
pub mod dma_out_conf1_ch1;
#[doc = "DMA_OUTFIFO_STATUS_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_outfifo_status_ch1](dma_outfifo_status_ch1) module"]
pub type DMA_OUTFIFO_STATUS_CH1 = crate::Reg<u32, _DMA_OUTFIFO_STATUS_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUTFIFO_STATUS_CH1;
#[doc = "`read()` method returns [dma_outfifo_status_ch1::R](dma_outfifo_status_ch1::R) reader structure"]
impl crate::Readable for DMA_OUTFIFO_STATUS_CH1 {}
#[doc = "DMA_OUTFIFO_STATUS_CH1"]
pub mod dma_outfifo_status_ch1;
#[doc = "DMA_OUT_PUSH_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_push_ch1](dma_out_push_ch1) module"]
pub type DMA_OUT_PUSH_CH1 = crate::Reg<u32, _DMA_OUT_PUSH_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PUSH_CH1;
#[doc = "`read()` method returns [dma_out_push_ch1::R](dma_out_push_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_PUSH_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_out_push_ch1::W](dma_out_push_ch1::W) writer structure"]
impl crate::Writable for DMA_OUT_PUSH_CH1 {}
#[doc = "DMA_OUT_PUSH_CH1"]
pub mod dma_out_push_ch1;
#[doc = "DMA_OUT_LINK_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_link_ch1](dma_out_link_ch1) module"]
pub type DMA_OUT_LINK_CH1 = crate::Reg<u32, _DMA_OUT_LINK_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_LINK_CH1;
#[doc = "`read()` method returns [dma_out_link_ch1::R](dma_out_link_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_LINK_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_out_link_ch1::W](dma_out_link_ch1::W) writer structure"]
impl crate::Writable for DMA_OUT_LINK_CH1 {}
#[doc = "DMA_OUT_LINK_CH1"]
pub mod dma_out_link_ch1;
#[doc = "DMA_OUT_STATE_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_state_ch1](dma_out_state_ch1) module"]
pub type DMA_OUT_STATE_CH1 = crate::Reg<u32, _DMA_OUT_STATE_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_STATE_CH1;
#[doc = "`read()` method returns [dma_out_state_ch1::R](dma_out_state_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_STATE_CH1 {}
#[doc = "DMA_OUT_STATE_CH1"]
pub mod dma_out_state_ch1;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_eof_des_addr_ch1](dma_out_eof_des_addr_ch1) module"]
pub type DMA_OUT_EOF_DES_ADDR_CH1 = crate::Reg<u32, _DMA_OUT_EOF_DES_ADDR_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_EOF_DES_ADDR_CH1;
#[doc = "`read()` method returns [dma_out_eof_des_addr_ch1::R](dma_out_eof_des_addr_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_DES_ADDR_CH1 {}
#[doc = "DMA_OUT_EOF_DES_ADDR_CH1"]
pub mod dma_out_eof_des_addr_ch1;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_eof_bfr_des_addr_ch1](dma_out_eof_bfr_des_addr_ch1) module"]
pub type DMA_OUT_EOF_BFR_DES_ADDR_CH1 = crate::Reg<u32, _DMA_OUT_EOF_BFR_DES_ADDR_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_EOF_BFR_DES_ADDR_CH1;
#[doc = "`read()` method returns [dma_out_eof_bfr_des_addr_ch1::R](dma_out_eof_bfr_des_addr_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_BFR_DES_ADDR_CH1 {}
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH1"]
pub mod dma_out_eof_bfr_des_addr_ch1;
#[doc = "DMA_OUT_DSCR_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_ch1](dma_out_dscr_ch1) module"]
pub type DMA_OUT_DSCR_CH1 = crate::Reg<u32, _DMA_OUT_DSCR_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_CH1;
#[doc = "`read()` method returns [dma_out_dscr_ch1::R](dma_out_dscr_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_CH1 {}
#[doc = "DMA_OUT_DSCR_CH1"]
pub mod dma_out_dscr_ch1;
#[doc = "DMA_OUT_DSCR_BF0_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf0_ch1](dma_out_dscr_bf0_ch1) module"]
pub type DMA_OUT_DSCR_BF0_CH1 = crate::Reg<u32, _DMA_OUT_DSCR_BF0_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_BF0_CH1;
#[doc = "`read()` method returns [dma_out_dscr_bf0_ch1::R](dma_out_dscr_bf0_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF0_CH1 {}
#[doc = "DMA_OUT_DSCR_BF0_CH1"]
pub mod dma_out_dscr_bf0_ch1;
#[doc = "DMA_OUT_DSCR_BF1_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf1_ch1](dma_out_dscr_bf1_ch1) module"]
pub type DMA_OUT_DSCR_BF1_CH1 = crate::Reg<u32, _DMA_OUT_DSCR_BF1_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_BF1_CH1;
#[doc = "`read()` method returns [dma_out_dscr_bf1_ch1::R](dma_out_dscr_bf1_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF1_CH1 {}
#[doc = "DMA_OUT_DSCR_BF1_CH1"]
pub mod dma_out_dscr_bf1_ch1;
#[doc = "DMA_OUT_PRI_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_pri_ch1](dma_out_pri_ch1) module"]
pub type DMA_OUT_PRI_CH1 = crate::Reg<u32, _DMA_OUT_PRI_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PRI_CH1;
#[doc = "`read()` method returns [dma_out_pri_ch1::R](dma_out_pri_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_PRI_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_out_pri_ch1::W](dma_out_pri_ch1::W) writer structure"]
impl crate::Writable for DMA_OUT_PRI_CH1 {}
#[doc = "DMA_OUT_PRI_CH1"]
pub mod dma_out_pri_ch1;
#[doc = "DMA_OUT_PERI_SEL_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_peri_sel_ch1](dma_out_peri_sel_ch1) module"]
pub type DMA_OUT_PERI_SEL_CH1 = crate::Reg<u32, _DMA_OUT_PERI_SEL_CH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PERI_SEL_CH1;
#[doc = "`read()` method returns [dma_out_peri_sel_ch1::R](dma_out_peri_sel_ch1::R) reader structure"]
impl crate::Readable for DMA_OUT_PERI_SEL_CH1 {}
#[doc = "`write(|w| ..)` method takes [dma_out_peri_sel_ch1::W](dma_out_peri_sel_ch1::W) writer structure"]
impl crate::Writable for DMA_OUT_PERI_SEL_CH1 {}
#[doc = "DMA_OUT_PERI_SEL_CH1"]
pub mod dma_out_peri_sel_ch1;
#[doc = "DMA_IN_CONF0_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_conf0_ch2](dma_in_conf0_ch2) module"]
pub type DMA_IN_CONF0_CH2 = crate::Reg<u32, _DMA_IN_CONF0_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_CONF0_CH2;
#[doc = "`read()` method returns [dma_in_conf0_ch2::R](dma_in_conf0_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_CONF0_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_in_conf0_ch2::W](dma_in_conf0_ch2::W) writer structure"]
impl crate::Writable for DMA_IN_CONF0_CH2 {}
#[doc = "DMA_IN_CONF0_CH2"]
pub mod dma_in_conf0_ch2;
#[doc = "DMA_IN_CONF1_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_conf1_ch2](dma_in_conf1_ch2) module"]
pub type DMA_IN_CONF1_CH2 = crate::Reg<u32, _DMA_IN_CONF1_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_CONF1_CH2;
#[doc = "`read()` method returns [dma_in_conf1_ch2::R](dma_in_conf1_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_CONF1_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_in_conf1_ch2::W](dma_in_conf1_ch2::W) writer structure"]
impl crate::Writable for DMA_IN_CONF1_CH2 {}
#[doc = "DMA_IN_CONF1_CH2"]
pub mod dma_in_conf1_ch2;
#[doc = "DMA_INFIFO_STATUS_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_infifo_status_ch2](dma_infifo_status_ch2) module"]
pub type DMA_INFIFO_STATUS_CH2 = crate::Reg<u32, _DMA_INFIFO_STATUS_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INFIFO_STATUS_CH2;
#[doc = "`read()` method returns [dma_infifo_status_ch2::R](dma_infifo_status_ch2::R) reader structure"]
impl crate::Readable for DMA_INFIFO_STATUS_CH2 {}
#[doc = "DMA_INFIFO_STATUS_CH2"]
pub mod dma_infifo_status_ch2;
#[doc = "DMA_IN_POP_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_pop_ch2](dma_in_pop_ch2) module"]
pub type DMA_IN_POP_CH2 = crate::Reg<u32, _DMA_IN_POP_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_POP_CH2;
#[doc = "`read()` method returns [dma_in_pop_ch2::R](dma_in_pop_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_POP_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_in_pop_ch2::W](dma_in_pop_ch2::W) writer structure"]
impl crate::Writable for DMA_IN_POP_CH2 {}
#[doc = "DMA_IN_POP_CH2"]
pub mod dma_in_pop_ch2;
#[doc = "DMA_IN_LINK_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_link_ch2](dma_in_link_ch2) module"]
pub type DMA_IN_LINK_CH2 = crate::Reg<u32, _DMA_IN_LINK_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_LINK_CH2;
#[doc = "`read()` method returns [dma_in_link_ch2::R](dma_in_link_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_LINK_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_in_link_ch2::W](dma_in_link_ch2::W) writer structure"]
impl crate::Writable for DMA_IN_LINK_CH2 {}
#[doc = "DMA_IN_LINK_CH2"]
pub mod dma_in_link_ch2;
#[doc = "DMA_IN_STATE_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_state_ch2](dma_in_state_ch2) module"]
pub type DMA_IN_STATE_CH2 = crate::Reg<u32, _DMA_IN_STATE_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_STATE_CH2;
#[doc = "`read()` method returns [dma_in_state_ch2::R](dma_in_state_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_STATE_CH2 {}
#[doc = "DMA_IN_STATE_CH2"]
pub mod dma_in_state_ch2;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_suc_eof_des_addr_ch2](dma_in_suc_eof_des_addr_ch2) module"]
pub type DMA_IN_SUC_EOF_DES_ADDR_CH2 = crate::Reg<u32, _DMA_IN_SUC_EOF_DES_ADDR_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_SUC_EOF_DES_ADDR_CH2;
#[doc = "`read()` method returns [dma_in_suc_eof_des_addr_ch2::R](dma_in_suc_eof_des_addr_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_SUC_EOF_DES_ADDR_CH2 {}
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH2"]
pub mod dma_in_suc_eof_des_addr_ch2;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_err_eof_des_addr_ch2](dma_in_err_eof_des_addr_ch2) module"]
pub type DMA_IN_ERR_EOF_DES_ADDR_CH2 = crate::Reg<u32, _DMA_IN_ERR_EOF_DES_ADDR_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_ERR_EOF_DES_ADDR_CH2;
#[doc = "`read()` method returns [dma_in_err_eof_des_addr_ch2::R](dma_in_err_eof_des_addr_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_ERR_EOF_DES_ADDR_CH2 {}
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH2"]
pub mod dma_in_err_eof_des_addr_ch2;
#[doc = "DMA_IN_DSCR_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_ch2](dma_in_dscr_ch2) module"]
pub type DMA_IN_DSCR_CH2 = crate::Reg<u32, _DMA_IN_DSCR_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_CH2;
#[doc = "`read()` method returns [dma_in_dscr_ch2::R](dma_in_dscr_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_CH2 {}
#[doc = "DMA_IN_DSCR_CH2"]
pub mod dma_in_dscr_ch2;
#[doc = "DMA_IN_DSCR_BF0_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_bf0_ch2](dma_in_dscr_bf0_ch2) module"]
pub type DMA_IN_DSCR_BF0_CH2 = crate::Reg<u32, _DMA_IN_DSCR_BF0_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_BF0_CH2;
#[doc = "`read()` method returns [dma_in_dscr_bf0_ch2::R](dma_in_dscr_bf0_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF0_CH2 {}
#[doc = "DMA_IN_DSCR_BF0_CH2"]
pub mod dma_in_dscr_bf0_ch2;
#[doc = "DMA_IN_DSCR_BF1_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_bf1_ch2](dma_in_dscr_bf1_ch2) module"]
pub type DMA_IN_DSCR_BF1_CH2 = crate::Reg<u32, _DMA_IN_DSCR_BF1_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_DSCR_BF1_CH2;
#[doc = "`read()` method returns [dma_in_dscr_bf1_ch2::R](dma_in_dscr_bf1_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF1_CH2 {}
#[doc = "DMA_IN_DSCR_BF1_CH2"]
pub mod dma_in_dscr_bf1_ch2;
#[doc = "DMA_IN_PRI_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_pri_ch2](dma_in_pri_ch2) module"]
pub type DMA_IN_PRI_CH2 = crate::Reg<u32, _DMA_IN_PRI_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_PRI_CH2;
#[doc = "`read()` method returns [dma_in_pri_ch2::R](dma_in_pri_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_PRI_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_in_pri_ch2::W](dma_in_pri_ch2::W) writer structure"]
impl crate::Writable for DMA_IN_PRI_CH2 {}
#[doc = "DMA_IN_PRI_CH2"]
pub mod dma_in_pri_ch2;
#[doc = "DMA_IN_PERI_SEL_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_peri_sel_ch2](dma_in_peri_sel_ch2) module"]
pub type DMA_IN_PERI_SEL_CH2 = crate::Reg<u32, _DMA_IN_PERI_SEL_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IN_PERI_SEL_CH2;
#[doc = "`read()` method returns [dma_in_peri_sel_ch2::R](dma_in_peri_sel_ch2::R) reader structure"]
impl crate::Readable for DMA_IN_PERI_SEL_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_in_peri_sel_ch2::W](dma_in_peri_sel_ch2::W) writer structure"]
impl crate::Writable for DMA_IN_PERI_SEL_CH2 {}
#[doc = "DMA_IN_PERI_SEL_CH2"]
pub mod dma_in_peri_sel_ch2;
#[doc = "DMA_OUT_CONF0_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_conf0_ch2](dma_out_conf0_ch2) module"]
pub type DMA_OUT_CONF0_CH2 = crate::Reg<u32, _DMA_OUT_CONF0_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_CONF0_CH2;
#[doc = "`read()` method returns [dma_out_conf0_ch2::R](dma_out_conf0_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_CONF0_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_out_conf0_ch2::W](dma_out_conf0_ch2::W) writer structure"]
impl crate::Writable for DMA_OUT_CONF0_CH2 {}
#[doc = "DMA_OUT_CONF0_CH2"]
pub mod dma_out_conf0_ch2;
#[doc = "DMA_OUT_CONF1_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_conf1_ch2](dma_out_conf1_ch2) module"]
pub type DMA_OUT_CONF1_CH2 = crate::Reg<u32, _DMA_OUT_CONF1_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_CONF1_CH2;
#[doc = "`read()` method returns [dma_out_conf1_ch2::R](dma_out_conf1_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_CONF1_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_out_conf1_ch2::W](dma_out_conf1_ch2::W) writer structure"]
impl crate::Writable for DMA_OUT_CONF1_CH2 {}
#[doc = "DMA_OUT_CONF1_CH2"]
pub mod dma_out_conf1_ch2;
#[doc = "DMA_OUTFIFO_STATUS_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_outfifo_status_ch2](dma_outfifo_status_ch2) module"]
pub type DMA_OUTFIFO_STATUS_CH2 = crate::Reg<u32, _DMA_OUTFIFO_STATUS_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUTFIFO_STATUS_CH2;
#[doc = "`read()` method returns [dma_outfifo_status_ch2::R](dma_outfifo_status_ch2::R) reader structure"]
impl crate::Readable for DMA_OUTFIFO_STATUS_CH2 {}
#[doc = "DMA_OUTFIFO_STATUS_CH2"]
pub mod dma_outfifo_status_ch2;
#[doc = "DMA_OUT_PUSH_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_push_ch2](dma_out_push_ch2) module"]
pub type DMA_OUT_PUSH_CH2 = crate::Reg<u32, _DMA_OUT_PUSH_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PUSH_CH2;
#[doc = "`read()` method returns [dma_out_push_ch2::R](dma_out_push_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_PUSH_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_out_push_ch2::W](dma_out_push_ch2::W) writer structure"]
impl crate::Writable for DMA_OUT_PUSH_CH2 {}
#[doc = "DMA_OUT_PUSH_CH2"]
pub mod dma_out_push_ch2;
#[doc = "DMA_OUT_LINK_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_link_ch2](dma_out_link_ch2) module"]
pub type DMA_OUT_LINK_CH2 = crate::Reg<u32, _DMA_OUT_LINK_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_LINK_CH2;
#[doc = "`read()` method returns [dma_out_link_ch2::R](dma_out_link_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_LINK_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_out_link_ch2::W](dma_out_link_ch2::W) writer structure"]
impl crate::Writable for DMA_OUT_LINK_CH2 {}
#[doc = "DMA_OUT_LINK_CH2"]
pub mod dma_out_link_ch2;
#[doc = "DMA_OUT_STATE_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_state_ch2](dma_out_state_ch2) module"]
pub type DMA_OUT_STATE_CH2 = crate::Reg<u32, _DMA_OUT_STATE_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_STATE_CH2;
#[doc = "`read()` method returns [dma_out_state_ch2::R](dma_out_state_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_STATE_CH2 {}
#[doc = "DMA_OUT_STATE_CH2"]
pub mod dma_out_state_ch2;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_eof_des_addr_ch2](dma_out_eof_des_addr_ch2) module"]
pub type DMA_OUT_EOF_DES_ADDR_CH2 = crate::Reg<u32, _DMA_OUT_EOF_DES_ADDR_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_EOF_DES_ADDR_CH2;
#[doc = "`read()` method returns [dma_out_eof_des_addr_ch2::R](dma_out_eof_des_addr_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_DES_ADDR_CH2 {}
#[doc = "DMA_OUT_EOF_DES_ADDR_CH2"]
pub mod dma_out_eof_des_addr_ch2;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_eof_bfr_des_addr_ch2](dma_out_eof_bfr_des_addr_ch2) module"]
pub type DMA_OUT_EOF_BFR_DES_ADDR_CH2 = crate::Reg<u32, _DMA_OUT_EOF_BFR_DES_ADDR_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_EOF_BFR_DES_ADDR_CH2;
#[doc = "`read()` method returns [dma_out_eof_bfr_des_addr_ch2::R](dma_out_eof_bfr_des_addr_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_BFR_DES_ADDR_CH2 {}
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH2"]
pub mod dma_out_eof_bfr_des_addr_ch2;
#[doc = "DMA_OUT_DSCR_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_ch2](dma_out_dscr_ch2) module"]
pub type DMA_OUT_DSCR_CH2 = crate::Reg<u32, _DMA_OUT_DSCR_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_CH2;
#[doc = "`read()` method returns [dma_out_dscr_ch2::R](dma_out_dscr_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_CH2 {}
#[doc = "DMA_OUT_DSCR_CH2"]
pub mod dma_out_dscr_ch2;
#[doc = "DMA_OUT_DSCR_BF0_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf0_ch2](dma_out_dscr_bf0_ch2) module"]
pub type DMA_OUT_DSCR_BF0_CH2 = crate::Reg<u32, _DMA_OUT_DSCR_BF0_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_BF0_CH2;
#[doc = "`read()` method returns [dma_out_dscr_bf0_ch2::R](dma_out_dscr_bf0_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF0_CH2 {}
#[doc = "DMA_OUT_DSCR_BF0_CH2"]
pub mod dma_out_dscr_bf0_ch2;
#[doc = "DMA_OUT_DSCR_BF1_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf1_ch2](dma_out_dscr_bf1_ch2) module"]
pub type DMA_OUT_DSCR_BF1_CH2 = crate::Reg<u32, _DMA_OUT_DSCR_BF1_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_DSCR_BF1_CH2;
#[doc = "`read()` method returns [dma_out_dscr_bf1_ch2::R](dma_out_dscr_bf1_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF1_CH2 {}
#[doc = "DMA_OUT_DSCR_BF1_CH2"]
pub mod dma_out_dscr_bf1_ch2;
#[doc = "DMA_OUT_PRI_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_pri_ch2](dma_out_pri_ch2) module"]
pub type DMA_OUT_PRI_CH2 = crate::Reg<u32, _DMA_OUT_PRI_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PRI_CH2;
#[doc = "`read()` method returns [dma_out_pri_ch2::R](dma_out_pri_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_PRI_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_out_pri_ch2::W](dma_out_pri_ch2::W) writer structure"]
impl crate::Writable for DMA_OUT_PRI_CH2 {}
#[doc = "DMA_OUT_PRI_CH2"]
pub mod dma_out_pri_ch2;
#[doc = "DMA_OUT_PERI_SEL_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_peri_sel_ch2](dma_out_peri_sel_ch2) module"]
pub type DMA_OUT_PERI_SEL_CH2 = crate::Reg<u32, _DMA_OUT_PERI_SEL_CH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OUT_PERI_SEL_CH2;
#[doc = "`read()` method returns [dma_out_peri_sel_ch2::R](dma_out_peri_sel_ch2::R) reader structure"]
impl crate::Readable for DMA_OUT_PERI_SEL_CH2 {}
#[doc = "`write(|w| ..)` method takes [dma_out_peri_sel_ch2::W](dma_out_peri_sel_ch2::W) writer structure"]
impl crate::Writable for DMA_OUT_PERI_SEL_CH2 {}
#[doc = "DMA_OUT_PERI_SEL_CH2"]
pub mod dma_out_peri_sel_ch2;
