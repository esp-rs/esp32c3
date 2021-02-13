#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - INTERRUPT_CORE0_MAC_INTR_MAP"]
    pub interrupt_core0_mac_intr_map: INTERRUPT_CORE0_MAC_INTR_MAP,
    #[doc = "0x04 - INTERRUPT_CORE0_MAC_NMI_MAP"]
    pub interrupt_core0_mac_nmi_map: INTERRUPT_CORE0_MAC_NMI_MAP,
    #[doc = "0x08 - INTERRUPT_CORE0_PWR_INTR_MAP"]
    pub interrupt_core0_pwr_intr_map: INTERRUPT_CORE0_PWR_INTR_MAP,
    #[doc = "0x0c - INTERRUPT_CORE0_BB_INT_MAP"]
    pub interrupt_core0_bb_int_map: INTERRUPT_CORE0_BB_INT_MAP,
    #[doc = "0x10 - INTERRUPT_CORE0_BT_MAC_INT_MAP"]
    pub interrupt_core0_bt_mac_int_map: INTERRUPT_CORE0_BT_MAC_INT_MAP,
    #[doc = "0x14 - INTERRUPT_CORE0_BT_BB_INT_MAP"]
    pub interrupt_core0_bt_bb_int_map: INTERRUPT_CORE0_BT_BB_INT_MAP,
    #[doc = "0x18 - INTERRUPT_CORE0_BT_BB_NMI_MAP"]
    pub interrupt_core0_bt_bb_nmi_map: INTERRUPT_CORE0_BT_BB_NMI_MAP,
    #[doc = "0x1c - INTERRUPT_CORE0_RWBT_IRQ_MAP"]
    pub interrupt_core0_rwbt_irq_map: INTERRUPT_CORE0_RWBT_IRQ_MAP,
    #[doc = "0x20 - INTERRUPT_CORE0_RWBLE_IRQ_MAP"]
    pub interrupt_core0_rwble_irq_map: INTERRUPT_CORE0_RWBLE_IRQ_MAP,
    #[doc = "0x24 - INTERRUPT_CORE0_RWBT_NMI_MAP"]
    pub interrupt_core0_rwbt_nmi_map: INTERRUPT_CORE0_RWBT_NMI_MAP,
    #[doc = "0x28 - INTERRUPT_CORE0_RWBLE_NMI_MAP"]
    pub interrupt_core0_rwble_nmi_map: INTERRUPT_CORE0_RWBLE_NMI_MAP,
    #[doc = "0x2c - INTERRUPT_CORE0_I2C_MST_INT_MAP"]
    pub interrupt_core0_i2c_mst_int_map: INTERRUPT_CORE0_I2C_MST_INT_MAP,
    #[doc = "0x30 - INTERRUPT_CORE0_SLC0_INTR_MAP"]
    pub interrupt_core0_slc0_intr_map: INTERRUPT_CORE0_SLC0_INTR_MAP,
    #[doc = "0x34 - INTERRUPT_CORE0_SLC1_INTR_MAP"]
    pub interrupt_core0_slc1_intr_map: INTERRUPT_CORE0_SLC1_INTR_MAP,
    #[doc = "0x38 - INTERRUPT_CORE0_APB_CTRL_INTR_MAP"]
    pub interrupt_core0_apb_ctrl_intr_map: INTERRUPT_CORE0_APB_CTRL_INTR_MAP,
    #[doc = "0x3c - INTERRUPT_CORE0_UHCI0_INTR_MAP"]
    pub interrupt_core0_uhci0_intr_map: INTERRUPT_CORE0_UHCI0_INTR_MAP,
    #[doc = "0x40 - INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP"]
    pub interrupt_core0_gpio_interrupt_pro_map: INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP,
    #[doc = "0x44 - INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP"]
    pub interrupt_core0_gpio_interrupt_pro_nmi_map: INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP,
    #[doc = "0x48 - INTERRUPT_CORE0_SPI_INTR_1_MAP"]
    pub interrupt_core0_spi_intr_1_map: INTERRUPT_CORE0_SPI_INTR_1_MAP,
    #[doc = "0x4c - INTERRUPT_CORE0_SPI_INTR_2_MAP"]
    pub interrupt_core0_spi_intr_2_map: INTERRUPT_CORE0_SPI_INTR_2_MAP,
    #[doc = "0x50 - INTERRUPT_CORE0_I2S1_INT_MAP"]
    pub interrupt_core0_i2s1_int_map: INTERRUPT_CORE0_I2S1_INT_MAP,
    #[doc = "0x54 - INTERRUPT_CORE0_UART_INTR_MAP"]
    pub interrupt_core0_uart_intr_map: INTERRUPT_CORE0_UART_INTR_MAP,
    #[doc = "0x58 - INTERRUPT_CORE0_UART1_INTR_MAP"]
    pub interrupt_core0_uart1_intr_map: INTERRUPT_CORE0_UART1_INTR_MAP,
    #[doc = "0x5c - INTERRUPT_CORE0_LEDC_INT_MAP"]
    pub interrupt_core0_ledc_int_map: INTERRUPT_CORE0_LEDC_INT_MAP,
    #[doc = "0x60 - INTERRUPT_CORE0_EFUSE_INT_MAP"]
    pub interrupt_core0_efuse_int_map: INTERRUPT_CORE0_EFUSE_INT_MAP,
    #[doc = "0x64 - INTERRUPT_CORE0_CAN_INT_MAP"]
    pub interrupt_core0_can_int_map: INTERRUPT_CORE0_CAN_INT_MAP,
    #[doc = "0x68 - INTERRUPT_CORE0_USB_INTR_MAP"]
    pub interrupt_core0_usb_intr_map: INTERRUPT_CORE0_USB_INTR_MAP,
    #[doc = "0x6c - INTERRUPT_CORE0_RTC_CORE_INTR_MAP"]
    pub interrupt_core0_rtc_core_intr_map: INTERRUPT_CORE0_RTC_CORE_INTR_MAP,
    #[doc = "0x70 - INTERRUPT_CORE0_RMT_INTR_MAP"]
    pub interrupt_core0_rmt_intr_map: INTERRUPT_CORE0_RMT_INTR_MAP,
    #[doc = "0x74 - INTERRUPT_CORE0_I2C_EXT0_INTR_MAP"]
    pub interrupt_core0_i2c_ext0_intr_map: INTERRUPT_CORE0_I2C_EXT0_INTR_MAP,
    #[doc = "0x78 - INTERRUPT_CORE0_TIMER_INT1_MAP"]
    pub interrupt_core0_timer_int1_map: INTERRUPT_CORE0_TIMER_INT1_MAP,
    #[doc = "0x7c - INTERRUPT_CORE0_TIMER_INT2_MAP"]
    pub interrupt_core0_timer_int2_map: INTERRUPT_CORE0_TIMER_INT2_MAP,
    #[doc = "0x80 - INTERRUPT_CORE0_TG_T0_INT_MAP"]
    pub interrupt_core0_tg_t0_int_map: INTERRUPT_CORE0_TG_T0_INT_MAP,
    #[doc = "0x84 - INTERRUPT_CORE0_TG_WDT_INT_MAP"]
    pub interrupt_core0_tg_wdt_int_map: INTERRUPT_CORE0_TG_WDT_INT_MAP,
    #[doc = "0x88 - INTERRUPT_CORE0_TG1_T0_INT_MAP"]
    pub interrupt_core0_tg1_t0_int_map: INTERRUPT_CORE0_TG1_T0_INT_MAP,
    #[doc = "0x8c - INTERRUPT_CORE0_TG1_WDT_INT_MAP"]
    pub interrupt_core0_tg1_wdt_int_map: INTERRUPT_CORE0_TG1_WDT_INT_MAP,
    #[doc = "0x90 - INTERRUPT_CORE0_CACHE_IA_INT_MAP"]
    pub interrupt_core0_cache_ia_int_map: INTERRUPT_CORE0_CACHE_IA_INT_MAP,
    #[doc = "0x94 - INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP"]
    pub interrupt_core0_systimer_target0_int_map: INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP,
    #[doc = "0x98 - INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP"]
    pub interrupt_core0_systimer_target1_int_map: INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP,
    #[doc = "0x9c - INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP"]
    pub interrupt_core0_systimer_target2_int_map: INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP,
    #[doc = "0xa0 - INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP"]
    pub interrupt_core0_spi_mem_reject_intr_map: INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP,
    #[doc = "0xa4 - INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP"]
    pub interrupt_core0_icache_preload_int_map: INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP,
    #[doc = "0xa8 - INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP"]
    pub interrupt_core0_icache_sync_int_map: INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP,
    #[doc = "0xac - INTERRUPT_CORE0_APB_ADC_INT_MAP"]
    pub interrupt_core0_apb_adc_int_map: INTERRUPT_CORE0_APB_ADC_INT_MAP,
    #[doc = "0xb0 - INTERRUPT_CORE0_DMA_CH0_INT_MAP"]
    pub interrupt_core0_dma_ch0_int_map: INTERRUPT_CORE0_DMA_CH0_INT_MAP,
    #[doc = "0xb4 - INTERRUPT_CORE0_DMA_CH1_INT_MAP"]
    pub interrupt_core0_dma_ch1_int_map: INTERRUPT_CORE0_DMA_CH1_INT_MAP,
    #[doc = "0xb8 - INTERRUPT_CORE0_DMA_CH2_INT_MAP"]
    pub interrupt_core0_dma_ch2_int_map: INTERRUPT_CORE0_DMA_CH2_INT_MAP,
    #[doc = "0xbc - INTERRUPT_CORE0_RSA_INT_MAP"]
    pub interrupt_core0_rsa_int_map: INTERRUPT_CORE0_RSA_INT_MAP,
    #[doc = "0xc0 - INTERRUPT_CORE0_AES_INT_MAP"]
    pub interrupt_core0_aes_int_map: INTERRUPT_CORE0_AES_INT_MAP,
    #[doc = "0xc4 - INTERRUPT_CORE0_SHA_INT_MAP"]
    pub interrupt_core0_sha_int_map: INTERRUPT_CORE0_SHA_INT_MAP,
    #[doc = "0xc8 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP"]
    pub interrupt_core0_cpu_intr_from_cpu_0_map: INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0xcc - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP"]
    pub interrupt_core0_cpu_intr_from_cpu_1_map: INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0xd0 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP"]
    pub interrupt_core0_cpu_intr_from_cpu_2_map: INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0xd4 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP"]
    pub interrupt_core0_cpu_intr_from_cpu_3_map: INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0xd8 - INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP"]
    pub interrupt_core0_assist_debug_intr_map: INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP,
    #[doc = "0xdc - INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP"]
    pub interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map:
        INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0xe0 - INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP"]
    pub interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map:
        INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0xe4 - INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP"]
    pub interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map:
        INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0xe8 - INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP"]
    pub interrupt_core0_core_0_pif_pms_monitor_violate_intr_map:
        INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0xec - INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP"]
    pub interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map:
        INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    #[doc = "0xf0 - INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP"]
    pub interrupt_core0_backup_pms_violate_intr_map: INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP,
    #[doc = "0xf4 - INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP"]
    pub interrupt_core0_cache_core0_acs_int_map: INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP,
    #[doc = "0xf8 - INTERRUPT_CORE0_INTR_STATUS_0"]
    pub interrupt_core0_intr_status_0: INTERRUPT_CORE0_INTR_STATUS_0,
    #[doc = "0xfc - INTERRUPT_CORE0_INTR_STATUS_1"]
    pub interrupt_core0_intr_status_1: INTERRUPT_CORE0_INTR_STATUS_1,
    #[doc = "0x100 - INTERRUPT_CORE0_CLOCK_GATE"]
    pub interrupt_core0_clock_gate: INTERRUPT_CORE0_CLOCK_GATE,
    #[doc = "0x104 - INTERRUPT_CORE0_CPU_INT_ENABLE"]
    pub interrupt_core0_cpu_int_enable: INTERRUPT_CORE0_CPU_INT_ENABLE,
    #[doc = "0x108 - INTERRUPT_CORE0_CPU_INT_TYPE"]
    pub interrupt_core0_cpu_int_type: INTERRUPT_CORE0_CPU_INT_TYPE,
    #[doc = "0x10c - INTERRUPT_CORE0_CPU_INT_CLEAR"]
    pub interrupt_core0_cpu_int_clear: INTERRUPT_CORE0_CPU_INT_CLEAR,
    #[doc = "0x110 - INTERRUPT_CORE0_CPU_INT_EIP_STATUS"]
    pub interrupt_core0_cpu_int_eip_status: INTERRUPT_CORE0_CPU_INT_EIP_STATUS,
    #[doc = "0x114 - INTERRUPT_CORE0_CPU_INT_PRI_0"]
    pub interrupt_core0_cpu_int_pri_0: INTERRUPT_CORE0_CPU_INT_PRI_0,
    #[doc = "0x118 - INTERRUPT_CORE0_CPU_INT_PRI_1"]
    pub interrupt_core0_cpu_int_pri_1: INTERRUPT_CORE0_CPU_INT_PRI_1,
    #[doc = "0x11c - INTERRUPT_CORE0_CPU_INT_PRI_2"]
    pub interrupt_core0_cpu_int_pri_2: INTERRUPT_CORE0_CPU_INT_PRI_2,
    #[doc = "0x120 - INTERRUPT_CORE0_CPU_INT_PRI_3"]
    pub interrupt_core0_cpu_int_pri_3: INTERRUPT_CORE0_CPU_INT_PRI_3,
    #[doc = "0x124 - INTERRUPT_CORE0_CPU_INT_PRI_4"]
    pub interrupt_core0_cpu_int_pri_4: INTERRUPT_CORE0_CPU_INT_PRI_4,
    #[doc = "0x128 - INTERRUPT_CORE0_CPU_INT_PRI_5"]
    pub interrupt_core0_cpu_int_pri_5: INTERRUPT_CORE0_CPU_INT_PRI_5,
    #[doc = "0x12c - INTERRUPT_CORE0_CPU_INT_PRI_6"]
    pub interrupt_core0_cpu_int_pri_6: INTERRUPT_CORE0_CPU_INT_PRI_6,
    #[doc = "0x130 - INTERRUPT_CORE0_CPU_INT_PRI_7"]
    pub interrupt_core0_cpu_int_pri_7: INTERRUPT_CORE0_CPU_INT_PRI_7,
    #[doc = "0x134 - INTERRUPT_CORE0_CPU_INT_PRI_8"]
    pub interrupt_core0_cpu_int_pri_8: INTERRUPT_CORE0_CPU_INT_PRI_8,
    #[doc = "0x138 - INTERRUPT_CORE0_CPU_INT_PRI_9"]
    pub interrupt_core0_cpu_int_pri_9: INTERRUPT_CORE0_CPU_INT_PRI_9,
    #[doc = "0x13c - INTERRUPT_CORE0_CPU_INT_PRI_10"]
    pub interrupt_core0_cpu_int_pri_10: INTERRUPT_CORE0_CPU_INT_PRI_10,
    #[doc = "0x140 - INTERRUPT_CORE0_CPU_INT_PRI_11"]
    pub interrupt_core0_cpu_int_pri_11: INTERRUPT_CORE0_CPU_INT_PRI_11,
    #[doc = "0x144 - INTERRUPT_CORE0_CPU_INT_PRI_12"]
    pub interrupt_core0_cpu_int_pri_12: INTERRUPT_CORE0_CPU_INT_PRI_12,
    #[doc = "0x148 - INTERRUPT_CORE0_CPU_INT_PRI_13"]
    pub interrupt_core0_cpu_int_pri_13: INTERRUPT_CORE0_CPU_INT_PRI_13,
    #[doc = "0x14c - INTERRUPT_CORE0_CPU_INT_PRI_14"]
    pub interrupt_core0_cpu_int_pri_14: INTERRUPT_CORE0_CPU_INT_PRI_14,
    #[doc = "0x150 - INTERRUPT_CORE0_CPU_INT_PRI_15"]
    pub interrupt_core0_cpu_int_pri_15: INTERRUPT_CORE0_CPU_INT_PRI_15,
    #[doc = "0x154 - INTERRUPT_CORE0_CPU_INT_PRI_16"]
    pub interrupt_core0_cpu_int_pri_16: INTERRUPT_CORE0_CPU_INT_PRI_16,
    #[doc = "0x158 - INTERRUPT_CORE0_CPU_INT_PRI_17"]
    pub interrupt_core0_cpu_int_pri_17: INTERRUPT_CORE0_CPU_INT_PRI_17,
    #[doc = "0x15c - INTERRUPT_CORE0_CPU_INT_PRI_18"]
    pub interrupt_core0_cpu_int_pri_18: INTERRUPT_CORE0_CPU_INT_PRI_18,
    #[doc = "0x160 - INTERRUPT_CORE0_CPU_INT_PRI_19"]
    pub interrupt_core0_cpu_int_pri_19: INTERRUPT_CORE0_CPU_INT_PRI_19,
    #[doc = "0x164 - INTERRUPT_CORE0_CPU_INT_PRI_20"]
    pub interrupt_core0_cpu_int_pri_20: INTERRUPT_CORE0_CPU_INT_PRI_20,
    #[doc = "0x168 - INTERRUPT_CORE0_CPU_INT_PRI_21"]
    pub interrupt_core0_cpu_int_pri_21: INTERRUPT_CORE0_CPU_INT_PRI_21,
    #[doc = "0x16c - INTERRUPT_CORE0_CPU_INT_PRI_22"]
    pub interrupt_core0_cpu_int_pri_22: INTERRUPT_CORE0_CPU_INT_PRI_22,
    #[doc = "0x170 - INTERRUPT_CORE0_CPU_INT_PRI_23"]
    pub interrupt_core0_cpu_int_pri_23: INTERRUPT_CORE0_CPU_INT_PRI_23,
    #[doc = "0x174 - INTERRUPT_CORE0_CPU_INT_PRI_24"]
    pub interrupt_core0_cpu_int_pri_24: INTERRUPT_CORE0_CPU_INT_PRI_24,
    #[doc = "0x178 - INTERRUPT_CORE0_CPU_INT_PRI_25"]
    pub interrupt_core0_cpu_int_pri_25: INTERRUPT_CORE0_CPU_INT_PRI_25,
    #[doc = "0x17c - INTERRUPT_CORE0_CPU_INT_PRI_26"]
    pub interrupt_core0_cpu_int_pri_26: INTERRUPT_CORE0_CPU_INT_PRI_26,
    #[doc = "0x180 - INTERRUPT_CORE0_CPU_INT_PRI_27"]
    pub interrupt_core0_cpu_int_pri_27: INTERRUPT_CORE0_CPU_INT_PRI_27,
    #[doc = "0x184 - INTERRUPT_CORE0_CPU_INT_PRI_28"]
    pub interrupt_core0_cpu_int_pri_28: INTERRUPT_CORE0_CPU_INT_PRI_28,
    #[doc = "0x188 - INTERRUPT_CORE0_CPU_INT_PRI_29"]
    pub interrupt_core0_cpu_int_pri_29: INTERRUPT_CORE0_CPU_INT_PRI_29,
    #[doc = "0x18c - INTERRUPT_CORE0_CPU_INT_PRI_30"]
    pub interrupt_core0_cpu_int_pri_30: INTERRUPT_CORE0_CPU_INT_PRI_30,
    #[doc = "0x190 - INTERRUPT_CORE0_CPU_INT_PRI_31"]
    pub interrupt_core0_cpu_int_pri_31: INTERRUPT_CORE0_CPU_INT_PRI_31,
    #[doc = "0x194 - INTERRUPT_CORE0_CPU_INT_THRESH"]
    pub interrupt_core0_cpu_int_thresh: INTERRUPT_CORE0_CPU_INT_THRESH,
    _reserved102: [u8; 1636usize],
    #[doc = "0x7fc - INTERRUPT_CORE0_INTERRUPT_DATE"]
    pub interrupt_core0_interrupt_date: INTERRUPT_CORE0_INTERRUPT_DATE,
}
#[doc = "INTERRUPT_CORE0_MAC_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_mac_intr_map](interrupt_core0_mac_intr_map) module"]
pub type INTERRUPT_CORE0_MAC_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_MAC_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_MAC_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_mac_intr_map::R](interrupt_core0_mac_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_MAC_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_mac_intr_map::W](interrupt_core0_mac_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_MAC_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_MAC_INTR_MAP"]
pub mod interrupt_core0_mac_intr_map;
#[doc = "INTERRUPT_CORE0_MAC_NMI_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_mac_nmi_map](interrupt_core0_mac_nmi_map) module"]
pub type INTERRUPT_CORE0_MAC_NMI_MAP = crate::Reg<u32, _INTERRUPT_CORE0_MAC_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_MAC_NMI_MAP;
#[doc = "`read()` method returns [interrupt_core0_mac_nmi_map::R](interrupt_core0_mac_nmi_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_MAC_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_mac_nmi_map::W](interrupt_core0_mac_nmi_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_MAC_NMI_MAP {}
#[doc = "INTERRUPT_CORE0_MAC_NMI_MAP"]
pub mod interrupt_core0_mac_nmi_map;
#[doc = "INTERRUPT_CORE0_PWR_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_pwr_intr_map](interrupt_core0_pwr_intr_map) module"]
pub type INTERRUPT_CORE0_PWR_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_PWR_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_PWR_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_pwr_intr_map::R](interrupt_core0_pwr_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_PWR_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_pwr_intr_map::W](interrupt_core0_pwr_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_PWR_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_PWR_INTR_MAP"]
pub mod interrupt_core0_pwr_intr_map;
#[doc = "INTERRUPT_CORE0_BB_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_bb_int_map](interrupt_core0_bb_int_map) module"]
pub type INTERRUPT_CORE0_BB_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_BB_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_BB_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_bb_int_map::R](interrupt_core0_bb_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_BB_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_bb_int_map::W](interrupt_core0_bb_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_BB_INT_MAP {}
#[doc = "INTERRUPT_CORE0_BB_INT_MAP"]
pub mod interrupt_core0_bb_int_map;
#[doc = "INTERRUPT_CORE0_BT_MAC_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_bt_mac_int_map](interrupt_core0_bt_mac_int_map) module"]
pub type INTERRUPT_CORE0_BT_MAC_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_BT_MAC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_BT_MAC_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_bt_mac_int_map::R](interrupt_core0_bt_mac_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_BT_MAC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_bt_mac_int_map::W](interrupt_core0_bt_mac_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_BT_MAC_INT_MAP {}
#[doc = "INTERRUPT_CORE0_BT_MAC_INT_MAP"]
pub mod interrupt_core0_bt_mac_int_map;
#[doc = "INTERRUPT_CORE0_BT_BB_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_bt_bb_int_map](interrupt_core0_bt_bb_int_map) module"]
pub type INTERRUPT_CORE0_BT_BB_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_BT_BB_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_BT_BB_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_bt_bb_int_map::R](interrupt_core0_bt_bb_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_BT_BB_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_bt_bb_int_map::W](interrupt_core0_bt_bb_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_BT_BB_INT_MAP {}
#[doc = "INTERRUPT_CORE0_BT_BB_INT_MAP"]
pub mod interrupt_core0_bt_bb_int_map;
#[doc = "INTERRUPT_CORE0_BT_BB_NMI_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_bt_bb_nmi_map](interrupt_core0_bt_bb_nmi_map) module"]
pub type INTERRUPT_CORE0_BT_BB_NMI_MAP = crate::Reg<u32, _INTERRUPT_CORE0_BT_BB_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_BT_BB_NMI_MAP;
#[doc = "`read()` method returns [interrupt_core0_bt_bb_nmi_map::R](interrupt_core0_bt_bb_nmi_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_BT_BB_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_bt_bb_nmi_map::W](interrupt_core0_bt_bb_nmi_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_BT_BB_NMI_MAP {}
#[doc = "INTERRUPT_CORE0_BT_BB_NMI_MAP"]
pub mod interrupt_core0_bt_bb_nmi_map;
#[doc = "INTERRUPT_CORE0_RWBT_IRQ_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_rwbt_irq_map](interrupt_core0_rwbt_irq_map) module"]
pub type INTERRUPT_CORE0_RWBT_IRQ_MAP = crate::Reg<u32, _INTERRUPT_CORE0_RWBT_IRQ_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_RWBT_IRQ_MAP;
#[doc = "`read()` method returns [interrupt_core0_rwbt_irq_map::R](interrupt_core0_rwbt_irq_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_RWBT_IRQ_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_rwbt_irq_map::W](interrupt_core0_rwbt_irq_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_RWBT_IRQ_MAP {}
#[doc = "INTERRUPT_CORE0_RWBT_IRQ_MAP"]
pub mod interrupt_core0_rwbt_irq_map;
#[doc = "INTERRUPT_CORE0_RWBLE_IRQ_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_rwble_irq_map](interrupt_core0_rwble_irq_map) module"]
pub type INTERRUPT_CORE0_RWBLE_IRQ_MAP = crate::Reg<u32, _INTERRUPT_CORE0_RWBLE_IRQ_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_RWBLE_IRQ_MAP;
#[doc = "`read()` method returns [interrupt_core0_rwble_irq_map::R](interrupt_core0_rwble_irq_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_RWBLE_IRQ_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_rwble_irq_map::W](interrupt_core0_rwble_irq_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_RWBLE_IRQ_MAP {}
#[doc = "INTERRUPT_CORE0_RWBLE_IRQ_MAP"]
pub mod interrupt_core0_rwble_irq_map;
#[doc = "INTERRUPT_CORE0_RWBT_NMI_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_rwbt_nmi_map](interrupt_core0_rwbt_nmi_map) module"]
pub type INTERRUPT_CORE0_RWBT_NMI_MAP = crate::Reg<u32, _INTERRUPT_CORE0_RWBT_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_RWBT_NMI_MAP;
#[doc = "`read()` method returns [interrupt_core0_rwbt_nmi_map::R](interrupt_core0_rwbt_nmi_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_RWBT_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_rwbt_nmi_map::W](interrupt_core0_rwbt_nmi_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_RWBT_NMI_MAP {}
#[doc = "INTERRUPT_CORE0_RWBT_NMI_MAP"]
pub mod interrupt_core0_rwbt_nmi_map;
#[doc = "INTERRUPT_CORE0_RWBLE_NMI_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_rwble_nmi_map](interrupt_core0_rwble_nmi_map) module"]
pub type INTERRUPT_CORE0_RWBLE_NMI_MAP = crate::Reg<u32, _INTERRUPT_CORE0_RWBLE_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_RWBLE_NMI_MAP;
#[doc = "`read()` method returns [interrupt_core0_rwble_nmi_map::R](interrupt_core0_rwble_nmi_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_RWBLE_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_rwble_nmi_map::W](interrupt_core0_rwble_nmi_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_RWBLE_NMI_MAP {}
#[doc = "INTERRUPT_CORE0_RWBLE_NMI_MAP"]
pub mod interrupt_core0_rwble_nmi_map;
#[doc = "INTERRUPT_CORE0_I2C_MST_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_i2c_mst_int_map](interrupt_core0_i2c_mst_int_map) module"]
pub type INTERRUPT_CORE0_I2C_MST_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_I2C_MST_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_I2C_MST_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_i2c_mst_int_map::R](interrupt_core0_i2c_mst_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_I2C_MST_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_i2c_mst_int_map::W](interrupt_core0_i2c_mst_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_I2C_MST_INT_MAP {}
#[doc = "INTERRUPT_CORE0_I2C_MST_INT_MAP"]
pub mod interrupt_core0_i2c_mst_int_map;
#[doc = "INTERRUPT_CORE0_SLC0_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_slc0_intr_map](interrupt_core0_slc0_intr_map) module"]
pub type INTERRUPT_CORE0_SLC0_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_SLC0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SLC0_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_slc0_intr_map::R](interrupt_core0_slc0_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SLC0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_slc0_intr_map::W](interrupt_core0_slc0_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SLC0_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_SLC0_INTR_MAP"]
pub mod interrupt_core0_slc0_intr_map;
#[doc = "INTERRUPT_CORE0_SLC1_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_slc1_intr_map](interrupt_core0_slc1_intr_map) module"]
pub type INTERRUPT_CORE0_SLC1_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_SLC1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SLC1_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_slc1_intr_map::R](interrupt_core0_slc1_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SLC1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_slc1_intr_map::W](interrupt_core0_slc1_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SLC1_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_SLC1_INTR_MAP"]
pub mod interrupt_core0_slc1_intr_map;
#[doc = "INTERRUPT_CORE0_APB_CTRL_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_apb_ctrl_intr_map](interrupt_core0_apb_ctrl_intr_map) module"]
pub type INTERRUPT_CORE0_APB_CTRL_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_APB_CTRL_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_APB_CTRL_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_apb_ctrl_intr_map::R](interrupt_core0_apb_ctrl_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_APB_CTRL_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_apb_ctrl_intr_map::W](interrupt_core0_apb_ctrl_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_APB_CTRL_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_APB_CTRL_INTR_MAP"]
pub mod interrupt_core0_apb_ctrl_intr_map;
#[doc = "INTERRUPT_CORE0_UHCI0_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_uhci0_intr_map](interrupt_core0_uhci0_intr_map) module"]
pub type INTERRUPT_CORE0_UHCI0_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_UHCI0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_UHCI0_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_uhci0_intr_map::R](interrupt_core0_uhci0_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_UHCI0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_uhci0_intr_map::W](interrupt_core0_uhci0_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_UHCI0_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_UHCI0_INTR_MAP"]
pub mod interrupt_core0_uhci0_intr_map;
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_gpio_interrupt_pro_map](interrupt_core0_gpio_interrupt_pro_map) module"]
pub type INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP;
#[doc = "`read()` method returns [interrupt_core0_gpio_interrupt_pro_map::R](interrupt_core0_gpio_interrupt_pro_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_gpio_interrupt_pro_map::W](interrupt_core0_gpio_interrupt_pro_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP {}
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP"]
pub mod interrupt_core0_gpio_interrupt_pro_map;
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_gpio_interrupt_pro_nmi_map](interrupt_core0_gpio_interrupt_pro_nmi_map) module"]
pub type INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP;
#[doc = "`read()` method returns [interrupt_core0_gpio_interrupt_pro_nmi_map::R](interrupt_core0_gpio_interrupt_pro_nmi_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_gpio_interrupt_pro_nmi_map::W](interrupt_core0_gpio_interrupt_pro_nmi_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP {}
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP"]
pub mod interrupt_core0_gpio_interrupt_pro_nmi_map;
#[doc = "INTERRUPT_CORE0_SPI_INTR_1_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_spi_intr_1_map](interrupt_core0_spi_intr_1_map) module"]
pub type INTERRUPT_CORE0_SPI_INTR_1_MAP = crate::Reg<u32, _INTERRUPT_CORE0_SPI_INTR_1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SPI_INTR_1_MAP;
#[doc = "`read()` method returns [interrupt_core0_spi_intr_1_map::R](interrupt_core0_spi_intr_1_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SPI_INTR_1_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_spi_intr_1_map::W](interrupt_core0_spi_intr_1_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SPI_INTR_1_MAP {}
#[doc = "INTERRUPT_CORE0_SPI_INTR_1_MAP"]
pub mod interrupt_core0_spi_intr_1_map;
#[doc = "INTERRUPT_CORE0_SPI_INTR_2_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_spi_intr_2_map](interrupt_core0_spi_intr_2_map) module"]
pub type INTERRUPT_CORE0_SPI_INTR_2_MAP = crate::Reg<u32, _INTERRUPT_CORE0_SPI_INTR_2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SPI_INTR_2_MAP;
#[doc = "`read()` method returns [interrupt_core0_spi_intr_2_map::R](interrupt_core0_spi_intr_2_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SPI_INTR_2_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_spi_intr_2_map::W](interrupt_core0_spi_intr_2_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SPI_INTR_2_MAP {}
#[doc = "INTERRUPT_CORE0_SPI_INTR_2_MAP"]
pub mod interrupt_core0_spi_intr_2_map;
#[doc = "INTERRUPT_CORE0_I2S1_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_i2s1_int_map](interrupt_core0_i2s1_int_map) module"]
pub type INTERRUPT_CORE0_I2S1_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_I2S1_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_I2S1_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_i2s1_int_map::R](interrupt_core0_i2s1_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_I2S1_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_i2s1_int_map::W](interrupt_core0_i2s1_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_I2S1_INT_MAP {}
#[doc = "INTERRUPT_CORE0_I2S1_INT_MAP"]
pub mod interrupt_core0_i2s1_int_map;
#[doc = "INTERRUPT_CORE0_UART_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_uart_intr_map](interrupt_core0_uart_intr_map) module"]
pub type INTERRUPT_CORE0_UART_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_UART_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_UART_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_uart_intr_map::R](interrupt_core0_uart_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_UART_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_uart_intr_map::W](interrupt_core0_uart_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_UART_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_UART_INTR_MAP"]
pub mod interrupt_core0_uart_intr_map;
#[doc = "INTERRUPT_CORE0_UART1_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_uart1_intr_map](interrupt_core0_uart1_intr_map) module"]
pub type INTERRUPT_CORE0_UART1_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_UART1_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_UART1_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_uart1_intr_map::R](interrupt_core0_uart1_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_UART1_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_uart1_intr_map::W](interrupt_core0_uart1_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_UART1_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_UART1_INTR_MAP"]
pub mod interrupt_core0_uart1_intr_map;
#[doc = "INTERRUPT_CORE0_LEDC_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_ledc_int_map](interrupt_core0_ledc_int_map) module"]
pub type INTERRUPT_CORE0_LEDC_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_LEDC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_LEDC_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_ledc_int_map::R](interrupt_core0_ledc_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_LEDC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_ledc_int_map::W](interrupt_core0_ledc_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_LEDC_INT_MAP {}
#[doc = "INTERRUPT_CORE0_LEDC_INT_MAP"]
pub mod interrupt_core0_ledc_int_map;
#[doc = "INTERRUPT_CORE0_EFUSE_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_efuse_int_map](interrupt_core0_efuse_int_map) module"]
pub type INTERRUPT_CORE0_EFUSE_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_EFUSE_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_EFUSE_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_efuse_int_map::R](interrupt_core0_efuse_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_EFUSE_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_efuse_int_map::W](interrupt_core0_efuse_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_EFUSE_INT_MAP {}
#[doc = "INTERRUPT_CORE0_EFUSE_INT_MAP"]
pub mod interrupt_core0_efuse_int_map;
#[doc = "INTERRUPT_CORE0_CAN_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_can_int_map](interrupt_core0_can_int_map) module"]
pub type INTERRUPT_CORE0_CAN_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_CAN_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CAN_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_can_int_map::R](interrupt_core0_can_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CAN_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_can_int_map::W](interrupt_core0_can_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CAN_INT_MAP {}
#[doc = "INTERRUPT_CORE0_CAN_INT_MAP"]
pub mod interrupt_core0_can_int_map;
#[doc = "INTERRUPT_CORE0_USB_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_usb_intr_map](interrupt_core0_usb_intr_map) module"]
pub type INTERRUPT_CORE0_USB_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_USB_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_USB_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_usb_intr_map::R](interrupt_core0_usb_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_USB_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_usb_intr_map::W](interrupt_core0_usb_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_USB_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_USB_INTR_MAP"]
pub mod interrupt_core0_usb_intr_map;
#[doc = "INTERRUPT_CORE0_RTC_CORE_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_rtc_core_intr_map](interrupt_core0_rtc_core_intr_map) module"]
pub type INTERRUPT_CORE0_RTC_CORE_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_RTC_CORE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_RTC_CORE_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_rtc_core_intr_map::R](interrupt_core0_rtc_core_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_RTC_CORE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_rtc_core_intr_map::W](interrupt_core0_rtc_core_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_RTC_CORE_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_RTC_CORE_INTR_MAP"]
pub mod interrupt_core0_rtc_core_intr_map;
#[doc = "INTERRUPT_CORE0_RMT_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_rmt_intr_map](interrupt_core0_rmt_intr_map) module"]
pub type INTERRUPT_CORE0_RMT_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_RMT_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_RMT_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_rmt_intr_map::R](interrupt_core0_rmt_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_RMT_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_rmt_intr_map::W](interrupt_core0_rmt_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_RMT_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_RMT_INTR_MAP"]
pub mod interrupt_core0_rmt_intr_map;
#[doc = "INTERRUPT_CORE0_I2C_EXT0_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_i2c_ext0_intr_map](interrupt_core0_i2c_ext0_intr_map) module"]
pub type INTERRUPT_CORE0_I2C_EXT0_INTR_MAP = crate::Reg<u32, _INTERRUPT_CORE0_I2C_EXT0_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_I2C_EXT0_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_i2c_ext0_intr_map::R](interrupt_core0_i2c_ext0_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_I2C_EXT0_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_i2c_ext0_intr_map::W](interrupt_core0_i2c_ext0_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_I2C_EXT0_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_I2C_EXT0_INTR_MAP"]
pub mod interrupt_core0_i2c_ext0_intr_map;
#[doc = "INTERRUPT_CORE0_TIMER_INT1_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_timer_int1_map](interrupt_core0_timer_int1_map) module"]
pub type INTERRUPT_CORE0_TIMER_INT1_MAP = crate::Reg<u32, _INTERRUPT_CORE0_TIMER_INT1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_TIMER_INT1_MAP;
#[doc = "`read()` method returns [interrupt_core0_timer_int1_map::R](interrupt_core0_timer_int1_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_TIMER_INT1_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_timer_int1_map::W](interrupt_core0_timer_int1_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_TIMER_INT1_MAP {}
#[doc = "INTERRUPT_CORE0_TIMER_INT1_MAP"]
pub mod interrupt_core0_timer_int1_map;
#[doc = "INTERRUPT_CORE0_TIMER_INT2_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_timer_int2_map](interrupt_core0_timer_int2_map) module"]
pub type INTERRUPT_CORE0_TIMER_INT2_MAP = crate::Reg<u32, _INTERRUPT_CORE0_TIMER_INT2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_TIMER_INT2_MAP;
#[doc = "`read()` method returns [interrupt_core0_timer_int2_map::R](interrupt_core0_timer_int2_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_TIMER_INT2_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_timer_int2_map::W](interrupt_core0_timer_int2_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_TIMER_INT2_MAP {}
#[doc = "INTERRUPT_CORE0_TIMER_INT2_MAP"]
pub mod interrupt_core0_timer_int2_map;
#[doc = "INTERRUPT_CORE0_TG_T0_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_tg_t0_int_map](interrupt_core0_tg_t0_int_map) module"]
pub type INTERRUPT_CORE0_TG_T0_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_TG_T0_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_TG_T0_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_tg_t0_int_map::R](interrupt_core0_tg_t0_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_TG_T0_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_tg_t0_int_map::W](interrupt_core0_tg_t0_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_TG_T0_INT_MAP {}
#[doc = "INTERRUPT_CORE0_TG_T0_INT_MAP"]
pub mod interrupt_core0_tg_t0_int_map;
#[doc = "INTERRUPT_CORE0_TG_WDT_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_tg_wdt_int_map](interrupt_core0_tg_wdt_int_map) module"]
pub type INTERRUPT_CORE0_TG_WDT_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_TG_WDT_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_TG_WDT_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_tg_wdt_int_map::R](interrupt_core0_tg_wdt_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_TG_WDT_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_tg_wdt_int_map::W](interrupt_core0_tg_wdt_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_TG_WDT_INT_MAP {}
#[doc = "INTERRUPT_CORE0_TG_WDT_INT_MAP"]
pub mod interrupt_core0_tg_wdt_int_map;
#[doc = "INTERRUPT_CORE0_TG1_T0_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_tg1_t0_int_map](interrupt_core0_tg1_t0_int_map) module"]
pub type INTERRUPT_CORE0_TG1_T0_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_TG1_T0_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_TG1_T0_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_tg1_t0_int_map::R](interrupt_core0_tg1_t0_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_TG1_T0_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_tg1_t0_int_map::W](interrupt_core0_tg1_t0_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_TG1_T0_INT_MAP {}
#[doc = "INTERRUPT_CORE0_TG1_T0_INT_MAP"]
pub mod interrupt_core0_tg1_t0_int_map;
#[doc = "INTERRUPT_CORE0_TG1_WDT_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_tg1_wdt_int_map](interrupt_core0_tg1_wdt_int_map) module"]
pub type INTERRUPT_CORE0_TG1_WDT_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_TG1_WDT_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_TG1_WDT_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_tg1_wdt_int_map::R](interrupt_core0_tg1_wdt_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_TG1_WDT_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_tg1_wdt_int_map::W](interrupt_core0_tg1_wdt_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_TG1_WDT_INT_MAP {}
#[doc = "INTERRUPT_CORE0_TG1_WDT_INT_MAP"]
pub mod interrupt_core0_tg1_wdt_int_map;
#[doc = "INTERRUPT_CORE0_CACHE_IA_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cache_ia_int_map](interrupt_core0_cache_ia_int_map) module"]
pub type INTERRUPT_CORE0_CACHE_IA_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_CACHE_IA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CACHE_IA_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_cache_ia_int_map::R](interrupt_core0_cache_ia_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CACHE_IA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cache_ia_int_map::W](interrupt_core0_cache_ia_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CACHE_IA_INT_MAP {}
#[doc = "INTERRUPT_CORE0_CACHE_IA_INT_MAP"]
pub mod interrupt_core0_cache_ia_int_map;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_systimer_target0_int_map](interrupt_core0_systimer_target0_int_map) module"]
pub type INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_systimer_target0_int_map::R](interrupt_core0_systimer_target0_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_systimer_target0_int_map::W](interrupt_core0_systimer_target0_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP {}
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP"]
pub mod interrupt_core0_systimer_target0_int_map;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_systimer_target1_int_map](interrupt_core0_systimer_target1_int_map) module"]
pub type INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_systimer_target1_int_map::R](interrupt_core0_systimer_target1_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_systimer_target1_int_map::W](interrupt_core0_systimer_target1_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP {}
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP"]
pub mod interrupt_core0_systimer_target1_int_map;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_systimer_target2_int_map](interrupt_core0_systimer_target2_int_map) module"]
pub type INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_systimer_target2_int_map::R](interrupt_core0_systimer_target2_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_systimer_target2_int_map::W](interrupt_core0_systimer_target2_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP {}
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP"]
pub mod interrupt_core0_systimer_target2_int_map;
#[doc = "INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_spi_mem_reject_intr_map](interrupt_core0_spi_mem_reject_intr_map) module"]
pub type INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_spi_mem_reject_intr_map::R](interrupt_core0_spi_mem_reject_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_spi_mem_reject_intr_map::W](interrupt_core0_spi_mem_reject_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP"]
pub mod interrupt_core0_spi_mem_reject_intr_map;
#[doc = "INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_icache_preload_int_map](interrupt_core0_icache_preload_int_map) module"]
pub type INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_icache_preload_int_map::R](interrupt_core0_icache_preload_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_icache_preload_int_map::W](interrupt_core0_icache_preload_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP {}
#[doc = "INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP"]
pub mod interrupt_core0_icache_preload_int_map;
#[doc = "INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_icache_sync_int_map](interrupt_core0_icache_sync_int_map) module"]
pub type INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_icache_sync_int_map::R](interrupt_core0_icache_sync_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_icache_sync_int_map::W](interrupt_core0_icache_sync_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP {}
#[doc = "INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP"]
pub mod interrupt_core0_icache_sync_int_map;
#[doc = "INTERRUPT_CORE0_APB_ADC_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_apb_adc_int_map](interrupt_core0_apb_adc_int_map) module"]
pub type INTERRUPT_CORE0_APB_ADC_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_APB_ADC_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_APB_ADC_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_apb_adc_int_map::R](interrupt_core0_apb_adc_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_APB_ADC_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_apb_adc_int_map::W](interrupt_core0_apb_adc_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_APB_ADC_INT_MAP {}
#[doc = "INTERRUPT_CORE0_APB_ADC_INT_MAP"]
pub mod interrupt_core0_apb_adc_int_map;
#[doc = "INTERRUPT_CORE0_DMA_CH0_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_dma_ch0_int_map](interrupt_core0_dma_ch0_int_map) module"]
pub type INTERRUPT_CORE0_DMA_CH0_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_DMA_CH0_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_DMA_CH0_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_dma_ch0_int_map::R](interrupt_core0_dma_ch0_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_DMA_CH0_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_dma_ch0_int_map::W](interrupt_core0_dma_ch0_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_DMA_CH0_INT_MAP {}
#[doc = "INTERRUPT_CORE0_DMA_CH0_INT_MAP"]
pub mod interrupt_core0_dma_ch0_int_map;
#[doc = "INTERRUPT_CORE0_DMA_CH1_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_dma_ch1_int_map](interrupt_core0_dma_ch1_int_map) module"]
pub type INTERRUPT_CORE0_DMA_CH1_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_DMA_CH1_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_DMA_CH1_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_dma_ch1_int_map::R](interrupt_core0_dma_ch1_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_DMA_CH1_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_dma_ch1_int_map::W](interrupt_core0_dma_ch1_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_DMA_CH1_INT_MAP {}
#[doc = "INTERRUPT_CORE0_DMA_CH1_INT_MAP"]
pub mod interrupt_core0_dma_ch1_int_map;
#[doc = "INTERRUPT_CORE0_DMA_CH2_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_dma_ch2_int_map](interrupt_core0_dma_ch2_int_map) module"]
pub type INTERRUPT_CORE0_DMA_CH2_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_DMA_CH2_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_DMA_CH2_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_dma_ch2_int_map::R](interrupt_core0_dma_ch2_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_DMA_CH2_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_dma_ch2_int_map::W](interrupt_core0_dma_ch2_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_DMA_CH2_INT_MAP {}
#[doc = "INTERRUPT_CORE0_DMA_CH2_INT_MAP"]
pub mod interrupt_core0_dma_ch2_int_map;
#[doc = "INTERRUPT_CORE0_RSA_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_rsa_int_map](interrupt_core0_rsa_int_map) module"]
pub type INTERRUPT_CORE0_RSA_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_RSA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_RSA_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_rsa_int_map::R](interrupt_core0_rsa_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_RSA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_rsa_int_map::W](interrupt_core0_rsa_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_RSA_INT_MAP {}
#[doc = "INTERRUPT_CORE0_RSA_INT_MAP"]
pub mod interrupt_core0_rsa_int_map;
#[doc = "INTERRUPT_CORE0_AES_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_aes_int_map](interrupt_core0_aes_int_map) module"]
pub type INTERRUPT_CORE0_AES_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_AES_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_AES_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_aes_int_map::R](interrupt_core0_aes_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_AES_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_aes_int_map::W](interrupt_core0_aes_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_AES_INT_MAP {}
#[doc = "INTERRUPT_CORE0_AES_INT_MAP"]
pub mod interrupt_core0_aes_int_map;
#[doc = "INTERRUPT_CORE0_SHA_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_sha_int_map](interrupt_core0_sha_int_map) module"]
pub type INTERRUPT_CORE0_SHA_INT_MAP = crate::Reg<u32, _INTERRUPT_CORE0_SHA_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_SHA_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_sha_int_map::R](interrupt_core0_sha_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_SHA_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_sha_int_map::W](interrupt_core0_sha_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_SHA_INT_MAP {}
#[doc = "INTERRUPT_CORE0_SHA_INT_MAP"]
pub mod interrupt_core0_sha_int_map;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_intr_from_cpu_0_map](interrupt_core0_cpu_intr_from_cpu_0_map) module"]
pub type INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP;
#[doc = "`read()` method returns [interrupt_core0_cpu_intr_from_cpu_0_map::R](interrupt_core0_cpu_intr_from_cpu_0_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_intr_from_cpu_0_map::W](interrupt_core0_cpu_intr_from_cpu_0_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP {}
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP"]
pub mod interrupt_core0_cpu_intr_from_cpu_0_map;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_intr_from_cpu_1_map](interrupt_core0_cpu_intr_from_cpu_1_map) module"]
pub type INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP;
#[doc = "`read()` method returns [interrupt_core0_cpu_intr_from_cpu_1_map::R](interrupt_core0_cpu_intr_from_cpu_1_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_intr_from_cpu_1_map::W](interrupt_core0_cpu_intr_from_cpu_1_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP {}
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP"]
pub mod interrupt_core0_cpu_intr_from_cpu_1_map;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_intr_from_cpu_2_map](interrupt_core0_cpu_intr_from_cpu_2_map) module"]
pub type INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP;
#[doc = "`read()` method returns [interrupt_core0_cpu_intr_from_cpu_2_map::R](interrupt_core0_cpu_intr_from_cpu_2_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_intr_from_cpu_2_map::W](interrupt_core0_cpu_intr_from_cpu_2_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP {}
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP"]
pub mod interrupt_core0_cpu_intr_from_cpu_2_map;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_intr_from_cpu_3_map](interrupt_core0_cpu_intr_from_cpu_3_map) module"]
pub type INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP;
#[doc = "`read()` method returns [interrupt_core0_cpu_intr_from_cpu_3_map::R](interrupt_core0_cpu_intr_from_cpu_3_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_intr_from_cpu_3_map::W](interrupt_core0_cpu_intr_from_cpu_3_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP {}
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP"]
pub mod interrupt_core0_cpu_intr_from_cpu_3_map;
#[doc = "INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_assist_debug_intr_map](interrupt_core0_assist_debug_intr_map) module"]
pub type INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_assist_debug_intr_map::R](interrupt_core0_assist_debug_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_assist_debug_intr_map::W](interrupt_core0_assist_debug_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP"]
pub mod interrupt_core0_assist_debug_intr_map;
#[doc = "INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map](interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map) module"]
pub type INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map::R](interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map::W](interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map](interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map) module"]
pub type INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map::R](interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map::W](interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map](interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map) module"]
pub type INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map::R](interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map::W](interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_core_0_pif_pms_monitor_violate_intr_map](interrupt_core0_core_0_pif_pms_monitor_violate_intr_map) module"]
pub type INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_core_0_pif_pms_monitor_violate_intr_map::R](interrupt_core0_core_0_pif_pms_monitor_violate_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_core_0_pif_pms_monitor_violate_intr_map::W](interrupt_core0_core_0_pif_pms_monitor_violate_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_core_0_pif_pms_monitor_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map](interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map) module"]
pub type INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map::R](interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map::W](interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP"]
pub mod interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map;
#[doc = "INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_backup_pms_violate_intr_map](interrupt_core0_backup_pms_violate_intr_map) module"]
pub type INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP;
#[doc = "`read()` method returns [interrupt_core0_backup_pms_violate_intr_map::R](interrupt_core0_backup_pms_violate_intr_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_backup_pms_violate_intr_map::W](interrupt_core0_backup_pms_violate_intr_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP {}
#[doc = "INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_backup_pms_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cache_core0_acs_int_map](interrupt_core0_cache_core0_acs_int_map) module"]
pub type INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP =
    crate::Reg<u32, _INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP;
#[doc = "`read()` method returns [interrupt_core0_cache_core0_acs_int_map::R](interrupt_core0_cache_core0_acs_int_map::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cache_core0_acs_int_map::W](interrupt_core0_cache_core0_acs_int_map::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP {}
#[doc = "INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP"]
pub mod interrupt_core0_cache_core0_acs_int_map;
#[doc = "INTERRUPT_CORE0_INTR_STATUS_0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_intr_status_0](interrupt_core0_intr_status_0) module"]
pub type INTERRUPT_CORE0_INTR_STATUS_0 = crate::Reg<u32, _INTERRUPT_CORE0_INTR_STATUS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_INTR_STATUS_0;
#[doc = "`read()` method returns [interrupt_core0_intr_status_0::R](interrupt_core0_intr_status_0::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_INTR_STATUS_0 {}
#[doc = "INTERRUPT_CORE0_INTR_STATUS_0"]
pub mod interrupt_core0_intr_status_0;
#[doc = "INTERRUPT_CORE0_INTR_STATUS_1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_intr_status_1](interrupt_core0_intr_status_1) module"]
pub type INTERRUPT_CORE0_INTR_STATUS_1 = crate::Reg<u32, _INTERRUPT_CORE0_INTR_STATUS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_INTR_STATUS_1;
#[doc = "`read()` method returns [interrupt_core0_intr_status_1::R](interrupt_core0_intr_status_1::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_INTR_STATUS_1 {}
#[doc = "INTERRUPT_CORE0_INTR_STATUS_1"]
pub mod interrupt_core0_intr_status_1;
#[doc = "INTERRUPT_CORE0_CLOCK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_clock_gate](interrupt_core0_clock_gate) module"]
pub type INTERRUPT_CORE0_CLOCK_GATE = crate::Reg<u32, _INTERRUPT_CORE0_CLOCK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CLOCK_GATE;
#[doc = "`read()` method returns [interrupt_core0_clock_gate::R](interrupt_core0_clock_gate::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CLOCK_GATE {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_clock_gate::W](interrupt_core0_clock_gate::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CLOCK_GATE {}
#[doc = "INTERRUPT_CORE0_CLOCK_GATE"]
pub mod interrupt_core0_clock_gate;
#[doc = "INTERRUPT_CORE0_CPU_INT_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_enable](interrupt_core0_cpu_int_enable) module"]
pub type INTERRUPT_CORE0_CPU_INT_ENABLE = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_ENABLE;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_enable::R](interrupt_core0_cpu_int_enable::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_enable::W](interrupt_core0_cpu_int_enable::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_ENABLE {}
#[doc = "INTERRUPT_CORE0_CPU_INT_ENABLE"]
pub mod interrupt_core0_cpu_int_enable;
#[doc = "INTERRUPT_CORE0_CPU_INT_TYPE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_type](interrupt_core0_cpu_int_type) module"]
pub type INTERRUPT_CORE0_CPU_INT_TYPE = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_TYPE;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_type::R](interrupt_core0_cpu_int_type::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_TYPE {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_type::W](interrupt_core0_cpu_int_type::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_TYPE {}
#[doc = "INTERRUPT_CORE0_CPU_INT_TYPE"]
pub mod interrupt_core0_cpu_int_type;
#[doc = "INTERRUPT_CORE0_CPU_INT_CLEAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_clear](interrupt_core0_cpu_int_clear) module"]
pub type INTERRUPT_CORE0_CPU_INT_CLEAR = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_CLEAR;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_clear::R](interrupt_core0_cpu_int_clear::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_CLEAR {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_clear::W](interrupt_core0_cpu_int_clear::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_CLEAR {}
#[doc = "INTERRUPT_CORE0_CPU_INT_CLEAR"]
pub mod interrupt_core0_cpu_int_clear;
#[doc = "INTERRUPT_CORE0_CPU_INT_EIP_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_eip_status](interrupt_core0_cpu_int_eip_status) module"]
pub type INTERRUPT_CORE0_CPU_INT_EIP_STATUS = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_EIP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_EIP_STATUS;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_eip_status::R](interrupt_core0_cpu_int_eip_status::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_EIP_STATUS {}
#[doc = "INTERRUPT_CORE0_CPU_INT_EIP_STATUS"]
pub mod interrupt_core0_cpu_int_eip_status;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_0](interrupt_core0_cpu_int_pri_0) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_0 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_0;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_0::R](interrupt_core0_cpu_int_pri_0::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_0 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_0::W](interrupt_core0_cpu_int_pri_0::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_0 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_0"]
pub mod interrupt_core0_cpu_int_pri_0;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_1](interrupt_core0_cpu_int_pri_1) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_1 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_1;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_1::R](interrupt_core0_cpu_int_pri_1::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_1 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_1::W](interrupt_core0_cpu_int_pri_1::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_1 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_1"]
pub mod interrupt_core0_cpu_int_pri_1;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_2](interrupt_core0_cpu_int_pri_2) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_2 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_2;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_2::R](interrupt_core0_cpu_int_pri_2::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_2 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_2::W](interrupt_core0_cpu_int_pri_2::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_2 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_2"]
pub mod interrupt_core0_cpu_int_pri_2;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_3](interrupt_core0_cpu_int_pri_3) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_3 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_3;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_3::R](interrupt_core0_cpu_int_pri_3::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_3 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_3::W](interrupt_core0_cpu_int_pri_3::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_3 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_3"]
pub mod interrupt_core0_cpu_int_pri_3;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_4](interrupt_core0_cpu_int_pri_4) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_4 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_4;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_4::R](interrupt_core0_cpu_int_pri_4::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_4 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_4::W](interrupt_core0_cpu_int_pri_4::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_4 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_4"]
pub mod interrupt_core0_cpu_int_pri_4;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_5](interrupt_core0_cpu_int_pri_5) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_5 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_5;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_5::R](interrupt_core0_cpu_int_pri_5::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_5 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_5::W](interrupt_core0_cpu_int_pri_5::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_5 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_5"]
pub mod interrupt_core0_cpu_int_pri_5;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_6](interrupt_core0_cpu_int_pri_6) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_6 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_6;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_6::R](interrupt_core0_cpu_int_pri_6::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_6 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_6::W](interrupt_core0_cpu_int_pri_6::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_6 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_6"]
pub mod interrupt_core0_cpu_int_pri_6;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_7](interrupt_core0_cpu_int_pri_7) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_7 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_7;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_7::R](interrupt_core0_cpu_int_pri_7::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_7 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_7::W](interrupt_core0_cpu_int_pri_7::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_7 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_7"]
pub mod interrupt_core0_cpu_int_pri_7;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_8](interrupt_core0_cpu_int_pri_8) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_8 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_8;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_8::R](interrupt_core0_cpu_int_pri_8::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_8 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_8::W](interrupt_core0_cpu_int_pri_8::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_8 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_8"]
pub mod interrupt_core0_cpu_int_pri_8;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_9](interrupt_core0_cpu_int_pri_9) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_9 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_9;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_9::R](interrupt_core0_cpu_int_pri_9::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_9 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_9::W](interrupt_core0_cpu_int_pri_9::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_9 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_9"]
pub mod interrupt_core0_cpu_int_pri_9;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_10](interrupt_core0_cpu_int_pri_10) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_10 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_10;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_10::R](interrupt_core0_cpu_int_pri_10::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_10 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_10::W](interrupt_core0_cpu_int_pri_10::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_10 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_10"]
pub mod interrupt_core0_cpu_int_pri_10;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_11](interrupt_core0_cpu_int_pri_11) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_11 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_11;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_11::R](interrupt_core0_cpu_int_pri_11::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_11 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_11::W](interrupt_core0_cpu_int_pri_11::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_11 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_11"]
pub mod interrupt_core0_cpu_int_pri_11;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_12](interrupt_core0_cpu_int_pri_12) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_12 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_12;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_12::R](interrupt_core0_cpu_int_pri_12::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_12 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_12::W](interrupt_core0_cpu_int_pri_12::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_12 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_12"]
pub mod interrupt_core0_cpu_int_pri_12;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_13](interrupt_core0_cpu_int_pri_13) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_13 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_13;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_13::R](interrupt_core0_cpu_int_pri_13::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_13 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_13::W](interrupt_core0_cpu_int_pri_13::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_13 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_13"]
pub mod interrupt_core0_cpu_int_pri_13;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_14](interrupt_core0_cpu_int_pri_14) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_14 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_14;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_14::R](interrupt_core0_cpu_int_pri_14::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_14 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_14::W](interrupt_core0_cpu_int_pri_14::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_14 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_14"]
pub mod interrupt_core0_cpu_int_pri_14;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_15](interrupt_core0_cpu_int_pri_15) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_15 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_15;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_15::R](interrupt_core0_cpu_int_pri_15::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_15 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_15::W](interrupt_core0_cpu_int_pri_15::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_15 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_15"]
pub mod interrupt_core0_cpu_int_pri_15;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_16](interrupt_core0_cpu_int_pri_16) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_16 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_16;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_16::R](interrupt_core0_cpu_int_pri_16::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_16 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_16::W](interrupt_core0_cpu_int_pri_16::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_16 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_16"]
pub mod interrupt_core0_cpu_int_pri_16;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_17](interrupt_core0_cpu_int_pri_17) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_17 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_17;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_17::R](interrupt_core0_cpu_int_pri_17::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_17 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_17::W](interrupt_core0_cpu_int_pri_17::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_17 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_17"]
pub mod interrupt_core0_cpu_int_pri_17;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_18](interrupt_core0_cpu_int_pri_18) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_18 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_18;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_18::R](interrupt_core0_cpu_int_pri_18::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_18 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_18::W](interrupt_core0_cpu_int_pri_18::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_18 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_18"]
pub mod interrupt_core0_cpu_int_pri_18;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_19](interrupt_core0_cpu_int_pri_19) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_19 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_19;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_19::R](interrupt_core0_cpu_int_pri_19::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_19 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_19::W](interrupt_core0_cpu_int_pri_19::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_19 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_19"]
pub mod interrupt_core0_cpu_int_pri_19;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_20](interrupt_core0_cpu_int_pri_20) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_20 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_20;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_20::R](interrupt_core0_cpu_int_pri_20::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_20 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_20::W](interrupt_core0_cpu_int_pri_20::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_20 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_20"]
pub mod interrupt_core0_cpu_int_pri_20;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_21](interrupt_core0_cpu_int_pri_21) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_21 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_21;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_21::R](interrupt_core0_cpu_int_pri_21::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_21 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_21::W](interrupt_core0_cpu_int_pri_21::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_21 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_21"]
pub mod interrupt_core0_cpu_int_pri_21;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_22](interrupt_core0_cpu_int_pri_22) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_22 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_22;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_22::R](interrupt_core0_cpu_int_pri_22::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_22 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_22::W](interrupt_core0_cpu_int_pri_22::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_22 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_22"]
pub mod interrupt_core0_cpu_int_pri_22;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_23](interrupt_core0_cpu_int_pri_23) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_23 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_23;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_23::R](interrupt_core0_cpu_int_pri_23::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_23 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_23::W](interrupt_core0_cpu_int_pri_23::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_23 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_23"]
pub mod interrupt_core0_cpu_int_pri_23;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_24](interrupt_core0_cpu_int_pri_24) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_24 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_24;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_24::R](interrupt_core0_cpu_int_pri_24::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_24 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_24::W](interrupt_core0_cpu_int_pri_24::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_24 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_24"]
pub mod interrupt_core0_cpu_int_pri_24;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_25](interrupt_core0_cpu_int_pri_25) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_25 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_25;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_25::R](interrupt_core0_cpu_int_pri_25::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_25 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_25::W](interrupt_core0_cpu_int_pri_25::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_25 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_25"]
pub mod interrupt_core0_cpu_int_pri_25;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_26](interrupt_core0_cpu_int_pri_26) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_26 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_26;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_26::R](interrupt_core0_cpu_int_pri_26::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_26 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_26::W](interrupt_core0_cpu_int_pri_26::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_26 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_26"]
pub mod interrupt_core0_cpu_int_pri_26;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_27](interrupt_core0_cpu_int_pri_27) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_27 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_27;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_27::R](interrupt_core0_cpu_int_pri_27::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_27 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_27::W](interrupt_core0_cpu_int_pri_27::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_27 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_27"]
pub mod interrupt_core0_cpu_int_pri_27;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_28](interrupt_core0_cpu_int_pri_28) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_28 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_28;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_28::R](interrupt_core0_cpu_int_pri_28::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_28 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_28::W](interrupt_core0_cpu_int_pri_28::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_28 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_28"]
pub mod interrupt_core0_cpu_int_pri_28;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_29](interrupt_core0_cpu_int_pri_29) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_29 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_29;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_29::R](interrupt_core0_cpu_int_pri_29::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_29 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_29::W](interrupt_core0_cpu_int_pri_29::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_29 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_29"]
pub mod interrupt_core0_cpu_int_pri_29;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_30](interrupt_core0_cpu_int_pri_30) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_30 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_30;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_30::R](interrupt_core0_cpu_int_pri_30::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_30 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_30::W](interrupt_core0_cpu_int_pri_30::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_30 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_30"]
pub mod interrupt_core0_cpu_int_pri_30;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_pri_31](interrupt_core0_cpu_int_pri_31) module"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_31 = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_PRI_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_PRI_31;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_pri_31::R](interrupt_core0_cpu_int_pri_31::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_PRI_31 {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_pri_31::W](interrupt_core0_cpu_int_pri_31::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_PRI_31 {}
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_31"]
pub mod interrupt_core0_cpu_int_pri_31;
#[doc = "INTERRUPT_CORE0_CPU_INT_THRESH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_thresh](interrupt_core0_cpu_int_thresh) module"]
pub type INTERRUPT_CORE0_CPU_INT_THRESH = crate::Reg<u32, _INTERRUPT_CORE0_CPU_INT_THRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_CPU_INT_THRESH;
#[doc = "`read()` method returns [interrupt_core0_cpu_int_thresh::R](interrupt_core0_cpu_int_thresh::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_THRESH {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_cpu_int_thresh::W](interrupt_core0_cpu_int_thresh::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CPU_INT_THRESH {}
#[doc = "INTERRUPT_CORE0_CPU_INT_THRESH"]
pub mod interrupt_core0_cpu_int_thresh;
#[doc = "INTERRUPT_CORE0_INTERRUPT_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_interrupt_date](interrupt_core0_interrupt_date) module"]
pub type INTERRUPT_CORE0_INTERRUPT_DATE = crate::Reg<u32, _INTERRUPT_CORE0_INTERRUPT_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CORE0_INTERRUPT_DATE;
#[doc = "`read()` method returns [interrupt_core0_interrupt_date::R](interrupt_core0_interrupt_date::R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_INTERRUPT_DATE {}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_interrupt_date::W](interrupt_core0_interrupt_date::W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_INTERRUPT_DATE {}
#[doc = "INTERRUPT_CORE0_INTERRUPT_DATE"]
pub mod interrupt_core0_interrupt_date;
