#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SENSITIVE_ROM_TABLE_LOCK"]
    pub sensitive_rom_table_lock: SENSITIVE_ROM_TABLE_LOCK,
    #[doc = "0x04 - SENSITIVE_ROM_TABLE"]
    pub sensitive_rom_table: SENSITIVE_ROM_TABLE,
    #[doc = "0x08 - SENSITIVE_PRIVILEGE_MODE_SEL_LOCK"]
    pub sensitive_privilege_mode_sel_lock: SENSITIVE_PRIVILEGE_MODE_SEL_LOCK,
    #[doc = "0x0c - SENSITIVE_PRIVILEGE_MODE_SEL"]
    pub sensitive_privilege_mode_sel: SENSITIVE_PRIVILEGE_MODE_SEL,
    #[doc = "0x10 - SENSITIVE_APB_PERIPHERAL_ACCESS_0"]
    pub sensitive_apb_peripheral_access_0: SENSITIVE_APB_PERIPHERAL_ACCESS_0,
    #[doc = "0x14 - SENSITIVE_APB_PERIPHERAL_ACCESS_1"]
    pub sensitive_apb_peripheral_access_1: SENSITIVE_APB_PERIPHERAL_ACCESS_1,
    #[doc = "0x18 - SENSITIVE_INTERNAL_SRAM_USAGE_0"]
    pub sensitive_internal_sram_usage_0: SENSITIVE_INTERNAL_SRAM_USAGE_0,
    #[doc = "0x1c - SENSITIVE_INTERNAL_SRAM_USAGE_1"]
    pub sensitive_internal_sram_usage_1: SENSITIVE_INTERNAL_SRAM_USAGE_1,
    #[doc = "0x20 - SENSITIVE_INTERNAL_SRAM_USAGE_3"]
    pub sensitive_internal_sram_usage_3: SENSITIVE_INTERNAL_SRAM_USAGE_3,
    #[doc = "0x24 - SENSITIVE_INTERNAL_SRAM_USAGE_4"]
    pub sensitive_internal_sram_usage_4: SENSITIVE_INTERNAL_SRAM_USAGE_4,
    #[doc = "0x28 - SENSITIVE_CACHE_TAG_ACCESS_0"]
    pub sensitive_cache_tag_access_0: SENSITIVE_CACHE_TAG_ACCESS_0,
    #[doc = "0x2c - SENSITIVE_CACHE_TAG_ACCESS_1"]
    pub sensitive_cache_tag_access_1: SENSITIVE_CACHE_TAG_ACCESS_1,
    #[doc = "0x30 - SENSITIVE_CACHE_MMU_ACCESS_0"]
    pub sensitive_cache_mmu_access_0: SENSITIVE_CACHE_MMU_ACCESS_0,
    #[doc = "0x34 - SENSITIVE_CACHE_MMU_ACCESS_1"]
    pub sensitive_cache_mmu_access_1: SENSITIVE_CACHE_MMU_ACCESS_1,
    #[doc = "0x38 - SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_spi2_pms_constrain_0: SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0,
    #[doc = "0x3c - SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_spi2_pms_constrain_1: SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1,
    #[doc = "0x40 - SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_uchi0_pms_constrain_0: SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0,
    #[doc = "0x44 - SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_uchi0_pms_constrain_1: SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1,
    #[doc = "0x48 - SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_i2s0_pms_constrain_0: SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0,
    #[doc = "0x4c - SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_i2s0_pms_constrain_1: SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1,
    #[doc = "0x50 - SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_mac_pms_constrain_0: SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0,
    #[doc = "0x54 - SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_mac_pms_constrain_1: SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1,
    #[doc = "0x58 - SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_backup_pms_constrain_0: SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0,
    #[doc = "0x5c - SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_backup_pms_constrain_1: SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1,
    #[doc = "0x60 - SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_lc_pms_constrain_0: SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0,
    #[doc = "0x64 - SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_lc_pms_constrain_1: SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1,
    #[doc = "0x68 - SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_aes_pms_constrain_0: SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0,
    #[doc = "0x6c - SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_aes_pms_constrain_1: SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1,
    #[doc = "0x70 - SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_sha_pms_constrain_0: SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0,
    #[doc = "0x74 - SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_sha_pms_constrain_1: SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1,
    #[doc = "0x78 - SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0"]
    pub sensitive_dma_apbperi_adc_dac_pms_constrain_0:
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0,
    #[doc = "0x7c - SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1"]
    pub sensitive_dma_apbperi_adc_dac_pms_constrain_1:
        SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1,
    #[doc = "0x80 - SENSITIVE_DMA_APBPERI_PMS_MONITOR_0"]
    pub sensitive_dma_apbperi_pms_monitor_0: SENSITIVE_DMA_APBPERI_PMS_MONITOR_0,
    #[doc = "0x84 - SENSITIVE_DMA_APBPERI_PMS_MONITOR_1"]
    pub sensitive_dma_apbperi_pms_monitor_1: SENSITIVE_DMA_APBPERI_PMS_MONITOR_1,
    #[doc = "0x88 - SENSITIVE_DMA_APBPERI_PMS_MONITOR_2"]
    pub sensitive_dma_apbperi_pms_monitor_2: SENSITIVE_DMA_APBPERI_PMS_MONITOR_2,
    #[doc = "0x8c - SENSITIVE_DMA_APBPERI_PMS_MONITOR_3"]
    pub sensitive_dma_apbperi_pms_monitor_3: SENSITIVE_DMA_APBPERI_PMS_MONITOR_3,
    #[doc = "0x90 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0"]
    pub sensitive_core_x_iram0_dram0_dma_split_line_constrain_0:
        SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0,
    #[doc = "0x94 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1"]
    pub sensitive_core_x_iram0_dram0_dma_split_line_constrain_1:
        SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1,
    #[doc = "0x98 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2"]
    pub sensitive_core_x_iram0_dram0_dma_split_line_constrain_2:
        SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2,
    #[doc = "0x9c - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3"]
    pub sensitive_core_x_iram0_dram0_dma_split_line_constrain_3:
        SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3,
    #[doc = "0xa0 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4"]
    pub sensitive_core_x_iram0_dram0_dma_split_line_constrain_4:
        SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4,
    #[doc = "0xa4 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5"]
    pub sensitive_core_x_iram0_dram0_dma_split_line_constrain_5:
        SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5,
    #[doc = "0xa8 - SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0"]
    pub sensitive_core_x_iram0_pms_constrain_0: SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0,
    #[doc = "0xac - SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1"]
    pub sensitive_core_x_iram0_pms_constrain_1: SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1,
    #[doc = "0xb0 - SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2"]
    pub sensitive_core_x_iram0_pms_constrain_2: SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2,
    #[doc = "0xb4 - SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0"]
    pub sensitive_core_0_iram0_pms_monitor_0: SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0,
    #[doc = "0xb8 - SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1"]
    pub sensitive_core_0_iram0_pms_monitor_1: SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1,
    #[doc = "0xbc - SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2"]
    pub sensitive_core_0_iram0_pms_monitor_2: SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2,
    #[doc = "0xc0 - SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0"]
    pub sensitive_core_x_dram0_pms_constrain_0: SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0,
    #[doc = "0xc4 - SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1"]
    pub sensitive_core_x_dram0_pms_constrain_1: SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1,
    #[doc = "0xc8 - SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0"]
    pub sensitive_core_0_dram0_pms_monitor_0: SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0,
    #[doc = "0xcc - SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1"]
    pub sensitive_core_0_dram0_pms_monitor_1: SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1,
    #[doc = "0xd0 - SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2"]
    pub sensitive_core_0_dram0_pms_monitor_2: SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2,
    #[doc = "0xd4 - SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3"]
    pub sensitive_core_0_dram0_pms_monitor_3: SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3,
    #[doc = "0xd8 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0"]
    pub sensitive_core_0_pif_pms_constrain_0: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0,
    #[doc = "0xdc - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1"]
    pub sensitive_core_0_pif_pms_constrain_1: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1,
    #[doc = "0xe0 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2"]
    pub sensitive_core_0_pif_pms_constrain_2: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2,
    #[doc = "0xe4 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3"]
    pub sensitive_core_0_pif_pms_constrain_3: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3,
    #[doc = "0xe8 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4"]
    pub sensitive_core_0_pif_pms_constrain_4: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4,
    #[doc = "0xec - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5"]
    pub sensitive_core_0_pif_pms_constrain_5: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5,
    #[doc = "0xf0 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6"]
    pub sensitive_core_0_pif_pms_constrain_6: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6,
    #[doc = "0xf4 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7"]
    pub sensitive_core_0_pif_pms_constrain_7: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7,
    #[doc = "0xf8 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8"]
    pub sensitive_core_0_pif_pms_constrain_8: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8,
    #[doc = "0xfc - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9"]
    pub sensitive_core_0_pif_pms_constrain_9: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9,
    #[doc = "0x100 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10"]
    pub sensitive_core_0_pif_pms_constrain_10: SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10,
    #[doc = "0x104 - SENSITIVE_REGION_PMS_CONSTRAIN_0"]
    pub sensitive_region_pms_constrain_0: SENSITIVE_REGION_PMS_CONSTRAIN_0,
    #[doc = "0x108 - SENSITIVE_REGION_PMS_CONSTRAIN_1"]
    pub sensitive_region_pms_constrain_1: SENSITIVE_REGION_PMS_CONSTRAIN_1,
    #[doc = "0x10c - SENSITIVE_REGION_PMS_CONSTRAIN_2"]
    pub sensitive_region_pms_constrain_2: SENSITIVE_REGION_PMS_CONSTRAIN_2,
    #[doc = "0x110 - SENSITIVE_REGION_PMS_CONSTRAIN_3"]
    pub sensitive_region_pms_constrain_3: SENSITIVE_REGION_PMS_CONSTRAIN_3,
    #[doc = "0x114 - SENSITIVE_REGION_PMS_CONSTRAIN_4"]
    pub sensitive_region_pms_constrain_4: SENSITIVE_REGION_PMS_CONSTRAIN_4,
    #[doc = "0x118 - SENSITIVE_REGION_PMS_CONSTRAIN_5"]
    pub sensitive_region_pms_constrain_5: SENSITIVE_REGION_PMS_CONSTRAIN_5,
    #[doc = "0x11c - SENSITIVE_REGION_PMS_CONSTRAIN_6"]
    pub sensitive_region_pms_constrain_6: SENSITIVE_REGION_PMS_CONSTRAIN_6,
    #[doc = "0x120 - SENSITIVE_REGION_PMS_CONSTRAIN_7"]
    pub sensitive_region_pms_constrain_7: SENSITIVE_REGION_PMS_CONSTRAIN_7,
    #[doc = "0x124 - SENSITIVE_REGION_PMS_CONSTRAIN_8"]
    pub sensitive_region_pms_constrain_8: SENSITIVE_REGION_PMS_CONSTRAIN_8,
    #[doc = "0x128 - SENSITIVE_REGION_PMS_CONSTRAIN_9"]
    pub sensitive_region_pms_constrain_9: SENSITIVE_REGION_PMS_CONSTRAIN_9,
    #[doc = "0x12c - SENSITIVE_REGION_PMS_CONSTRAIN_10"]
    pub sensitive_region_pms_constrain_10: SENSITIVE_REGION_PMS_CONSTRAIN_10,
    #[doc = "0x130 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_0"]
    pub sensitive_core_0_pif_pms_monitor_0: SENSITIVE_CORE_0_PIF_PMS_MONITOR_0,
    #[doc = "0x134 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_1"]
    pub sensitive_core_0_pif_pms_monitor_1: SENSITIVE_CORE_0_PIF_PMS_MONITOR_1,
    #[doc = "0x138 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_2"]
    pub sensitive_core_0_pif_pms_monitor_2: SENSITIVE_CORE_0_PIF_PMS_MONITOR_2,
    #[doc = "0x13c - SENSITIVE_CORE_0_PIF_PMS_MONITOR_3"]
    pub sensitive_core_0_pif_pms_monitor_3: SENSITIVE_CORE_0_PIF_PMS_MONITOR_3,
    #[doc = "0x140 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_4"]
    pub sensitive_core_0_pif_pms_monitor_4: SENSITIVE_CORE_0_PIF_PMS_MONITOR_4,
    #[doc = "0x144 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_5"]
    pub sensitive_core_0_pif_pms_monitor_5: SENSITIVE_CORE_0_PIF_PMS_MONITOR_5,
    #[doc = "0x148 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_6"]
    pub sensitive_core_0_pif_pms_monitor_6: SENSITIVE_CORE_0_PIF_PMS_MONITOR_6,
    #[doc = "0x14c - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0"]
    pub sensitive_backup_bus_pms_constrain_0: SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0,
    #[doc = "0x150 - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1"]
    pub sensitive_backup_bus_pms_constrain_1: SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1,
    #[doc = "0x154 - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2"]
    pub sensitive_backup_bus_pms_constrain_2: SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2,
    #[doc = "0x158 - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3"]
    pub sensitive_backup_bus_pms_constrain_3: SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3,
    #[doc = "0x15c - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4"]
    pub sensitive_backup_bus_pms_constrain_4: SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4,
    #[doc = "0x160 - SENSITIVE_BACKUP_BUS_PMS_MONITOR_0"]
    pub sensitive_backup_bus_pms_monitor_0: SENSITIVE_BACKUP_BUS_PMS_MONITOR_0,
    #[doc = "0x164 - SENSITIVE_BACKUP_BUS_PMS_MONITOR_1"]
    pub sensitive_backup_bus_pms_monitor_1: SENSITIVE_BACKUP_BUS_PMS_MONITOR_1,
    #[doc = "0x168 - SENSITIVE_BACKUP_BUS_PMS_MONITOR_2"]
    pub sensitive_backup_bus_pms_monitor_2: SENSITIVE_BACKUP_BUS_PMS_MONITOR_2,
    #[doc = "0x16c - SENSITIVE_BACKUP_BUS_PMS_MONITOR_3"]
    pub sensitive_backup_bus_pms_monitor_3: SENSITIVE_BACKUP_BUS_PMS_MONITOR_3,
    #[doc = "0x170 - SENSITIVE_CLOCK_GATE"]
    pub sensitive_clock_gate: SENSITIVE_CLOCK_GATE,
    _reserved93: [u8; 3720usize],
    #[doc = "0xffc - SENSITIVE_DATE"]
    pub sensitive_date: SENSITIVE_DATE,
}
#[doc = "SENSITIVE_ROM_TABLE_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_rom_table_lock](sensitive_rom_table_lock) module"]
pub type SENSITIVE_ROM_TABLE_LOCK = crate::Reg<u32, _SENSITIVE_ROM_TABLE_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_ROM_TABLE_LOCK;
#[doc = "`read()` method returns [sensitive_rom_table_lock::R](sensitive_rom_table_lock::R) reader structure"]
impl crate::Readable for SENSITIVE_ROM_TABLE_LOCK {}
#[doc = "`write(|w| ..)` method takes [sensitive_rom_table_lock::W](sensitive_rom_table_lock::W) writer structure"]
impl crate::Writable for SENSITIVE_ROM_TABLE_LOCK {}
#[doc = "SENSITIVE_ROM_TABLE_LOCK"]
pub mod sensitive_rom_table_lock;
#[doc = "SENSITIVE_ROM_TABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_rom_table](sensitive_rom_table) module"]
pub type SENSITIVE_ROM_TABLE = crate::Reg<u32, _SENSITIVE_ROM_TABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_ROM_TABLE;
#[doc = "`read()` method returns [sensitive_rom_table::R](sensitive_rom_table::R) reader structure"]
impl crate::Readable for SENSITIVE_ROM_TABLE {}
#[doc = "`write(|w| ..)` method takes [sensitive_rom_table::W](sensitive_rom_table::W) writer structure"]
impl crate::Writable for SENSITIVE_ROM_TABLE {}
#[doc = "SENSITIVE_ROM_TABLE"]
pub mod sensitive_rom_table;
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_privilege_mode_sel_lock](sensitive_privilege_mode_sel_lock) module"]
pub type SENSITIVE_PRIVILEGE_MODE_SEL_LOCK = crate::Reg<u32, _SENSITIVE_PRIVILEGE_MODE_SEL_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_PRIVILEGE_MODE_SEL_LOCK;
#[doc = "`read()` method returns [sensitive_privilege_mode_sel_lock::R](sensitive_privilege_mode_sel_lock::R) reader structure"]
impl crate::Readable for SENSITIVE_PRIVILEGE_MODE_SEL_LOCK {}
#[doc = "`write(|w| ..)` method takes [sensitive_privilege_mode_sel_lock::W](sensitive_privilege_mode_sel_lock::W) writer structure"]
impl crate::Writable for SENSITIVE_PRIVILEGE_MODE_SEL_LOCK {}
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_LOCK"]
pub mod sensitive_privilege_mode_sel_lock;
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_privilege_mode_sel](sensitive_privilege_mode_sel) module"]
pub type SENSITIVE_PRIVILEGE_MODE_SEL = crate::Reg<u32, _SENSITIVE_PRIVILEGE_MODE_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_PRIVILEGE_MODE_SEL;
#[doc = "`read()` method returns [sensitive_privilege_mode_sel::R](sensitive_privilege_mode_sel::R) reader structure"]
impl crate::Readable for SENSITIVE_PRIVILEGE_MODE_SEL {}
#[doc = "`write(|w| ..)` method takes [sensitive_privilege_mode_sel::W](sensitive_privilege_mode_sel::W) writer structure"]
impl crate::Writable for SENSITIVE_PRIVILEGE_MODE_SEL {}
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL"]
pub mod sensitive_privilege_mode_sel;
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_apb_peripheral_access_0](sensitive_apb_peripheral_access_0) module"]
pub type SENSITIVE_APB_PERIPHERAL_ACCESS_0 = crate::Reg<u32, _SENSITIVE_APB_PERIPHERAL_ACCESS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_APB_PERIPHERAL_ACCESS_0;
#[doc = "`read()` method returns [sensitive_apb_peripheral_access_0::R](sensitive_apb_peripheral_access_0::R) reader structure"]
impl crate::Readable for SENSITIVE_APB_PERIPHERAL_ACCESS_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_apb_peripheral_access_0::W](sensitive_apb_peripheral_access_0::W) writer structure"]
impl crate::Writable for SENSITIVE_APB_PERIPHERAL_ACCESS_0 {}
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_0"]
pub mod sensitive_apb_peripheral_access_0;
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_apb_peripheral_access_1](sensitive_apb_peripheral_access_1) module"]
pub type SENSITIVE_APB_PERIPHERAL_ACCESS_1 = crate::Reg<u32, _SENSITIVE_APB_PERIPHERAL_ACCESS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_APB_PERIPHERAL_ACCESS_1;
#[doc = "`read()` method returns [sensitive_apb_peripheral_access_1::R](sensitive_apb_peripheral_access_1::R) reader structure"]
impl crate::Readable for SENSITIVE_APB_PERIPHERAL_ACCESS_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_apb_peripheral_access_1::W](sensitive_apb_peripheral_access_1::W) writer structure"]
impl crate::Writable for SENSITIVE_APB_PERIPHERAL_ACCESS_1 {}
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_1"]
pub mod sensitive_apb_peripheral_access_1;
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_internal_sram_usage_0](sensitive_internal_sram_usage_0) module"]
pub type SENSITIVE_INTERNAL_SRAM_USAGE_0 = crate::Reg<u32, _SENSITIVE_INTERNAL_SRAM_USAGE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_INTERNAL_SRAM_USAGE_0;
#[doc = "`read()` method returns [sensitive_internal_sram_usage_0::R](sensitive_internal_sram_usage_0::R) reader structure"]
impl crate::Readable for SENSITIVE_INTERNAL_SRAM_USAGE_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_internal_sram_usage_0::W](sensitive_internal_sram_usage_0::W) writer structure"]
impl crate::Writable for SENSITIVE_INTERNAL_SRAM_USAGE_0 {}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_0"]
pub mod sensitive_internal_sram_usage_0;
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_internal_sram_usage_1](sensitive_internal_sram_usage_1) module"]
pub type SENSITIVE_INTERNAL_SRAM_USAGE_1 = crate::Reg<u32, _SENSITIVE_INTERNAL_SRAM_USAGE_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_INTERNAL_SRAM_USAGE_1;
#[doc = "`read()` method returns [sensitive_internal_sram_usage_1::R](sensitive_internal_sram_usage_1::R) reader structure"]
impl crate::Readable for SENSITIVE_INTERNAL_SRAM_USAGE_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_internal_sram_usage_1::W](sensitive_internal_sram_usage_1::W) writer structure"]
impl crate::Writable for SENSITIVE_INTERNAL_SRAM_USAGE_1 {}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_1"]
pub mod sensitive_internal_sram_usage_1;
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_internal_sram_usage_3](sensitive_internal_sram_usage_3) module"]
pub type SENSITIVE_INTERNAL_SRAM_USAGE_3 = crate::Reg<u32, _SENSITIVE_INTERNAL_SRAM_USAGE_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_INTERNAL_SRAM_USAGE_3;
#[doc = "`read()` method returns [sensitive_internal_sram_usage_3::R](sensitive_internal_sram_usage_3::R) reader structure"]
impl crate::Readable for SENSITIVE_INTERNAL_SRAM_USAGE_3 {}
#[doc = "`write(|w| ..)` method takes [sensitive_internal_sram_usage_3::W](sensitive_internal_sram_usage_3::W) writer structure"]
impl crate::Writable for SENSITIVE_INTERNAL_SRAM_USAGE_3 {}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_3"]
pub mod sensitive_internal_sram_usage_3;
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_internal_sram_usage_4](sensitive_internal_sram_usage_4) module"]
pub type SENSITIVE_INTERNAL_SRAM_USAGE_4 = crate::Reg<u32, _SENSITIVE_INTERNAL_SRAM_USAGE_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_INTERNAL_SRAM_USAGE_4;
#[doc = "`read()` method returns [sensitive_internal_sram_usage_4::R](sensitive_internal_sram_usage_4::R) reader structure"]
impl crate::Readable for SENSITIVE_INTERNAL_SRAM_USAGE_4 {}
#[doc = "`write(|w| ..)` method takes [sensitive_internal_sram_usage_4::W](sensitive_internal_sram_usage_4::W) writer structure"]
impl crate::Writable for SENSITIVE_INTERNAL_SRAM_USAGE_4 {}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_4"]
pub mod sensitive_internal_sram_usage_4;
#[doc = "SENSITIVE_CACHE_TAG_ACCESS_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_cache_tag_access_0](sensitive_cache_tag_access_0) module"]
pub type SENSITIVE_CACHE_TAG_ACCESS_0 = crate::Reg<u32, _SENSITIVE_CACHE_TAG_ACCESS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CACHE_TAG_ACCESS_0;
#[doc = "`read()` method returns [sensitive_cache_tag_access_0::R](sensitive_cache_tag_access_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CACHE_TAG_ACCESS_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_cache_tag_access_0::W](sensitive_cache_tag_access_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CACHE_TAG_ACCESS_0 {}
#[doc = "SENSITIVE_CACHE_TAG_ACCESS_0"]
pub mod sensitive_cache_tag_access_0;
#[doc = "SENSITIVE_CACHE_TAG_ACCESS_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_cache_tag_access_1](sensitive_cache_tag_access_1) module"]
pub type SENSITIVE_CACHE_TAG_ACCESS_1 = crate::Reg<u32, _SENSITIVE_CACHE_TAG_ACCESS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CACHE_TAG_ACCESS_1;
#[doc = "`read()` method returns [sensitive_cache_tag_access_1::R](sensitive_cache_tag_access_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CACHE_TAG_ACCESS_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_cache_tag_access_1::W](sensitive_cache_tag_access_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CACHE_TAG_ACCESS_1 {}
#[doc = "SENSITIVE_CACHE_TAG_ACCESS_1"]
pub mod sensitive_cache_tag_access_1;
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_cache_mmu_access_0](sensitive_cache_mmu_access_0) module"]
pub type SENSITIVE_CACHE_MMU_ACCESS_0 = crate::Reg<u32, _SENSITIVE_CACHE_MMU_ACCESS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CACHE_MMU_ACCESS_0;
#[doc = "`read()` method returns [sensitive_cache_mmu_access_0::R](sensitive_cache_mmu_access_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CACHE_MMU_ACCESS_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_cache_mmu_access_0::W](sensitive_cache_mmu_access_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CACHE_MMU_ACCESS_0 {}
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_0"]
pub mod sensitive_cache_mmu_access_0;
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_cache_mmu_access_1](sensitive_cache_mmu_access_1) module"]
pub type SENSITIVE_CACHE_MMU_ACCESS_1 = crate::Reg<u32, _SENSITIVE_CACHE_MMU_ACCESS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CACHE_MMU_ACCESS_1;
#[doc = "`read()` method returns [sensitive_cache_mmu_access_1::R](sensitive_cache_mmu_access_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CACHE_MMU_ACCESS_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_cache_mmu_access_1::W](sensitive_cache_mmu_access_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CACHE_MMU_ACCESS_1 {}
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_1"]
pub mod sensitive_cache_mmu_access_1;
#[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_spi2_pms_constrain_0](sensitive_dma_apbperi_spi2_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_spi2_pms_constrain_0::R](sensitive_dma_apbperi_spi2_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_spi2_pms_constrain_0::W](sensitive_dma_apbperi_spi2_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_spi2_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_spi2_pms_constrain_1](sensitive_dma_apbperi_spi2_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_spi2_pms_constrain_1::R](sensitive_dma_apbperi_spi2_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_spi2_pms_constrain_1::W](sensitive_dma_apbperi_spi2_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_spi2_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_uchi0_pms_constrain_0](sensitive_dma_apbperi_uchi0_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_uchi0_pms_constrain_0::R](sensitive_dma_apbperi_uchi0_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_uchi0_pms_constrain_0::W](sensitive_dma_apbperi_uchi0_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_uchi0_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_uchi0_pms_constrain_1](sensitive_dma_apbperi_uchi0_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_uchi0_pms_constrain_1::R](sensitive_dma_apbperi_uchi0_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_uchi0_pms_constrain_1::W](sensitive_dma_apbperi_uchi0_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_uchi0_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_i2s0_pms_constrain_0](sensitive_dma_apbperi_i2s0_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_i2s0_pms_constrain_0::R](sensitive_dma_apbperi_i2s0_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_i2s0_pms_constrain_0::W](sensitive_dma_apbperi_i2s0_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_i2s0_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_i2s0_pms_constrain_1](sensitive_dma_apbperi_i2s0_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_i2s0_pms_constrain_1::R](sensitive_dma_apbperi_i2s0_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_i2s0_pms_constrain_1::W](sensitive_dma_apbperi_i2s0_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_i2s0_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_mac_pms_constrain_0](sensitive_dma_apbperi_mac_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_mac_pms_constrain_0::R](sensitive_dma_apbperi_mac_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_mac_pms_constrain_0::W](sensitive_dma_apbperi_mac_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_mac_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_mac_pms_constrain_1](sensitive_dma_apbperi_mac_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_mac_pms_constrain_1::R](sensitive_dma_apbperi_mac_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_mac_pms_constrain_1::W](sensitive_dma_apbperi_mac_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_mac_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_backup_pms_constrain_0](sensitive_dma_apbperi_backup_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_backup_pms_constrain_0::R](sensitive_dma_apbperi_backup_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_backup_pms_constrain_0::W](sensitive_dma_apbperi_backup_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_backup_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_backup_pms_constrain_1](sensitive_dma_apbperi_backup_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_backup_pms_constrain_1::R](sensitive_dma_apbperi_backup_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_backup_pms_constrain_1::W](sensitive_dma_apbperi_backup_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_backup_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_lc_pms_constrain_0](sensitive_dma_apbperi_lc_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_lc_pms_constrain_0::R](sensitive_dma_apbperi_lc_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_lc_pms_constrain_0::W](sensitive_dma_apbperi_lc_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_lc_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_lc_pms_constrain_1](sensitive_dma_apbperi_lc_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_lc_pms_constrain_1::R](sensitive_dma_apbperi_lc_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_lc_pms_constrain_1::W](sensitive_dma_apbperi_lc_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_lc_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_aes_pms_constrain_0](sensitive_dma_apbperi_aes_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_aes_pms_constrain_0::R](sensitive_dma_apbperi_aes_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_aes_pms_constrain_0::W](sensitive_dma_apbperi_aes_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_aes_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_aes_pms_constrain_1](sensitive_dma_apbperi_aes_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_aes_pms_constrain_1::R](sensitive_dma_apbperi_aes_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_aes_pms_constrain_1::W](sensitive_dma_apbperi_aes_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_aes_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_sha_pms_constrain_0](sensitive_dma_apbperi_sha_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_sha_pms_constrain_0::R](sensitive_dma_apbperi_sha_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_sha_pms_constrain_0::W](sensitive_dma_apbperi_sha_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_sha_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_sha_pms_constrain_1](sensitive_dma_apbperi_sha_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_sha_pms_constrain_1::R](sensitive_dma_apbperi_sha_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_sha_pms_constrain_1::W](sensitive_dma_apbperi_sha_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_sha_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_adc_dac_pms_constrain_0](sensitive_dma_apbperi_adc_dac_pms_constrain_0) module"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_adc_dac_pms_constrain_0::R](sensitive_dma_apbperi_adc_dac_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_adc_dac_pms_constrain_0::W](sensitive_dma_apbperi_adc_dac_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0"]
pub mod sensitive_dma_apbperi_adc_dac_pms_constrain_0;
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_adc_dac_pms_constrain_1](sensitive_dma_apbperi_adc_dac_pms_constrain_1) module"]
pub type SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_adc_dac_pms_constrain_1::R](sensitive_dma_apbperi_adc_dac_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_adc_dac_pms_constrain_1::W](sensitive_dma_apbperi_adc_dac_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1"]
pub mod sensitive_dma_apbperi_adc_dac_pms_constrain_1;
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_pms_monitor_0](sensitive_dma_apbperi_pms_monitor_0) module"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_0 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_PMS_MONITOR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_PMS_MONITOR_0;
#[doc = "`read()` method returns [sensitive_dma_apbperi_pms_monitor_0::R](sensitive_dma_apbperi_pms_monitor_0::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_PMS_MONITOR_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_pms_monitor_0::W](sensitive_dma_apbperi_pms_monitor_0::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_PMS_MONITOR_0 {}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_0"]
pub mod sensitive_dma_apbperi_pms_monitor_0;
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_pms_monitor_1](sensitive_dma_apbperi_pms_monitor_1) module"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_1 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_PMS_MONITOR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_PMS_MONITOR_1;
#[doc = "`read()` method returns [sensitive_dma_apbperi_pms_monitor_1::R](sensitive_dma_apbperi_pms_monitor_1::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_PMS_MONITOR_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_dma_apbperi_pms_monitor_1::W](sensitive_dma_apbperi_pms_monitor_1::W) writer structure"]
impl crate::Writable for SENSITIVE_DMA_APBPERI_PMS_MONITOR_1 {}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_1"]
pub mod sensitive_dma_apbperi_pms_monitor_1;
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_pms_monitor_2](sensitive_dma_apbperi_pms_monitor_2) module"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_2 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_PMS_MONITOR_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_PMS_MONITOR_2;
#[doc = "`read()` method returns [sensitive_dma_apbperi_pms_monitor_2::R](sensitive_dma_apbperi_pms_monitor_2::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_PMS_MONITOR_2 {}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_2"]
pub mod sensitive_dma_apbperi_pms_monitor_2;
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_dma_apbperi_pms_monitor_3](sensitive_dma_apbperi_pms_monitor_3) module"]
pub type SENSITIVE_DMA_APBPERI_PMS_MONITOR_3 =
    crate::Reg<u32, _SENSITIVE_DMA_APBPERI_PMS_MONITOR_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DMA_APBPERI_PMS_MONITOR_3;
#[doc = "`read()` method returns [sensitive_dma_apbperi_pms_monitor_3::R](sensitive_dma_apbperi_pms_monitor_3::R) reader structure"]
impl crate::Readable for SENSITIVE_DMA_APBPERI_PMS_MONITOR_3 {}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_3"]
pub mod sensitive_dma_apbperi_pms_monitor_3;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_dram0_dma_split_line_constrain_0](sensitive_core_x_iram0_dram0_dma_split_line_constrain_0) module"]
pub type SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_core_x_iram0_dram0_dma_split_line_constrain_0::R](sensitive_core_x_iram0_dram0_dma_split_line_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_dram0_dma_split_line_constrain_0::W](sensitive_core_x_iram0_dram0_dma_split_line_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0"]
pub mod sensitive_core_x_iram0_dram0_dma_split_line_constrain_0;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_dram0_dma_split_line_constrain_1](sensitive_core_x_iram0_dram0_dma_split_line_constrain_1) module"]
pub type SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_core_x_iram0_dram0_dma_split_line_constrain_1::R](sensitive_core_x_iram0_dram0_dma_split_line_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_dram0_dma_split_line_constrain_1::W](sensitive_core_x_iram0_dram0_dma_split_line_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1"]
pub mod sensitive_core_x_iram0_dram0_dma_split_line_constrain_1;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_dram0_dma_split_line_constrain_2](sensitive_core_x_iram0_dram0_dma_split_line_constrain_2) module"]
pub type SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2;
#[doc = "`read()` method returns [sensitive_core_x_iram0_dram0_dma_split_line_constrain_2::R](sensitive_core_x_iram0_dram0_dma_split_line_constrain_2::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_dram0_dma_split_line_constrain_2::W](sensitive_core_x_iram0_dram0_dma_split_line_constrain_2::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2"]
pub mod sensitive_core_x_iram0_dram0_dma_split_line_constrain_2;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_dram0_dma_split_line_constrain_3](sensitive_core_x_iram0_dram0_dma_split_line_constrain_3) module"]
pub type SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3;
#[doc = "`read()` method returns [sensitive_core_x_iram0_dram0_dma_split_line_constrain_3::R](sensitive_core_x_iram0_dram0_dma_split_line_constrain_3::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_dram0_dma_split_line_constrain_3::W](sensitive_core_x_iram0_dram0_dma_split_line_constrain_3::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3"]
pub mod sensitive_core_x_iram0_dram0_dma_split_line_constrain_3;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_dram0_dma_split_line_constrain_4](sensitive_core_x_iram0_dram0_dma_split_line_constrain_4) module"]
pub type SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4;
#[doc = "`read()` method returns [sensitive_core_x_iram0_dram0_dma_split_line_constrain_4::R](sensitive_core_x_iram0_dram0_dma_split_line_constrain_4::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_dram0_dma_split_line_constrain_4::W](sensitive_core_x_iram0_dram0_dma_split_line_constrain_4::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4"]
pub mod sensitive_core_x_iram0_dram0_dma_split_line_constrain_4;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_dram0_dma_split_line_constrain_5](sensitive_core_x_iram0_dram0_dma_split_line_constrain_5) module"]
pub type SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5;
#[doc = "`read()` method returns [sensitive_core_x_iram0_dram0_dma_split_line_constrain_5::R](sensitive_core_x_iram0_dram0_dma_split_line_constrain_5::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_dram0_dma_split_line_constrain_5::W](sensitive_core_x_iram0_dram0_dma_split_line_constrain_5::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5"]
pub mod sensitive_core_x_iram0_dram0_dma_split_line_constrain_5;
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_pms_constrain_0](sensitive_core_x_iram0_pms_constrain_0) module"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_core_x_iram0_pms_constrain_0::R](sensitive_core_x_iram0_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_pms_constrain_0::W](sensitive_core_x_iram0_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0"]
pub mod sensitive_core_x_iram0_pms_constrain_0;
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_pms_constrain_1](sensitive_core_x_iram0_pms_constrain_1) module"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_core_x_iram0_pms_constrain_1::R](sensitive_core_x_iram0_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_pms_constrain_1::W](sensitive_core_x_iram0_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1"]
pub mod sensitive_core_x_iram0_pms_constrain_1;
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_iram0_pms_constrain_2](sensitive_core_x_iram0_pms_constrain_2) module"]
pub type SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2 =
    crate::Reg<u32, _SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2;
#[doc = "`read()` method returns [sensitive_core_x_iram0_pms_constrain_2::R](sensitive_core_x_iram0_pms_constrain_2::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_iram0_pms_constrain_2::W](sensitive_core_x_iram0_pms_constrain_2::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2 {}
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2"]
pub mod sensitive_core_x_iram0_pms_constrain_2;
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_iram0_pms_monitor_0](sensitive_core_0_iram0_pms_monitor_0) module"]
pub type SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0 =
    crate::Reg<u32, _SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0;
#[doc = "`read()` method returns [sensitive_core_0_iram0_pms_monitor_0::R](sensitive_core_0_iram0_pms_monitor_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_iram0_pms_monitor_0::W](sensitive_core_0_iram0_pms_monitor_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0 {}
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0"]
pub mod sensitive_core_0_iram0_pms_monitor_0;
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_iram0_pms_monitor_1](sensitive_core_0_iram0_pms_monitor_1) module"]
pub type SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1 =
    crate::Reg<u32, _SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1;
#[doc = "`read()` method returns [sensitive_core_0_iram0_pms_monitor_1::R](sensitive_core_0_iram0_pms_monitor_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_iram0_pms_monitor_1::W](sensitive_core_0_iram0_pms_monitor_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1 {}
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1"]
pub mod sensitive_core_0_iram0_pms_monitor_1;
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_iram0_pms_monitor_2](sensitive_core_0_iram0_pms_monitor_2) module"]
pub type SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2 =
    crate::Reg<u32, _SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2;
#[doc = "`read()` method returns [sensitive_core_0_iram0_pms_monitor_2::R](sensitive_core_0_iram0_pms_monitor_2::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2 {}
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2"]
pub mod sensitive_core_0_iram0_pms_monitor_2;
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_dram0_pms_constrain_0](sensitive_core_x_dram0_pms_constrain_0) module"]
pub type SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_core_x_dram0_pms_constrain_0::R](sensitive_core_x_dram0_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_dram0_pms_constrain_0::W](sensitive_core_x_dram0_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0"]
pub mod sensitive_core_x_dram0_pms_constrain_0;
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_x_dram0_pms_constrain_1](sensitive_core_x_dram0_pms_constrain_1) module"]
pub type SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_core_x_dram0_pms_constrain_1::R](sensitive_core_x_dram0_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_x_dram0_pms_constrain_1::W](sensitive_core_x_dram0_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1"]
pub mod sensitive_core_x_dram0_pms_constrain_1;
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_dram0_pms_monitor_0](sensitive_core_0_dram0_pms_monitor_0) module"]
pub type SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0 =
    crate::Reg<u32, _SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0;
#[doc = "`read()` method returns [sensitive_core_0_dram0_pms_monitor_0::R](sensitive_core_0_dram0_pms_monitor_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_dram0_pms_monitor_0::W](sensitive_core_0_dram0_pms_monitor_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0 {}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0"]
pub mod sensitive_core_0_dram0_pms_monitor_0;
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_dram0_pms_monitor_1](sensitive_core_0_dram0_pms_monitor_1) module"]
pub type SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1 =
    crate::Reg<u32, _SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1;
#[doc = "`read()` method returns [sensitive_core_0_dram0_pms_monitor_1::R](sensitive_core_0_dram0_pms_monitor_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_dram0_pms_monitor_1::W](sensitive_core_0_dram0_pms_monitor_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1 {}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1"]
pub mod sensitive_core_0_dram0_pms_monitor_1;
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_dram0_pms_monitor_2](sensitive_core_0_dram0_pms_monitor_2) module"]
pub type SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2 =
    crate::Reg<u32, _SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2;
#[doc = "`read()` method returns [sensitive_core_0_dram0_pms_monitor_2::R](sensitive_core_0_dram0_pms_monitor_2::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2 {}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2"]
pub mod sensitive_core_0_dram0_pms_monitor_2;
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_dram0_pms_monitor_3](sensitive_core_0_dram0_pms_monitor_3) module"]
pub type SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3 =
    crate::Reg<u32, _SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3;
#[doc = "`read()` method returns [sensitive_core_0_dram0_pms_monitor_3::R](sensitive_core_0_dram0_pms_monitor_3::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3 {}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3"]
pub mod sensitive_core_0_dram0_pms_monitor_3;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_0](sensitive_core_0_pif_pms_constrain_0) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_0::R](sensitive_core_0_pif_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_0::W](sensitive_core_0_pif_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0"]
pub mod sensitive_core_0_pif_pms_constrain_0;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_1](sensitive_core_0_pif_pms_constrain_1) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_1::R](sensitive_core_0_pif_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_1::W](sensitive_core_0_pif_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1"]
pub mod sensitive_core_0_pif_pms_constrain_1;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_2](sensitive_core_0_pif_pms_constrain_2) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_2::R](sensitive_core_0_pif_pms_constrain_2::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_2::W](sensitive_core_0_pif_pms_constrain_2::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2"]
pub mod sensitive_core_0_pif_pms_constrain_2;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_3](sensitive_core_0_pif_pms_constrain_3) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_3::R](sensitive_core_0_pif_pms_constrain_3::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_3::W](sensitive_core_0_pif_pms_constrain_3::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3"]
pub mod sensitive_core_0_pif_pms_constrain_3;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_4](sensitive_core_0_pif_pms_constrain_4) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_4::R](sensitive_core_0_pif_pms_constrain_4::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_4::W](sensitive_core_0_pif_pms_constrain_4::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4"]
pub mod sensitive_core_0_pif_pms_constrain_4;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_5](sensitive_core_0_pif_pms_constrain_5) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_5::R](sensitive_core_0_pif_pms_constrain_5::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_5::W](sensitive_core_0_pif_pms_constrain_5::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5"]
pub mod sensitive_core_0_pif_pms_constrain_5;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_6](sensitive_core_0_pif_pms_constrain_6) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_6::R](sensitive_core_0_pif_pms_constrain_6::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_6::W](sensitive_core_0_pif_pms_constrain_6::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6"]
pub mod sensitive_core_0_pif_pms_constrain_6;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_7](sensitive_core_0_pif_pms_constrain_7) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_7::R](sensitive_core_0_pif_pms_constrain_7::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_7::W](sensitive_core_0_pif_pms_constrain_7::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7"]
pub mod sensitive_core_0_pif_pms_constrain_7;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_8](sensitive_core_0_pif_pms_constrain_8) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_8::R](sensitive_core_0_pif_pms_constrain_8::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_8::W](sensitive_core_0_pif_pms_constrain_8::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8"]
pub mod sensitive_core_0_pif_pms_constrain_8;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_9](sensitive_core_0_pif_pms_constrain_9) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_9::R](sensitive_core_0_pif_pms_constrain_9::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_9::W](sensitive_core_0_pif_pms_constrain_9::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9"]
pub mod sensitive_core_0_pif_pms_constrain_9;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_constrain_10](sensitive_core_0_pif_pms_constrain_10) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10 =
    crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_constrain_10::R](sensitive_core_0_pif_pms_constrain_10::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_constrain_10::W](sensitive_core_0_pif_pms_constrain_10::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10"]
pub mod sensitive_core_0_pif_pms_constrain_10;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_0](sensitive_region_pms_constrain_0) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_0 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_0::R](sensitive_region_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_0::W](sensitive_region_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_0"]
pub mod sensitive_region_pms_constrain_0;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_1](sensitive_region_pms_constrain_1) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_1 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_1::R](sensitive_region_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_1::W](sensitive_region_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_1"]
pub mod sensitive_region_pms_constrain_1;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_2](sensitive_region_pms_constrain_2) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_2 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_2;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_2::R](sensitive_region_pms_constrain_2::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_2 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_2::W](sensitive_region_pms_constrain_2::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_2 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_2"]
pub mod sensitive_region_pms_constrain_2;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_3](sensitive_region_pms_constrain_3) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_3 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_3;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_3::R](sensitive_region_pms_constrain_3::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_3 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_3::W](sensitive_region_pms_constrain_3::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_3 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_3"]
pub mod sensitive_region_pms_constrain_3;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_4](sensitive_region_pms_constrain_4) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_4 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_4;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_4::R](sensitive_region_pms_constrain_4::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_4 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_4::W](sensitive_region_pms_constrain_4::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_4 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_4"]
pub mod sensitive_region_pms_constrain_4;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_5](sensitive_region_pms_constrain_5) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_5 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_5;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_5::R](sensitive_region_pms_constrain_5::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_5 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_5::W](sensitive_region_pms_constrain_5::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_5 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_5"]
pub mod sensitive_region_pms_constrain_5;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_6](sensitive_region_pms_constrain_6) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_6 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_6;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_6::R](sensitive_region_pms_constrain_6::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_6 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_6::W](sensitive_region_pms_constrain_6::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_6 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_6"]
pub mod sensitive_region_pms_constrain_6;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_7](sensitive_region_pms_constrain_7) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_7 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_7;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_7::R](sensitive_region_pms_constrain_7::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_7 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_7::W](sensitive_region_pms_constrain_7::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_7 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_7"]
pub mod sensitive_region_pms_constrain_7;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_8](sensitive_region_pms_constrain_8) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_8 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_8;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_8::R](sensitive_region_pms_constrain_8::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_8 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_8::W](sensitive_region_pms_constrain_8::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_8 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_8"]
pub mod sensitive_region_pms_constrain_8;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_9](sensitive_region_pms_constrain_9) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_9 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_9;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_9::R](sensitive_region_pms_constrain_9::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_9 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_9::W](sensitive_region_pms_constrain_9::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_9 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_9"]
pub mod sensitive_region_pms_constrain_9;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_region_pms_constrain_10](sensitive_region_pms_constrain_10) module"]
pub type SENSITIVE_REGION_PMS_CONSTRAIN_10 = crate::Reg<u32, _SENSITIVE_REGION_PMS_CONSTRAIN_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_REGION_PMS_CONSTRAIN_10;
#[doc = "`read()` method returns [sensitive_region_pms_constrain_10::R](sensitive_region_pms_constrain_10::R) reader structure"]
impl crate::Readable for SENSITIVE_REGION_PMS_CONSTRAIN_10 {}
#[doc = "`write(|w| ..)` method takes [sensitive_region_pms_constrain_10::W](sensitive_region_pms_constrain_10::W) writer structure"]
impl crate::Writable for SENSITIVE_REGION_PMS_CONSTRAIN_10 {}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_10"]
pub mod sensitive_region_pms_constrain_10;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_monitor_0](sensitive_core_0_pif_pms_monitor_0) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_0 = crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_MONITOR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_MONITOR_0;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_monitor_0::R](sensitive_core_0_pif_pms_monitor_0::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_monitor_0::W](sensitive_core_0_pif_pms_monitor_0::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_0 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_0"]
pub mod sensitive_core_0_pif_pms_monitor_0;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_monitor_1](sensitive_core_0_pif_pms_monitor_1) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_1 = crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_MONITOR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_MONITOR_1;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_monitor_1::R](sensitive_core_0_pif_pms_monitor_1::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_monitor_1::W](sensitive_core_0_pif_pms_monitor_1::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_1 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_1"]
pub mod sensitive_core_0_pif_pms_monitor_1;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_monitor_2](sensitive_core_0_pif_pms_monitor_2) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_2 = crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_MONITOR_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_MONITOR_2;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_monitor_2::R](sensitive_core_0_pif_pms_monitor_2::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_2 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_2"]
pub mod sensitive_core_0_pif_pms_monitor_2;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_monitor_3](sensitive_core_0_pif_pms_monitor_3) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_3 = crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_MONITOR_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_MONITOR_3;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_monitor_3::R](sensitive_core_0_pif_pms_monitor_3::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_3 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_3"]
pub mod sensitive_core_0_pif_pms_monitor_3;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_monitor_4](sensitive_core_0_pif_pms_monitor_4) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_4 = crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_MONITOR_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_MONITOR_4;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_monitor_4::R](sensitive_core_0_pif_pms_monitor_4::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_4 {}
#[doc = "`write(|w| ..)` method takes [sensitive_core_0_pif_pms_monitor_4::W](sensitive_core_0_pif_pms_monitor_4::W) writer structure"]
impl crate::Writable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_4 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_4"]
pub mod sensitive_core_0_pif_pms_monitor_4;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_monitor_5](sensitive_core_0_pif_pms_monitor_5) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_5 = crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_MONITOR_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_MONITOR_5;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_monitor_5::R](sensitive_core_0_pif_pms_monitor_5::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_5 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_5"]
pub mod sensitive_core_0_pif_pms_monitor_5;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_core_0_pif_pms_monitor_6](sensitive_core_0_pif_pms_monitor_6) module"]
pub type SENSITIVE_CORE_0_PIF_PMS_MONITOR_6 = crate::Reg<u32, _SENSITIVE_CORE_0_PIF_PMS_MONITOR_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CORE_0_PIF_PMS_MONITOR_6;
#[doc = "`read()` method returns [sensitive_core_0_pif_pms_monitor_6::R](sensitive_core_0_pif_pms_monitor_6::R) reader structure"]
impl crate::Readable for SENSITIVE_CORE_0_PIF_PMS_MONITOR_6 {}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_6"]
pub mod sensitive_core_0_pif_pms_monitor_6;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_constrain_0](sensitive_backup_bus_pms_constrain_0) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0 =
    crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_constrain_0::R](sensitive_backup_bus_pms_constrain_0::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_backup_bus_pms_constrain_0::W](sensitive_backup_bus_pms_constrain_0::W) writer structure"]
impl crate::Writable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0"]
pub mod sensitive_backup_bus_pms_constrain_0;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_constrain_1](sensitive_backup_bus_pms_constrain_1) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1 =
    crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_constrain_1::R](sensitive_backup_bus_pms_constrain_1::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_backup_bus_pms_constrain_1::W](sensitive_backup_bus_pms_constrain_1::W) writer structure"]
impl crate::Writable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1"]
pub mod sensitive_backup_bus_pms_constrain_1;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_constrain_2](sensitive_backup_bus_pms_constrain_2) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2 =
    crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_constrain_2::R](sensitive_backup_bus_pms_constrain_2::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2 {}
#[doc = "`write(|w| ..)` method takes [sensitive_backup_bus_pms_constrain_2::W](sensitive_backup_bus_pms_constrain_2::W) writer structure"]
impl crate::Writable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2"]
pub mod sensitive_backup_bus_pms_constrain_2;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_constrain_3](sensitive_backup_bus_pms_constrain_3) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3 =
    crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_constrain_3::R](sensitive_backup_bus_pms_constrain_3::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3 {}
#[doc = "`write(|w| ..)` method takes [sensitive_backup_bus_pms_constrain_3::W](sensitive_backup_bus_pms_constrain_3::W) writer structure"]
impl crate::Writable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3"]
pub mod sensitive_backup_bus_pms_constrain_3;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_constrain_4](sensitive_backup_bus_pms_constrain_4) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4 =
    crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_constrain_4::R](sensitive_backup_bus_pms_constrain_4::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4 {}
#[doc = "`write(|w| ..)` method takes [sensitive_backup_bus_pms_constrain_4::W](sensitive_backup_bus_pms_constrain_4::W) writer structure"]
impl crate::Writable for SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4"]
pub mod sensitive_backup_bus_pms_constrain_4;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_monitor_0](sensitive_backup_bus_pms_monitor_0) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_0 = crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_MONITOR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_MONITOR_0;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_monitor_0::R](sensitive_backup_bus_pms_monitor_0::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_MONITOR_0 {}
#[doc = "`write(|w| ..)` method takes [sensitive_backup_bus_pms_monitor_0::W](sensitive_backup_bus_pms_monitor_0::W) writer structure"]
impl crate::Writable for SENSITIVE_BACKUP_BUS_PMS_MONITOR_0 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_0"]
pub mod sensitive_backup_bus_pms_monitor_0;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_monitor_1](sensitive_backup_bus_pms_monitor_1) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_1 = crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_MONITOR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_MONITOR_1;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_monitor_1::R](sensitive_backup_bus_pms_monitor_1::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_MONITOR_1 {}
#[doc = "`write(|w| ..)` method takes [sensitive_backup_bus_pms_monitor_1::W](sensitive_backup_bus_pms_monitor_1::W) writer structure"]
impl crate::Writable for SENSITIVE_BACKUP_BUS_PMS_MONITOR_1 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_1"]
pub mod sensitive_backup_bus_pms_monitor_1;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_monitor_2](sensitive_backup_bus_pms_monitor_2) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_2 = crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_MONITOR_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_MONITOR_2;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_monitor_2::R](sensitive_backup_bus_pms_monitor_2::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_MONITOR_2 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_2"]
pub mod sensitive_backup_bus_pms_monitor_2;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_backup_bus_pms_monitor_3](sensitive_backup_bus_pms_monitor_3) module"]
pub type SENSITIVE_BACKUP_BUS_PMS_MONITOR_3 = crate::Reg<u32, _SENSITIVE_BACKUP_BUS_PMS_MONITOR_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_BACKUP_BUS_PMS_MONITOR_3;
#[doc = "`read()` method returns [sensitive_backup_bus_pms_monitor_3::R](sensitive_backup_bus_pms_monitor_3::R) reader structure"]
impl crate::Readable for SENSITIVE_BACKUP_BUS_PMS_MONITOR_3 {}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_3"]
pub mod sensitive_backup_bus_pms_monitor_3;
#[doc = "SENSITIVE_CLOCK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_clock_gate](sensitive_clock_gate) module"]
pub type SENSITIVE_CLOCK_GATE = crate::Reg<u32, _SENSITIVE_CLOCK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_CLOCK_GATE;
#[doc = "`read()` method returns [sensitive_clock_gate::R](sensitive_clock_gate::R) reader structure"]
impl crate::Readable for SENSITIVE_CLOCK_GATE {}
#[doc = "`write(|w| ..)` method takes [sensitive_clock_gate::W](sensitive_clock_gate::W) writer structure"]
impl crate::Writable for SENSITIVE_CLOCK_GATE {}
#[doc = "SENSITIVE_CLOCK_GATE"]
pub mod sensitive_clock_gate;
#[doc = "SENSITIVE_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_date](sensitive_date) module"]
pub type SENSITIVE_DATE = crate::Reg<u32, _SENSITIVE_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSITIVE_DATE;
#[doc = "`read()` method returns [sensitive_date::R](sensitive_date::R) reader structure"]
impl crate::Readable for SENSITIVE_DATE {}
#[doc = "`write(|w| ..)` method takes [sensitive_date::W](sensitive_date::W) writer structure"]
impl crate::Writable for SENSITIVE_DATE {}
#[doc = "SENSITIVE_DATE"]
pub mod sensitive_date;
