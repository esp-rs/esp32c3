#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSTEM_CPU_PERI_CLK_EN"]
    pub system_cpu_peri_clk_en: SYSTEM_CPU_PERI_CLK_EN,
    #[doc = "0x04 - SYSTEM_CPU_PERI_RST_EN"]
    pub system_cpu_peri_rst_en: SYSTEM_CPU_PERI_RST_EN,
    #[doc = "0x08 - SYSTEM_CPU_PER_CONF"]
    pub system_cpu_per_conf: SYSTEM_CPU_PER_CONF,
    #[doc = "0x0c - SYSTEM_MEM_PD_MASK"]
    pub system_mem_pd_mask: SYSTEM_MEM_PD_MASK,
    #[doc = "0x10 - SYSTEM_PERIP_CLK_EN0"]
    pub system_perip_clk_en0: SYSTEM_PERIP_CLK_EN0,
    #[doc = "0x14 - SYSTEM_PERIP_CLK_EN1"]
    pub system_perip_clk_en1: SYSTEM_PERIP_CLK_EN1,
    #[doc = "0x18 - SYSTEM_PERIP_RST_EN0"]
    pub system_perip_rst_en0: SYSTEM_PERIP_RST_EN0,
    #[doc = "0x1c - SYSTEM_PERIP_RST_EN1"]
    pub system_perip_rst_en1: SYSTEM_PERIP_RST_EN1,
    #[doc = "0x20 - SYSTEM_BT_LPCK_DIV_INT"]
    pub system_bt_lpck_div_int: SYSTEM_BT_LPCK_DIV_INT,
    #[doc = "0x24 - SYSTEM_BT_LPCK_DIV_FRAC"]
    pub system_bt_lpck_div_frac: SYSTEM_BT_LPCK_DIV_FRAC,
    #[doc = "0x28 - SYSTEM_CPU_INTR_FROM_CPU_0"]
    pub system_cpu_intr_from_cpu_0: SYSTEM_CPU_INTR_FROM_CPU_0,
    #[doc = "0x2c - SYSTEM_CPU_INTR_FROM_CPU_1"]
    pub system_cpu_intr_from_cpu_1: SYSTEM_CPU_INTR_FROM_CPU_1,
    #[doc = "0x30 - SYSTEM_CPU_INTR_FROM_CPU_2"]
    pub system_cpu_intr_from_cpu_2: SYSTEM_CPU_INTR_FROM_CPU_2,
    #[doc = "0x34 - SYSTEM_CPU_INTR_FROM_CPU_3"]
    pub system_cpu_intr_from_cpu_3: SYSTEM_CPU_INTR_FROM_CPU_3,
    #[doc = "0x38 - SYSTEM_RSA_PD_CTRL"]
    pub system_rsa_pd_ctrl: SYSTEM_RSA_PD_CTRL,
    #[doc = "0x3c - SYSTEM_EDMA_CTRL"]
    pub system_edma_ctrl: SYSTEM_EDMA_CTRL,
    #[doc = "0x40 - SYSTEM_CACHE_CONTROL"]
    pub system_cache_control: SYSTEM_CACHE_CONTROL,
    #[doc = "0x44 - SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL"]
    pub system_external_device_encrypt_decrypt_control:
        SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL,
    #[doc = "0x48 - SYSTEM_RTC_FASTMEM_CONFIG"]
    pub system_rtc_fastmem_config: SYSTEM_RTC_FASTMEM_CONFIG,
    #[doc = "0x4c - SYSTEM_RTC_FASTMEM_CRC"]
    pub system_rtc_fastmem_crc: SYSTEM_RTC_FASTMEM_CRC,
    #[doc = "0x50 - SYSTEM_REDUNDANT_ECO_CTRL"]
    pub system_redundant_eco_ctrl: SYSTEM_REDUNDANT_ECO_CTRL,
    #[doc = "0x54 - SYSTEM_CLOCK_GATE"]
    pub system_clock_gate: SYSTEM_CLOCK_GATE,
    #[doc = "0x58 - SYSTEM_SYSCLK_CONF"]
    pub system_sysclk_conf: SYSTEM_SYSCLK_CONF,
    #[doc = "0x5c - SYSTEM_MEM_PVT"]
    pub system_mem_pvt: SYSTEM_MEM_PVT,
    #[doc = "0x60 - SYSTEM_COMB_PVT_LVT_CONF"]
    pub system_comb_pvt_lvt_conf: SYSTEM_COMB_PVT_LVT_CONF,
    #[doc = "0x64 - SYSTEM_COMB_PVT_NVT_CONF"]
    pub system_comb_pvt_nvt_conf: SYSTEM_COMB_PVT_NVT_CONF,
    #[doc = "0x68 - SYSTEM_COMB_PVT_HVT_CONF"]
    pub system_comb_pvt_hvt_conf: SYSTEM_COMB_PVT_HVT_CONF,
    #[doc = "0x6c - SYSTEM_COMB_PVT_ERR_LVT_SITE0"]
    pub system_comb_pvt_err_lvt_site0: SYSTEM_COMB_PVT_ERR_LVT_SITE0,
    #[doc = "0x70 - SYSTEM_COMB_PVT_ERR_NVT_SITE0"]
    pub system_comb_pvt_err_nvt_site0: SYSTEM_COMB_PVT_ERR_NVT_SITE0,
    #[doc = "0x74 - SYSTEM_COMB_PVT_ERR_HVT_SITE0"]
    pub system_comb_pvt_err_hvt_site0: SYSTEM_COMB_PVT_ERR_HVT_SITE0,
    #[doc = "0x78 - SYSTEM_COMB_PVT_ERR_LVT_SITE1"]
    pub system_comb_pvt_err_lvt_site1: SYSTEM_COMB_PVT_ERR_LVT_SITE1,
    #[doc = "0x7c - SYSTEM_COMB_PVT_ERR_NVT_SITE1"]
    pub system_comb_pvt_err_nvt_site1: SYSTEM_COMB_PVT_ERR_NVT_SITE1,
    #[doc = "0x80 - SYSTEM_COMB_PVT_ERR_HVT_SITE1"]
    pub system_comb_pvt_err_hvt_site1: SYSTEM_COMB_PVT_ERR_HVT_SITE1,
    #[doc = "0x84 - SYSTEM_COMB_PVT_ERR_LVT_SITE2"]
    pub system_comb_pvt_err_lvt_site2: SYSTEM_COMB_PVT_ERR_LVT_SITE2,
    #[doc = "0x88 - SYSTEM_COMB_PVT_ERR_NVT_SITE2"]
    pub system_comb_pvt_err_nvt_site2: SYSTEM_COMB_PVT_ERR_NVT_SITE2,
    #[doc = "0x8c - SYSTEM_COMB_PVT_ERR_HVT_SITE2"]
    pub system_comb_pvt_err_hvt_site2: SYSTEM_COMB_PVT_ERR_HVT_SITE2,
    #[doc = "0x90 - SYSTEM_COMB_PVT_ERR_LVT_SITE3"]
    pub system_comb_pvt_err_lvt_site3: SYSTEM_COMB_PVT_ERR_LVT_SITE3,
    #[doc = "0x94 - SYSTEM_COMB_PVT_ERR_NVT_SITE3"]
    pub system_comb_pvt_err_nvt_site3: SYSTEM_COMB_PVT_ERR_NVT_SITE3,
    #[doc = "0x98 - SYSTEM_COMB_PVT_ERR_HVT_SITE3"]
    pub system_comb_pvt_err_hvt_site3: SYSTEM_COMB_PVT_ERR_HVT_SITE3,
    _reserved39: [u8; 3936usize],
    #[doc = "0xffc - SYSTEM_DATE"]
    pub system_date: SYSTEM_DATE,
}
#[doc = "SYSTEM_CPU_PERI_CLK_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_cpu_peri_clk_en](system_cpu_peri_clk_en) module"]
pub type SYSTEM_CPU_PERI_CLK_EN = crate::Reg<u32, _SYSTEM_CPU_PERI_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CPU_PERI_CLK_EN;
#[doc = "`read()` method returns [system_cpu_peri_clk_en::R](system_cpu_peri_clk_en::R) reader structure"]
impl crate::Readable for SYSTEM_CPU_PERI_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [system_cpu_peri_clk_en::W](system_cpu_peri_clk_en::W) writer structure"]
impl crate::Writable for SYSTEM_CPU_PERI_CLK_EN {}
#[doc = "SYSTEM_CPU_PERI_CLK_EN"]
pub mod system_cpu_peri_clk_en;
#[doc = "SYSTEM_CPU_PERI_RST_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_cpu_peri_rst_en](system_cpu_peri_rst_en) module"]
pub type SYSTEM_CPU_PERI_RST_EN = crate::Reg<u32, _SYSTEM_CPU_PERI_RST_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CPU_PERI_RST_EN;
#[doc = "`read()` method returns [system_cpu_peri_rst_en::R](system_cpu_peri_rst_en::R) reader structure"]
impl crate::Readable for SYSTEM_CPU_PERI_RST_EN {}
#[doc = "`write(|w| ..)` method takes [system_cpu_peri_rst_en::W](system_cpu_peri_rst_en::W) writer structure"]
impl crate::Writable for SYSTEM_CPU_PERI_RST_EN {}
#[doc = "SYSTEM_CPU_PERI_RST_EN"]
pub mod system_cpu_peri_rst_en;
#[doc = "SYSTEM_CPU_PER_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_cpu_per_conf](system_cpu_per_conf) module"]
pub type SYSTEM_CPU_PER_CONF = crate::Reg<u32, _SYSTEM_CPU_PER_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CPU_PER_CONF;
#[doc = "`read()` method returns [system_cpu_per_conf::R](system_cpu_per_conf::R) reader structure"]
impl crate::Readable for SYSTEM_CPU_PER_CONF {}
#[doc = "`write(|w| ..)` method takes [system_cpu_per_conf::W](system_cpu_per_conf::W) writer structure"]
impl crate::Writable for SYSTEM_CPU_PER_CONF {}
#[doc = "SYSTEM_CPU_PER_CONF"]
pub mod system_cpu_per_conf;
#[doc = "SYSTEM_MEM_PD_MASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_mem_pd_mask](system_mem_pd_mask) module"]
pub type SYSTEM_MEM_PD_MASK = crate::Reg<u32, _SYSTEM_MEM_PD_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_MEM_PD_MASK;
#[doc = "`read()` method returns [system_mem_pd_mask::R](system_mem_pd_mask::R) reader structure"]
impl crate::Readable for SYSTEM_MEM_PD_MASK {}
#[doc = "`write(|w| ..)` method takes [system_mem_pd_mask::W](system_mem_pd_mask::W) writer structure"]
impl crate::Writable for SYSTEM_MEM_PD_MASK {}
#[doc = "SYSTEM_MEM_PD_MASK"]
pub mod system_mem_pd_mask;
#[doc = "SYSTEM_PERIP_CLK_EN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_perip_clk_en0](system_perip_clk_en0) module"]
pub type SYSTEM_PERIP_CLK_EN0 = crate::Reg<u32, _SYSTEM_PERIP_CLK_EN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_PERIP_CLK_EN0;
#[doc = "`read()` method returns [system_perip_clk_en0::R](system_perip_clk_en0::R) reader structure"]
impl crate::Readable for SYSTEM_PERIP_CLK_EN0 {}
#[doc = "`write(|w| ..)` method takes [system_perip_clk_en0::W](system_perip_clk_en0::W) writer structure"]
impl crate::Writable for SYSTEM_PERIP_CLK_EN0 {}
#[doc = "SYSTEM_PERIP_CLK_EN0"]
pub mod system_perip_clk_en0;
#[doc = "SYSTEM_PERIP_CLK_EN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_perip_clk_en1](system_perip_clk_en1) module"]
pub type SYSTEM_PERIP_CLK_EN1 = crate::Reg<u32, _SYSTEM_PERIP_CLK_EN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_PERIP_CLK_EN1;
#[doc = "`read()` method returns [system_perip_clk_en1::R](system_perip_clk_en1::R) reader structure"]
impl crate::Readable for SYSTEM_PERIP_CLK_EN1 {}
#[doc = "`write(|w| ..)` method takes [system_perip_clk_en1::W](system_perip_clk_en1::W) writer structure"]
impl crate::Writable for SYSTEM_PERIP_CLK_EN1 {}
#[doc = "SYSTEM_PERIP_CLK_EN1"]
pub mod system_perip_clk_en1;
#[doc = "SYSTEM_PERIP_RST_EN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_perip_rst_en0](system_perip_rst_en0) module"]
pub type SYSTEM_PERIP_RST_EN0 = crate::Reg<u32, _SYSTEM_PERIP_RST_EN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_PERIP_RST_EN0;
#[doc = "`read()` method returns [system_perip_rst_en0::R](system_perip_rst_en0::R) reader structure"]
impl crate::Readable for SYSTEM_PERIP_RST_EN0 {}
#[doc = "`write(|w| ..)` method takes [system_perip_rst_en0::W](system_perip_rst_en0::W) writer structure"]
impl crate::Writable for SYSTEM_PERIP_RST_EN0 {}
#[doc = "SYSTEM_PERIP_RST_EN0"]
pub mod system_perip_rst_en0;
#[doc = "SYSTEM_PERIP_RST_EN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_perip_rst_en1](system_perip_rst_en1) module"]
pub type SYSTEM_PERIP_RST_EN1 = crate::Reg<u32, _SYSTEM_PERIP_RST_EN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_PERIP_RST_EN1;
#[doc = "`read()` method returns [system_perip_rst_en1::R](system_perip_rst_en1::R) reader structure"]
impl crate::Readable for SYSTEM_PERIP_RST_EN1 {}
#[doc = "`write(|w| ..)` method takes [system_perip_rst_en1::W](system_perip_rst_en1::W) writer structure"]
impl crate::Writable for SYSTEM_PERIP_RST_EN1 {}
#[doc = "SYSTEM_PERIP_RST_EN1"]
pub mod system_perip_rst_en1;
#[doc = "SYSTEM_BT_LPCK_DIV_INT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_bt_lpck_div_int](system_bt_lpck_div_int) module"]
pub type SYSTEM_BT_LPCK_DIV_INT = crate::Reg<u32, _SYSTEM_BT_LPCK_DIV_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_BT_LPCK_DIV_INT;
#[doc = "`read()` method returns [system_bt_lpck_div_int::R](system_bt_lpck_div_int::R) reader structure"]
impl crate::Readable for SYSTEM_BT_LPCK_DIV_INT {}
#[doc = "`write(|w| ..)` method takes [system_bt_lpck_div_int::W](system_bt_lpck_div_int::W) writer structure"]
impl crate::Writable for SYSTEM_BT_LPCK_DIV_INT {}
#[doc = "SYSTEM_BT_LPCK_DIV_INT"]
pub mod system_bt_lpck_div_int;
#[doc = "SYSTEM_BT_LPCK_DIV_FRAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_bt_lpck_div_frac](system_bt_lpck_div_frac) module"]
pub type SYSTEM_BT_LPCK_DIV_FRAC = crate::Reg<u32, _SYSTEM_BT_LPCK_DIV_FRAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_BT_LPCK_DIV_FRAC;
#[doc = "`read()` method returns [system_bt_lpck_div_frac::R](system_bt_lpck_div_frac::R) reader structure"]
impl crate::Readable for SYSTEM_BT_LPCK_DIV_FRAC {}
#[doc = "`write(|w| ..)` method takes [system_bt_lpck_div_frac::W](system_bt_lpck_div_frac::W) writer structure"]
impl crate::Writable for SYSTEM_BT_LPCK_DIV_FRAC {}
#[doc = "SYSTEM_BT_LPCK_DIV_FRAC"]
pub mod system_bt_lpck_div_frac;
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_cpu_intr_from_cpu_0](system_cpu_intr_from_cpu_0) module"]
pub type SYSTEM_CPU_INTR_FROM_CPU_0 = crate::Reg<u32, _SYSTEM_CPU_INTR_FROM_CPU_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CPU_INTR_FROM_CPU_0;
#[doc = "`read()` method returns [system_cpu_intr_from_cpu_0::R](system_cpu_intr_from_cpu_0::R) reader structure"]
impl crate::Readable for SYSTEM_CPU_INTR_FROM_CPU_0 {}
#[doc = "`write(|w| ..)` method takes [system_cpu_intr_from_cpu_0::W](system_cpu_intr_from_cpu_0::W) writer structure"]
impl crate::Writable for SYSTEM_CPU_INTR_FROM_CPU_0 {}
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_0"]
pub mod system_cpu_intr_from_cpu_0;
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_cpu_intr_from_cpu_1](system_cpu_intr_from_cpu_1) module"]
pub type SYSTEM_CPU_INTR_FROM_CPU_1 = crate::Reg<u32, _SYSTEM_CPU_INTR_FROM_CPU_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CPU_INTR_FROM_CPU_1;
#[doc = "`read()` method returns [system_cpu_intr_from_cpu_1::R](system_cpu_intr_from_cpu_1::R) reader structure"]
impl crate::Readable for SYSTEM_CPU_INTR_FROM_CPU_1 {}
#[doc = "`write(|w| ..)` method takes [system_cpu_intr_from_cpu_1::W](system_cpu_intr_from_cpu_1::W) writer structure"]
impl crate::Writable for SYSTEM_CPU_INTR_FROM_CPU_1 {}
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_1"]
pub mod system_cpu_intr_from_cpu_1;
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_cpu_intr_from_cpu_2](system_cpu_intr_from_cpu_2) module"]
pub type SYSTEM_CPU_INTR_FROM_CPU_2 = crate::Reg<u32, _SYSTEM_CPU_INTR_FROM_CPU_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CPU_INTR_FROM_CPU_2;
#[doc = "`read()` method returns [system_cpu_intr_from_cpu_2::R](system_cpu_intr_from_cpu_2::R) reader structure"]
impl crate::Readable for SYSTEM_CPU_INTR_FROM_CPU_2 {}
#[doc = "`write(|w| ..)` method takes [system_cpu_intr_from_cpu_2::W](system_cpu_intr_from_cpu_2::W) writer structure"]
impl crate::Writable for SYSTEM_CPU_INTR_FROM_CPU_2 {}
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_2"]
pub mod system_cpu_intr_from_cpu_2;
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_cpu_intr_from_cpu_3](system_cpu_intr_from_cpu_3) module"]
pub type SYSTEM_CPU_INTR_FROM_CPU_3 = crate::Reg<u32, _SYSTEM_CPU_INTR_FROM_CPU_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CPU_INTR_FROM_CPU_3;
#[doc = "`read()` method returns [system_cpu_intr_from_cpu_3::R](system_cpu_intr_from_cpu_3::R) reader structure"]
impl crate::Readable for SYSTEM_CPU_INTR_FROM_CPU_3 {}
#[doc = "`write(|w| ..)` method takes [system_cpu_intr_from_cpu_3::W](system_cpu_intr_from_cpu_3::W) writer structure"]
impl crate::Writable for SYSTEM_CPU_INTR_FROM_CPU_3 {}
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_3"]
pub mod system_cpu_intr_from_cpu_3;
#[doc = "SYSTEM_RSA_PD_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_rsa_pd_ctrl](system_rsa_pd_ctrl) module"]
pub type SYSTEM_RSA_PD_CTRL = crate::Reg<u32, _SYSTEM_RSA_PD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_RSA_PD_CTRL;
#[doc = "`read()` method returns [system_rsa_pd_ctrl::R](system_rsa_pd_ctrl::R) reader structure"]
impl crate::Readable for SYSTEM_RSA_PD_CTRL {}
#[doc = "`write(|w| ..)` method takes [system_rsa_pd_ctrl::W](system_rsa_pd_ctrl::W) writer structure"]
impl crate::Writable for SYSTEM_RSA_PD_CTRL {}
#[doc = "SYSTEM_RSA_PD_CTRL"]
pub mod system_rsa_pd_ctrl;
#[doc = "SYSTEM_EDMA_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_edma_ctrl](system_edma_ctrl) module"]
pub type SYSTEM_EDMA_CTRL = crate::Reg<u32, _SYSTEM_EDMA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_EDMA_CTRL;
#[doc = "`read()` method returns [system_edma_ctrl::R](system_edma_ctrl::R) reader structure"]
impl crate::Readable for SYSTEM_EDMA_CTRL {}
#[doc = "`write(|w| ..)` method takes [system_edma_ctrl::W](system_edma_ctrl::W) writer structure"]
impl crate::Writable for SYSTEM_EDMA_CTRL {}
#[doc = "SYSTEM_EDMA_CTRL"]
pub mod system_edma_ctrl;
#[doc = "SYSTEM_CACHE_CONTROL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_cache_control](system_cache_control) module"]
pub type SYSTEM_CACHE_CONTROL = crate::Reg<u32, _SYSTEM_CACHE_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CACHE_CONTROL;
#[doc = "`read()` method returns [system_cache_control::R](system_cache_control::R) reader structure"]
impl crate::Readable for SYSTEM_CACHE_CONTROL {}
#[doc = "`write(|w| ..)` method takes [system_cache_control::W](system_cache_control::W) writer structure"]
impl crate::Writable for SYSTEM_CACHE_CONTROL {}
#[doc = "SYSTEM_CACHE_CONTROL"]
pub mod system_cache_control;
#[doc = "SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_external_device_encrypt_decrypt_control](system_external_device_encrypt_decrypt_control) module"]
pub type SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL =
    crate::Reg<u32, _SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL;
#[doc = "`read()` method returns [system_external_device_encrypt_decrypt_control::R](system_external_device_encrypt_decrypt_control::R) reader structure"]
impl crate::Readable for SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL {}
#[doc = "`write(|w| ..)` method takes [system_external_device_encrypt_decrypt_control::W](system_external_device_encrypt_decrypt_control::W) writer structure"]
impl crate::Writable for SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL {}
#[doc = "SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL"]
pub mod system_external_device_encrypt_decrypt_control;
#[doc = "SYSTEM_RTC_FASTMEM_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_rtc_fastmem_config](system_rtc_fastmem_config) module"]
pub type SYSTEM_RTC_FASTMEM_CONFIG = crate::Reg<u32, _SYSTEM_RTC_FASTMEM_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_RTC_FASTMEM_CONFIG;
#[doc = "`read()` method returns [system_rtc_fastmem_config::R](system_rtc_fastmem_config::R) reader structure"]
impl crate::Readable for SYSTEM_RTC_FASTMEM_CONFIG {}
#[doc = "`write(|w| ..)` method takes [system_rtc_fastmem_config::W](system_rtc_fastmem_config::W) writer structure"]
impl crate::Writable for SYSTEM_RTC_FASTMEM_CONFIG {}
#[doc = "SYSTEM_RTC_FASTMEM_CONFIG"]
pub mod system_rtc_fastmem_config;
#[doc = "SYSTEM_RTC_FASTMEM_CRC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_rtc_fastmem_crc](system_rtc_fastmem_crc) module"]
pub type SYSTEM_RTC_FASTMEM_CRC = crate::Reg<u32, _SYSTEM_RTC_FASTMEM_CRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_RTC_FASTMEM_CRC;
#[doc = "`read()` method returns [system_rtc_fastmem_crc::R](system_rtc_fastmem_crc::R) reader structure"]
impl crate::Readable for SYSTEM_RTC_FASTMEM_CRC {}
#[doc = "SYSTEM_RTC_FASTMEM_CRC"]
pub mod system_rtc_fastmem_crc;
#[doc = "SYSTEM_REDUNDANT_ECO_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_redundant_eco_ctrl](system_redundant_eco_ctrl) module"]
pub type SYSTEM_REDUNDANT_ECO_CTRL = crate::Reg<u32, _SYSTEM_REDUNDANT_ECO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_REDUNDANT_ECO_CTRL;
#[doc = "`read()` method returns [system_redundant_eco_ctrl::R](system_redundant_eco_ctrl::R) reader structure"]
impl crate::Readable for SYSTEM_REDUNDANT_ECO_CTRL {}
#[doc = "`write(|w| ..)` method takes [system_redundant_eco_ctrl::W](system_redundant_eco_ctrl::W) writer structure"]
impl crate::Writable for SYSTEM_REDUNDANT_ECO_CTRL {}
#[doc = "SYSTEM_REDUNDANT_ECO_CTRL"]
pub mod system_redundant_eco_ctrl;
#[doc = "SYSTEM_CLOCK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_clock_gate](system_clock_gate) module"]
pub type SYSTEM_CLOCK_GATE = crate::Reg<u32, _SYSTEM_CLOCK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_CLOCK_GATE;
#[doc = "`read()` method returns [system_clock_gate::R](system_clock_gate::R) reader structure"]
impl crate::Readable for SYSTEM_CLOCK_GATE {}
#[doc = "`write(|w| ..)` method takes [system_clock_gate::W](system_clock_gate::W) writer structure"]
impl crate::Writable for SYSTEM_CLOCK_GATE {}
#[doc = "SYSTEM_CLOCK_GATE"]
pub mod system_clock_gate;
#[doc = "SYSTEM_SYSCLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_sysclk_conf](system_sysclk_conf) module"]
pub type SYSTEM_SYSCLK_CONF = crate::Reg<u32, _SYSTEM_SYSCLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_SYSCLK_CONF;
#[doc = "`read()` method returns [system_sysclk_conf::R](system_sysclk_conf::R) reader structure"]
impl crate::Readable for SYSTEM_SYSCLK_CONF {}
#[doc = "`write(|w| ..)` method takes [system_sysclk_conf::W](system_sysclk_conf::W) writer structure"]
impl crate::Writable for SYSTEM_SYSCLK_CONF {}
#[doc = "SYSTEM_SYSCLK_CONF"]
pub mod system_sysclk_conf;
#[doc = "SYSTEM_MEM_PVT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_mem_pvt](system_mem_pvt) module"]
pub type SYSTEM_MEM_PVT = crate::Reg<u32, _SYSTEM_MEM_PVT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_MEM_PVT;
#[doc = "`read()` method returns [system_mem_pvt::R](system_mem_pvt::R) reader structure"]
impl crate::Readable for SYSTEM_MEM_PVT {}
#[doc = "`write(|w| ..)` method takes [system_mem_pvt::W](system_mem_pvt::W) writer structure"]
impl crate::Writable for SYSTEM_MEM_PVT {}
#[doc = "SYSTEM_MEM_PVT"]
pub mod system_mem_pvt;
#[doc = "SYSTEM_COMB_PVT_LVT_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_lvt_conf](system_comb_pvt_lvt_conf) module"]
pub type SYSTEM_COMB_PVT_LVT_CONF = crate::Reg<u32, _SYSTEM_COMB_PVT_LVT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_LVT_CONF;
#[doc = "`read()` method returns [system_comb_pvt_lvt_conf::R](system_comb_pvt_lvt_conf::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_LVT_CONF {}
#[doc = "`write(|w| ..)` method takes [system_comb_pvt_lvt_conf::W](system_comb_pvt_lvt_conf::W) writer structure"]
impl crate::Writable for SYSTEM_COMB_PVT_LVT_CONF {}
#[doc = "SYSTEM_COMB_PVT_LVT_CONF"]
pub mod system_comb_pvt_lvt_conf;
#[doc = "SYSTEM_COMB_PVT_NVT_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_nvt_conf](system_comb_pvt_nvt_conf) module"]
pub type SYSTEM_COMB_PVT_NVT_CONF = crate::Reg<u32, _SYSTEM_COMB_PVT_NVT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_NVT_CONF;
#[doc = "`read()` method returns [system_comb_pvt_nvt_conf::R](system_comb_pvt_nvt_conf::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_NVT_CONF {}
#[doc = "`write(|w| ..)` method takes [system_comb_pvt_nvt_conf::W](system_comb_pvt_nvt_conf::W) writer structure"]
impl crate::Writable for SYSTEM_COMB_PVT_NVT_CONF {}
#[doc = "SYSTEM_COMB_PVT_NVT_CONF"]
pub mod system_comb_pvt_nvt_conf;
#[doc = "SYSTEM_COMB_PVT_HVT_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_hvt_conf](system_comb_pvt_hvt_conf) module"]
pub type SYSTEM_COMB_PVT_HVT_CONF = crate::Reg<u32, _SYSTEM_COMB_PVT_HVT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_HVT_CONF;
#[doc = "`read()` method returns [system_comb_pvt_hvt_conf::R](system_comb_pvt_hvt_conf::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_HVT_CONF {}
#[doc = "`write(|w| ..)` method takes [system_comb_pvt_hvt_conf::W](system_comb_pvt_hvt_conf::W) writer structure"]
impl crate::Writable for SYSTEM_COMB_PVT_HVT_CONF {}
#[doc = "SYSTEM_COMB_PVT_HVT_CONF"]
pub mod system_comb_pvt_hvt_conf;
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_lvt_site0](system_comb_pvt_err_lvt_site0) module"]
pub type SYSTEM_COMB_PVT_ERR_LVT_SITE0 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_LVT_SITE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_LVT_SITE0;
#[doc = "`read()` method returns [system_comb_pvt_err_lvt_site0::R](system_comb_pvt_err_lvt_site0::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_LVT_SITE0 {}
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE0"]
pub mod system_comb_pvt_err_lvt_site0;
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_nvt_site0](system_comb_pvt_err_nvt_site0) module"]
pub type SYSTEM_COMB_PVT_ERR_NVT_SITE0 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_NVT_SITE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_NVT_SITE0;
#[doc = "`read()` method returns [system_comb_pvt_err_nvt_site0::R](system_comb_pvt_err_nvt_site0::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_NVT_SITE0 {}
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE0"]
pub mod system_comb_pvt_err_nvt_site0;
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_hvt_site0](system_comb_pvt_err_hvt_site0) module"]
pub type SYSTEM_COMB_PVT_ERR_HVT_SITE0 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_HVT_SITE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_HVT_SITE0;
#[doc = "`read()` method returns [system_comb_pvt_err_hvt_site0::R](system_comb_pvt_err_hvt_site0::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_HVT_SITE0 {}
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE0"]
pub mod system_comb_pvt_err_hvt_site0;
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_lvt_site1](system_comb_pvt_err_lvt_site1) module"]
pub type SYSTEM_COMB_PVT_ERR_LVT_SITE1 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_LVT_SITE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_LVT_SITE1;
#[doc = "`read()` method returns [system_comb_pvt_err_lvt_site1::R](system_comb_pvt_err_lvt_site1::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_LVT_SITE1 {}
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE1"]
pub mod system_comb_pvt_err_lvt_site1;
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_nvt_site1](system_comb_pvt_err_nvt_site1) module"]
pub type SYSTEM_COMB_PVT_ERR_NVT_SITE1 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_NVT_SITE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_NVT_SITE1;
#[doc = "`read()` method returns [system_comb_pvt_err_nvt_site1::R](system_comb_pvt_err_nvt_site1::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_NVT_SITE1 {}
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE1"]
pub mod system_comb_pvt_err_nvt_site1;
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_hvt_site1](system_comb_pvt_err_hvt_site1) module"]
pub type SYSTEM_COMB_PVT_ERR_HVT_SITE1 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_HVT_SITE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_HVT_SITE1;
#[doc = "`read()` method returns [system_comb_pvt_err_hvt_site1::R](system_comb_pvt_err_hvt_site1::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_HVT_SITE1 {}
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE1"]
pub mod system_comb_pvt_err_hvt_site1;
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_lvt_site2](system_comb_pvt_err_lvt_site2) module"]
pub type SYSTEM_COMB_PVT_ERR_LVT_SITE2 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_LVT_SITE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_LVT_SITE2;
#[doc = "`read()` method returns [system_comb_pvt_err_lvt_site2::R](system_comb_pvt_err_lvt_site2::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_LVT_SITE2 {}
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE2"]
pub mod system_comb_pvt_err_lvt_site2;
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_nvt_site2](system_comb_pvt_err_nvt_site2) module"]
pub type SYSTEM_COMB_PVT_ERR_NVT_SITE2 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_NVT_SITE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_NVT_SITE2;
#[doc = "`read()` method returns [system_comb_pvt_err_nvt_site2::R](system_comb_pvt_err_nvt_site2::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_NVT_SITE2 {}
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE2"]
pub mod system_comb_pvt_err_nvt_site2;
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_hvt_site2](system_comb_pvt_err_hvt_site2) module"]
pub type SYSTEM_COMB_PVT_ERR_HVT_SITE2 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_HVT_SITE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_HVT_SITE2;
#[doc = "`read()` method returns [system_comb_pvt_err_hvt_site2::R](system_comb_pvt_err_hvt_site2::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_HVT_SITE2 {}
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE2"]
pub mod system_comb_pvt_err_hvt_site2;
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_lvt_site3](system_comb_pvt_err_lvt_site3) module"]
pub type SYSTEM_COMB_PVT_ERR_LVT_SITE3 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_LVT_SITE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_LVT_SITE3;
#[doc = "`read()` method returns [system_comb_pvt_err_lvt_site3::R](system_comb_pvt_err_lvt_site3::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_LVT_SITE3 {}
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE3"]
pub mod system_comb_pvt_err_lvt_site3;
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_nvt_site3](system_comb_pvt_err_nvt_site3) module"]
pub type SYSTEM_COMB_PVT_ERR_NVT_SITE3 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_NVT_SITE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_NVT_SITE3;
#[doc = "`read()` method returns [system_comb_pvt_err_nvt_site3::R](system_comb_pvt_err_nvt_site3::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_NVT_SITE3 {}
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE3"]
pub mod system_comb_pvt_err_nvt_site3;
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_comb_pvt_err_hvt_site3](system_comb_pvt_err_hvt_site3) module"]
pub type SYSTEM_COMB_PVT_ERR_HVT_SITE3 = crate::Reg<u32, _SYSTEM_COMB_PVT_ERR_HVT_SITE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_COMB_PVT_ERR_HVT_SITE3;
#[doc = "`read()` method returns [system_comb_pvt_err_hvt_site3::R](system_comb_pvt_err_hvt_site3::R) reader structure"]
impl crate::Readable for SYSTEM_COMB_PVT_ERR_HVT_SITE3 {}
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE3"]
pub mod system_comb_pvt_err_hvt_site3;
#[doc = "SYSTEM_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_date](system_date) module"]
pub type SYSTEM_DATE = crate::Reg<u32, _SYSTEM_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_DATE;
#[doc = "`read()` method returns [system_date::R](system_date::R) reader structure"]
impl crate::Readable for SYSTEM_DATE {}
#[doc = "`write(|w| ..)` method takes [system_date::W](system_date::W) writer structure"]
impl crate::Writable for SYSTEM_DATE {}
#[doc = "SYSTEM_DATE"]
pub mod system_date;
