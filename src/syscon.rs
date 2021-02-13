#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCON_SYSCLK_CONF"]
    pub syscon_sysclk_conf: SYSCON_SYSCLK_CONF,
    #[doc = "0x04 - SYSCON_TICK_CONF"]
    pub syscon_tick_conf: SYSCON_TICK_CONF,
    #[doc = "0x08 - SYSCON_CLK_OUT_EN"]
    pub syscon_clk_out_en: SYSCON_CLK_OUT_EN,
    #[doc = "0x0c - SYSCON_WIFI_BB_CFG"]
    pub syscon_wifi_bb_cfg: SYSCON_WIFI_BB_CFG,
    #[doc = "0x10 - SYSCON_WIFI_BB_CFG_2"]
    pub syscon_wifi_bb_cfg_2: SYSCON_WIFI_BB_CFG_2,
    #[doc = "0x14 - SYSCON_WIFI_CLK_EN"]
    pub syscon_wifi_clk_en: SYSCON_WIFI_CLK_EN,
    #[doc = "0x18 - SYSCON_WIFI_RST_EN"]
    pub syscon_wifi_rst_en: SYSCON_WIFI_RST_EN,
    #[doc = "0x1c - SYSCON_HOST_INF_SEL"]
    pub syscon_host_inf_sel: SYSCON_HOST_INF_SEL,
    #[doc = "0x20 - SYSCON_EXT_MEM_PMS_LOCK"]
    pub syscon_ext_mem_pms_lock: SYSCON_EXT_MEM_PMS_LOCK,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - SYSCON_FLASH_ACE0_ATTR"]
    pub syscon_flash_ace0_attr: SYSCON_FLASH_ACE0_ATTR,
    #[doc = "0x2c - SYSCON_FLASH_ACE1_ATTR"]
    pub syscon_flash_ace1_attr: SYSCON_FLASH_ACE1_ATTR,
    #[doc = "0x30 - SYSCON_FLASH_ACE2_ATTR"]
    pub syscon_flash_ace2_attr: SYSCON_FLASH_ACE2_ATTR,
    #[doc = "0x34 - SYSCON_FLASH_ACE3_ATTR"]
    pub syscon_flash_ace3_attr: SYSCON_FLASH_ACE3_ATTR,
    #[doc = "0x38 - SYSCON_FLASH_ACE0_ADDR"]
    pub syscon_flash_ace0_addr: SYSCON_FLASH_ACE0_ADDR,
    #[doc = "0x3c - SYSCON_FLASH_ACE1_ADDR"]
    pub syscon_flash_ace1_addr: SYSCON_FLASH_ACE1_ADDR,
    #[doc = "0x40 - SYSCON_FLASH_ACE2_ADDR"]
    pub syscon_flash_ace2_addr: SYSCON_FLASH_ACE2_ADDR,
    #[doc = "0x44 - SYSCON_FLASH_ACE3_ADDR"]
    pub syscon_flash_ace3_addr: SYSCON_FLASH_ACE3_ADDR,
    #[doc = "0x48 - SYSCON_FLASH_ACE0_SIZE"]
    pub syscon_flash_ace0_size: SYSCON_FLASH_ACE0_SIZE,
    #[doc = "0x4c - SYSCON_FLASH_ACE1_SIZE"]
    pub syscon_flash_ace1_size: SYSCON_FLASH_ACE1_SIZE,
    #[doc = "0x50 - SYSCON_FLASH_ACE2_SIZE"]
    pub syscon_flash_ace2_size: SYSCON_FLASH_ACE2_SIZE,
    #[doc = "0x54 - SYSCON_FLASH_ACE3_SIZE"]
    pub syscon_flash_ace3_size: SYSCON_FLASH_ACE3_SIZE,
    _reserved21: [u8; 48usize],
    #[doc = "0x88 - SYSCON_SPI_MEM_PMS_CTRL"]
    pub syscon_spi_mem_pms_ctrl: SYSCON_SPI_MEM_PMS_CTRL,
    #[doc = "0x8c - SYSCON_SPI_MEM_REJECT_ADDR"]
    pub syscon_spi_mem_reject_addr: SYSCON_SPI_MEM_REJECT_ADDR,
    #[doc = "0x90 - SYSCON_SDIO_CTRL"]
    pub syscon_sdio_ctrl: SYSCON_SDIO_CTRL,
    #[doc = "0x94 - SYSCON_REDCY_SIG0"]
    pub syscon_redcy_sig0: SYSCON_REDCY_SIG0,
    #[doc = "0x98 - SYSCON_REDCY_SIG1"]
    pub syscon_redcy_sig1: SYSCON_REDCY_SIG1,
    #[doc = "0x9c - SYSCON_FRONT_END_MEM_PD"]
    pub syscon_front_end_mem_pd: SYSCON_FRONT_END_MEM_PD,
    #[doc = "0xa0 - SYSCON_RETENTION_CTRL"]
    pub syscon_retention_ctrl: SYSCON_RETENTION_CTRL,
    #[doc = "0xa4 - SYSCON_CLKGATE_FORCE_ON"]
    pub syscon_clkgate_force_on: SYSCON_CLKGATE_FORCE_ON,
    #[doc = "0xa8 - SYSCON_MEM_POWER_DOWN"]
    pub syscon_mem_power_down: SYSCON_MEM_POWER_DOWN,
    #[doc = "0xac - SYSCON_MEM_POWER_UP"]
    pub syscon_mem_power_up: SYSCON_MEM_POWER_UP,
    #[doc = "0xb0 - SYSCON_RND_DATA"]
    pub syscon_rnd_data: SYSCON_RND_DATA,
    #[doc = "0xb4 - SYSCON_PERI_BACKUP_CONFIG"]
    pub syscon_peri_backup_config: SYSCON_PERI_BACKUP_CONFIG,
    #[doc = "0xb8 - SYSCON_PERI_BACKUP_APB_ADDR"]
    pub syscon_peri_backup_apb_addr: SYSCON_PERI_BACKUP_APB_ADDR,
    #[doc = "0xbc - SYSCON_PERI_BACKUP_MEM_ADDR"]
    pub syscon_peri_backup_mem_addr: SYSCON_PERI_BACKUP_MEM_ADDR,
    #[doc = "0xc0 - SYSCON_PERI_BACKUP_INT_RAW"]
    pub syscon_peri_backup_int_raw: SYSCON_PERI_BACKUP_INT_RAW,
    #[doc = "0xc4 - SYSCON_PERI_BACKUP_INT_ST"]
    pub syscon_peri_backup_int_st: SYSCON_PERI_BACKUP_INT_ST,
    #[doc = "0xc8 - SYSCON_PERI_BACKUP_INT_ENA"]
    pub syscon_peri_backup_int_ena: SYSCON_PERI_BACKUP_INT_ENA,
    _reserved38: [u8; 4usize],
    #[doc = "0xd0 - SYSCON_PERI_BACKUP_INT_CLR"]
    pub syscon_peri_backup_int_clr: SYSCON_PERI_BACKUP_INT_CLR,
    _reserved39: [u8; 808usize],
    #[doc = "0x3fc - SYSCON_DATE"]
    pub syscon_date: SYSCON_DATE,
}
#[doc = "SYSCON_SYSCLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_sysclk_conf](syscon_sysclk_conf) module"]
pub type SYSCON_SYSCLK_CONF = crate::Reg<u32, _SYSCON_SYSCLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SYSCLK_CONF;
#[doc = "`read()` method returns [syscon_sysclk_conf::R](syscon_sysclk_conf::R) reader structure"]
impl crate::Readable for SYSCON_SYSCLK_CONF {}
#[doc = "`write(|w| ..)` method takes [syscon_sysclk_conf::W](syscon_sysclk_conf::W) writer structure"]
impl crate::Writable for SYSCON_SYSCLK_CONF {}
#[doc = "SYSCON_SYSCLK_CONF"]
pub mod syscon_sysclk_conf;
#[doc = "SYSCON_TICK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_tick_conf](syscon_tick_conf) module"]
pub type SYSCON_TICK_CONF = crate::Reg<u32, _SYSCON_TICK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_TICK_CONF;
#[doc = "`read()` method returns [syscon_tick_conf::R](syscon_tick_conf::R) reader structure"]
impl crate::Readable for SYSCON_TICK_CONF {}
#[doc = "`write(|w| ..)` method takes [syscon_tick_conf::W](syscon_tick_conf::W) writer structure"]
impl crate::Writable for SYSCON_TICK_CONF {}
#[doc = "SYSCON_TICK_CONF"]
pub mod syscon_tick_conf;
#[doc = "SYSCON_CLK_OUT_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_clk_out_en](syscon_clk_out_en) module"]
pub type SYSCON_CLK_OUT_EN = crate::Reg<u32, _SYSCON_CLK_OUT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_CLK_OUT_EN;
#[doc = "`read()` method returns [syscon_clk_out_en::R](syscon_clk_out_en::R) reader structure"]
impl crate::Readable for SYSCON_CLK_OUT_EN {}
#[doc = "`write(|w| ..)` method takes [syscon_clk_out_en::W](syscon_clk_out_en::W) writer structure"]
impl crate::Writable for SYSCON_CLK_OUT_EN {}
#[doc = "SYSCON_CLK_OUT_EN"]
pub mod syscon_clk_out_en;
#[doc = "SYSCON_WIFI_BB_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_wifi_bb_cfg](syscon_wifi_bb_cfg) module"]
pub type SYSCON_WIFI_BB_CFG = crate::Reg<u32, _SYSCON_WIFI_BB_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_WIFI_BB_CFG;
#[doc = "`read()` method returns [syscon_wifi_bb_cfg::R](syscon_wifi_bb_cfg::R) reader structure"]
impl crate::Readable for SYSCON_WIFI_BB_CFG {}
#[doc = "`write(|w| ..)` method takes [syscon_wifi_bb_cfg::W](syscon_wifi_bb_cfg::W) writer structure"]
impl crate::Writable for SYSCON_WIFI_BB_CFG {}
#[doc = "SYSCON_WIFI_BB_CFG"]
pub mod syscon_wifi_bb_cfg;
#[doc = "SYSCON_WIFI_BB_CFG_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_wifi_bb_cfg_2](syscon_wifi_bb_cfg_2) module"]
pub type SYSCON_WIFI_BB_CFG_2 = crate::Reg<u32, _SYSCON_WIFI_BB_CFG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_WIFI_BB_CFG_2;
#[doc = "`read()` method returns [syscon_wifi_bb_cfg_2::R](syscon_wifi_bb_cfg_2::R) reader structure"]
impl crate::Readable for SYSCON_WIFI_BB_CFG_2 {}
#[doc = "`write(|w| ..)` method takes [syscon_wifi_bb_cfg_2::W](syscon_wifi_bb_cfg_2::W) writer structure"]
impl crate::Writable for SYSCON_WIFI_BB_CFG_2 {}
#[doc = "SYSCON_WIFI_BB_CFG_2"]
pub mod syscon_wifi_bb_cfg_2;
#[doc = "SYSCON_WIFI_CLK_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_wifi_clk_en](syscon_wifi_clk_en) module"]
pub type SYSCON_WIFI_CLK_EN = crate::Reg<u32, _SYSCON_WIFI_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_WIFI_CLK_EN;
#[doc = "`read()` method returns [syscon_wifi_clk_en::R](syscon_wifi_clk_en::R) reader structure"]
impl crate::Readable for SYSCON_WIFI_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [syscon_wifi_clk_en::W](syscon_wifi_clk_en::W) writer structure"]
impl crate::Writable for SYSCON_WIFI_CLK_EN {}
#[doc = "SYSCON_WIFI_CLK_EN"]
pub mod syscon_wifi_clk_en;
#[doc = "SYSCON_WIFI_RST_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_wifi_rst_en](syscon_wifi_rst_en) module"]
pub type SYSCON_WIFI_RST_EN = crate::Reg<u32, _SYSCON_WIFI_RST_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_WIFI_RST_EN;
#[doc = "`read()` method returns [syscon_wifi_rst_en::R](syscon_wifi_rst_en::R) reader structure"]
impl crate::Readable for SYSCON_WIFI_RST_EN {}
#[doc = "`write(|w| ..)` method takes [syscon_wifi_rst_en::W](syscon_wifi_rst_en::W) writer structure"]
impl crate::Writable for SYSCON_WIFI_RST_EN {}
#[doc = "SYSCON_WIFI_RST_EN"]
pub mod syscon_wifi_rst_en;
#[doc = "SYSCON_HOST_INF_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_host_inf_sel](syscon_host_inf_sel) module"]
pub type SYSCON_HOST_INF_SEL = crate::Reg<u32, _SYSCON_HOST_INF_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_HOST_INF_SEL;
#[doc = "`read()` method returns [syscon_host_inf_sel::R](syscon_host_inf_sel::R) reader structure"]
impl crate::Readable for SYSCON_HOST_INF_SEL {}
#[doc = "`write(|w| ..)` method takes [syscon_host_inf_sel::W](syscon_host_inf_sel::W) writer structure"]
impl crate::Writable for SYSCON_HOST_INF_SEL {}
#[doc = "SYSCON_HOST_INF_SEL"]
pub mod syscon_host_inf_sel;
#[doc = "SYSCON_EXT_MEM_PMS_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_ext_mem_pms_lock](syscon_ext_mem_pms_lock) module"]
pub type SYSCON_EXT_MEM_PMS_LOCK = crate::Reg<u32, _SYSCON_EXT_MEM_PMS_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_EXT_MEM_PMS_LOCK;
#[doc = "`read()` method returns [syscon_ext_mem_pms_lock::R](syscon_ext_mem_pms_lock::R) reader structure"]
impl crate::Readable for SYSCON_EXT_MEM_PMS_LOCK {}
#[doc = "`write(|w| ..)` method takes [syscon_ext_mem_pms_lock::W](syscon_ext_mem_pms_lock::W) writer structure"]
impl crate::Writable for SYSCON_EXT_MEM_PMS_LOCK {}
#[doc = "SYSCON_EXT_MEM_PMS_LOCK"]
pub mod syscon_ext_mem_pms_lock;
#[doc = "SYSCON_FLASH_ACE0_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace0_attr](syscon_flash_ace0_attr) module"]
pub type SYSCON_FLASH_ACE0_ATTR = crate::Reg<u32, _SYSCON_FLASH_ACE0_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE0_ATTR;
#[doc = "`read()` method returns [syscon_flash_ace0_attr::R](syscon_flash_ace0_attr::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE0_ATTR {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace0_attr::W](syscon_flash_ace0_attr::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE0_ATTR {}
#[doc = "SYSCON_FLASH_ACE0_ATTR"]
pub mod syscon_flash_ace0_attr;
#[doc = "SYSCON_FLASH_ACE1_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace1_attr](syscon_flash_ace1_attr) module"]
pub type SYSCON_FLASH_ACE1_ATTR = crate::Reg<u32, _SYSCON_FLASH_ACE1_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE1_ATTR;
#[doc = "`read()` method returns [syscon_flash_ace1_attr::R](syscon_flash_ace1_attr::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE1_ATTR {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace1_attr::W](syscon_flash_ace1_attr::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE1_ATTR {}
#[doc = "SYSCON_FLASH_ACE1_ATTR"]
pub mod syscon_flash_ace1_attr;
#[doc = "SYSCON_FLASH_ACE2_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace2_attr](syscon_flash_ace2_attr) module"]
pub type SYSCON_FLASH_ACE2_ATTR = crate::Reg<u32, _SYSCON_FLASH_ACE2_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE2_ATTR;
#[doc = "`read()` method returns [syscon_flash_ace2_attr::R](syscon_flash_ace2_attr::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE2_ATTR {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace2_attr::W](syscon_flash_ace2_attr::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE2_ATTR {}
#[doc = "SYSCON_FLASH_ACE2_ATTR"]
pub mod syscon_flash_ace2_attr;
#[doc = "SYSCON_FLASH_ACE3_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace3_attr](syscon_flash_ace3_attr) module"]
pub type SYSCON_FLASH_ACE3_ATTR = crate::Reg<u32, _SYSCON_FLASH_ACE3_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE3_ATTR;
#[doc = "`read()` method returns [syscon_flash_ace3_attr::R](syscon_flash_ace3_attr::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE3_ATTR {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace3_attr::W](syscon_flash_ace3_attr::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE3_ATTR {}
#[doc = "SYSCON_FLASH_ACE3_ATTR"]
pub mod syscon_flash_ace3_attr;
#[doc = "SYSCON_FLASH_ACE0_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace0_addr](syscon_flash_ace0_addr) module"]
pub type SYSCON_FLASH_ACE0_ADDR = crate::Reg<u32, _SYSCON_FLASH_ACE0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE0_ADDR;
#[doc = "`read()` method returns [syscon_flash_ace0_addr::R](syscon_flash_ace0_addr::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE0_ADDR {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace0_addr::W](syscon_flash_ace0_addr::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE0_ADDR {}
#[doc = "SYSCON_FLASH_ACE0_ADDR"]
pub mod syscon_flash_ace0_addr;
#[doc = "SYSCON_FLASH_ACE1_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace1_addr](syscon_flash_ace1_addr) module"]
pub type SYSCON_FLASH_ACE1_ADDR = crate::Reg<u32, _SYSCON_FLASH_ACE1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE1_ADDR;
#[doc = "`read()` method returns [syscon_flash_ace1_addr::R](syscon_flash_ace1_addr::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE1_ADDR {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace1_addr::W](syscon_flash_ace1_addr::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE1_ADDR {}
#[doc = "SYSCON_FLASH_ACE1_ADDR"]
pub mod syscon_flash_ace1_addr;
#[doc = "SYSCON_FLASH_ACE2_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace2_addr](syscon_flash_ace2_addr) module"]
pub type SYSCON_FLASH_ACE2_ADDR = crate::Reg<u32, _SYSCON_FLASH_ACE2_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE2_ADDR;
#[doc = "`read()` method returns [syscon_flash_ace2_addr::R](syscon_flash_ace2_addr::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE2_ADDR {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace2_addr::W](syscon_flash_ace2_addr::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE2_ADDR {}
#[doc = "SYSCON_FLASH_ACE2_ADDR"]
pub mod syscon_flash_ace2_addr;
#[doc = "SYSCON_FLASH_ACE3_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace3_addr](syscon_flash_ace3_addr) module"]
pub type SYSCON_FLASH_ACE3_ADDR = crate::Reg<u32, _SYSCON_FLASH_ACE3_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE3_ADDR;
#[doc = "`read()` method returns [syscon_flash_ace3_addr::R](syscon_flash_ace3_addr::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE3_ADDR {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace3_addr::W](syscon_flash_ace3_addr::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE3_ADDR {}
#[doc = "SYSCON_FLASH_ACE3_ADDR"]
pub mod syscon_flash_ace3_addr;
#[doc = "SYSCON_FLASH_ACE0_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace0_size](syscon_flash_ace0_size) module"]
pub type SYSCON_FLASH_ACE0_SIZE = crate::Reg<u32, _SYSCON_FLASH_ACE0_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE0_SIZE;
#[doc = "`read()` method returns [syscon_flash_ace0_size::R](syscon_flash_ace0_size::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE0_SIZE {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace0_size::W](syscon_flash_ace0_size::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE0_SIZE {}
#[doc = "SYSCON_FLASH_ACE0_SIZE"]
pub mod syscon_flash_ace0_size;
#[doc = "SYSCON_FLASH_ACE1_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace1_size](syscon_flash_ace1_size) module"]
pub type SYSCON_FLASH_ACE1_SIZE = crate::Reg<u32, _SYSCON_FLASH_ACE1_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE1_SIZE;
#[doc = "`read()` method returns [syscon_flash_ace1_size::R](syscon_flash_ace1_size::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE1_SIZE {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace1_size::W](syscon_flash_ace1_size::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE1_SIZE {}
#[doc = "SYSCON_FLASH_ACE1_SIZE"]
pub mod syscon_flash_ace1_size;
#[doc = "SYSCON_FLASH_ACE2_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace2_size](syscon_flash_ace2_size) module"]
pub type SYSCON_FLASH_ACE2_SIZE = crate::Reg<u32, _SYSCON_FLASH_ACE2_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE2_SIZE;
#[doc = "`read()` method returns [syscon_flash_ace2_size::R](syscon_flash_ace2_size::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE2_SIZE {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace2_size::W](syscon_flash_ace2_size::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE2_SIZE {}
#[doc = "SYSCON_FLASH_ACE2_SIZE"]
pub mod syscon_flash_ace2_size;
#[doc = "SYSCON_FLASH_ACE3_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_flash_ace3_size](syscon_flash_ace3_size) module"]
pub type SYSCON_FLASH_ACE3_SIZE = crate::Reg<u32, _SYSCON_FLASH_ACE3_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FLASH_ACE3_SIZE;
#[doc = "`read()` method returns [syscon_flash_ace3_size::R](syscon_flash_ace3_size::R) reader structure"]
impl crate::Readable for SYSCON_FLASH_ACE3_SIZE {}
#[doc = "`write(|w| ..)` method takes [syscon_flash_ace3_size::W](syscon_flash_ace3_size::W) writer structure"]
impl crate::Writable for SYSCON_FLASH_ACE3_SIZE {}
#[doc = "SYSCON_FLASH_ACE3_SIZE"]
pub mod syscon_flash_ace3_size;
#[doc = "SYSCON_SPI_MEM_PMS_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_spi_mem_pms_ctrl](syscon_spi_mem_pms_ctrl) module"]
pub type SYSCON_SPI_MEM_PMS_CTRL = crate::Reg<u32, _SYSCON_SPI_MEM_PMS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SPI_MEM_PMS_CTRL;
#[doc = "`read()` method returns [syscon_spi_mem_pms_ctrl::R](syscon_spi_mem_pms_ctrl::R) reader structure"]
impl crate::Readable for SYSCON_SPI_MEM_PMS_CTRL {}
#[doc = "`write(|w| ..)` method takes [syscon_spi_mem_pms_ctrl::W](syscon_spi_mem_pms_ctrl::W) writer structure"]
impl crate::Writable for SYSCON_SPI_MEM_PMS_CTRL {}
#[doc = "SYSCON_SPI_MEM_PMS_CTRL"]
pub mod syscon_spi_mem_pms_ctrl;
#[doc = "SYSCON_SPI_MEM_REJECT_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_spi_mem_reject_addr](syscon_spi_mem_reject_addr) module"]
pub type SYSCON_SPI_MEM_REJECT_ADDR = crate::Reg<u32, _SYSCON_SPI_MEM_REJECT_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SPI_MEM_REJECT_ADDR;
#[doc = "`read()` method returns [syscon_spi_mem_reject_addr::R](syscon_spi_mem_reject_addr::R) reader structure"]
impl crate::Readable for SYSCON_SPI_MEM_REJECT_ADDR {}
#[doc = "SYSCON_SPI_MEM_REJECT_ADDR"]
pub mod syscon_spi_mem_reject_addr;
#[doc = "SYSCON_SDIO_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_sdio_ctrl](syscon_sdio_ctrl) module"]
pub type SYSCON_SDIO_CTRL = crate::Reg<u32, _SYSCON_SDIO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SDIO_CTRL;
#[doc = "`read()` method returns [syscon_sdio_ctrl::R](syscon_sdio_ctrl::R) reader structure"]
impl crate::Readable for SYSCON_SDIO_CTRL {}
#[doc = "`write(|w| ..)` method takes [syscon_sdio_ctrl::W](syscon_sdio_ctrl::W) writer structure"]
impl crate::Writable for SYSCON_SDIO_CTRL {}
#[doc = "SYSCON_SDIO_CTRL"]
pub mod syscon_sdio_ctrl;
#[doc = "SYSCON_REDCY_SIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_redcy_sig0](syscon_redcy_sig0) module"]
pub type SYSCON_REDCY_SIG0 = crate::Reg<u32, _SYSCON_REDCY_SIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_REDCY_SIG0;
#[doc = "`read()` method returns [syscon_redcy_sig0::R](syscon_redcy_sig0::R) reader structure"]
impl crate::Readable for SYSCON_REDCY_SIG0 {}
#[doc = "`write(|w| ..)` method takes [syscon_redcy_sig0::W](syscon_redcy_sig0::W) writer structure"]
impl crate::Writable for SYSCON_REDCY_SIG0 {}
#[doc = "SYSCON_REDCY_SIG0"]
pub mod syscon_redcy_sig0;
#[doc = "SYSCON_REDCY_SIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_redcy_sig1](syscon_redcy_sig1) module"]
pub type SYSCON_REDCY_SIG1 = crate::Reg<u32, _SYSCON_REDCY_SIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_REDCY_SIG1;
#[doc = "`read()` method returns [syscon_redcy_sig1::R](syscon_redcy_sig1::R) reader structure"]
impl crate::Readable for SYSCON_REDCY_SIG1 {}
#[doc = "`write(|w| ..)` method takes [syscon_redcy_sig1::W](syscon_redcy_sig1::W) writer structure"]
impl crate::Writable for SYSCON_REDCY_SIG1 {}
#[doc = "SYSCON_REDCY_SIG1"]
pub mod syscon_redcy_sig1;
#[doc = "SYSCON_FRONT_END_MEM_PD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_front_end_mem_pd](syscon_front_end_mem_pd) module"]
pub type SYSCON_FRONT_END_MEM_PD = crate::Reg<u32, _SYSCON_FRONT_END_MEM_PD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_FRONT_END_MEM_PD;
#[doc = "`read()` method returns [syscon_front_end_mem_pd::R](syscon_front_end_mem_pd::R) reader structure"]
impl crate::Readable for SYSCON_FRONT_END_MEM_PD {}
#[doc = "`write(|w| ..)` method takes [syscon_front_end_mem_pd::W](syscon_front_end_mem_pd::W) writer structure"]
impl crate::Writable for SYSCON_FRONT_END_MEM_PD {}
#[doc = "SYSCON_FRONT_END_MEM_PD"]
pub mod syscon_front_end_mem_pd;
#[doc = "SYSCON_RETENTION_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_retention_ctrl](syscon_retention_ctrl) module"]
pub type SYSCON_RETENTION_CTRL = crate::Reg<u32, _SYSCON_RETENTION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_RETENTION_CTRL;
#[doc = "`read()` method returns [syscon_retention_ctrl::R](syscon_retention_ctrl::R) reader structure"]
impl crate::Readable for SYSCON_RETENTION_CTRL {}
#[doc = "`write(|w| ..)` method takes [syscon_retention_ctrl::W](syscon_retention_ctrl::W) writer structure"]
impl crate::Writable for SYSCON_RETENTION_CTRL {}
#[doc = "SYSCON_RETENTION_CTRL"]
pub mod syscon_retention_ctrl;
#[doc = "SYSCON_CLKGATE_FORCE_ON\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_clkgate_force_on](syscon_clkgate_force_on) module"]
pub type SYSCON_CLKGATE_FORCE_ON = crate::Reg<u32, _SYSCON_CLKGATE_FORCE_ON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_CLKGATE_FORCE_ON;
#[doc = "`read()` method returns [syscon_clkgate_force_on::R](syscon_clkgate_force_on::R) reader structure"]
impl crate::Readable for SYSCON_CLKGATE_FORCE_ON {}
#[doc = "`write(|w| ..)` method takes [syscon_clkgate_force_on::W](syscon_clkgate_force_on::W) writer structure"]
impl crate::Writable for SYSCON_CLKGATE_FORCE_ON {}
#[doc = "SYSCON_CLKGATE_FORCE_ON"]
pub mod syscon_clkgate_force_on;
#[doc = "SYSCON_MEM_POWER_DOWN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_mem_power_down](syscon_mem_power_down) module"]
pub type SYSCON_MEM_POWER_DOWN = crate::Reg<u32, _SYSCON_MEM_POWER_DOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_MEM_POWER_DOWN;
#[doc = "`read()` method returns [syscon_mem_power_down::R](syscon_mem_power_down::R) reader structure"]
impl crate::Readable for SYSCON_MEM_POWER_DOWN {}
#[doc = "`write(|w| ..)` method takes [syscon_mem_power_down::W](syscon_mem_power_down::W) writer structure"]
impl crate::Writable for SYSCON_MEM_POWER_DOWN {}
#[doc = "SYSCON_MEM_POWER_DOWN"]
pub mod syscon_mem_power_down;
#[doc = "SYSCON_MEM_POWER_UP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_mem_power_up](syscon_mem_power_up) module"]
pub type SYSCON_MEM_POWER_UP = crate::Reg<u32, _SYSCON_MEM_POWER_UP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_MEM_POWER_UP;
#[doc = "`read()` method returns [syscon_mem_power_up::R](syscon_mem_power_up::R) reader structure"]
impl crate::Readable for SYSCON_MEM_POWER_UP {}
#[doc = "`write(|w| ..)` method takes [syscon_mem_power_up::W](syscon_mem_power_up::W) writer structure"]
impl crate::Writable for SYSCON_MEM_POWER_UP {}
#[doc = "SYSCON_MEM_POWER_UP"]
pub mod syscon_mem_power_up;
#[doc = "SYSCON_RND_DATA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_rnd_data](syscon_rnd_data) module"]
pub type SYSCON_RND_DATA = crate::Reg<u32, _SYSCON_RND_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_RND_DATA;
#[doc = "`read()` method returns [syscon_rnd_data::R](syscon_rnd_data::R) reader structure"]
impl crate::Readable for SYSCON_RND_DATA {}
#[doc = "SYSCON_RND_DATA"]
pub mod syscon_rnd_data;
#[doc = "SYSCON_PERI_BACKUP_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_peri_backup_config](syscon_peri_backup_config) module"]
pub type SYSCON_PERI_BACKUP_CONFIG = crate::Reg<u32, _SYSCON_PERI_BACKUP_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_PERI_BACKUP_CONFIG;
#[doc = "`read()` method returns [syscon_peri_backup_config::R](syscon_peri_backup_config::R) reader structure"]
impl crate::Readable for SYSCON_PERI_BACKUP_CONFIG {}
#[doc = "`write(|w| ..)` method takes [syscon_peri_backup_config::W](syscon_peri_backup_config::W) writer structure"]
impl crate::Writable for SYSCON_PERI_BACKUP_CONFIG {}
#[doc = "SYSCON_PERI_BACKUP_CONFIG"]
pub mod syscon_peri_backup_config;
#[doc = "SYSCON_PERI_BACKUP_APB_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_peri_backup_apb_addr](syscon_peri_backup_apb_addr) module"]
pub type SYSCON_PERI_BACKUP_APB_ADDR = crate::Reg<u32, _SYSCON_PERI_BACKUP_APB_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_PERI_BACKUP_APB_ADDR;
#[doc = "`read()` method returns [syscon_peri_backup_apb_addr::R](syscon_peri_backup_apb_addr::R) reader structure"]
impl crate::Readable for SYSCON_PERI_BACKUP_APB_ADDR {}
#[doc = "`write(|w| ..)` method takes [syscon_peri_backup_apb_addr::W](syscon_peri_backup_apb_addr::W) writer structure"]
impl crate::Writable for SYSCON_PERI_BACKUP_APB_ADDR {}
#[doc = "SYSCON_PERI_BACKUP_APB_ADDR"]
pub mod syscon_peri_backup_apb_addr;
#[doc = "SYSCON_PERI_BACKUP_MEM_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_peri_backup_mem_addr](syscon_peri_backup_mem_addr) module"]
pub type SYSCON_PERI_BACKUP_MEM_ADDR = crate::Reg<u32, _SYSCON_PERI_BACKUP_MEM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_PERI_BACKUP_MEM_ADDR;
#[doc = "`read()` method returns [syscon_peri_backup_mem_addr::R](syscon_peri_backup_mem_addr::R) reader structure"]
impl crate::Readable for SYSCON_PERI_BACKUP_MEM_ADDR {}
#[doc = "`write(|w| ..)` method takes [syscon_peri_backup_mem_addr::W](syscon_peri_backup_mem_addr::W) writer structure"]
impl crate::Writable for SYSCON_PERI_BACKUP_MEM_ADDR {}
#[doc = "SYSCON_PERI_BACKUP_MEM_ADDR"]
pub mod syscon_peri_backup_mem_addr;
#[doc = "SYSCON_PERI_BACKUP_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_peri_backup_int_raw](syscon_peri_backup_int_raw) module"]
pub type SYSCON_PERI_BACKUP_INT_RAW = crate::Reg<u32, _SYSCON_PERI_BACKUP_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_PERI_BACKUP_INT_RAW;
#[doc = "`read()` method returns [syscon_peri_backup_int_raw::R](syscon_peri_backup_int_raw::R) reader structure"]
impl crate::Readable for SYSCON_PERI_BACKUP_INT_RAW {}
#[doc = "SYSCON_PERI_BACKUP_INT_RAW"]
pub mod syscon_peri_backup_int_raw;
#[doc = "SYSCON_PERI_BACKUP_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_peri_backup_int_st](syscon_peri_backup_int_st) module"]
pub type SYSCON_PERI_BACKUP_INT_ST = crate::Reg<u32, _SYSCON_PERI_BACKUP_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_PERI_BACKUP_INT_ST;
#[doc = "`read()` method returns [syscon_peri_backup_int_st::R](syscon_peri_backup_int_st::R) reader structure"]
impl crate::Readable for SYSCON_PERI_BACKUP_INT_ST {}
#[doc = "SYSCON_PERI_BACKUP_INT_ST"]
pub mod syscon_peri_backup_int_st;
#[doc = "SYSCON_PERI_BACKUP_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_peri_backup_int_ena](syscon_peri_backup_int_ena) module"]
pub type SYSCON_PERI_BACKUP_INT_ENA = crate::Reg<u32, _SYSCON_PERI_BACKUP_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_PERI_BACKUP_INT_ENA;
#[doc = "`read()` method returns [syscon_peri_backup_int_ena::R](syscon_peri_backup_int_ena::R) reader structure"]
impl crate::Readable for SYSCON_PERI_BACKUP_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [syscon_peri_backup_int_ena::W](syscon_peri_backup_int_ena::W) writer structure"]
impl crate::Writable for SYSCON_PERI_BACKUP_INT_ENA {}
#[doc = "SYSCON_PERI_BACKUP_INT_ENA"]
pub mod syscon_peri_backup_int_ena;
#[doc = "SYSCON_PERI_BACKUP_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_peri_backup_int_clr](syscon_peri_backup_int_clr) module"]
pub type SYSCON_PERI_BACKUP_INT_CLR = crate::Reg<u32, _SYSCON_PERI_BACKUP_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_PERI_BACKUP_INT_CLR;
#[doc = "`write(|w| ..)` method takes [syscon_peri_backup_int_clr::W](syscon_peri_backup_int_clr::W) writer structure"]
impl crate::Writable for SYSCON_PERI_BACKUP_INT_CLR {}
#[doc = "SYSCON_PERI_BACKUP_INT_CLR"]
pub mod syscon_peri_backup_int_clr;
#[doc = "SYSCON_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscon_date](syscon_date) module"]
pub type SYSCON_DATE = crate::Reg<u32, _SYSCON_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_DATE;
#[doc = "`read()` method returns [syscon_date::R](syscon_date::R) reader structure"]
impl crate::Readable for SYSCON_DATE {}
#[doc = "`write(|w| ..)` method takes [syscon_date::W](syscon_date::W) writer structure"]
impl crate::Writable for SYSCON_DATE {}
#[doc = "SYSCON_DATE"]
pub mod syscon_date;
