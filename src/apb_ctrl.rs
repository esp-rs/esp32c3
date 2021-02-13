#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - APB_CTRL_SYSCLK_CONF"]
    pub apb_ctrl_sysclk_conf: APB_CTRL_SYSCLK_CONF,
    #[doc = "0x04 - APB_CTRL_TICK_CONF"]
    pub apb_ctrl_tick_conf: APB_CTRL_TICK_CONF,
    #[doc = "0x08 - APB_CTRL_CLK_OUT_EN"]
    pub apb_ctrl_clk_out_en: APB_CTRL_CLK_OUT_EN,
    #[doc = "0x0c - APB_CTRL_WIFI_BB_CFG"]
    pub apb_ctrl_wifi_bb_cfg: APB_CTRL_WIFI_BB_CFG,
    #[doc = "0x10 - APB_CTRL_WIFI_BB_CFG_2"]
    pub apb_ctrl_wifi_bb_cfg_2: APB_CTRL_WIFI_BB_CFG_2,
    #[doc = "0x14 - APB_CTRL_WIFI_CLK_EN"]
    pub apb_ctrl_wifi_clk_en: APB_CTRL_WIFI_CLK_EN,
    #[doc = "0x18 - APB_CTRL_WIFI_RST_EN"]
    pub apb_ctrl_wifi_rst_en: APB_CTRL_WIFI_RST_EN,
    #[doc = "0x1c - APB_CTRL_HOST_INF_SEL"]
    pub apb_ctrl_host_inf_sel: APB_CTRL_HOST_INF_SEL,
    #[doc = "0x20 - APB_CTRL_EXT_MEM_PMS_LOCK"]
    pub apb_ctrl_ext_mem_pms_lock: APB_CTRL_EXT_MEM_PMS_LOCK,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - APB_CTRL_FLASH_ACE0_ATTR"]
    pub apb_ctrl_flash_ace0_attr: APB_CTRL_FLASH_ACE0_ATTR,
    #[doc = "0x2c - APB_CTRL_FLASH_ACE1_ATTR"]
    pub apb_ctrl_flash_ace1_attr: APB_CTRL_FLASH_ACE1_ATTR,
    #[doc = "0x30 - APB_CTRL_FLASH_ACE2_ATTR"]
    pub apb_ctrl_flash_ace2_attr: APB_CTRL_FLASH_ACE2_ATTR,
    #[doc = "0x34 - APB_CTRL_FLASH_ACE3_ATTR"]
    pub apb_ctrl_flash_ace3_attr: APB_CTRL_FLASH_ACE3_ATTR,
    #[doc = "0x38 - APB_CTRL_FLASH_ACE0_ADDR"]
    pub apb_ctrl_flash_ace0_addr: APB_CTRL_FLASH_ACE0_ADDR,
    #[doc = "0x3c - APB_CTRL_FLASH_ACE1_ADDR"]
    pub apb_ctrl_flash_ace1_addr: APB_CTRL_FLASH_ACE1_ADDR,
    #[doc = "0x40 - APB_CTRL_FLASH_ACE2_ADDR"]
    pub apb_ctrl_flash_ace2_addr: APB_CTRL_FLASH_ACE2_ADDR,
    #[doc = "0x44 - APB_CTRL_FLASH_ACE3_ADDR"]
    pub apb_ctrl_flash_ace3_addr: APB_CTRL_FLASH_ACE3_ADDR,
    #[doc = "0x48 - APB_CTRL_FLASH_ACE0_SIZE"]
    pub apb_ctrl_flash_ace0_size: APB_CTRL_FLASH_ACE0_SIZE,
    #[doc = "0x4c - APB_CTRL_FLASH_ACE1_SIZE"]
    pub apb_ctrl_flash_ace1_size: APB_CTRL_FLASH_ACE1_SIZE,
    #[doc = "0x50 - APB_CTRL_FLASH_ACE2_SIZE"]
    pub apb_ctrl_flash_ace2_size: APB_CTRL_FLASH_ACE2_SIZE,
    #[doc = "0x54 - APB_CTRL_FLASH_ACE3_SIZE"]
    pub apb_ctrl_flash_ace3_size: APB_CTRL_FLASH_ACE3_SIZE,
    _reserved21: [u8; 48usize],
    #[doc = "0x88 - APB_CTRL_SPI_MEM_PMS_CTRL"]
    pub apb_ctrl_spi_mem_pms_ctrl: APB_CTRL_SPI_MEM_PMS_CTRL,
    #[doc = "0x8c - APB_CTRL_SPI_MEM_REJECT_ADDR"]
    pub apb_ctrl_spi_mem_reject_addr: APB_CTRL_SPI_MEM_REJECT_ADDR,
    #[doc = "0x90 - APB_CTRL_SDIO_CTRL"]
    pub apb_ctrl_sdio_ctrl: APB_CTRL_SDIO_CTRL,
    #[doc = "0x94 - APB_CTRL_REDCY_SIG0"]
    pub apb_ctrl_redcy_sig0: APB_CTRL_REDCY_SIG0,
    #[doc = "0x98 - APB_CTRL_REDCY_SIG1"]
    pub apb_ctrl_redcy_sig1: APB_CTRL_REDCY_SIG1,
    #[doc = "0x9c - APB_CTRL_FRONT_END_MEM_PD"]
    pub apb_ctrl_front_end_mem_pd: APB_CTRL_FRONT_END_MEM_PD,
    #[doc = "0xa0 - APB_CTRL_RETENTION_CTRL"]
    pub apb_ctrl_retention_ctrl: APB_CTRL_RETENTION_CTRL,
    #[doc = "0xa4 - APB_CTRL_CLKGATE_FORCE_ON"]
    pub apb_ctrl_clkgate_force_on: APB_CTRL_CLKGATE_FORCE_ON,
    #[doc = "0xa8 - APB_CTRL_MEM_POWER_DOWN"]
    pub apb_ctrl_mem_power_down: APB_CTRL_MEM_POWER_DOWN,
    #[doc = "0xac - APB_CTRL_MEM_POWER_UP"]
    pub apb_ctrl_mem_power_up: APB_CTRL_MEM_POWER_UP,
    #[doc = "0xb0 - APB_CTRL_RND_DATA"]
    pub apb_ctrl_rnd_data: APB_CTRL_RND_DATA,
    #[doc = "0xb4 - APB_CTRL_PERI_BACKUP_CONFIG"]
    pub apb_ctrl_peri_backup_config: APB_CTRL_PERI_BACKUP_CONFIG,
    #[doc = "0xb8 - APB_CTRL_PERI_BACKUP_APB_ADDR"]
    pub apb_ctrl_peri_backup_apb_addr: APB_CTRL_PERI_BACKUP_APB_ADDR,
    #[doc = "0xbc - APB_CTRL_PERI_BACKUP_MEM_ADDR"]
    pub apb_ctrl_peri_backup_mem_addr: APB_CTRL_PERI_BACKUP_MEM_ADDR,
    #[doc = "0xc0 - APB_CTRL_PERI_BACKUP_INT_RAW"]
    pub apb_ctrl_peri_backup_int_raw: APB_CTRL_PERI_BACKUP_INT_RAW,
    #[doc = "0xc4 - APB_CTRL_PERI_BACKUP_INT_ST"]
    pub apb_ctrl_peri_backup_int_st: APB_CTRL_PERI_BACKUP_INT_ST,
    #[doc = "0xc8 - APB_CTRL_PERI_BACKUP_INT_ENA"]
    pub apb_ctrl_peri_backup_int_ena: APB_CTRL_PERI_BACKUP_INT_ENA,
    _reserved38: [u8; 4usize],
    #[doc = "0xd0 - APB_CTRL_PERI_BACKUP_INT_CLR"]
    pub apb_ctrl_peri_backup_int_clr: APB_CTRL_PERI_BACKUP_INT_CLR,
    _reserved39: [u8; 808usize],
    #[doc = "0x3fc - APB_CTRL_DATE"]
    pub apb_ctrl_date: APB_CTRL_DATE,
}
#[doc = "APB_CTRL_SYSCLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_sysclk_conf](apb_ctrl_sysclk_conf) module"]
pub type APB_CTRL_SYSCLK_CONF = crate::Reg<u32, _APB_CTRL_SYSCLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_SYSCLK_CONF;
#[doc = "`read()` method returns [apb_ctrl_sysclk_conf::R](apb_ctrl_sysclk_conf::R) reader structure"]
impl crate::Readable for APB_CTRL_SYSCLK_CONF {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_sysclk_conf::W](apb_ctrl_sysclk_conf::W) writer structure"]
impl crate::Writable for APB_CTRL_SYSCLK_CONF {}
#[doc = "APB_CTRL_SYSCLK_CONF"]
pub mod apb_ctrl_sysclk_conf;
#[doc = "APB_CTRL_TICK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_tick_conf](apb_ctrl_tick_conf) module"]
pub type APB_CTRL_TICK_CONF = crate::Reg<u32, _APB_CTRL_TICK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_TICK_CONF;
#[doc = "`read()` method returns [apb_ctrl_tick_conf::R](apb_ctrl_tick_conf::R) reader structure"]
impl crate::Readable for APB_CTRL_TICK_CONF {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_tick_conf::W](apb_ctrl_tick_conf::W) writer structure"]
impl crate::Writable for APB_CTRL_TICK_CONF {}
#[doc = "APB_CTRL_TICK_CONF"]
pub mod apb_ctrl_tick_conf;
#[doc = "APB_CTRL_CLK_OUT_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_clk_out_en](apb_ctrl_clk_out_en) module"]
pub type APB_CTRL_CLK_OUT_EN = crate::Reg<u32, _APB_CTRL_CLK_OUT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_CLK_OUT_EN;
#[doc = "`read()` method returns [apb_ctrl_clk_out_en::R](apb_ctrl_clk_out_en::R) reader structure"]
impl crate::Readable for APB_CTRL_CLK_OUT_EN {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_clk_out_en::W](apb_ctrl_clk_out_en::W) writer structure"]
impl crate::Writable for APB_CTRL_CLK_OUT_EN {}
#[doc = "APB_CTRL_CLK_OUT_EN"]
pub mod apb_ctrl_clk_out_en;
#[doc = "APB_CTRL_WIFI_BB_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_wifi_bb_cfg](apb_ctrl_wifi_bb_cfg) module"]
pub type APB_CTRL_WIFI_BB_CFG = crate::Reg<u32, _APB_CTRL_WIFI_BB_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_WIFI_BB_CFG;
#[doc = "`read()` method returns [apb_ctrl_wifi_bb_cfg::R](apb_ctrl_wifi_bb_cfg::R) reader structure"]
impl crate::Readable for APB_CTRL_WIFI_BB_CFG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_wifi_bb_cfg::W](apb_ctrl_wifi_bb_cfg::W) writer structure"]
impl crate::Writable for APB_CTRL_WIFI_BB_CFG {}
#[doc = "APB_CTRL_WIFI_BB_CFG"]
pub mod apb_ctrl_wifi_bb_cfg;
#[doc = "APB_CTRL_WIFI_BB_CFG_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_wifi_bb_cfg_2](apb_ctrl_wifi_bb_cfg_2) module"]
pub type APB_CTRL_WIFI_BB_CFG_2 = crate::Reg<u32, _APB_CTRL_WIFI_BB_CFG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_WIFI_BB_CFG_2;
#[doc = "`read()` method returns [apb_ctrl_wifi_bb_cfg_2::R](apb_ctrl_wifi_bb_cfg_2::R) reader structure"]
impl crate::Readable for APB_CTRL_WIFI_BB_CFG_2 {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_wifi_bb_cfg_2::W](apb_ctrl_wifi_bb_cfg_2::W) writer structure"]
impl crate::Writable for APB_CTRL_WIFI_BB_CFG_2 {}
#[doc = "APB_CTRL_WIFI_BB_CFG_2"]
pub mod apb_ctrl_wifi_bb_cfg_2;
#[doc = "APB_CTRL_WIFI_CLK_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_wifi_clk_en](apb_ctrl_wifi_clk_en) module"]
pub type APB_CTRL_WIFI_CLK_EN = crate::Reg<u32, _APB_CTRL_WIFI_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_WIFI_CLK_EN;
#[doc = "`read()` method returns [apb_ctrl_wifi_clk_en::R](apb_ctrl_wifi_clk_en::R) reader structure"]
impl crate::Readable for APB_CTRL_WIFI_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_wifi_clk_en::W](apb_ctrl_wifi_clk_en::W) writer structure"]
impl crate::Writable for APB_CTRL_WIFI_CLK_EN {}
#[doc = "APB_CTRL_WIFI_CLK_EN"]
pub mod apb_ctrl_wifi_clk_en;
#[doc = "APB_CTRL_WIFI_RST_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_wifi_rst_en](apb_ctrl_wifi_rst_en) module"]
pub type APB_CTRL_WIFI_RST_EN = crate::Reg<u32, _APB_CTRL_WIFI_RST_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_WIFI_RST_EN;
#[doc = "`read()` method returns [apb_ctrl_wifi_rst_en::R](apb_ctrl_wifi_rst_en::R) reader structure"]
impl crate::Readable for APB_CTRL_WIFI_RST_EN {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_wifi_rst_en::W](apb_ctrl_wifi_rst_en::W) writer structure"]
impl crate::Writable for APB_CTRL_WIFI_RST_EN {}
#[doc = "APB_CTRL_WIFI_RST_EN"]
pub mod apb_ctrl_wifi_rst_en;
#[doc = "APB_CTRL_HOST_INF_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_host_inf_sel](apb_ctrl_host_inf_sel) module"]
pub type APB_CTRL_HOST_INF_SEL = crate::Reg<u32, _APB_CTRL_HOST_INF_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_HOST_INF_SEL;
#[doc = "`read()` method returns [apb_ctrl_host_inf_sel::R](apb_ctrl_host_inf_sel::R) reader structure"]
impl crate::Readable for APB_CTRL_HOST_INF_SEL {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_host_inf_sel::W](apb_ctrl_host_inf_sel::W) writer structure"]
impl crate::Writable for APB_CTRL_HOST_INF_SEL {}
#[doc = "APB_CTRL_HOST_INF_SEL"]
pub mod apb_ctrl_host_inf_sel;
#[doc = "APB_CTRL_EXT_MEM_PMS_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_ext_mem_pms_lock](apb_ctrl_ext_mem_pms_lock) module"]
pub type APB_CTRL_EXT_MEM_PMS_LOCK = crate::Reg<u32, _APB_CTRL_EXT_MEM_PMS_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_EXT_MEM_PMS_LOCK;
#[doc = "`read()` method returns [apb_ctrl_ext_mem_pms_lock::R](apb_ctrl_ext_mem_pms_lock::R) reader structure"]
impl crate::Readable for APB_CTRL_EXT_MEM_PMS_LOCK {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_ext_mem_pms_lock::W](apb_ctrl_ext_mem_pms_lock::W) writer structure"]
impl crate::Writable for APB_CTRL_EXT_MEM_PMS_LOCK {}
#[doc = "APB_CTRL_EXT_MEM_PMS_LOCK"]
pub mod apb_ctrl_ext_mem_pms_lock;
#[doc = "APB_CTRL_FLASH_ACE0_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace0_attr](apb_ctrl_flash_ace0_attr) module"]
pub type APB_CTRL_FLASH_ACE0_ATTR = crate::Reg<u32, _APB_CTRL_FLASH_ACE0_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE0_ATTR;
#[doc = "`read()` method returns [apb_ctrl_flash_ace0_attr::R](apb_ctrl_flash_ace0_attr::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE0_ATTR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace0_attr::W](apb_ctrl_flash_ace0_attr::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE0_ATTR {}
#[doc = "APB_CTRL_FLASH_ACE0_ATTR"]
pub mod apb_ctrl_flash_ace0_attr;
#[doc = "APB_CTRL_FLASH_ACE1_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace1_attr](apb_ctrl_flash_ace1_attr) module"]
pub type APB_CTRL_FLASH_ACE1_ATTR = crate::Reg<u32, _APB_CTRL_FLASH_ACE1_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE1_ATTR;
#[doc = "`read()` method returns [apb_ctrl_flash_ace1_attr::R](apb_ctrl_flash_ace1_attr::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE1_ATTR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace1_attr::W](apb_ctrl_flash_ace1_attr::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE1_ATTR {}
#[doc = "APB_CTRL_FLASH_ACE1_ATTR"]
pub mod apb_ctrl_flash_ace1_attr;
#[doc = "APB_CTRL_FLASH_ACE2_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace2_attr](apb_ctrl_flash_ace2_attr) module"]
pub type APB_CTRL_FLASH_ACE2_ATTR = crate::Reg<u32, _APB_CTRL_FLASH_ACE2_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE2_ATTR;
#[doc = "`read()` method returns [apb_ctrl_flash_ace2_attr::R](apb_ctrl_flash_ace2_attr::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE2_ATTR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace2_attr::W](apb_ctrl_flash_ace2_attr::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE2_ATTR {}
#[doc = "APB_CTRL_FLASH_ACE2_ATTR"]
pub mod apb_ctrl_flash_ace2_attr;
#[doc = "APB_CTRL_FLASH_ACE3_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace3_attr](apb_ctrl_flash_ace3_attr) module"]
pub type APB_CTRL_FLASH_ACE3_ATTR = crate::Reg<u32, _APB_CTRL_FLASH_ACE3_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE3_ATTR;
#[doc = "`read()` method returns [apb_ctrl_flash_ace3_attr::R](apb_ctrl_flash_ace3_attr::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE3_ATTR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace3_attr::W](apb_ctrl_flash_ace3_attr::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE3_ATTR {}
#[doc = "APB_CTRL_FLASH_ACE3_ATTR"]
pub mod apb_ctrl_flash_ace3_attr;
#[doc = "APB_CTRL_FLASH_ACE0_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace0_addr](apb_ctrl_flash_ace0_addr) module"]
pub type APB_CTRL_FLASH_ACE0_ADDR = crate::Reg<u32, _APB_CTRL_FLASH_ACE0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE0_ADDR;
#[doc = "`read()` method returns [apb_ctrl_flash_ace0_addr::R](apb_ctrl_flash_ace0_addr::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE0_ADDR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace0_addr::W](apb_ctrl_flash_ace0_addr::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE0_ADDR {}
#[doc = "APB_CTRL_FLASH_ACE0_ADDR"]
pub mod apb_ctrl_flash_ace0_addr;
#[doc = "APB_CTRL_FLASH_ACE1_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace1_addr](apb_ctrl_flash_ace1_addr) module"]
pub type APB_CTRL_FLASH_ACE1_ADDR = crate::Reg<u32, _APB_CTRL_FLASH_ACE1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE1_ADDR;
#[doc = "`read()` method returns [apb_ctrl_flash_ace1_addr::R](apb_ctrl_flash_ace1_addr::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE1_ADDR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace1_addr::W](apb_ctrl_flash_ace1_addr::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE1_ADDR {}
#[doc = "APB_CTRL_FLASH_ACE1_ADDR"]
pub mod apb_ctrl_flash_ace1_addr;
#[doc = "APB_CTRL_FLASH_ACE2_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace2_addr](apb_ctrl_flash_ace2_addr) module"]
pub type APB_CTRL_FLASH_ACE2_ADDR = crate::Reg<u32, _APB_CTRL_FLASH_ACE2_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE2_ADDR;
#[doc = "`read()` method returns [apb_ctrl_flash_ace2_addr::R](apb_ctrl_flash_ace2_addr::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE2_ADDR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace2_addr::W](apb_ctrl_flash_ace2_addr::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE2_ADDR {}
#[doc = "APB_CTRL_FLASH_ACE2_ADDR"]
pub mod apb_ctrl_flash_ace2_addr;
#[doc = "APB_CTRL_FLASH_ACE3_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace3_addr](apb_ctrl_flash_ace3_addr) module"]
pub type APB_CTRL_FLASH_ACE3_ADDR = crate::Reg<u32, _APB_CTRL_FLASH_ACE3_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE3_ADDR;
#[doc = "`read()` method returns [apb_ctrl_flash_ace3_addr::R](apb_ctrl_flash_ace3_addr::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE3_ADDR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace3_addr::W](apb_ctrl_flash_ace3_addr::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE3_ADDR {}
#[doc = "APB_CTRL_FLASH_ACE3_ADDR"]
pub mod apb_ctrl_flash_ace3_addr;
#[doc = "APB_CTRL_FLASH_ACE0_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace0_size](apb_ctrl_flash_ace0_size) module"]
pub type APB_CTRL_FLASH_ACE0_SIZE = crate::Reg<u32, _APB_CTRL_FLASH_ACE0_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE0_SIZE;
#[doc = "`read()` method returns [apb_ctrl_flash_ace0_size::R](apb_ctrl_flash_ace0_size::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE0_SIZE {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace0_size::W](apb_ctrl_flash_ace0_size::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE0_SIZE {}
#[doc = "APB_CTRL_FLASH_ACE0_SIZE"]
pub mod apb_ctrl_flash_ace0_size;
#[doc = "APB_CTRL_FLASH_ACE1_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace1_size](apb_ctrl_flash_ace1_size) module"]
pub type APB_CTRL_FLASH_ACE1_SIZE = crate::Reg<u32, _APB_CTRL_FLASH_ACE1_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE1_SIZE;
#[doc = "`read()` method returns [apb_ctrl_flash_ace1_size::R](apb_ctrl_flash_ace1_size::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE1_SIZE {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace1_size::W](apb_ctrl_flash_ace1_size::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE1_SIZE {}
#[doc = "APB_CTRL_FLASH_ACE1_SIZE"]
pub mod apb_ctrl_flash_ace1_size;
#[doc = "APB_CTRL_FLASH_ACE2_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace2_size](apb_ctrl_flash_ace2_size) module"]
pub type APB_CTRL_FLASH_ACE2_SIZE = crate::Reg<u32, _APB_CTRL_FLASH_ACE2_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE2_SIZE;
#[doc = "`read()` method returns [apb_ctrl_flash_ace2_size::R](apb_ctrl_flash_ace2_size::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE2_SIZE {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace2_size::W](apb_ctrl_flash_ace2_size::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE2_SIZE {}
#[doc = "APB_CTRL_FLASH_ACE2_SIZE"]
pub mod apb_ctrl_flash_ace2_size;
#[doc = "APB_CTRL_FLASH_ACE3_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_flash_ace3_size](apb_ctrl_flash_ace3_size) module"]
pub type APB_CTRL_FLASH_ACE3_SIZE = crate::Reg<u32, _APB_CTRL_FLASH_ACE3_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FLASH_ACE3_SIZE;
#[doc = "`read()` method returns [apb_ctrl_flash_ace3_size::R](apb_ctrl_flash_ace3_size::R) reader structure"]
impl crate::Readable for APB_CTRL_FLASH_ACE3_SIZE {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_flash_ace3_size::W](apb_ctrl_flash_ace3_size::W) writer structure"]
impl crate::Writable for APB_CTRL_FLASH_ACE3_SIZE {}
#[doc = "APB_CTRL_FLASH_ACE3_SIZE"]
pub mod apb_ctrl_flash_ace3_size;
#[doc = "APB_CTRL_SPI_MEM_PMS_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_spi_mem_pms_ctrl](apb_ctrl_spi_mem_pms_ctrl) module"]
pub type APB_CTRL_SPI_MEM_PMS_CTRL = crate::Reg<u32, _APB_CTRL_SPI_MEM_PMS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_SPI_MEM_PMS_CTRL;
#[doc = "`read()` method returns [apb_ctrl_spi_mem_pms_ctrl::R](apb_ctrl_spi_mem_pms_ctrl::R) reader structure"]
impl crate::Readable for APB_CTRL_SPI_MEM_PMS_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_spi_mem_pms_ctrl::W](apb_ctrl_spi_mem_pms_ctrl::W) writer structure"]
impl crate::Writable for APB_CTRL_SPI_MEM_PMS_CTRL {}
#[doc = "APB_CTRL_SPI_MEM_PMS_CTRL"]
pub mod apb_ctrl_spi_mem_pms_ctrl;
#[doc = "APB_CTRL_SPI_MEM_REJECT_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_spi_mem_reject_addr](apb_ctrl_spi_mem_reject_addr) module"]
pub type APB_CTRL_SPI_MEM_REJECT_ADDR = crate::Reg<u32, _APB_CTRL_SPI_MEM_REJECT_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_SPI_MEM_REJECT_ADDR;
#[doc = "`read()` method returns [apb_ctrl_spi_mem_reject_addr::R](apb_ctrl_spi_mem_reject_addr::R) reader structure"]
impl crate::Readable for APB_CTRL_SPI_MEM_REJECT_ADDR {}
#[doc = "APB_CTRL_SPI_MEM_REJECT_ADDR"]
pub mod apb_ctrl_spi_mem_reject_addr;
#[doc = "APB_CTRL_SDIO_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_sdio_ctrl](apb_ctrl_sdio_ctrl) module"]
pub type APB_CTRL_SDIO_CTRL = crate::Reg<u32, _APB_CTRL_SDIO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_SDIO_CTRL;
#[doc = "`read()` method returns [apb_ctrl_sdio_ctrl::R](apb_ctrl_sdio_ctrl::R) reader structure"]
impl crate::Readable for APB_CTRL_SDIO_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_sdio_ctrl::W](apb_ctrl_sdio_ctrl::W) writer structure"]
impl crate::Writable for APB_CTRL_SDIO_CTRL {}
#[doc = "APB_CTRL_SDIO_CTRL"]
pub mod apb_ctrl_sdio_ctrl;
#[doc = "APB_CTRL_REDCY_SIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_redcy_sig0](apb_ctrl_redcy_sig0) module"]
pub type APB_CTRL_REDCY_SIG0 = crate::Reg<u32, _APB_CTRL_REDCY_SIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_REDCY_SIG0;
#[doc = "`read()` method returns [apb_ctrl_redcy_sig0::R](apb_ctrl_redcy_sig0::R) reader structure"]
impl crate::Readable for APB_CTRL_REDCY_SIG0 {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_redcy_sig0::W](apb_ctrl_redcy_sig0::W) writer structure"]
impl crate::Writable for APB_CTRL_REDCY_SIG0 {}
#[doc = "APB_CTRL_REDCY_SIG0"]
pub mod apb_ctrl_redcy_sig0;
#[doc = "APB_CTRL_REDCY_SIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_redcy_sig1](apb_ctrl_redcy_sig1) module"]
pub type APB_CTRL_REDCY_SIG1 = crate::Reg<u32, _APB_CTRL_REDCY_SIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_REDCY_SIG1;
#[doc = "`read()` method returns [apb_ctrl_redcy_sig1::R](apb_ctrl_redcy_sig1::R) reader structure"]
impl crate::Readable for APB_CTRL_REDCY_SIG1 {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_redcy_sig1::W](apb_ctrl_redcy_sig1::W) writer structure"]
impl crate::Writable for APB_CTRL_REDCY_SIG1 {}
#[doc = "APB_CTRL_REDCY_SIG1"]
pub mod apb_ctrl_redcy_sig1;
#[doc = "APB_CTRL_FRONT_END_MEM_PD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_front_end_mem_pd](apb_ctrl_front_end_mem_pd) module"]
pub type APB_CTRL_FRONT_END_MEM_PD = crate::Reg<u32, _APB_CTRL_FRONT_END_MEM_PD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_FRONT_END_MEM_PD;
#[doc = "`read()` method returns [apb_ctrl_front_end_mem_pd::R](apb_ctrl_front_end_mem_pd::R) reader structure"]
impl crate::Readable for APB_CTRL_FRONT_END_MEM_PD {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_front_end_mem_pd::W](apb_ctrl_front_end_mem_pd::W) writer structure"]
impl crate::Writable for APB_CTRL_FRONT_END_MEM_PD {}
#[doc = "APB_CTRL_FRONT_END_MEM_PD"]
pub mod apb_ctrl_front_end_mem_pd;
#[doc = "APB_CTRL_RETENTION_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_retention_ctrl](apb_ctrl_retention_ctrl) module"]
pub type APB_CTRL_RETENTION_CTRL = crate::Reg<u32, _APB_CTRL_RETENTION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_RETENTION_CTRL;
#[doc = "`read()` method returns [apb_ctrl_retention_ctrl::R](apb_ctrl_retention_ctrl::R) reader structure"]
impl crate::Readable for APB_CTRL_RETENTION_CTRL {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_retention_ctrl::W](apb_ctrl_retention_ctrl::W) writer structure"]
impl crate::Writable for APB_CTRL_RETENTION_CTRL {}
#[doc = "APB_CTRL_RETENTION_CTRL"]
pub mod apb_ctrl_retention_ctrl;
#[doc = "APB_CTRL_CLKGATE_FORCE_ON\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_clkgate_force_on](apb_ctrl_clkgate_force_on) module"]
pub type APB_CTRL_CLKGATE_FORCE_ON = crate::Reg<u32, _APB_CTRL_CLKGATE_FORCE_ON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_CLKGATE_FORCE_ON;
#[doc = "`read()` method returns [apb_ctrl_clkgate_force_on::R](apb_ctrl_clkgate_force_on::R) reader structure"]
impl crate::Readable for APB_CTRL_CLKGATE_FORCE_ON {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_clkgate_force_on::W](apb_ctrl_clkgate_force_on::W) writer structure"]
impl crate::Writable for APB_CTRL_CLKGATE_FORCE_ON {}
#[doc = "APB_CTRL_CLKGATE_FORCE_ON"]
pub mod apb_ctrl_clkgate_force_on;
#[doc = "APB_CTRL_MEM_POWER_DOWN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_mem_power_down](apb_ctrl_mem_power_down) module"]
pub type APB_CTRL_MEM_POWER_DOWN = crate::Reg<u32, _APB_CTRL_MEM_POWER_DOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_MEM_POWER_DOWN;
#[doc = "`read()` method returns [apb_ctrl_mem_power_down::R](apb_ctrl_mem_power_down::R) reader structure"]
impl crate::Readable for APB_CTRL_MEM_POWER_DOWN {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_mem_power_down::W](apb_ctrl_mem_power_down::W) writer structure"]
impl crate::Writable for APB_CTRL_MEM_POWER_DOWN {}
#[doc = "APB_CTRL_MEM_POWER_DOWN"]
pub mod apb_ctrl_mem_power_down;
#[doc = "APB_CTRL_MEM_POWER_UP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_mem_power_up](apb_ctrl_mem_power_up) module"]
pub type APB_CTRL_MEM_POWER_UP = crate::Reg<u32, _APB_CTRL_MEM_POWER_UP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_MEM_POWER_UP;
#[doc = "`read()` method returns [apb_ctrl_mem_power_up::R](apb_ctrl_mem_power_up::R) reader structure"]
impl crate::Readable for APB_CTRL_MEM_POWER_UP {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_mem_power_up::W](apb_ctrl_mem_power_up::W) writer structure"]
impl crate::Writable for APB_CTRL_MEM_POWER_UP {}
#[doc = "APB_CTRL_MEM_POWER_UP"]
pub mod apb_ctrl_mem_power_up;
#[doc = "APB_CTRL_RND_DATA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_rnd_data](apb_ctrl_rnd_data) module"]
pub type APB_CTRL_RND_DATA = crate::Reg<u32, _APB_CTRL_RND_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_RND_DATA;
#[doc = "`read()` method returns [apb_ctrl_rnd_data::R](apb_ctrl_rnd_data::R) reader structure"]
impl crate::Readable for APB_CTRL_RND_DATA {}
#[doc = "APB_CTRL_RND_DATA"]
pub mod apb_ctrl_rnd_data;
#[doc = "APB_CTRL_PERI_BACKUP_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_peri_backup_config](apb_ctrl_peri_backup_config) module"]
pub type APB_CTRL_PERI_BACKUP_CONFIG = crate::Reg<u32, _APB_CTRL_PERI_BACKUP_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_PERI_BACKUP_CONFIG;
#[doc = "`read()` method returns [apb_ctrl_peri_backup_config::R](apb_ctrl_peri_backup_config::R) reader structure"]
impl crate::Readable for APB_CTRL_PERI_BACKUP_CONFIG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_peri_backup_config::W](apb_ctrl_peri_backup_config::W) writer structure"]
impl crate::Writable for APB_CTRL_PERI_BACKUP_CONFIG {}
#[doc = "APB_CTRL_PERI_BACKUP_CONFIG"]
pub mod apb_ctrl_peri_backup_config;
#[doc = "APB_CTRL_PERI_BACKUP_APB_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_peri_backup_apb_addr](apb_ctrl_peri_backup_apb_addr) module"]
pub type APB_CTRL_PERI_BACKUP_APB_ADDR = crate::Reg<u32, _APB_CTRL_PERI_BACKUP_APB_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_PERI_BACKUP_APB_ADDR;
#[doc = "`read()` method returns [apb_ctrl_peri_backup_apb_addr::R](apb_ctrl_peri_backup_apb_addr::R) reader structure"]
impl crate::Readable for APB_CTRL_PERI_BACKUP_APB_ADDR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_peri_backup_apb_addr::W](apb_ctrl_peri_backup_apb_addr::W) writer structure"]
impl crate::Writable for APB_CTRL_PERI_BACKUP_APB_ADDR {}
#[doc = "APB_CTRL_PERI_BACKUP_APB_ADDR"]
pub mod apb_ctrl_peri_backup_apb_addr;
#[doc = "APB_CTRL_PERI_BACKUP_MEM_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_peri_backup_mem_addr](apb_ctrl_peri_backup_mem_addr) module"]
pub type APB_CTRL_PERI_BACKUP_MEM_ADDR = crate::Reg<u32, _APB_CTRL_PERI_BACKUP_MEM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_PERI_BACKUP_MEM_ADDR;
#[doc = "`read()` method returns [apb_ctrl_peri_backup_mem_addr::R](apb_ctrl_peri_backup_mem_addr::R) reader structure"]
impl crate::Readable for APB_CTRL_PERI_BACKUP_MEM_ADDR {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_peri_backup_mem_addr::W](apb_ctrl_peri_backup_mem_addr::W) writer structure"]
impl crate::Writable for APB_CTRL_PERI_BACKUP_MEM_ADDR {}
#[doc = "APB_CTRL_PERI_BACKUP_MEM_ADDR"]
pub mod apb_ctrl_peri_backup_mem_addr;
#[doc = "APB_CTRL_PERI_BACKUP_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_peri_backup_int_raw](apb_ctrl_peri_backup_int_raw) module"]
pub type APB_CTRL_PERI_BACKUP_INT_RAW = crate::Reg<u32, _APB_CTRL_PERI_BACKUP_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_PERI_BACKUP_INT_RAW;
#[doc = "`read()` method returns [apb_ctrl_peri_backup_int_raw::R](apb_ctrl_peri_backup_int_raw::R) reader structure"]
impl crate::Readable for APB_CTRL_PERI_BACKUP_INT_RAW {}
#[doc = "APB_CTRL_PERI_BACKUP_INT_RAW"]
pub mod apb_ctrl_peri_backup_int_raw;
#[doc = "APB_CTRL_PERI_BACKUP_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_peri_backup_int_st](apb_ctrl_peri_backup_int_st) module"]
pub type APB_CTRL_PERI_BACKUP_INT_ST = crate::Reg<u32, _APB_CTRL_PERI_BACKUP_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_PERI_BACKUP_INT_ST;
#[doc = "`read()` method returns [apb_ctrl_peri_backup_int_st::R](apb_ctrl_peri_backup_int_st::R) reader structure"]
impl crate::Readable for APB_CTRL_PERI_BACKUP_INT_ST {}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ST"]
pub mod apb_ctrl_peri_backup_int_st;
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_peri_backup_int_ena](apb_ctrl_peri_backup_int_ena) module"]
pub type APB_CTRL_PERI_BACKUP_INT_ENA = crate::Reg<u32, _APB_CTRL_PERI_BACKUP_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_PERI_BACKUP_INT_ENA;
#[doc = "`read()` method returns [apb_ctrl_peri_backup_int_ena::R](apb_ctrl_peri_backup_int_ena::R) reader structure"]
impl crate::Readable for APB_CTRL_PERI_BACKUP_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_peri_backup_int_ena::W](apb_ctrl_peri_backup_int_ena::W) writer structure"]
impl crate::Writable for APB_CTRL_PERI_BACKUP_INT_ENA {}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA"]
pub mod apb_ctrl_peri_backup_int_ena;
#[doc = "APB_CTRL_PERI_BACKUP_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_peri_backup_int_clr](apb_ctrl_peri_backup_int_clr) module"]
pub type APB_CTRL_PERI_BACKUP_INT_CLR = crate::Reg<u32, _APB_CTRL_PERI_BACKUP_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_PERI_BACKUP_INT_CLR;
#[doc = "`write(|w| ..)` method takes [apb_ctrl_peri_backup_int_clr::W](apb_ctrl_peri_backup_int_clr::W) writer structure"]
impl crate::Writable for APB_CTRL_PERI_BACKUP_INT_CLR {}
#[doc = "APB_CTRL_PERI_BACKUP_INT_CLR"]
pub mod apb_ctrl_peri_backup_int_clr;
#[doc = "APB_CTRL_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_date](apb_ctrl_date) module"]
pub type APB_CTRL_DATE = crate::Reg<u32, _APB_CTRL_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_DATE;
#[doc = "`read()` method returns [apb_ctrl_date::R](apb_ctrl_date::R) reader structure"]
impl crate::Readable for APB_CTRL_DATE {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_date::W](apb_ctrl_date::W) writer structure"]
impl crate::Writable for APB_CTRL_DATE {}
#[doc = "APB_CTRL_DATE"]
pub mod apb_ctrl_date;
