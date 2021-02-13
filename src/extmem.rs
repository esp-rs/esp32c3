#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTMEM_ICACHE_CTRL"]
    pub extmem_icache_ctrl: EXTMEM_ICACHE_CTRL,
    #[doc = "0x04 - EXTMEM_ICACHE_CTRL1"]
    pub extmem_icache_ctrl1: EXTMEM_ICACHE_CTRL1,
    #[doc = "0x08 - EXTMEM_ICACHE_TAG_POWER_CTRL"]
    pub extmem_icache_tag_power_ctrl: EXTMEM_ICACHE_TAG_POWER_CTRL,
    #[doc = "0x0c - EXTMEM_ICACHE_PRELOCK_CTRL"]
    pub extmem_icache_prelock_ctrl: EXTMEM_ICACHE_PRELOCK_CTRL,
    #[doc = "0x10 - EXTMEM_ICACHE_PRELOCK_SCT0_ADDR"]
    pub extmem_icache_prelock_sct0_addr: EXTMEM_ICACHE_PRELOCK_SCT0_ADDR,
    #[doc = "0x14 - EXTMEM_ICACHE_PRELOCK_SCT1_ADDR"]
    pub extmem_icache_prelock_sct1_addr: EXTMEM_ICACHE_PRELOCK_SCT1_ADDR,
    #[doc = "0x18 - EXTMEM_ICACHE_PRELOCK_SCT_SIZE"]
    pub extmem_icache_prelock_sct_size: EXTMEM_ICACHE_PRELOCK_SCT_SIZE,
    #[doc = "0x1c - EXTMEM_ICACHE_LOCK_CTRL"]
    pub extmem_icache_lock_ctrl: EXTMEM_ICACHE_LOCK_CTRL,
    #[doc = "0x20 - EXTMEM_ICACHE_LOCK_ADDR"]
    pub extmem_icache_lock_addr: EXTMEM_ICACHE_LOCK_ADDR,
    #[doc = "0x24 - EXTMEM_ICACHE_LOCK_SIZE"]
    pub extmem_icache_lock_size: EXTMEM_ICACHE_LOCK_SIZE,
    #[doc = "0x28 - EXTMEM_ICACHE_SYNC_CTRL"]
    pub extmem_icache_sync_ctrl: EXTMEM_ICACHE_SYNC_CTRL,
    #[doc = "0x2c - EXTMEM_ICACHE_SYNC_ADDR"]
    pub extmem_icache_sync_addr: EXTMEM_ICACHE_SYNC_ADDR,
    #[doc = "0x30 - EXTMEM_ICACHE_SYNC_SIZE"]
    pub extmem_icache_sync_size: EXTMEM_ICACHE_SYNC_SIZE,
    #[doc = "0x34 - EXTMEM_ICACHE_PRELOAD_CTRL"]
    pub extmem_icache_preload_ctrl: EXTMEM_ICACHE_PRELOAD_CTRL,
    #[doc = "0x38 - EXTMEM_ICACHE_PRELOAD_ADDR"]
    pub extmem_icache_preload_addr: EXTMEM_ICACHE_PRELOAD_ADDR,
    #[doc = "0x3c - EXTMEM_ICACHE_PRELOAD_SIZE"]
    pub extmem_icache_preload_size: EXTMEM_ICACHE_PRELOAD_SIZE,
    #[doc = "0x40 - EXTMEM_ICACHE_AUTOLOAD_CTRL"]
    pub extmem_icache_autoload_ctrl: EXTMEM_ICACHE_AUTOLOAD_CTRL,
    #[doc = "0x44 - EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR"]
    pub extmem_icache_autoload_sct0_addr: EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR,
    #[doc = "0x48 - EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE"]
    pub extmem_icache_autoload_sct0_size: EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE,
    #[doc = "0x4c - EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR"]
    pub extmem_icache_autoload_sct1_addr: EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR,
    #[doc = "0x50 - EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE"]
    pub extmem_icache_autoload_sct1_size: EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE,
    #[doc = "0x54 - EXTMEM_IBUS_TO_FLASH_START_VADDR"]
    pub extmem_ibus_to_flash_start_vaddr: EXTMEM_IBUS_TO_FLASH_START_VADDR,
    #[doc = "0x58 - EXTMEM_IBUS_TO_FLASH_END_VADDR"]
    pub extmem_ibus_to_flash_end_vaddr: EXTMEM_IBUS_TO_FLASH_END_VADDR,
    #[doc = "0x5c - EXTMEM_DBUS_TO_FLASH_START_VADDR"]
    pub extmem_dbus_to_flash_start_vaddr: EXTMEM_DBUS_TO_FLASH_START_VADDR,
    #[doc = "0x60 - EXTMEM_DBUS_TO_FLASH_END_VADDR"]
    pub extmem_dbus_to_flash_end_vaddr: EXTMEM_DBUS_TO_FLASH_END_VADDR,
    #[doc = "0x64 - EXTMEM_CACHE_ACS_CNT_CLR"]
    pub extmem_cache_acs_cnt_clr: EXTMEM_CACHE_ACS_CNT_CLR,
    #[doc = "0x68 - EXTMEM_IBUS_ACS_MISS_CNT"]
    pub extmem_ibus_acs_miss_cnt: EXTMEM_IBUS_ACS_MISS_CNT,
    #[doc = "0x6c - EXTMEM_IBUS_ACS_CNT"]
    pub extmem_ibus_acs_cnt: EXTMEM_IBUS_ACS_CNT,
    #[doc = "0x70 - EXTMEM_DBUS_ACS_FLASH_MISS_CNT"]
    pub extmem_dbus_acs_flash_miss_cnt: EXTMEM_DBUS_ACS_FLASH_MISS_CNT,
    #[doc = "0x74 - EXTMEM_DBUS_ACS_CNT"]
    pub extmem_dbus_acs_cnt: EXTMEM_DBUS_ACS_CNT,
    #[doc = "0x78 - EXTMEM_CACHE_ILG_INT_ENA"]
    pub extmem_cache_ilg_int_ena: EXTMEM_CACHE_ILG_INT_ENA,
    #[doc = "0x7c - EXTMEM_CACHE_ILG_INT_CLR"]
    pub extmem_cache_ilg_int_clr: EXTMEM_CACHE_ILG_INT_CLR,
    #[doc = "0x80 - EXTMEM_CACHE_ILG_INT_ST"]
    pub extmem_cache_ilg_int_st: EXTMEM_CACHE_ILG_INT_ST,
    #[doc = "0x84 - EXTMEM_CORE0_ACS_CACHE_INT_ENA"]
    pub extmem_core0_acs_cache_int_ena: EXTMEM_CORE0_ACS_CACHE_INT_ENA,
    #[doc = "0x88 - EXTMEM_CORE0_ACS_CACHE_INT_CLR"]
    pub extmem_core0_acs_cache_int_clr: EXTMEM_CORE0_ACS_CACHE_INT_CLR,
    #[doc = "0x8c - EXTMEM_CORE0_ACS_CACHE_INT_ST"]
    pub extmem_core0_acs_cache_int_st: EXTMEM_CORE0_ACS_CACHE_INT_ST,
    #[doc = "0x90 - EXTMEM_CORE0_DBUS_REJECT_ST"]
    pub extmem_core0_dbus_reject_st: EXTMEM_CORE0_DBUS_REJECT_ST,
    #[doc = "0x94 - EXTMEM_CORE0_DBUS_REJECT_VADDR"]
    pub extmem_core0_dbus_reject_vaddr: EXTMEM_CORE0_DBUS_REJECT_VADDR,
    #[doc = "0x98 - EXTMEM_CORE0_IBUS_REJECT_ST"]
    pub extmem_core0_ibus_reject_st: EXTMEM_CORE0_IBUS_REJECT_ST,
    #[doc = "0x9c - EXTMEM_CORE0_IBUS_REJECT_VADDR"]
    pub extmem_core0_ibus_reject_vaddr: EXTMEM_CORE0_IBUS_REJECT_VADDR,
    #[doc = "0xa0 - EXTMEM_CACHE_MMU_FAULT_CONTENT"]
    pub extmem_cache_mmu_fault_content: EXTMEM_CACHE_MMU_FAULT_CONTENT,
    #[doc = "0xa4 - EXTMEM_CACHE_MMU_FAULT_VADDR"]
    pub extmem_cache_mmu_fault_vaddr: EXTMEM_CACHE_MMU_FAULT_VADDR,
    #[doc = "0xa8 - EXTMEM_CACHE_WRAP_AROUND_CTRL"]
    pub extmem_cache_wrap_around_ctrl: EXTMEM_CACHE_WRAP_AROUND_CTRL,
    #[doc = "0xac - EXTMEM_CACHE_MMU_POWER_CTRL"]
    pub extmem_cache_mmu_power_ctrl: EXTMEM_CACHE_MMU_POWER_CTRL,
    #[doc = "0xb0 - EXTMEM_CACHE_STATE"]
    pub extmem_cache_state: EXTMEM_CACHE_STATE,
    #[doc = "0xb4 - EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE"]
    pub extmem_cache_encrypt_decrypt_record_disable: EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE,
    #[doc = "0xb8 - EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON"]
    pub extmem_cache_encrypt_decrypt_clk_force_on: EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON,
    #[doc = "0xbc - EXTMEM_CACHE_PRELOAD_INT_CTRL"]
    pub extmem_cache_preload_int_ctrl: EXTMEM_CACHE_PRELOAD_INT_CTRL,
    #[doc = "0xc0 - EXTMEM_CACHE_SYNC_INT_CTRL"]
    pub extmem_cache_sync_int_ctrl: EXTMEM_CACHE_SYNC_INT_CTRL,
    #[doc = "0xc4 - EXTMEM_CACHE_MMU_OWNER"]
    pub extmem_cache_mmu_owner: EXTMEM_CACHE_MMU_OWNER,
    #[doc = "0xc8 - EXTMEM_CACHE_CONF_MISC"]
    pub extmem_cache_conf_misc: EXTMEM_CACHE_CONF_MISC,
    #[doc = "0xcc - EXTMEM_ICACHE_FREEZE"]
    pub extmem_icache_freeze: EXTMEM_ICACHE_FREEZE,
    #[doc = "0xd0 - EXTMEM_ICACHE_ATOMIC_OPERATE_ENA"]
    pub extmem_icache_atomic_operate_ena: EXTMEM_ICACHE_ATOMIC_OPERATE_ENA,
    #[doc = "0xd4 - EXTMEM_CACHE_REQUEST"]
    pub extmem_cache_request: EXTMEM_CACHE_REQUEST,
    #[doc = "0xd8 - EXTMEM_IBUS_PMS_TBL_LOCK"]
    pub extmem_ibus_pms_tbl_lock: EXTMEM_IBUS_PMS_TBL_LOCK,
    #[doc = "0xdc - EXTMEM_IBUS_PMS_TBL_BOUNDARY0"]
    pub extmem_ibus_pms_tbl_boundary0: EXTMEM_IBUS_PMS_TBL_BOUNDARY0,
    #[doc = "0xe0 - EXTMEM_IBUS_PMS_TBL_BOUNDARY1"]
    pub extmem_ibus_pms_tbl_boundary1: EXTMEM_IBUS_PMS_TBL_BOUNDARY1,
    #[doc = "0xe4 - EXTMEM_IBUS_PMS_TBL_BOUNDARY2"]
    pub extmem_ibus_pms_tbl_boundary2: EXTMEM_IBUS_PMS_TBL_BOUNDARY2,
    #[doc = "0xe8 - EXTMEM_IBUS_PMS_TBL_ATTR"]
    pub extmem_ibus_pms_tbl_attr: EXTMEM_IBUS_PMS_TBL_ATTR,
    #[doc = "0xec - EXTMEM_DBUS_PMS_TBL_LOCK"]
    pub extmem_dbus_pms_tbl_lock: EXTMEM_DBUS_PMS_TBL_LOCK,
    #[doc = "0xf0 - EXTMEM_DBUS_PMS_TBL_BOUNDARY0"]
    pub extmem_dbus_pms_tbl_boundary0: EXTMEM_DBUS_PMS_TBL_BOUNDARY0,
    #[doc = "0xf4 - EXTMEM_DBUS_PMS_TBL_BOUNDARY1"]
    pub extmem_dbus_pms_tbl_boundary1: EXTMEM_DBUS_PMS_TBL_BOUNDARY1,
    #[doc = "0xf8 - EXTMEM_DBUS_PMS_TBL_BOUNDARY2"]
    pub extmem_dbus_pms_tbl_boundary2: EXTMEM_DBUS_PMS_TBL_BOUNDARY2,
    #[doc = "0xfc - EXTMEM_DBUS_PMS_TBL_ATTR"]
    pub extmem_dbus_pms_tbl_attr: EXTMEM_DBUS_PMS_TBL_ATTR,
    #[doc = "0x100 - EXTMEM_CLOCK_GATE"]
    pub extmem_clock_gate: EXTMEM_CLOCK_GATE,
    _reserved65: [u8; 760usize],
    #[doc = "0x3fc - EXTMEM_DATE"]
    pub extmem_date: EXTMEM_DATE,
}
#[doc = "EXTMEM_ICACHE_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_ctrl](extmem_icache_ctrl) module"]
pub type EXTMEM_ICACHE_CTRL = crate::Reg<u32, _EXTMEM_ICACHE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_CTRL;
#[doc = "`read()` method returns [extmem_icache_ctrl::R](extmem_icache_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_ctrl::W](extmem_icache_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_CTRL {}
#[doc = "EXTMEM_ICACHE_CTRL"]
pub mod extmem_icache_ctrl;
#[doc = "EXTMEM_ICACHE_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_ctrl1](extmem_icache_ctrl1) module"]
pub type EXTMEM_ICACHE_CTRL1 = crate::Reg<u32, _EXTMEM_ICACHE_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_CTRL1;
#[doc = "`read()` method returns [extmem_icache_ctrl1::R](extmem_icache_ctrl1::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_ctrl1::W](extmem_icache_ctrl1::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_CTRL1 {}
#[doc = "EXTMEM_ICACHE_CTRL1"]
pub mod extmem_icache_ctrl1;
#[doc = "EXTMEM_ICACHE_TAG_POWER_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_tag_power_ctrl](extmem_icache_tag_power_ctrl) module"]
pub type EXTMEM_ICACHE_TAG_POWER_CTRL = crate::Reg<u32, _EXTMEM_ICACHE_TAG_POWER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_TAG_POWER_CTRL;
#[doc = "`read()` method returns [extmem_icache_tag_power_ctrl::R](extmem_icache_tag_power_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_TAG_POWER_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_tag_power_ctrl::W](extmem_icache_tag_power_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_TAG_POWER_CTRL {}
#[doc = "EXTMEM_ICACHE_TAG_POWER_CTRL"]
pub mod extmem_icache_tag_power_ctrl;
#[doc = "EXTMEM_ICACHE_PRELOCK_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_prelock_ctrl](extmem_icache_prelock_ctrl) module"]
pub type EXTMEM_ICACHE_PRELOCK_CTRL = crate::Reg<u32, _EXTMEM_ICACHE_PRELOCK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_PRELOCK_CTRL;
#[doc = "`read()` method returns [extmem_icache_prelock_ctrl::R](extmem_icache_prelock_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_PRELOCK_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_prelock_ctrl::W](extmem_icache_prelock_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_PRELOCK_CTRL {}
#[doc = "EXTMEM_ICACHE_PRELOCK_CTRL"]
pub mod extmem_icache_prelock_ctrl;
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT0_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_prelock_sct0_addr](extmem_icache_prelock_sct0_addr) module"]
pub type EXTMEM_ICACHE_PRELOCK_SCT0_ADDR = crate::Reg<u32, _EXTMEM_ICACHE_PRELOCK_SCT0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_PRELOCK_SCT0_ADDR;
#[doc = "`read()` method returns [extmem_icache_prelock_sct0_addr::R](extmem_icache_prelock_sct0_addr::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_PRELOCK_SCT0_ADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_prelock_sct0_addr::W](extmem_icache_prelock_sct0_addr::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_PRELOCK_SCT0_ADDR {}
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT0_ADDR"]
pub mod extmem_icache_prelock_sct0_addr;
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT1_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_prelock_sct1_addr](extmem_icache_prelock_sct1_addr) module"]
pub type EXTMEM_ICACHE_PRELOCK_SCT1_ADDR = crate::Reg<u32, _EXTMEM_ICACHE_PRELOCK_SCT1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_PRELOCK_SCT1_ADDR;
#[doc = "`read()` method returns [extmem_icache_prelock_sct1_addr::R](extmem_icache_prelock_sct1_addr::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_PRELOCK_SCT1_ADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_prelock_sct1_addr::W](extmem_icache_prelock_sct1_addr::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_PRELOCK_SCT1_ADDR {}
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT1_ADDR"]
pub mod extmem_icache_prelock_sct1_addr;
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_prelock_sct_size](extmem_icache_prelock_sct_size) module"]
pub type EXTMEM_ICACHE_PRELOCK_SCT_SIZE = crate::Reg<u32, _EXTMEM_ICACHE_PRELOCK_SCT_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_PRELOCK_SCT_SIZE;
#[doc = "`read()` method returns [extmem_icache_prelock_sct_size::R](extmem_icache_prelock_sct_size::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_PRELOCK_SCT_SIZE {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_prelock_sct_size::W](extmem_icache_prelock_sct_size::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_PRELOCK_SCT_SIZE {}
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT_SIZE"]
pub mod extmem_icache_prelock_sct_size;
#[doc = "EXTMEM_ICACHE_LOCK_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_lock_ctrl](extmem_icache_lock_ctrl) module"]
pub type EXTMEM_ICACHE_LOCK_CTRL = crate::Reg<u32, _EXTMEM_ICACHE_LOCK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_LOCK_CTRL;
#[doc = "`read()` method returns [extmem_icache_lock_ctrl::R](extmem_icache_lock_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_LOCK_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_lock_ctrl::W](extmem_icache_lock_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_LOCK_CTRL {}
#[doc = "EXTMEM_ICACHE_LOCK_CTRL"]
pub mod extmem_icache_lock_ctrl;
#[doc = "EXTMEM_ICACHE_LOCK_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_lock_addr](extmem_icache_lock_addr) module"]
pub type EXTMEM_ICACHE_LOCK_ADDR = crate::Reg<u32, _EXTMEM_ICACHE_LOCK_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_LOCK_ADDR;
#[doc = "`read()` method returns [extmem_icache_lock_addr::R](extmem_icache_lock_addr::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_LOCK_ADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_lock_addr::W](extmem_icache_lock_addr::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_LOCK_ADDR {}
#[doc = "EXTMEM_ICACHE_LOCK_ADDR"]
pub mod extmem_icache_lock_addr;
#[doc = "EXTMEM_ICACHE_LOCK_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_lock_size](extmem_icache_lock_size) module"]
pub type EXTMEM_ICACHE_LOCK_SIZE = crate::Reg<u32, _EXTMEM_ICACHE_LOCK_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_LOCK_SIZE;
#[doc = "`read()` method returns [extmem_icache_lock_size::R](extmem_icache_lock_size::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_LOCK_SIZE {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_lock_size::W](extmem_icache_lock_size::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_LOCK_SIZE {}
#[doc = "EXTMEM_ICACHE_LOCK_SIZE"]
pub mod extmem_icache_lock_size;
#[doc = "EXTMEM_ICACHE_SYNC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_sync_ctrl](extmem_icache_sync_ctrl) module"]
pub type EXTMEM_ICACHE_SYNC_CTRL = crate::Reg<u32, _EXTMEM_ICACHE_SYNC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_SYNC_CTRL;
#[doc = "`read()` method returns [extmem_icache_sync_ctrl::R](extmem_icache_sync_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_SYNC_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_sync_ctrl::W](extmem_icache_sync_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_SYNC_CTRL {}
#[doc = "EXTMEM_ICACHE_SYNC_CTRL"]
pub mod extmem_icache_sync_ctrl;
#[doc = "EXTMEM_ICACHE_SYNC_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_sync_addr](extmem_icache_sync_addr) module"]
pub type EXTMEM_ICACHE_SYNC_ADDR = crate::Reg<u32, _EXTMEM_ICACHE_SYNC_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_SYNC_ADDR;
#[doc = "`read()` method returns [extmem_icache_sync_addr::R](extmem_icache_sync_addr::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_SYNC_ADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_sync_addr::W](extmem_icache_sync_addr::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_SYNC_ADDR {}
#[doc = "EXTMEM_ICACHE_SYNC_ADDR"]
pub mod extmem_icache_sync_addr;
#[doc = "EXTMEM_ICACHE_SYNC_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_sync_size](extmem_icache_sync_size) module"]
pub type EXTMEM_ICACHE_SYNC_SIZE = crate::Reg<u32, _EXTMEM_ICACHE_SYNC_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_SYNC_SIZE;
#[doc = "`read()` method returns [extmem_icache_sync_size::R](extmem_icache_sync_size::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_SYNC_SIZE {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_sync_size::W](extmem_icache_sync_size::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_SYNC_SIZE {}
#[doc = "EXTMEM_ICACHE_SYNC_SIZE"]
pub mod extmem_icache_sync_size;
#[doc = "EXTMEM_ICACHE_PRELOAD_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_preload_ctrl](extmem_icache_preload_ctrl) module"]
pub type EXTMEM_ICACHE_PRELOAD_CTRL = crate::Reg<u32, _EXTMEM_ICACHE_PRELOAD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_PRELOAD_CTRL;
#[doc = "`read()` method returns [extmem_icache_preload_ctrl::R](extmem_icache_preload_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_PRELOAD_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_preload_ctrl::W](extmem_icache_preload_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_PRELOAD_CTRL {}
#[doc = "EXTMEM_ICACHE_PRELOAD_CTRL"]
pub mod extmem_icache_preload_ctrl;
#[doc = "EXTMEM_ICACHE_PRELOAD_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_preload_addr](extmem_icache_preload_addr) module"]
pub type EXTMEM_ICACHE_PRELOAD_ADDR = crate::Reg<u32, _EXTMEM_ICACHE_PRELOAD_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_PRELOAD_ADDR;
#[doc = "`read()` method returns [extmem_icache_preload_addr::R](extmem_icache_preload_addr::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_PRELOAD_ADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_preload_addr::W](extmem_icache_preload_addr::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_PRELOAD_ADDR {}
#[doc = "EXTMEM_ICACHE_PRELOAD_ADDR"]
pub mod extmem_icache_preload_addr;
#[doc = "EXTMEM_ICACHE_PRELOAD_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_preload_size](extmem_icache_preload_size) module"]
pub type EXTMEM_ICACHE_PRELOAD_SIZE = crate::Reg<u32, _EXTMEM_ICACHE_PRELOAD_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_PRELOAD_SIZE;
#[doc = "`read()` method returns [extmem_icache_preload_size::R](extmem_icache_preload_size::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_PRELOAD_SIZE {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_preload_size::W](extmem_icache_preload_size::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_PRELOAD_SIZE {}
#[doc = "EXTMEM_ICACHE_PRELOAD_SIZE"]
pub mod extmem_icache_preload_size;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_autoload_ctrl](extmem_icache_autoload_ctrl) module"]
pub type EXTMEM_ICACHE_AUTOLOAD_CTRL = crate::Reg<u32, _EXTMEM_ICACHE_AUTOLOAD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_AUTOLOAD_CTRL;
#[doc = "`read()` method returns [extmem_icache_autoload_ctrl::R](extmem_icache_autoload_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_AUTOLOAD_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_autoload_ctrl::W](extmem_icache_autoload_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_AUTOLOAD_CTRL {}
#[doc = "EXTMEM_ICACHE_AUTOLOAD_CTRL"]
pub mod extmem_icache_autoload_ctrl;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_autoload_sct0_addr](extmem_icache_autoload_sct0_addr) module"]
pub type EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR = crate::Reg<u32, _EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR;
#[doc = "`read()` method returns [extmem_icache_autoload_sct0_addr::R](extmem_icache_autoload_sct0_addr::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_autoload_sct0_addr::W](extmem_icache_autoload_sct0_addr::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR {}
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR"]
pub mod extmem_icache_autoload_sct0_addr;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_autoload_sct0_size](extmem_icache_autoload_sct0_size) module"]
pub type EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE = crate::Reg<u32, _EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE;
#[doc = "`read()` method returns [extmem_icache_autoload_sct0_size::R](extmem_icache_autoload_sct0_size::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_autoload_sct0_size::W](extmem_icache_autoload_sct0_size::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE {}
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE"]
pub mod extmem_icache_autoload_sct0_size;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_autoload_sct1_addr](extmem_icache_autoload_sct1_addr) module"]
pub type EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR = crate::Reg<u32, _EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR;
#[doc = "`read()` method returns [extmem_icache_autoload_sct1_addr::R](extmem_icache_autoload_sct1_addr::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_autoload_sct1_addr::W](extmem_icache_autoload_sct1_addr::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR {}
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR"]
pub mod extmem_icache_autoload_sct1_addr;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_autoload_sct1_size](extmem_icache_autoload_sct1_size) module"]
pub type EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE = crate::Reg<u32, _EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE;
#[doc = "`read()` method returns [extmem_icache_autoload_sct1_size::R](extmem_icache_autoload_sct1_size::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_autoload_sct1_size::W](extmem_icache_autoload_sct1_size::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE {}
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE"]
pub mod extmem_icache_autoload_sct1_size;
#[doc = "EXTMEM_IBUS_TO_FLASH_START_VADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_to_flash_start_vaddr](extmem_ibus_to_flash_start_vaddr) module"]
pub type EXTMEM_IBUS_TO_FLASH_START_VADDR = crate::Reg<u32, _EXTMEM_IBUS_TO_FLASH_START_VADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_TO_FLASH_START_VADDR;
#[doc = "`read()` method returns [extmem_ibus_to_flash_start_vaddr::R](extmem_ibus_to_flash_start_vaddr::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_TO_FLASH_START_VADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_ibus_to_flash_start_vaddr::W](extmem_ibus_to_flash_start_vaddr::W) writer structure"]
impl crate::Writable for EXTMEM_IBUS_TO_FLASH_START_VADDR {}
#[doc = "EXTMEM_IBUS_TO_FLASH_START_VADDR"]
pub mod extmem_ibus_to_flash_start_vaddr;
#[doc = "EXTMEM_IBUS_TO_FLASH_END_VADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_to_flash_end_vaddr](extmem_ibus_to_flash_end_vaddr) module"]
pub type EXTMEM_IBUS_TO_FLASH_END_VADDR = crate::Reg<u32, _EXTMEM_IBUS_TO_FLASH_END_VADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_TO_FLASH_END_VADDR;
#[doc = "`read()` method returns [extmem_ibus_to_flash_end_vaddr::R](extmem_ibus_to_flash_end_vaddr::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_TO_FLASH_END_VADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_ibus_to_flash_end_vaddr::W](extmem_ibus_to_flash_end_vaddr::W) writer structure"]
impl crate::Writable for EXTMEM_IBUS_TO_FLASH_END_VADDR {}
#[doc = "EXTMEM_IBUS_TO_FLASH_END_VADDR"]
pub mod extmem_ibus_to_flash_end_vaddr;
#[doc = "EXTMEM_DBUS_TO_FLASH_START_VADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_to_flash_start_vaddr](extmem_dbus_to_flash_start_vaddr) module"]
pub type EXTMEM_DBUS_TO_FLASH_START_VADDR = crate::Reg<u32, _EXTMEM_DBUS_TO_FLASH_START_VADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_TO_FLASH_START_VADDR;
#[doc = "`read()` method returns [extmem_dbus_to_flash_start_vaddr::R](extmem_dbus_to_flash_start_vaddr::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_TO_FLASH_START_VADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_dbus_to_flash_start_vaddr::W](extmem_dbus_to_flash_start_vaddr::W) writer structure"]
impl crate::Writable for EXTMEM_DBUS_TO_FLASH_START_VADDR {}
#[doc = "EXTMEM_DBUS_TO_FLASH_START_VADDR"]
pub mod extmem_dbus_to_flash_start_vaddr;
#[doc = "EXTMEM_DBUS_TO_FLASH_END_VADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_to_flash_end_vaddr](extmem_dbus_to_flash_end_vaddr) module"]
pub type EXTMEM_DBUS_TO_FLASH_END_VADDR = crate::Reg<u32, _EXTMEM_DBUS_TO_FLASH_END_VADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_TO_FLASH_END_VADDR;
#[doc = "`read()` method returns [extmem_dbus_to_flash_end_vaddr::R](extmem_dbus_to_flash_end_vaddr::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_TO_FLASH_END_VADDR {}
#[doc = "`write(|w| ..)` method takes [extmem_dbus_to_flash_end_vaddr::W](extmem_dbus_to_flash_end_vaddr::W) writer structure"]
impl crate::Writable for EXTMEM_DBUS_TO_FLASH_END_VADDR {}
#[doc = "EXTMEM_DBUS_TO_FLASH_END_VADDR"]
pub mod extmem_dbus_to_flash_end_vaddr;
#[doc = "EXTMEM_CACHE_ACS_CNT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_acs_cnt_clr](extmem_cache_acs_cnt_clr) module"]
pub type EXTMEM_CACHE_ACS_CNT_CLR = crate::Reg<u32, _EXTMEM_CACHE_ACS_CNT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_ACS_CNT_CLR;
#[doc = "`write(|w| ..)` method takes [extmem_cache_acs_cnt_clr::W](extmem_cache_acs_cnt_clr::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_ACS_CNT_CLR {}
#[doc = "EXTMEM_CACHE_ACS_CNT_CLR"]
pub mod extmem_cache_acs_cnt_clr;
#[doc = "EXTMEM_IBUS_ACS_MISS_CNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_acs_miss_cnt](extmem_ibus_acs_miss_cnt) module"]
pub type EXTMEM_IBUS_ACS_MISS_CNT = crate::Reg<u32, _EXTMEM_IBUS_ACS_MISS_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_ACS_MISS_CNT;
#[doc = "`read()` method returns [extmem_ibus_acs_miss_cnt::R](extmem_ibus_acs_miss_cnt::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_ACS_MISS_CNT {}
#[doc = "EXTMEM_IBUS_ACS_MISS_CNT"]
pub mod extmem_ibus_acs_miss_cnt;
#[doc = "EXTMEM_IBUS_ACS_CNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_acs_cnt](extmem_ibus_acs_cnt) module"]
pub type EXTMEM_IBUS_ACS_CNT = crate::Reg<u32, _EXTMEM_IBUS_ACS_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_ACS_CNT;
#[doc = "`read()` method returns [extmem_ibus_acs_cnt::R](extmem_ibus_acs_cnt::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_ACS_CNT {}
#[doc = "EXTMEM_IBUS_ACS_CNT"]
pub mod extmem_ibus_acs_cnt;
#[doc = "EXTMEM_DBUS_ACS_FLASH_MISS_CNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_acs_flash_miss_cnt](extmem_dbus_acs_flash_miss_cnt) module"]
pub type EXTMEM_DBUS_ACS_FLASH_MISS_CNT = crate::Reg<u32, _EXTMEM_DBUS_ACS_FLASH_MISS_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_ACS_FLASH_MISS_CNT;
#[doc = "`read()` method returns [extmem_dbus_acs_flash_miss_cnt::R](extmem_dbus_acs_flash_miss_cnt::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_ACS_FLASH_MISS_CNT {}
#[doc = "EXTMEM_DBUS_ACS_FLASH_MISS_CNT"]
pub mod extmem_dbus_acs_flash_miss_cnt;
#[doc = "EXTMEM_DBUS_ACS_CNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_acs_cnt](extmem_dbus_acs_cnt) module"]
pub type EXTMEM_DBUS_ACS_CNT = crate::Reg<u32, _EXTMEM_DBUS_ACS_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_ACS_CNT;
#[doc = "`read()` method returns [extmem_dbus_acs_cnt::R](extmem_dbus_acs_cnt::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_ACS_CNT {}
#[doc = "EXTMEM_DBUS_ACS_CNT"]
pub mod extmem_dbus_acs_cnt;
#[doc = "EXTMEM_CACHE_ILG_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_ilg_int_ena](extmem_cache_ilg_int_ena) module"]
pub type EXTMEM_CACHE_ILG_INT_ENA = crate::Reg<u32, _EXTMEM_CACHE_ILG_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_ILG_INT_ENA;
#[doc = "`read()` method returns [extmem_cache_ilg_int_ena::R](extmem_cache_ilg_int_ena::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_ILG_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_ilg_int_ena::W](extmem_cache_ilg_int_ena::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_ILG_INT_ENA {}
#[doc = "EXTMEM_CACHE_ILG_INT_ENA"]
pub mod extmem_cache_ilg_int_ena;
#[doc = "EXTMEM_CACHE_ILG_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_ilg_int_clr](extmem_cache_ilg_int_clr) module"]
pub type EXTMEM_CACHE_ILG_INT_CLR = crate::Reg<u32, _EXTMEM_CACHE_ILG_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_ILG_INT_CLR;
#[doc = "`write(|w| ..)` method takes [extmem_cache_ilg_int_clr::W](extmem_cache_ilg_int_clr::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_ILG_INT_CLR {}
#[doc = "EXTMEM_CACHE_ILG_INT_CLR"]
pub mod extmem_cache_ilg_int_clr;
#[doc = "EXTMEM_CACHE_ILG_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_ilg_int_st](extmem_cache_ilg_int_st) module"]
pub type EXTMEM_CACHE_ILG_INT_ST = crate::Reg<u32, _EXTMEM_CACHE_ILG_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_ILG_INT_ST;
#[doc = "`read()` method returns [extmem_cache_ilg_int_st::R](extmem_cache_ilg_int_st::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_ILG_INT_ST {}
#[doc = "EXTMEM_CACHE_ILG_INT_ST"]
pub mod extmem_cache_ilg_int_st;
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_core0_acs_cache_int_ena](extmem_core0_acs_cache_int_ena) module"]
pub type EXTMEM_CORE0_ACS_CACHE_INT_ENA = crate::Reg<u32, _EXTMEM_CORE0_ACS_CACHE_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CORE0_ACS_CACHE_INT_ENA;
#[doc = "`read()` method returns [extmem_core0_acs_cache_int_ena::R](extmem_core0_acs_cache_int_ena::R) reader structure"]
impl crate::Readable for EXTMEM_CORE0_ACS_CACHE_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [extmem_core0_acs_cache_int_ena::W](extmem_core0_acs_cache_int_ena::W) writer structure"]
impl crate::Writable for EXTMEM_CORE0_ACS_CACHE_INT_ENA {}
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_ENA"]
pub mod extmem_core0_acs_cache_int_ena;
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_core0_acs_cache_int_clr](extmem_core0_acs_cache_int_clr) module"]
pub type EXTMEM_CORE0_ACS_CACHE_INT_CLR = crate::Reg<u32, _EXTMEM_CORE0_ACS_CACHE_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CORE0_ACS_CACHE_INT_CLR;
#[doc = "`write(|w| ..)` method takes [extmem_core0_acs_cache_int_clr::W](extmem_core0_acs_cache_int_clr::W) writer structure"]
impl crate::Writable for EXTMEM_CORE0_ACS_CACHE_INT_CLR {}
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_CLR"]
pub mod extmem_core0_acs_cache_int_clr;
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_core0_acs_cache_int_st](extmem_core0_acs_cache_int_st) module"]
pub type EXTMEM_CORE0_ACS_CACHE_INT_ST = crate::Reg<u32, _EXTMEM_CORE0_ACS_CACHE_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CORE0_ACS_CACHE_INT_ST;
#[doc = "`read()` method returns [extmem_core0_acs_cache_int_st::R](extmem_core0_acs_cache_int_st::R) reader structure"]
impl crate::Readable for EXTMEM_CORE0_ACS_CACHE_INT_ST {}
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_ST"]
pub mod extmem_core0_acs_cache_int_st;
#[doc = "EXTMEM_CORE0_DBUS_REJECT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_core0_dbus_reject_st](extmem_core0_dbus_reject_st) module"]
pub type EXTMEM_CORE0_DBUS_REJECT_ST = crate::Reg<u32, _EXTMEM_CORE0_DBUS_REJECT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CORE0_DBUS_REJECT_ST;
#[doc = "`read()` method returns [extmem_core0_dbus_reject_st::R](extmem_core0_dbus_reject_st::R) reader structure"]
impl crate::Readable for EXTMEM_CORE0_DBUS_REJECT_ST {}
#[doc = "EXTMEM_CORE0_DBUS_REJECT_ST"]
pub mod extmem_core0_dbus_reject_st;
#[doc = "EXTMEM_CORE0_DBUS_REJECT_VADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_core0_dbus_reject_vaddr](extmem_core0_dbus_reject_vaddr) module"]
pub type EXTMEM_CORE0_DBUS_REJECT_VADDR = crate::Reg<u32, _EXTMEM_CORE0_DBUS_REJECT_VADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CORE0_DBUS_REJECT_VADDR;
#[doc = "`read()` method returns [extmem_core0_dbus_reject_vaddr::R](extmem_core0_dbus_reject_vaddr::R) reader structure"]
impl crate::Readable for EXTMEM_CORE0_DBUS_REJECT_VADDR {}
#[doc = "EXTMEM_CORE0_DBUS_REJECT_VADDR"]
pub mod extmem_core0_dbus_reject_vaddr;
#[doc = "EXTMEM_CORE0_IBUS_REJECT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_core0_ibus_reject_st](extmem_core0_ibus_reject_st) module"]
pub type EXTMEM_CORE0_IBUS_REJECT_ST = crate::Reg<u32, _EXTMEM_CORE0_IBUS_REJECT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CORE0_IBUS_REJECT_ST;
#[doc = "`read()` method returns [extmem_core0_ibus_reject_st::R](extmem_core0_ibus_reject_st::R) reader structure"]
impl crate::Readable for EXTMEM_CORE0_IBUS_REJECT_ST {}
#[doc = "EXTMEM_CORE0_IBUS_REJECT_ST"]
pub mod extmem_core0_ibus_reject_st;
#[doc = "EXTMEM_CORE0_IBUS_REJECT_VADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_core0_ibus_reject_vaddr](extmem_core0_ibus_reject_vaddr) module"]
pub type EXTMEM_CORE0_IBUS_REJECT_VADDR = crate::Reg<u32, _EXTMEM_CORE0_IBUS_REJECT_VADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CORE0_IBUS_REJECT_VADDR;
#[doc = "`read()` method returns [extmem_core0_ibus_reject_vaddr::R](extmem_core0_ibus_reject_vaddr::R) reader structure"]
impl crate::Readable for EXTMEM_CORE0_IBUS_REJECT_VADDR {}
#[doc = "EXTMEM_CORE0_IBUS_REJECT_VADDR"]
pub mod extmem_core0_ibus_reject_vaddr;
#[doc = "EXTMEM_CACHE_MMU_FAULT_CONTENT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_mmu_fault_content](extmem_cache_mmu_fault_content) module"]
pub type EXTMEM_CACHE_MMU_FAULT_CONTENT = crate::Reg<u32, _EXTMEM_CACHE_MMU_FAULT_CONTENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_MMU_FAULT_CONTENT;
#[doc = "`read()` method returns [extmem_cache_mmu_fault_content::R](extmem_cache_mmu_fault_content::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_MMU_FAULT_CONTENT {}
#[doc = "EXTMEM_CACHE_MMU_FAULT_CONTENT"]
pub mod extmem_cache_mmu_fault_content;
#[doc = "EXTMEM_CACHE_MMU_FAULT_VADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_mmu_fault_vaddr](extmem_cache_mmu_fault_vaddr) module"]
pub type EXTMEM_CACHE_MMU_FAULT_VADDR = crate::Reg<u32, _EXTMEM_CACHE_MMU_FAULT_VADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_MMU_FAULT_VADDR;
#[doc = "`read()` method returns [extmem_cache_mmu_fault_vaddr::R](extmem_cache_mmu_fault_vaddr::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_MMU_FAULT_VADDR {}
#[doc = "EXTMEM_CACHE_MMU_FAULT_VADDR"]
pub mod extmem_cache_mmu_fault_vaddr;
#[doc = "EXTMEM_CACHE_WRAP_AROUND_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_wrap_around_ctrl](extmem_cache_wrap_around_ctrl) module"]
pub type EXTMEM_CACHE_WRAP_AROUND_CTRL = crate::Reg<u32, _EXTMEM_CACHE_WRAP_AROUND_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_WRAP_AROUND_CTRL;
#[doc = "`read()` method returns [extmem_cache_wrap_around_ctrl::R](extmem_cache_wrap_around_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_WRAP_AROUND_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_wrap_around_ctrl::W](extmem_cache_wrap_around_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_WRAP_AROUND_CTRL {}
#[doc = "EXTMEM_CACHE_WRAP_AROUND_CTRL"]
pub mod extmem_cache_wrap_around_ctrl;
#[doc = "EXTMEM_CACHE_MMU_POWER_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_mmu_power_ctrl](extmem_cache_mmu_power_ctrl) module"]
pub type EXTMEM_CACHE_MMU_POWER_CTRL = crate::Reg<u32, _EXTMEM_CACHE_MMU_POWER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_MMU_POWER_CTRL;
#[doc = "`read()` method returns [extmem_cache_mmu_power_ctrl::R](extmem_cache_mmu_power_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_MMU_POWER_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_mmu_power_ctrl::W](extmem_cache_mmu_power_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_MMU_POWER_CTRL {}
#[doc = "EXTMEM_CACHE_MMU_POWER_CTRL"]
pub mod extmem_cache_mmu_power_ctrl;
#[doc = "EXTMEM_CACHE_STATE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_state](extmem_cache_state) module"]
pub type EXTMEM_CACHE_STATE = crate::Reg<u32, _EXTMEM_CACHE_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_STATE;
#[doc = "`read()` method returns [extmem_cache_state::R](extmem_cache_state::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_STATE {}
#[doc = "EXTMEM_CACHE_STATE"]
pub mod extmem_cache_state;
#[doc = "EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_encrypt_decrypt_record_disable](extmem_cache_encrypt_decrypt_record_disable) module"]
pub type EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE =
    crate::Reg<u32, _EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE;
#[doc = "`read()` method returns [extmem_cache_encrypt_decrypt_record_disable::R](extmem_cache_encrypt_decrypt_record_disable::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_encrypt_decrypt_record_disable::W](extmem_cache_encrypt_decrypt_record_disable::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE {}
#[doc = "EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE"]
pub mod extmem_cache_encrypt_decrypt_record_disable;
#[doc = "EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_encrypt_decrypt_clk_force_on](extmem_cache_encrypt_decrypt_clk_force_on) module"]
pub type EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON =
    crate::Reg<u32, _EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON;
#[doc = "`read()` method returns [extmem_cache_encrypt_decrypt_clk_force_on::R](extmem_cache_encrypt_decrypt_clk_force_on::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_encrypt_decrypt_clk_force_on::W](extmem_cache_encrypt_decrypt_clk_force_on::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON {}
#[doc = "EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON"]
pub mod extmem_cache_encrypt_decrypt_clk_force_on;
#[doc = "EXTMEM_CACHE_PRELOAD_INT_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_preload_int_ctrl](extmem_cache_preload_int_ctrl) module"]
pub type EXTMEM_CACHE_PRELOAD_INT_CTRL = crate::Reg<u32, _EXTMEM_CACHE_PRELOAD_INT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_PRELOAD_INT_CTRL;
#[doc = "`read()` method returns [extmem_cache_preload_int_ctrl::R](extmem_cache_preload_int_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_PRELOAD_INT_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_preload_int_ctrl::W](extmem_cache_preload_int_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_PRELOAD_INT_CTRL {}
#[doc = "EXTMEM_CACHE_PRELOAD_INT_CTRL"]
pub mod extmem_cache_preload_int_ctrl;
#[doc = "EXTMEM_CACHE_SYNC_INT_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_sync_int_ctrl](extmem_cache_sync_int_ctrl) module"]
pub type EXTMEM_CACHE_SYNC_INT_CTRL = crate::Reg<u32, _EXTMEM_CACHE_SYNC_INT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_SYNC_INT_CTRL;
#[doc = "`read()` method returns [extmem_cache_sync_int_ctrl::R](extmem_cache_sync_int_ctrl::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_SYNC_INT_CTRL {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_sync_int_ctrl::W](extmem_cache_sync_int_ctrl::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_SYNC_INT_CTRL {}
#[doc = "EXTMEM_CACHE_SYNC_INT_CTRL"]
pub mod extmem_cache_sync_int_ctrl;
#[doc = "EXTMEM_CACHE_MMU_OWNER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_mmu_owner](extmem_cache_mmu_owner) module"]
pub type EXTMEM_CACHE_MMU_OWNER = crate::Reg<u32, _EXTMEM_CACHE_MMU_OWNER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_MMU_OWNER;
#[doc = "`read()` method returns [extmem_cache_mmu_owner::R](extmem_cache_mmu_owner::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_MMU_OWNER {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_mmu_owner::W](extmem_cache_mmu_owner::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_MMU_OWNER {}
#[doc = "EXTMEM_CACHE_MMU_OWNER"]
pub mod extmem_cache_mmu_owner;
#[doc = "EXTMEM_CACHE_CONF_MISC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_conf_misc](extmem_cache_conf_misc) module"]
pub type EXTMEM_CACHE_CONF_MISC = crate::Reg<u32, _EXTMEM_CACHE_CONF_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_CONF_MISC;
#[doc = "`read()` method returns [extmem_cache_conf_misc::R](extmem_cache_conf_misc::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_CONF_MISC {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_conf_misc::W](extmem_cache_conf_misc::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_CONF_MISC {}
#[doc = "EXTMEM_CACHE_CONF_MISC"]
pub mod extmem_cache_conf_misc;
#[doc = "EXTMEM_ICACHE_FREEZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_freeze](extmem_icache_freeze) module"]
pub type EXTMEM_ICACHE_FREEZE = crate::Reg<u32, _EXTMEM_ICACHE_FREEZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_FREEZE;
#[doc = "`read()` method returns [extmem_icache_freeze::R](extmem_icache_freeze::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_FREEZE {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_freeze::W](extmem_icache_freeze::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_FREEZE {}
#[doc = "EXTMEM_ICACHE_FREEZE"]
pub mod extmem_icache_freeze;
#[doc = "EXTMEM_ICACHE_ATOMIC_OPERATE_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_icache_atomic_operate_ena](extmem_icache_atomic_operate_ena) module"]
pub type EXTMEM_ICACHE_ATOMIC_OPERATE_ENA = crate::Reg<u32, _EXTMEM_ICACHE_ATOMIC_OPERATE_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_ICACHE_ATOMIC_OPERATE_ENA;
#[doc = "`read()` method returns [extmem_icache_atomic_operate_ena::R](extmem_icache_atomic_operate_ena::R) reader structure"]
impl crate::Readable for EXTMEM_ICACHE_ATOMIC_OPERATE_ENA {}
#[doc = "`write(|w| ..)` method takes [extmem_icache_atomic_operate_ena::W](extmem_icache_atomic_operate_ena::W) writer structure"]
impl crate::Writable for EXTMEM_ICACHE_ATOMIC_OPERATE_ENA {}
#[doc = "EXTMEM_ICACHE_ATOMIC_OPERATE_ENA"]
pub mod extmem_icache_atomic_operate_ena;
#[doc = "EXTMEM_CACHE_REQUEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_cache_request](extmem_cache_request) module"]
pub type EXTMEM_CACHE_REQUEST = crate::Reg<u32, _EXTMEM_CACHE_REQUEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CACHE_REQUEST;
#[doc = "`read()` method returns [extmem_cache_request::R](extmem_cache_request::R) reader structure"]
impl crate::Readable for EXTMEM_CACHE_REQUEST {}
#[doc = "`write(|w| ..)` method takes [extmem_cache_request::W](extmem_cache_request::W) writer structure"]
impl crate::Writable for EXTMEM_CACHE_REQUEST {}
#[doc = "EXTMEM_CACHE_REQUEST"]
pub mod extmem_cache_request;
#[doc = "EXTMEM_IBUS_PMS_TBL_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_pms_tbl_lock](extmem_ibus_pms_tbl_lock) module"]
pub type EXTMEM_IBUS_PMS_TBL_LOCK = crate::Reg<u32, _EXTMEM_IBUS_PMS_TBL_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_PMS_TBL_LOCK;
#[doc = "`read()` method returns [extmem_ibus_pms_tbl_lock::R](extmem_ibus_pms_tbl_lock::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_PMS_TBL_LOCK {}
#[doc = "`write(|w| ..)` method takes [extmem_ibus_pms_tbl_lock::W](extmem_ibus_pms_tbl_lock::W) writer structure"]
impl crate::Writable for EXTMEM_IBUS_PMS_TBL_LOCK {}
#[doc = "EXTMEM_IBUS_PMS_TBL_LOCK"]
pub mod extmem_ibus_pms_tbl_lock;
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_pms_tbl_boundary0](extmem_ibus_pms_tbl_boundary0) module"]
pub type EXTMEM_IBUS_PMS_TBL_BOUNDARY0 = crate::Reg<u32, _EXTMEM_IBUS_PMS_TBL_BOUNDARY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_PMS_TBL_BOUNDARY0;
#[doc = "`read()` method returns [extmem_ibus_pms_tbl_boundary0::R](extmem_ibus_pms_tbl_boundary0::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_PMS_TBL_BOUNDARY0 {}
#[doc = "`write(|w| ..)` method takes [extmem_ibus_pms_tbl_boundary0::W](extmem_ibus_pms_tbl_boundary0::W) writer structure"]
impl crate::Writable for EXTMEM_IBUS_PMS_TBL_BOUNDARY0 {}
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY0"]
pub mod extmem_ibus_pms_tbl_boundary0;
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_pms_tbl_boundary1](extmem_ibus_pms_tbl_boundary1) module"]
pub type EXTMEM_IBUS_PMS_TBL_BOUNDARY1 = crate::Reg<u32, _EXTMEM_IBUS_PMS_TBL_BOUNDARY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_PMS_TBL_BOUNDARY1;
#[doc = "`read()` method returns [extmem_ibus_pms_tbl_boundary1::R](extmem_ibus_pms_tbl_boundary1::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_PMS_TBL_BOUNDARY1 {}
#[doc = "`write(|w| ..)` method takes [extmem_ibus_pms_tbl_boundary1::W](extmem_ibus_pms_tbl_boundary1::W) writer structure"]
impl crate::Writable for EXTMEM_IBUS_PMS_TBL_BOUNDARY1 {}
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY1"]
pub mod extmem_ibus_pms_tbl_boundary1;
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_pms_tbl_boundary2](extmem_ibus_pms_tbl_boundary2) module"]
pub type EXTMEM_IBUS_PMS_TBL_BOUNDARY2 = crate::Reg<u32, _EXTMEM_IBUS_PMS_TBL_BOUNDARY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_PMS_TBL_BOUNDARY2;
#[doc = "`read()` method returns [extmem_ibus_pms_tbl_boundary2::R](extmem_ibus_pms_tbl_boundary2::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_PMS_TBL_BOUNDARY2 {}
#[doc = "`write(|w| ..)` method takes [extmem_ibus_pms_tbl_boundary2::W](extmem_ibus_pms_tbl_boundary2::W) writer structure"]
impl crate::Writable for EXTMEM_IBUS_PMS_TBL_BOUNDARY2 {}
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY2"]
pub mod extmem_ibus_pms_tbl_boundary2;
#[doc = "EXTMEM_IBUS_PMS_TBL_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_ibus_pms_tbl_attr](extmem_ibus_pms_tbl_attr) module"]
pub type EXTMEM_IBUS_PMS_TBL_ATTR = crate::Reg<u32, _EXTMEM_IBUS_PMS_TBL_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_IBUS_PMS_TBL_ATTR;
#[doc = "`read()` method returns [extmem_ibus_pms_tbl_attr::R](extmem_ibus_pms_tbl_attr::R) reader structure"]
impl crate::Readable for EXTMEM_IBUS_PMS_TBL_ATTR {}
#[doc = "`write(|w| ..)` method takes [extmem_ibus_pms_tbl_attr::W](extmem_ibus_pms_tbl_attr::W) writer structure"]
impl crate::Writable for EXTMEM_IBUS_PMS_TBL_ATTR {}
#[doc = "EXTMEM_IBUS_PMS_TBL_ATTR"]
pub mod extmem_ibus_pms_tbl_attr;
#[doc = "EXTMEM_DBUS_PMS_TBL_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_pms_tbl_lock](extmem_dbus_pms_tbl_lock) module"]
pub type EXTMEM_DBUS_PMS_TBL_LOCK = crate::Reg<u32, _EXTMEM_DBUS_PMS_TBL_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_PMS_TBL_LOCK;
#[doc = "`read()` method returns [extmem_dbus_pms_tbl_lock::R](extmem_dbus_pms_tbl_lock::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_PMS_TBL_LOCK {}
#[doc = "`write(|w| ..)` method takes [extmem_dbus_pms_tbl_lock::W](extmem_dbus_pms_tbl_lock::W) writer structure"]
impl crate::Writable for EXTMEM_DBUS_PMS_TBL_LOCK {}
#[doc = "EXTMEM_DBUS_PMS_TBL_LOCK"]
pub mod extmem_dbus_pms_tbl_lock;
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_pms_tbl_boundary0](extmem_dbus_pms_tbl_boundary0) module"]
pub type EXTMEM_DBUS_PMS_TBL_BOUNDARY0 = crate::Reg<u32, _EXTMEM_DBUS_PMS_TBL_BOUNDARY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_PMS_TBL_BOUNDARY0;
#[doc = "`read()` method returns [extmem_dbus_pms_tbl_boundary0::R](extmem_dbus_pms_tbl_boundary0::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_PMS_TBL_BOUNDARY0 {}
#[doc = "`write(|w| ..)` method takes [extmem_dbus_pms_tbl_boundary0::W](extmem_dbus_pms_tbl_boundary0::W) writer structure"]
impl crate::Writable for EXTMEM_DBUS_PMS_TBL_BOUNDARY0 {}
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY0"]
pub mod extmem_dbus_pms_tbl_boundary0;
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_pms_tbl_boundary1](extmem_dbus_pms_tbl_boundary1) module"]
pub type EXTMEM_DBUS_PMS_TBL_BOUNDARY1 = crate::Reg<u32, _EXTMEM_DBUS_PMS_TBL_BOUNDARY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_PMS_TBL_BOUNDARY1;
#[doc = "`read()` method returns [extmem_dbus_pms_tbl_boundary1::R](extmem_dbus_pms_tbl_boundary1::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_PMS_TBL_BOUNDARY1 {}
#[doc = "`write(|w| ..)` method takes [extmem_dbus_pms_tbl_boundary1::W](extmem_dbus_pms_tbl_boundary1::W) writer structure"]
impl crate::Writable for EXTMEM_DBUS_PMS_TBL_BOUNDARY1 {}
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY1"]
pub mod extmem_dbus_pms_tbl_boundary1;
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_pms_tbl_boundary2](extmem_dbus_pms_tbl_boundary2) module"]
pub type EXTMEM_DBUS_PMS_TBL_BOUNDARY2 = crate::Reg<u32, _EXTMEM_DBUS_PMS_TBL_BOUNDARY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_PMS_TBL_BOUNDARY2;
#[doc = "`read()` method returns [extmem_dbus_pms_tbl_boundary2::R](extmem_dbus_pms_tbl_boundary2::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_PMS_TBL_BOUNDARY2 {}
#[doc = "`write(|w| ..)` method takes [extmem_dbus_pms_tbl_boundary2::W](extmem_dbus_pms_tbl_boundary2::W) writer structure"]
impl crate::Writable for EXTMEM_DBUS_PMS_TBL_BOUNDARY2 {}
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY2"]
pub mod extmem_dbus_pms_tbl_boundary2;
#[doc = "EXTMEM_DBUS_PMS_TBL_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_dbus_pms_tbl_attr](extmem_dbus_pms_tbl_attr) module"]
pub type EXTMEM_DBUS_PMS_TBL_ATTR = crate::Reg<u32, _EXTMEM_DBUS_PMS_TBL_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DBUS_PMS_TBL_ATTR;
#[doc = "`read()` method returns [extmem_dbus_pms_tbl_attr::R](extmem_dbus_pms_tbl_attr::R) reader structure"]
impl crate::Readable for EXTMEM_DBUS_PMS_TBL_ATTR {}
#[doc = "`write(|w| ..)` method takes [extmem_dbus_pms_tbl_attr::W](extmem_dbus_pms_tbl_attr::W) writer structure"]
impl crate::Writable for EXTMEM_DBUS_PMS_TBL_ATTR {}
#[doc = "EXTMEM_DBUS_PMS_TBL_ATTR"]
pub mod extmem_dbus_pms_tbl_attr;
#[doc = "EXTMEM_CLOCK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_clock_gate](extmem_clock_gate) module"]
pub type EXTMEM_CLOCK_GATE = crate::Reg<u32, _EXTMEM_CLOCK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_CLOCK_GATE;
#[doc = "`read()` method returns [extmem_clock_gate::R](extmem_clock_gate::R) reader structure"]
impl crate::Readable for EXTMEM_CLOCK_GATE {}
#[doc = "`write(|w| ..)` method takes [extmem_clock_gate::W](extmem_clock_gate::W) writer structure"]
impl crate::Writable for EXTMEM_CLOCK_GATE {}
#[doc = "EXTMEM_CLOCK_GATE"]
pub mod extmem_clock_gate;
#[doc = "EXTMEM_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_date](extmem_date) module"]
pub type EXTMEM_DATE = crate::Reg<u32, _EXTMEM_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMEM_DATE;
#[doc = "`read()` method returns [extmem_date::R](extmem_date::R) reader structure"]
impl crate::Readable for EXTMEM_DATE {}
#[doc = "`write(|w| ..)` method takes [extmem_date::W](extmem_date::W) writer structure"]
impl crate::Writable for EXTMEM_DATE {}
#[doc = "EXTMEM_DATE"]
pub mod extmem_date;
