#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_BT_SELECT"]
    pub gpio_bt_select: GPIO_BT_SELECT,
    #[doc = "0x04 - GPIO_OUT"]
    pub gpio_out: GPIO_OUT,
    #[doc = "0x08 - GPIO_OUT_W1TS"]
    pub gpio_out_w1ts: GPIO_OUT_W1TS,
    #[doc = "0x0c - GPIO_OUT_W1TC"]
    pub gpio_out_w1tc: GPIO_OUT_W1TC,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - GPIO_SDIO_SELECT"]
    pub gpio_sdio_select: GPIO_SDIO_SELECT,
    #[doc = "0x20 - GPIO_ENABLE"]
    pub gpio_enable: GPIO_ENABLE,
    #[doc = "0x24 - GPIO_ENABLE_W1TS"]
    pub gpio_enable_w1ts: GPIO_ENABLE_W1TS,
    #[doc = "0x28 - GPIO_ENABLE_W1TC"]
    pub gpio_enable_w1tc: GPIO_ENABLE_W1TC,
    _reserved8: [u8; 12usize],
    #[doc = "0x38 - GPIO_STRAP"]
    pub gpio_strap: GPIO_STRAP,
    #[doc = "0x3c - GPIO_IN"]
    pub gpio_in: GPIO_IN,
    _reserved10: [u8; 4usize],
    #[doc = "0x44 - GPIO_STATUS"]
    pub gpio_status: GPIO_STATUS,
    #[doc = "0x48 - GPIO_STATUS_W1TS"]
    pub gpio_status_w1ts: GPIO_STATUS_W1TS,
    #[doc = "0x4c - GPIO_STATUS_W1TC"]
    pub gpio_status_w1tc: GPIO_STATUS_W1TC,
    _reserved13: [u8; 12usize],
    #[doc = "0x5c - GPIO_PCPU_INT"]
    pub gpio_pcpu_int: GPIO_PCPU_INT,
    #[doc = "0x60 - GPIO_PCPU_NMI_INT"]
    pub gpio_pcpu_nmi_int: GPIO_PCPU_NMI_INT,
    #[doc = "0x64 - GPIO_CPUSDIO_INT"]
    pub gpio_cpusdio_int: GPIO_CPUSDIO_INT,
    _reserved16: [u8; 12usize],
    #[doc = "0x74 - GPIO_PIN0"]
    pub gpio_pin0: GPIO_PIN0,
    #[doc = "0x78 - GPIO_PIN1"]
    pub gpio_pin1: GPIO_PIN1,
    #[doc = "0x7c - GPIO_PIN2"]
    pub gpio_pin2: GPIO_PIN2,
    #[doc = "0x80 - GPIO_PIN3"]
    pub gpio_pin3: GPIO_PIN3,
    #[doc = "0x84 - GPIO_PIN4"]
    pub gpio_pin4: GPIO_PIN4,
    #[doc = "0x88 - GPIO_PIN5"]
    pub gpio_pin5: GPIO_PIN5,
    #[doc = "0x8c - GPIO_PIN6"]
    pub gpio_pin6: GPIO_PIN6,
    #[doc = "0x90 - GPIO_PIN7"]
    pub gpio_pin7: GPIO_PIN7,
    #[doc = "0x94 - GPIO_PIN8"]
    pub gpio_pin8: GPIO_PIN8,
    #[doc = "0x98 - GPIO_PIN9"]
    pub gpio_pin9: GPIO_PIN9,
    #[doc = "0x9c - GPIO_PIN10"]
    pub gpio_pin10: GPIO_PIN10,
    #[doc = "0xa0 - GPIO_PIN11"]
    pub gpio_pin11: GPIO_PIN11,
    #[doc = "0xa4 - GPIO_PIN12"]
    pub gpio_pin12: GPIO_PIN12,
    #[doc = "0xa8 - GPIO_PIN13"]
    pub gpio_pin13: GPIO_PIN13,
    #[doc = "0xac - GPIO_PIN14"]
    pub gpio_pin14: GPIO_PIN14,
    #[doc = "0xb0 - GPIO_PIN15"]
    pub gpio_pin15: GPIO_PIN15,
    #[doc = "0xb4 - GPIO_PIN16"]
    pub gpio_pin16: GPIO_PIN16,
    #[doc = "0xb8 - GPIO_PIN17"]
    pub gpio_pin17: GPIO_PIN17,
    #[doc = "0xbc - GPIO_PIN18"]
    pub gpio_pin18: GPIO_PIN18,
    #[doc = "0xc0 - GPIO_PIN19"]
    pub gpio_pin19: GPIO_PIN19,
    #[doc = "0xc4 - GPIO_PIN20"]
    pub gpio_pin20: GPIO_PIN20,
    #[doc = "0xc8 - GPIO_PIN21"]
    pub gpio_pin21: GPIO_PIN21,
    #[doc = "0xcc - GPIO_PIN22"]
    pub gpio_pin22: GPIO_PIN22,
    #[doc = "0xd0 - GPIO_PIN23"]
    pub gpio_pin23: GPIO_PIN23,
    #[doc = "0xd4 - GPIO_PIN24"]
    pub gpio_pin24: GPIO_PIN24,
    #[doc = "0xd8 - GPIO_PIN25"]
    pub gpio_pin25: GPIO_PIN25,
    _reserved42: [u8; 112usize],
    #[doc = "0x14c - GPIO_STATUS_NEXT"]
    pub gpio_status_next: GPIO_STATUS_NEXT,
    _reserved43: [u8; 4usize],
    #[doc = "0x154 - GPIO_FUNC0_IN_SEL_CFG"]
    pub gpio_func0_in_sel_cfg: GPIO_FUNC0_IN_SEL_CFG,
    #[doc = "0x158 - GPIO_FUNC1_IN_SEL_CFG"]
    pub gpio_func1_in_sel_cfg: GPIO_FUNC1_IN_SEL_CFG,
    #[doc = "0x15c - GPIO_FUNC2_IN_SEL_CFG"]
    pub gpio_func2_in_sel_cfg: GPIO_FUNC2_IN_SEL_CFG,
    #[doc = "0x160 - GPIO_FUNC3_IN_SEL_CFG"]
    pub gpio_func3_in_sel_cfg: GPIO_FUNC3_IN_SEL_CFG,
    #[doc = "0x164 - GPIO_FUNC4_IN_SEL_CFG"]
    pub gpio_func4_in_sel_cfg: GPIO_FUNC4_IN_SEL_CFG,
    #[doc = "0x168 - GPIO_FUNC5_IN_SEL_CFG"]
    pub gpio_func5_in_sel_cfg: GPIO_FUNC5_IN_SEL_CFG,
    #[doc = "0x16c - GPIO_FUNC6_IN_SEL_CFG"]
    pub gpio_func6_in_sel_cfg: GPIO_FUNC6_IN_SEL_CFG,
    #[doc = "0x170 - GPIO_FUNC7_IN_SEL_CFG"]
    pub gpio_func7_in_sel_cfg: GPIO_FUNC7_IN_SEL_CFG,
    #[doc = "0x174 - GPIO_FUNC8_IN_SEL_CFG"]
    pub gpio_func8_in_sel_cfg: GPIO_FUNC8_IN_SEL_CFG,
    #[doc = "0x178 - GPIO_FUNC9_IN_SEL_CFG"]
    pub gpio_func9_in_sel_cfg: GPIO_FUNC9_IN_SEL_CFG,
    #[doc = "0x17c - GPIO_FUNC10_IN_SEL_CFG"]
    pub gpio_func10_in_sel_cfg: GPIO_FUNC10_IN_SEL_CFG,
    #[doc = "0x180 - GPIO_FUNC11_IN_SEL_CFG"]
    pub gpio_func11_in_sel_cfg: GPIO_FUNC11_IN_SEL_CFG,
    #[doc = "0x184 - GPIO_FUNC12_IN_SEL_CFG"]
    pub gpio_func12_in_sel_cfg: GPIO_FUNC12_IN_SEL_CFG,
    #[doc = "0x188 - GPIO_FUNC13_IN_SEL_CFG"]
    pub gpio_func13_in_sel_cfg: GPIO_FUNC13_IN_SEL_CFG,
    #[doc = "0x18c - GPIO_FUNC14_IN_SEL_CFG"]
    pub gpio_func14_in_sel_cfg: GPIO_FUNC14_IN_SEL_CFG,
    #[doc = "0x190 - GPIO_FUNC15_IN_SEL_CFG"]
    pub gpio_func15_in_sel_cfg: GPIO_FUNC15_IN_SEL_CFG,
    #[doc = "0x194 - GPIO_FUNC16_IN_SEL_CFG"]
    pub gpio_func16_in_sel_cfg: GPIO_FUNC16_IN_SEL_CFG,
    #[doc = "0x198 - GPIO_FUNC17_IN_SEL_CFG"]
    pub gpio_func17_in_sel_cfg: GPIO_FUNC17_IN_SEL_CFG,
    #[doc = "0x19c - GPIO_FUNC18_IN_SEL_CFG"]
    pub gpio_func18_in_sel_cfg: GPIO_FUNC18_IN_SEL_CFG,
    #[doc = "0x1a0 - GPIO_FUNC19_IN_SEL_CFG"]
    pub gpio_func19_in_sel_cfg: GPIO_FUNC19_IN_SEL_CFG,
    #[doc = "0x1a4 - GPIO_FUNC20_IN_SEL_CFG"]
    pub gpio_func20_in_sel_cfg: GPIO_FUNC20_IN_SEL_CFG,
    #[doc = "0x1a8 - GPIO_FUNC21_IN_SEL_CFG"]
    pub gpio_func21_in_sel_cfg: GPIO_FUNC21_IN_SEL_CFG,
    #[doc = "0x1ac - GPIO_FUNC22_IN_SEL_CFG"]
    pub gpio_func22_in_sel_cfg: GPIO_FUNC22_IN_SEL_CFG,
    #[doc = "0x1b0 - GPIO_FUNC23_IN_SEL_CFG"]
    pub gpio_func23_in_sel_cfg: GPIO_FUNC23_IN_SEL_CFG,
    #[doc = "0x1b4 - GPIO_FUNC24_IN_SEL_CFG"]
    pub gpio_func24_in_sel_cfg: GPIO_FUNC24_IN_SEL_CFG,
    #[doc = "0x1b8 - GPIO_FUNC25_IN_SEL_CFG"]
    pub gpio_func25_in_sel_cfg: GPIO_FUNC25_IN_SEL_CFG,
    #[doc = "0x1bc - GPIO_FUNC26_IN_SEL_CFG"]
    pub gpio_func26_in_sel_cfg: GPIO_FUNC26_IN_SEL_CFG,
    #[doc = "0x1c0 - GPIO_FUNC27_IN_SEL_CFG"]
    pub gpio_func27_in_sel_cfg: GPIO_FUNC27_IN_SEL_CFG,
    #[doc = "0x1c4 - GPIO_FUNC28_IN_SEL_CFG"]
    pub gpio_func28_in_sel_cfg: GPIO_FUNC28_IN_SEL_CFG,
    #[doc = "0x1c8 - GPIO_FUNC29_IN_SEL_CFG"]
    pub gpio_func29_in_sel_cfg: GPIO_FUNC29_IN_SEL_CFG,
    #[doc = "0x1cc - GPIO_FUNC30_IN_SEL_CFG"]
    pub gpio_func30_in_sel_cfg: GPIO_FUNC30_IN_SEL_CFG,
    #[doc = "0x1d0 - GPIO_FUNC31_IN_SEL_CFG"]
    pub gpio_func31_in_sel_cfg: GPIO_FUNC31_IN_SEL_CFG,
    #[doc = "0x1d4 - GPIO_FUNC32_IN_SEL_CFG"]
    pub gpio_func32_in_sel_cfg: GPIO_FUNC32_IN_SEL_CFG,
    #[doc = "0x1d8 - GPIO_FUNC33_IN_SEL_CFG"]
    pub gpio_func33_in_sel_cfg: GPIO_FUNC33_IN_SEL_CFG,
    #[doc = "0x1dc - GPIO_FUNC34_IN_SEL_CFG"]
    pub gpio_func34_in_sel_cfg: GPIO_FUNC34_IN_SEL_CFG,
    #[doc = "0x1e0 - GPIO_FUNC35_IN_SEL_CFG"]
    pub gpio_func35_in_sel_cfg: GPIO_FUNC35_IN_SEL_CFG,
    #[doc = "0x1e4 - GPIO_FUNC36_IN_SEL_CFG"]
    pub gpio_func36_in_sel_cfg: GPIO_FUNC36_IN_SEL_CFG,
    #[doc = "0x1e8 - GPIO_FUNC37_IN_SEL_CFG"]
    pub gpio_func37_in_sel_cfg: GPIO_FUNC37_IN_SEL_CFG,
    #[doc = "0x1ec - GPIO_FUNC38_IN_SEL_CFG"]
    pub gpio_func38_in_sel_cfg: GPIO_FUNC38_IN_SEL_CFG,
    #[doc = "0x1f0 - GPIO_FUNC39_IN_SEL_CFG"]
    pub gpio_func39_in_sel_cfg: GPIO_FUNC39_IN_SEL_CFG,
    #[doc = "0x1f4 - GPIO_FUNC40_IN_SEL_CFG"]
    pub gpio_func40_in_sel_cfg: GPIO_FUNC40_IN_SEL_CFG,
    #[doc = "0x1f8 - GPIO_FUNC41_IN_SEL_CFG"]
    pub gpio_func41_in_sel_cfg: GPIO_FUNC41_IN_SEL_CFG,
    #[doc = "0x1fc - GPIO_FUNC42_IN_SEL_CFG"]
    pub gpio_func42_in_sel_cfg: GPIO_FUNC42_IN_SEL_CFG,
    #[doc = "0x200 - GPIO_FUNC43_IN_SEL_CFG"]
    pub gpio_func43_in_sel_cfg: GPIO_FUNC43_IN_SEL_CFG,
    #[doc = "0x204 - GPIO_FUNC44_IN_SEL_CFG"]
    pub gpio_func44_in_sel_cfg: GPIO_FUNC44_IN_SEL_CFG,
    #[doc = "0x208 - GPIO_FUNC45_IN_SEL_CFG"]
    pub gpio_func45_in_sel_cfg: GPIO_FUNC45_IN_SEL_CFG,
    #[doc = "0x20c - GPIO_FUNC46_IN_SEL_CFG"]
    pub gpio_func46_in_sel_cfg: GPIO_FUNC46_IN_SEL_CFG,
    #[doc = "0x210 - GPIO_FUNC47_IN_SEL_CFG"]
    pub gpio_func47_in_sel_cfg: GPIO_FUNC47_IN_SEL_CFG,
    #[doc = "0x214 - GPIO_FUNC48_IN_SEL_CFG"]
    pub gpio_func48_in_sel_cfg: GPIO_FUNC48_IN_SEL_CFG,
    #[doc = "0x218 - GPIO_FUNC49_IN_SEL_CFG"]
    pub gpio_func49_in_sel_cfg: GPIO_FUNC49_IN_SEL_CFG,
    #[doc = "0x21c - GPIO_FUNC50_IN_SEL_CFG"]
    pub gpio_func50_in_sel_cfg: GPIO_FUNC50_IN_SEL_CFG,
    #[doc = "0x220 - GPIO_FUNC51_IN_SEL_CFG"]
    pub gpio_func51_in_sel_cfg: GPIO_FUNC51_IN_SEL_CFG,
    #[doc = "0x224 - GPIO_FUNC52_IN_SEL_CFG"]
    pub gpio_func52_in_sel_cfg: GPIO_FUNC52_IN_SEL_CFG,
    #[doc = "0x228 - GPIO_FUNC53_IN_SEL_CFG"]
    pub gpio_func53_in_sel_cfg: GPIO_FUNC53_IN_SEL_CFG,
    #[doc = "0x22c - GPIO_FUNC54_IN_SEL_CFG"]
    pub gpio_func54_in_sel_cfg: GPIO_FUNC54_IN_SEL_CFG,
    #[doc = "0x230 - GPIO_FUNC55_IN_SEL_CFG"]
    pub gpio_func55_in_sel_cfg: GPIO_FUNC55_IN_SEL_CFG,
    #[doc = "0x234 - GPIO_FUNC56_IN_SEL_CFG"]
    pub gpio_func56_in_sel_cfg: GPIO_FUNC56_IN_SEL_CFG,
    #[doc = "0x238 - GPIO_FUNC57_IN_SEL_CFG"]
    pub gpio_func57_in_sel_cfg: GPIO_FUNC57_IN_SEL_CFG,
    #[doc = "0x23c - GPIO_FUNC58_IN_SEL_CFG"]
    pub gpio_func58_in_sel_cfg: GPIO_FUNC58_IN_SEL_CFG,
    #[doc = "0x240 - GPIO_FUNC59_IN_SEL_CFG"]
    pub gpio_func59_in_sel_cfg: GPIO_FUNC59_IN_SEL_CFG,
    #[doc = "0x244 - GPIO_FUNC60_IN_SEL_CFG"]
    pub gpio_func60_in_sel_cfg: GPIO_FUNC60_IN_SEL_CFG,
    #[doc = "0x248 - GPIO_FUNC61_IN_SEL_CFG"]
    pub gpio_func61_in_sel_cfg: GPIO_FUNC61_IN_SEL_CFG,
    #[doc = "0x24c - GPIO_FUNC62_IN_SEL_CFG"]
    pub gpio_func62_in_sel_cfg: GPIO_FUNC62_IN_SEL_CFG,
    #[doc = "0x250 - GPIO_FUNC63_IN_SEL_CFG"]
    pub gpio_func63_in_sel_cfg: GPIO_FUNC63_IN_SEL_CFG,
    #[doc = "0x254 - GPIO_FUNC64_IN_SEL_CFG"]
    pub gpio_func64_in_sel_cfg: GPIO_FUNC64_IN_SEL_CFG,
    #[doc = "0x258 - GPIO_FUNC65_IN_SEL_CFG"]
    pub gpio_func65_in_sel_cfg: GPIO_FUNC65_IN_SEL_CFG,
    #[doc = "0x25c - GPIO_FUNC66_IN_SEL_CFG"]
    pub gpio_func66_in_sel_cfg: GPIO_FUNC66_IN_SEL_CFG,
    #[doc = "0x260 - GPIO_FUNC67_IN_SEL_CFG"]
    pub gpio_func67_in_sel_cfg: GPIO_FUNC67_IN_SEL_CFG,
    #[doc = "0x264 - GPIO_FUNC68_IN_SEL_CFG"]
    pub gpio_func68_in_sel_cfg: GPIO_FUNC68_IN_SEL_CFG,
    #[doc = "0x268 - GPIO_FUNC69_IN_SEL_CFG"]
    pub gpio_func69_in_sel_cfg: GPIO_FUNC69_IN_SEL_CFG,
    #[doc = "0x26c - GPIO_FUNC70_IN_SEL_CFG"]
    pub gpio_func70_in_sel_cfg: GPIO_FUNC70_IN_SEL_CFG,
    #[doc = "0x270 - GPIO_FUNC71_IN_SEL_CFG"]
    pub gpio_func71_in_sel_cfg: GPIO_FUNC71_IN_SEL_CFG,
    #[doc = "0x274 - GPIO_FUNC72_IN_SEL_CFG"]
    pub gpio_func72_in_sel_cfg: GPIO_FUNC72_IN_SEL_CFG,
    #[doc = "0x278 - GPIO_FUNC73_IN_SEL_CFG"]
    pub gpio_func73_in_sel_cfg: GPIO_FUNC73_IN_SEL_CFG,
    #[doc = "0x27c - GPIO_FUNC74_IN_SEL_CFG"]
    pub gpio_func74_in_sel_cfg: GPIO_FUNC74_IN_SEL_CFG,
    #[doc = "0x280 - GPIO_FUNC75_IN_SEL_CFG"]
    pub gpio_func75_in_sel_cfg: GPIO_FUNC75_IN_SEL_CFG,
    #[doc = "0x284 - GPIO_FUNC76_IN_SEL_CFG"]
    pub gpio_func76_in_sel_cfg: GPIO_FUNC76_IN_SEL_CFG,
    #[doc = "0x288 - GPIO_FUNC77_IN_SEL_CFG"]
    pub gpio_func77_in_sel_cfg: GPIO_FUNC77_IN_SEL_CFG,
    #[doc = "0x28c - GPIO_FUNC78_IN_SEL_CFG"]
    pub gpio_func78_in_sel_cfg: GPIO_FUNC78_IN_SEL_CFG,
    #[doc = "0x290 - GPIO_FUNC79_IN_SEL_CFG"]
    pub gpio_func79_in_sel_cfg: GPIO_FUNC79_IN_SEL_CFG,
    #[doc = "0x294 - GPIO_FUNC80_IN_SEL_CFG"]
    pub gpio_func80_in_sel_cfg: GPIO_FUNC80_IN_SEL_CFG,
    #[doc = "0x298 - GPIO_FUNC81_IN_SEL_CFG"]
    pub gpio_func81_in_sel_cfg: GPIO_FUNC81_IN_SEL_CFG,
    #[doc = "0x29c - GPIO_FUNC82_IN_SEL_CFG"]
    pub gpio_func82_in_sel_cfg: GPIO_FUNC82_IN_SEL_CFG,
    #[doc = "0x2a0 - GPIO_FUNC83_IN_SEL_CFG"]
    pub gpio_func83_in_sel_cfg: GPIO_FUNC83_IN_SEL_CFG,
    #[doc = "0x2a4 - GPIO_FUNC84_IN_SEL_CFG"]
    pub gpio_func84_in_sel_cfg: GPIO_FUNC84_IN_SEL_CFG,
    #[doc = "0x2a8 - GPIO_FUNC85_IN_SEL_CFG"]
    pub gpio_func85_in_sel_cfg: GPIO_FUNC85_IN_SEL_CFG,
    #[doc = "0x2ac - GPIO_FUNC86_IN_SEL_CFG"]
    pub gpio_func86_in_sel_cfg: GPIO_FUNC86_IN_SEL_CFG,
    #[doc = "0x2b0 - GPIO_FUNC87_IN_SEL_CFG"]
    pub gpio_func87_in_sel_cfg: GPIO_FUNC87_IN_SEL_CFG,
    #[doc = "0x2b4 - GPIO_FUNC88_IN_SEL_CFG"]
    pub gpio_func88_in_sel_cfg: GPIO_FUNC88_IN_SEL_CFG,
    #[doc = "0x2b8 - GPIO_FUNC89_IN_SEL_CFG"]
    pub gpio_func89_in_sel_cfg: GPIO_FUNC89_IN_SEL_CFG,
    #[doc = "0x2bc - GPIO_FUNC90_IN_SEL_CFG"]
    pub gpio_func90_in_sel_cfg: GPIO_FUNC90_IN_SEL_CFG,
    #[doc = "0x2c0 - GPIO_FUNC91_IN_SEL_CFG"]
    pub gpio_func91_in_sel_cfg: GPIO_FUNC91_IN_SEL_CFG,
    #[doc = "0x2c4 - GPIO_FUNC92_IN_SEL_CFG"]
    pub gpio_func92_in_sel_cfg: GPIO_FUNC92_IN_SEL_CFG,
    #[doc = "0x2c8 - GPIO_FUNC93_IN_SEL_CFG"]
    pub gpio_func93_in_sel_cfg: GPIO_FUNC93_IN_SEL_CFG,
    #[doc = "0x2cc - GPIO_FUNC94_IN_SEL_CFG"]
    pub gpio_func94_in_sel_cfg: GPIO_FUNC94_IN_SEL_CFG,
    #[doc = "0x2d0 - GPIO_FUNC95_IN_SEL_CFG"]
    pub gpio_func95_in_sel_cfg: GPIO_FUNC95_IN_SEL_CFG,
    #[doc = "0x2d4 - GPIO_FUNC96_IN_SEL_CFG"]
    pub gpio_func96_in_sel_cfg: GPIO_FUNC96_IN_SEL_CFG,
    #[doc = "0x2d8 - GPIO_FUNC97_IN_SEL_CFG"]
    pub gpio_func97_in_sel_cfg: GPIO_FUNC97_IN_SEL_CFG,
    #[doc = "0x2dc - GPIO_FUNC98_IN_SEL_CFG"]
    pub gpio_func98_in_sel_cfg: GPIO_FUNC98_IN_SEL_CFG,
    #[doc = "0x2e0 - GPIO_FUNC99_IN_SEL_CFG"]
    pub gpio_func99_in_sel_cfg: GPIO_FUNC99_IN_SEL_CFG,
    #[doc = "0x2e4 - GPIO_FUNC100_IN_SEL_CFG"]
    pub gpio_func100_in_sel_cfg: GPIO_FUNC100_IN_SEL_CFG,
    #[doc = "0x2e8 - GPIO_FUNC101_IN_SEL_CFG"]
    pub gpio_func101_in_sel_cfg: GPIO_FUNC101_IN_SEL_CFG,
    #[doc = "0x2ec - GPIO_FUNC102_IN_SEL_CFG"]
    pub gpio_func102_in_sel_cfg: GPIO_FUNC102_IN_SEL_CFG,
    #[doc = "0x2f0 - GPIO_FUNC103_IN_SEL_CFG"]
    pub gpio_func103_in_sel_cfg: GPIO_FUNC103_IN_SEL_CFG,
    #[doc = "0x2f4 - GPIO_FUNC104_IN_SEL_CFG"]
    pub gpio_func104_in_sel_cfg: GPIO_FUNC104_IN_SEL_CFG,
    #[doc = "0x2f8 - GPIO_FUNC105_IN_SEL_CFG"]
    pub gpio_func105_in_sel_cfg: GPIO_FUNC105_IN_SEL_CFG,
    #[doc = "0x2fc - GPIO_FUNC106_IN_SEL_CFG"]
    pub gpio_func106_in_sel_cfg: GPIO_FUNC106_IN_SEL_CFG,
    #[doc = "0x300 - GPIO_FUNC107_IN_SEL_CFG"]
    pub gpio_func107_in_sel_cfg: GPIO_FUNC107_IN_SEL_CFG,
    #[doc = "0x304 - GPIO_FUNC108_IN_SEL_CFG"]
    pub gpio_func108_in_sel_cfg: GPIO_FUNC108_IN_SEL_CFG,
    #[doc = "0x308 - GPIO_FUNC109_IN_SEL_CFG"]
    pub gpio_func109_in_sel_cfg: GPIO_FUNC109_IN_SEL_CFG,
    #[doc = "0x30c - GPIO_FUNC110_IN_SEL_CFG"]
    pub gpio_func110_in_sel_cfg: GPIO_FUNC110_IN_SEL_CFG,
    #[doc = "0x310 - GPIO_FUNC111_IN_SEL_CFG"]
    pub gpio_func111_in_sel_cfg: GPIO_FUNC111_IN_SEL_CFG,
    #[doc = "0x314 - GPIO_FUNC112_IN_SEL_CFG"]
    pub gpio_func112_in_sel_cfg: GPIO_FUNC112_IN_SEL_CFG,
    #[doc = "0x318 - GPIO_FUNC113_IN_SEL_CFG"]
    pub gpio_func113_in_sel_cfg: GPIO_FUNC113_IN_SEL_CFG,
    #[doc = "0x31c - GPIO_FUNC114_IN_SEL_CFG"]
    pub gpio_func114_in_sel_cfg: GPIO_FUNC114_IN_SEL_CFG,
    #[doc = "0x320 - GPIO_FUNC115_IN_SEL_CFG"]
    pub gpio_func115_in_sel_cfg: GPIO_FUNC115_IN_SEL_CFG,
    #[doc = "0x324 - GPIO_FUNC116_IN_SEL_CFG"]
    pub gpio_func116_in_sel_cfg: GPIO_FUNC116_IN_SEL_CFG,
    #[doc = "0x328 - GPIO_FUNC117_IN_SEL_CFG"]
    pub gpio_func117_in_sel_cfg: GPIO_FUNC117_IN_SEL_CFG,
    #[doc = "0x32c - GPIO_FUNC118_IN_SEL_CFG"]
    pub gpio_func118_in_sel_cfg: GPIO_FUNC118_IN_SEL_CFG,
    #[doc = "0x330 - GPIO_FUNC119_IN_SEL_CFG"]
    pub gpio_func119_in_sel_cfg: GPIO_FUNC119_IN_SEL_CFG,
    #[doc = "0x334 - GPIO_FUNC120_IN_SEL_CFG"]
    pub gpio_func120_in_sel_cfg: GPIO_FUNC120_IN_SEL_CFG,
    #[doc = "0x338 - GPIO_FUNC121_IN_SEL_CFG"]
    pub gpio_func121_in_sel_cfg: GPIO_FUNC121_IN_SEL_CFG,
    #[doc = "0x33c - GPIO_FUNC122_IN_SEL_CFG"]
    pub gpio_func122_in_sel_cfg: GPIO_FUNC122_IN_SEL_CFG,
    #[doc = "0x340 - GPIO_FUNC123_IN_SEL_CFG"]
    pub gpio_func123_in_sel_cfg: GPIO_FUNC123_IN_SEL_CFG,
    #[doc = "0x344 - GPIO_FUNC124_IN_SEL_CFG"]
    pub gpio_func124_in_sel_cfg: GPIO_FUNC124_IN_SEL_CFG,
    #[doc = "0x348 - GPIO_FUNC125_IN_SEL_CFG"]
    pub gpio_func125_in_sel_cfg: GPIO_FUNC125_IN_SEL_CFG,
    #[doc = "0x34c - GPIO_FUNC126_IN_SEL_CFG"]
    pub gpio_func126_in_sel_cfg: GPIO_FUNC126_IN_SEL_CFG,
    #[doc = "0x350 - GPIO_FUNC127_IN_SEL_CFG"]
    pub gpio_func127_in_sel_cfg: GPIO_FUNC127_IN_SEL_CFG,
    _reserved171: [u8; 512usize],
    #[doc = "0x554 - GPIO_FUNC0_OUT_SEL_CFG"]
    pub gpio_func0_out_sel_cfg: GPIO_FUNC0_OUT_SEL_CFG,
    #[doc = "0x558 - GPIO_FUNC1_OUT_SEL_CFG"]
    pub gpio_func1_out_sel_cfg: GPIO_FUNC1_OUT_SEL_CFG,
    #[doc = "0x55c - GPIO_FUNC2_OUT_SEL_CFG"]
    pub gpio_func2_out_sel_cfg: GPIO_FUNC2_OUT_SEL_CFG,
    #[doc = "0x560 - GPIO_FUNC3_OUT_SEL_CFG"]
    pub gpio_func3_out_sel_cfg: GPIO_FUNC3_OUT_SEL_CFG,
    #[doc = "0x564 - GPIO_FUNC4_OUT_SEL_CFG"]
    pub gpio_func4_out_sel_cfg: GPIO_FUNC4_OUT_SEL_CFG,
    #[doc = "0x568 - GPIO_FUNC5_OUT_SEL_CFG"]
    pub gpio_func5_out_sel_cfg: GPIO_FUNC5_OUT_SEL_CFG,
    #[doc = "0x56c - GPIO_FUNC6_OUT_SEL_CFG"]
    pub gpio_func6_out_sel_cfg: GPIO_FUNC6_OUT_SEL_CFG,
    #[doc = "0x570 - GPIO_FUNC7_OUT_SEL_CFG"]
    pub gpio_func7_out_sel_cfg: GPIO_FUNC7_OUT_SEL_CFG,
    #[doc = "0x574 - GPIO_FUNC8_OUT_SEL_CFG"]
    pub gpio_func8_out_sel_cfg: GPIO_FUNC8_OUT_SEL_CFG,
    #[doc = "0x578 - GPIO_FUNC9_OUT_SEL_CFG"]
    pub gpio_func9_out_sel_cfg: GPIO_FUNC9_OUT_SEL_CFG,
    #[doc = "0x57c - GPIO_FUNC10_OUT_SEL_CFG"]
    pub gpio_func10_out_sel_cfg: GPIO_FUNC10_OUT_SEL_CFG,
    #[doc = "0x580 - GPIO_FUNC11_OUT_SEL_CFG"]
    pub gpio_func11_out_sel_cfg: GPIO_FUNC11_OUT_SEL_CFG,
    #[doc = "0x584 - GPIO_FUNC12_OUT_SEL_CFG"]
    pub gpio_func12_out_sel_cfg: GPIO_FUNC12_OUT_SEL_CFG,
    #[doc = "0x588 - GPIO_FUNC13_OUT_SEL_CFG"]
    pub gpio_func13_out_sel_cfg: GPIO_FUNC13_OUT_SEL_CFG,
    #[doc = "0x58c - GPIO_FUNC14_OUT_SEL_CFG"]
    pub gpio_func14_out_sel_cfg: GPIO_FUNC14_OUT_SEL_CFG,
    #[doc = "0x590 - GPIO_FUNC15_OUT_SEL_CFG"]
    pub gpio_func15_out_sel_cfg: GPIO_FUNC15_OUT_SEL_CFG,
    #[doc = "0x594 - GPIO_FUNC16_OUT_SEL_CFG"]
    pub gpio_func16_out_sel_cfg: GPIO_FUNC16_OUT_SEL_CFG,
    #[doc = "0x598 - GPIO_FUNC17_OUT_SEL_CFG"]
    pub gpio_func17_out_sel_cfg: GPIO_FUNC17_OUT_SEL_CFG,
    #[doc = "0x59c - GPIO_FUNC18_OUT_SEL_CFG"]
    pub gpio_func18_out_sel_cfg: GPIO_FUNC18_OUT_SEL_CFG,
    #[doc = "0x5a0 - GPIO_FUNC19_OUT_SEL_CFG"]
    pub gpio_func19_out_sel_cfg: GPIO_FUNC19_OUT_SEL_CFG,
    #[doc = "0x5a4 - GPIO_FUNC20_OUT_SEL_CFG"]
    pub gpio_func20_out_sel_cfg: GPIO_FUNC20_OUT_SEL_CFG,
    #[doc = "0x5a8 - GPIO_FUNC21_OUT_SEL_CFG"]
    pub gpio_func21_out_sel_cfg: GPIO_FUNC21_OUT_SEL_CFG,
    #[doc = "0x5ac - GPIO_FUNC22_OUT_SEL_CFG"]
    pub gpio_func22_out_sel_cfg: GPIO_FUNC22_OUT_SEL_CFG,
    #[doc = "0x5b0 - GPIO_FUNC23_OUT_SEL_CFG"]
    pub gpio_func23_out_sel_cfg: GPIO_FUNC23_OUT_SEL_CFG,
    #[doc = "0x5b4 - GPIO_FUNC24_OUT_SEL_CFG"]
    pub gpio_func24_out_sel_cfg: GPIO_FUNC24_OUT_SEL_CFG,
    #[doc = "0x5b8 - GPIO_FUNC25_OUT_SEL_CFG"]
    pub gpio_func25_out_sel_cfg: GPIO_FUNC25_OUT_SEL_CFG,
    _reserved197: [u8; 112usize],
    #[doc = "0x62c - GPIO_CLOCK_GATE"]
    pub gpio_clock_gate: GPIO_CLOCK_GATE,
    _reserved198: [u8; 204usize],
    #[doc = "0x6fc - GPIO_DATE"]
    pub gpio_date: GPIO_DATE,
}
#[doc = "GPIO_BT_SELECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_bt_select](gpio_bt_select) module"]
pub type GPIO_BT_SELECT = crate::Reg<u32, _GPIO_BT_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_BT_SELECT;
#[doc = "`read()` method returns [gpio_bt_select::R](gpio_bt_select::R) reader structure"]
impl crate::Readable for GPIO_BT_SELECT {}
#[doc = "`write(|w| ..)` method takes [gpio_bt_select::W](gpio_bt_select::W) writer structure"]
impl crate::Writable for GPIO_BT_SELECT {}
#[doc = "GPIO_BT_SELECT"]
pub mod gpio_bt_select;
#[doc = "GPIO_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out](gpio_out) module"]
pub type GPIO_OUT = crate::Reg<u32, _GPIO_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT;
#[doc = "`read()` method returns [gpio_out::R](gpio_out::R) reader structure"]
impl crate::Readable for GPIO_OUT {}
#[doc = "`write(|w| ..)` method takes [gpio_out::W](gpio_out::W) writer structure"]
impl crate::Writable for GPIO_OUT {}
#[doc = "GPIO_OUT"]
pub mod gpio_out;
#[doc = "GPIO_OUT_W1TS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out_w1ts](gpio_out_w1ts) module"]
pub type GPIO_OUT_W1TS = crate::Reg<u32, _GPIO_OUT_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT_W1TS;
#[doc = "`write(|w| ..)` method takes [gpio_out_w1ts::W](gpio_out_w1ts::W) writer structure"]
impl crate::Writable for GPIO_OUT_W1TS {}
#[doc = "GPIO_OUT_W1TS"]
pub mod gpio_out_w1ts;
#[doc = "GPIO_OUT_W1TC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out_w1tc](gpio_out_w1tc) module"]
pub type GPIO_OUT_W1TC = crate::Reg<u32, _GPIO_OUT_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT_W1TC;
#[doc = "`write(|w| ..)` method takes [gpio_out_w1tc::W](gpio_out_w1tc::W) writer structure"]
impl crate::Writable for GPIO_OUT_W1TC {}
#[doc = "GPIO_OUT_W1TC"]
pub mod gpio_out_w1tc;
#[doc = "GPIO_SDIO_SELECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_sdio_select](gpio_sdio_select) module"]
pub type GPIO_SDIO_SELECT = crate::Reg<u32, _GPIO_SDIO_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SDIO_SELECT;
#[doc = "`read()` method returns [gpio_sdio_select::R](gpio_sdio_select::R) reader structure"]
impl crate::Readable for GPIO_SDIO_SELECT {}
#[doc = "`write(|w| ..)` method takes [gpio_sdio_select::W](gpio_sdio_select::W) writer structure"]
impl crate::Writable for GPIO_SDIO_SELECT {}
#[doc = "GPIO_SDIO_SELECT"]
pub mod gpio_sdio_select;
#[doc = "GPIO_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_enable](gpio_enable) module"]
pub type GPIO_ENABLE = crate::Reg<u32, _GPIO_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE;
#[doc = "`read()` method returns [gpio_enable::R](gpio_enable::R) reader structure"]
impl crate::Readable for GPIO_ENABLE {}
#[doc = "`write(|w| ..)` method takes [gpio_enable::W](gpio_enable::W) writer structure"]
impl crate::Writable for GPIO_ENABLE {}
#[doc = "GPIO_ENABLE"]
pub mod gpio_enable;
#[doc = "GPIO_ENABLE_W1TS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_enable_w1ts](gpio_enable_w1ts) module"]
pub type GPIO_ENABLE_W1TS = crate::Reg<u32, _GPIO_ENABLE_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE_W1TS;
#[doc = "`write(|w| ..)` method takes [gpio_enable_w1ts::W](gpio_enable_w1ts::W) writer structure"]
impl crate::Writable for GPIO_ENABLE_W1TS {}
#[doc = "GPIO_ENABLE_W1TS"]
pub mod gpio_enable_w1ts;
#[doc = "GPIO_ENABLE_W1TC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_enable_w1tc](gpio_enable_w1tc) module"]
pub type GPIO_ENABLE_W1TC = crate::Reg<u32, _GPIO_ENABLE_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE_W1TC;
#[doc = "`write(|w| ..)` method takes [gpio_enable_w1tc::W](gpio_enable_w1tc::W) writer structure"]
impl crate::Writable for GPIO_ENABLE_W1TC {}
#[doc = "GPIO_ENABLE_W1TC"]
pub mod gpio_enable_w1tc;
#[doc = "GPIO_STRAP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_strap](gpio_strap) module"]
pub type GPIO_STRAP = crate::Reg<u32, _GPIO_STRAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STRAP;
#[doc = "`read()` method returns [gpio_strap::R](gpio_strap::R) reader structure"]
impl crate::Readable for GPIO_STRAP {}
#[doc = "GPIO_STRAP"]
pub mod gpio_strap;
#[doc = "GPIO_IN\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_in](gpio_in) module"]
pub type GPIO_IN = crate::Reg<u32, _GPIO_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_IN;
#[doc = "`read()` method returns [gpio_in::R](gpio_in::R) reader structure"]
impl crate::Readable for GPIO_IN {}
#[doc = "GPIO_IN"]
pub mod gpio_in;
#[doc = "GPIO_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status](gpio_status) module"]
pub type GPIO_STATUS = crate::Reg<u32, _GPIO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS;
#[doc = "`read()` method returns [gpio_status::R](gpio_status::R) reader structure"]
impl crate::Readable for GPIO_STATUS {}
#[doc = "`write(|w| ..)` method takes [gpio_status::W](gpio_status::W) writer structure"]
impl crate::Writable for GPIO_STATUS {}
#[doc = "GPIO_STATUS"]
pub mod gpio_status;
#[doc = "GPIO_STATUS_W1TS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status_w1ts](gpio_status_w1ts) module"]
pub type GPIO_STATUS_W1TS = crate::Reg<u32, _GPIO_STATUS_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS_W1TS;
#[doc = "`write(|w| ..)` method takes [gpio_status_w1ts::W](gpio_status_w1ts::W) writer structure"]
impl crate::Writable for GPIO_STATUS_W1TS {}
#[doc = "GPIO_STATUS_W1TS"]
pub mod gpio_status_w1ts;
#[doc = "GPIO_STATUS_W1TC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status_w1tc](gpio_status_w1tc) module"]
pub type GPIO_STATUS_W1TC = crate::Reg<u32, _GPIO_STATUS_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS_W1TC;
#[doc = "`write(|w| ..)` method takes [gpio_status_w1tc::W](gpio_status_w1tc::W) writer structure"]
impl crate::Writable for GPIO_STATUS_W1TC {}
#[doc = "GPIO_STATUS_W1TC"]
pub mod gpio_status_w1tc;
#[doc = "GPIO_PCPU_INT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pcpu_int](gpio_pcpu_int) module"]
pub type GPIO_PCPU_INT = crate::Reg<u32, _GPIO_PCPU_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PCPU_INT;
#[doc = "`read()` method returns [gpio_pcpu_int::R](gpio_pcpu_int::R) reader structure"]
impl crate::Readable for GPIO_PCPU_INT {}
#[doc = "GPIO_PCPU_INT"]
pub mod gpio_pcpu_int;
#[doc = "GPIO_PCPU_NMI_INT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pcpu_nmi_int](gpio_pcpu_nmi_int) module"]
pub type GPIO_PCPU_NMI_INT = crate::Reg<u32, _GPIO_PCPU_NMI_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PCPU_NMI_INT;
#[doc = "`read()` method returns [gpio_pcpu_nmi_int::R](gpio_pcpu_nmi_int::R) reader structure"]
impl crate::Readable for GPIO_PCPU_NMI_INT {}
#[doc = "GPIO_PCPU_NMI_INT"]
pub mod gpio_pcpu_nmi_int;
#[doc = "GPIO_CPUSDIO_INT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cpusdio_int](gpio_cpusdio_int) module"]
pub type GPIO_CPUSDIO_INT = crate::Reg<u32, _GPIO_CPUSDIO_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CPUSDIO_INT;
#[doc = "`read()` method returns [gpio_cpusdio_int::R](gpio_cpusdio_int::R) reader structure"]
impl crate::Readable for GPIO_CPUSDIO_INT {}
#[doc = "GPIO_CPUSDIO_INT"]
pub mod gpio_cpusdio_int;
#[doc = "GPIO_PIN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin0](gpio_pin0) module"]
pub type GPIO_PIN0 = crate::Reg<u32, _GPIO_PIN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN0;
#[doc = "`read()` method returns [gpio_pin0::R](gpio_pin0::R) reader structure"]
impl crate::Readable for GPIO_PIN0 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin0::W](gpio_pin0::W) writer structure"]
impl crate::Writable for GPIO_PIN0 {}
#[doc = "GPIO_PIN0"]
pub mod gpio_pin0;
#[doc = "GPIO_PIN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin1](gpio_pin1) module"]
pub type GPIO_PIN1 = crate::Reg<u32, _GPIO_PIN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN1;
#[doc = "`read()` method returns [gpio_pin1::R](gpio_pin1::R) reader structure"]
impl crate::Readable for GPIO_PIN1 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin1::W](gpio_pin1::W) writer structure"]
impl crate::Writable for GPIO_PIN1 {}
#[doc = "GPIO_PIN1"]
pub mod gpio_pin1;
#[doc = "GPIO_PIN2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin2](gpio_pin2) module"]
pub type GPIO_PIN2 = crate::Reg<u32, _GPIO_PIN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN2;
#[doc = "`read()` method returns [gpio_pin2::R](gpio_pin2::R) reader structure"]
impl crate::Readable for GPIO_PIN2 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin2::W](gpio_pin2::W) writer structure"]
impl crate::Writable for GPIO_PIN2 {}
#[doc = "GPIO_PIN2"]
pub mod gpio_pin2;
#[doc = "GPIO_PIN3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin3](gpio_pin3) module"]
pub type GPIO_PIN3 = crate::Reg<u32, _GPIO_PIN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN3;
#[doc = "`read()` method returns [gpio_pin3::R](gpio_pin3::R) reader structure"]
impl crate::Readable for GPIO_PIN3 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin3::W](gpio_pin3::W) writer structure"]
impl crate::Writable for GPIO_PIN3 {}
#[doc = "GPIO_PIN3"]
pub mod gpio_pin3;
#[doc = "GPIO_PIN4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin4](gpio_pin4) module"]
pub type GPIO_PIN4 = crate::Reg<u32, _GPIO_PIN4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN4;
#[doc = "`read()` method returns [gpio_pin4::R](gpio_pin4::R) reader structure"]
impl crate::Readable for GPIO_PIN4 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin4::W](gpio_pin4::W) writer structure"]
impl crate::Writable for GPIO_PIN4 {}
#[doc = "GPIO_PIN4"]
pub mod gpio_pin4;
#[doc = "GPIO_PIN5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin5](gpio_pin5) module"]
pub type GPIO_PIN5 = crate::Reg<u32, _GPIO_PIN5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN5;
#[doc = "`read()` method returns [gpio_pin5::R](gpio_pin5::R) reader structure"]
impl crate::Readable for GPIO_PIN5 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin5::W](gpio_pin5::W) writer structure"]
impl crate::Writable for GPIO_PIN5 {}
#[doc = "GPIO_PIN5"]
pub mod gpio_pin5;
#[doc = "GPIO_PIN6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin6](gpio_pin6) module"]
pub type GPIO_PIN6 = crate::Reg<u32, _GPIO_PIN6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN6;
#[doc = "`read()` method returns [gpio_pin6::R](gpio_pin6::R) reader structure"]
impl crate::Readable for GPIO_PIN6 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin6::W](gpio_pin6::W) writer structure"]
impl crate::Writable for GPIO_PIN6 {}
#[doc = "GPIO_PIN6"]
pub mod gpio_pin6;
#[doc = "GPIO_PIN7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin7](gpio_pin7) module"]
pub type GPIO_PIN7 = crate::Reg<u32, _GPIO_PIN7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN7;
#[doc = "`read()` method returns [gpio_pin7::R](gpio_pin7::R) reader structure"]
impl crate::Readable for GPIO_PIN7 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin7::W](gpio_pin7::W) writer structure"]
impl crate::Writable for GPIO_PIN7 {}
#[doc = "GPIO_PIN7"]
pub mod gpio_pin7;
#[doc = "GPIO_PIN8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin8](gpio_pin8) module"]
pub type GPIO_PIN8 = crate::Reg<u32, _GPIO_PIN8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN8;
#[doc = "`read()` method returns [gpio_pin8::R](gpio_pin8::R) reader structure"]
impl crate::Readable for GPIO_PIN8 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin8::W](gpio_pin8::W) writer structure"]
impl crate::Writable for GPIO_PIN8 {}
#[doc = "GPIO_PIN8"]
pub mod gpio_pin8;
#[doc = "GPIO_PIN9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin9](gpio_pin9) module"]
pub type GPIO_PIN9 = crate::Reg<u32, _GPIO_PIN9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN9;
#[doc = "`read()` method returns [gpio_pin9::R](gpio_pin9::R) reader structure"]
impl crate::Readable for GPIO_PIN9 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin9::W](gpio_pin9::W) writer structure"]
impl crate::Writable for GPIO_PIN9 {}
#[doc = "GPIO_PIN9"]
pub mod gpio_pin9;
#[doc = "GPIO_PIN10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin10](gpio_pin10) module"]
pub type GPIO_PIN10 = crate::Reg<u32, _GPIO_PIN10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN10;
#[doc = "`read()` method returns [gpio_pin10::R](gpio_pin10::R) reader structure"]
impl crate::Readable for GPIO_PIN10 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin10::W](gpio_pin10::W) writer structure"]
impl crate::Writable for GPIO_PIN10 {}
#[doc = "GPIO_PIN10"]
pub mod gpio_pin10;
#[doc = "GPIO_PIN11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin11](gpio_pin11) module"]
pub type GPIO_PIN11 = crate::Reg<u32, _GPIO_PIN11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN11;
#[doc = "`read()` method returns [gpio_pin11::R](gpio_pin11::R) reader structure"]
impl crate::Readable for GPIO_PIN11 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin11::W](gpio_pin11::W) writer structure"]
impl crate::Writable for GPIO_PIN11 {}
#[doc = "GPIO_PIN11"]
pub mod gpio_pin11;
#[doc = "GPIO_PIN12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin12](gpio_pin12) module"]
pub type GPIO_PIN12 = crate::Reg<u32, _GPIO_PIN12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN12;
#[doc = "`read()` method returns [gpio_pin12::R](gpio_pin12::R) reader structure"]
impl crate::Readable for GPIO_PIN12 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin12::W](gpio_pin12::W) writer structure"]
impl crate::Writable for GPIO_PIN12 {}
#[doc = "GPIO_PIN12"]
pub mod gpio_pin12;
#[doc = "GPIO_PIN13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin13](gpio_pin13) module"]
pub type GPIO_PIN13 = crate::Reg<u32, _GPIO_PIN13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN13;
#[doc = "`read()` method returns [gpio_pin13::R](gpio_pin13::R) reader structure"]
impl crate::Readable for GPIO_PIN13 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin13::W](gpio_pin13::W) writer structure"]
impl crate::Writable for GPIO_PIN13 {}
#[doc = "GPIO_PIN13"]
pub mod gpio_pin13;
#[doc = "GPIO_PIN14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin14](gpio_pin14) module"]
pub type GPIO_PIN14 = crate::Reg<u32, _GPIO_PIN14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN14;
#[doc = "`read()` method returns [gpio_pin14::R](gpio_pin14::R) reader structure"]
impl crate::Readable for GPIO_PIN14 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin14::W](gpio_pin14::W) writer structure"]
impl crate::Writable for GPIO_PIN14 {}
#[doc = "GPIO_PIN14"]
pub mod gpio_pin14;
#[doc = "GPIO_PIN15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin15](gpio_pin15) module"]
pub type GPIO_PIN15 = crate::Reg<u32, _GPIO_PIN15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN15;
#[doc = "`read()` method returns [gpio_pin15::R](gpio_pin15::R) reader structure"]
impl crate::Readable for GPIO_PIN15 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin15::W](gpio_pin15::W) writer structure"]
impl crate::Writable for GPIO_PIN15 {}
#[doc = "GPIO_PIN15"]
pub mod gpio_pin15;
#[doc = "GPIO_PIN16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin16](gpio_pin16) module"]
pub type GPIO_PIN16 = crate::Reg<u32, _GPIO_PIN16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN16;
#[doc = "`read()` method returns [gpio_pin16::R](gpio_pin16::R) reader structure"]
impl crate::Readable for GPIO_PIN16 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin16::W](gpio_pin16::W) writer structure"]
impl crate::Writable for GPIO_PIN16 {}
#[doc = "GPIO_PIN16"]
pub mod gpio_pin16;
#[doc = "GPIO_PIN17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin17](gpio_pin17) module"]
pub type GPIO_PIN17 = crate::Reg<u32, _GPIO_PIN17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN17;
#[doc = "`read()` method returns [gpio_pin17::R](gpio_pin17::R) reader structure"]
impl crate::Readable for GPIO_PIN17 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin17::W](gpio_pin17::W) writer structure"]
impl crate::Writable for GPIO_PIN17 {}
#[doc = "GPIO_PIN17"]
pub mod gpio_pin17;
#[doc = "GPIO_PIN18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin18](gpio_pin18) module"]
pub type GPIO_PIN18 = crate::Reg<u32, _GPIO_PIN18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN18;
#[doc = "`read()` method returns [gpio_pin18::R](gpio_pin18::R) reader structure"]
impl crate::Readable for GPIO_PIN18 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin18::W](gpio_pin18::W) writer structure"]
impl crate::Writable for GPIO_PIN18 {}
#[doc = "GPIO_PIN18"]
pub mod gpio_pin18;
#[doc = "GPIO_PIN19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin19](gpio_pin19) module"]
pub type GPIO_PIN19 = crate::Reg<u32, _GPIO_PIN19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN19;
#[doc = "`read()` method returns [gpio_pin19::R](gpio_pin19::R) reader structure"]
impl crate::Readable for GPIO_PIN19 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin19::W](gpio_pin19::W) writer structure"]
impl crate::Writable for GPIO_PIN19 {}
#[doc = "GPIO_PIN19"]
pub mod gpio_pin19;
#[doc = "GPIO_PIN20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin20](gpio_pin20) module"]
pub type GPIO_PIN20 = crate::Reg<u32, _GPIO_PIN20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN20;
#[doc = "`read()` method returns [gpio_pin20::R](gpio_pin20::R) reader structure"]
impl crate::Readable for GPIO_PIN20 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin20::W](gpio_pin20::W) writer structure"]
impl crate::Writable for GPIO_PIN20 {}
#[doc = "GPIO_PIN20"]
pub mod gpio_pin20;
#[doc = "GPIO_PIN21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin21](gpio_pin21) module"]
pub type GPIO_PIN21 = crate::Reg<u32, _GPIO_PIN21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN21;
#[doc = "`read()` method returns [gpio_pin21::R](gpio_pin21::R) reader structure"]
impl crate::Readable for GPIO_PIN21 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin21::W](gpio_pin21::W) writer structure"]
impl crate::Writable for GPIO_PIN21 {}
#[doc = "GPIO_PIN21"]
pub mod gpio_pin21;
#[doc = "GPIO_PIN22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin22](gpio_pin22) module"]
pub type GPIO_PIN22 = crate::Reg<u32, _GPIO_PIN22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN22;
#[doc = "`read()` method returns [gpio_pin22::R](gpio_pin22::R) reader structure"]
impl crate::Readable for GPIO_PIN22 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin22::W](gpio_pin22::W) writer structure"]
impl crate::Writable for GPIO_PIN22 {}
#[doc = "GPIO_PIN22"]
pub mod gpio_pin22;
#[doc = "GPIO_PIN23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin23](gpio_pin23) module"]
pub type GPIO_PIN23 = crate::Reg<u32, _GPIO_PIN23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN23;
#[doc = "`read()` method returns [gpio_pin23::R](gpio_pin23::R) reader structure"]
impl crate::Readable for GPIO_PIN23 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin23::W](gpio_pin23::W) writer structure"]
impl crate::Writable for GPIO_PIN23 {}
#[doc = "GPIO_PIN23"]
pub mod gpio_pin23;
#[doc = "GPIO_PIN24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin24](gpio_pin24) module"]
pub type GPIO_PIN24 = crate::Reg<u32, _GPIO_PIN24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN24;
#[doc = "`read()` method returns [gpio_pin24::R](gpio_pin24::R) reader structure"]
impl crate::Readable for GPIO_PIN24 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin24::W](gpio_pin24::W) writer structure"]
impl crate::Writable for GPIO_PIN24 {}
#[doc = "GPIO_PIN24"]
pub mod gpio_pin24;
#[doc = "GPIO_PIN25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin25](gpio_pin25) module"]
pub type GPIO_PIN25 = crate::Reg<u32, _GPIO_PIN25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN25;
#[doc = "`read()` method returns [gpio_pin25::R](gpio_pin25::R) reader structure"]
impl crate::Readable for GPIO_PIN25 {}
#[doc = "`write(|w| ..)` method takes [gpio_pin25::W](gpio_pin25::W) writer structure"]
impl crate::Writable for GPIO_PIN25 {}
#[doc = "GPIO_PIN25"]
pub mod gpio_pin25;
#[doc = "GPIO_STATUS_NEXT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status_next](gpio_status_next) module"]
pub type GPIO_STATUS_NEXT = crate::Reg<u32, _GPIO_STATUS_NEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS_NEXT;
#[doc = "`read()` method returns [gpio_status_next::R](gpio_status_next::R) reader structure"]
impl crate::Readable for GPIO_STATUS_NEXT {}
#[doc = "GPIO_STATUS_NEXT"]
pub mod gpio_status_next;
#[doc = "GPIO_FUNC0_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func0_in_sel_cfg](gpio_func0_in_sel_cfg) module"]
pub type GPIO_FUNC0_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC0_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC0_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func0_in_sel_cfg::R](gpio_func0_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC0_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func0_in_sel_cfg::W](gpio_func0_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC0_IN_SEL_CFG {}
#[doc = "GPIO_FUNC0_IN_SEL_CFG"]
pub mod gpio_func0_in_sel_cfg;
#[doc = "GPIO_FUNC1_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func1_in_sel_cfg](gpio_func1_in_sel_cfg) module"]
pub type GPIO_FUNC1_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC1_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC1_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func1_in_sel_cfg::R](gpio_func1_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC1_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func1_in_sel_cfg::W](gpio_func1_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC1_IN_SEL_CFG {}
#[doc = "GPIO_FUNC1_IN_SEL_CFG"]
pub mod gpio_func1_in_sel_cfg;
#[doc = "GPIO_FUNC2_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func2_in_sel_cfg](gpio_func2_in_sel_cfg) module"]
pub type GPIO_FUNC2_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC2_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC2_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func2_in_sel_cfg::R](gpio_func2_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC2_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func2_in_sel_cfg::W](gpio_func2_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC2_IN_SEL_CFG {}
#[doc = "GPIO_FUNC2_IN_SEL_CFG"]
pub mod gpio_func2_in_sel_cfg;
#[doc = "GPIO_FUNC3_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func3_in_sel_cfg](gpio_func3_in_sel_cfg) module"]
pub type GPIO_FUNC3_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC3_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC3_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func3_in_sel_cfg::R](gpio_func3_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC3_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func3_in_sel_cfg::W](gpio_func3_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC3_IN_SEL_CFG {}
#[doc = "GPIO_FUNC3_IN_SEL_CFG"]
pub mod gpio_func3_in_sel_cfg;
#[doc = "GPIO_FUNC4_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func4_in_sel_cfg](gpio_func4_in_sel_cfg) module"]
pub type GPIO_FUNC4_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC4_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC4_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func4_in_sel_cfg::R](gpio_func4_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC4_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func4_in_sel_cfg::W](gpio_func4_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC4_IN_SEL_CFG {}
#[doc = "GPIO_FUNC4_IN_SEL_CFG"]
pub mod gpio_func4_in_sel_cfg;
#[doc = "GPIO_FUNC5_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func5_in_sel_cfg](gpio_func5_in_sel_cfg) module"]
pub type GPIO_FUNC5_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC5_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC5_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func5_in_sel_cfg::R](gpio_func5_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC5_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func5_in_sel_cfg::W](gpio_func5_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC5_IN_SEL_CFG {}
#[doc = "GPIO_FUNC5_IN_SEL_CFG"]
pub mod gpio_func5_in_sel_cfg;
#[doc = "GPIO_FUNC6_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func6_in_sel_cfg](gpio_func6_in_sel_cfg) module"]
pub type GPIO_FUNC6_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC6_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC6_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func6_in_sel_cfg::R](gpio_func6_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC6_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func6_in_sel_cfg::W](gpio_func6_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC6_IN_SEL_CFG {}
#[doc = "GPIO_FUNC6_IN_SEL_CFG"]
pub mod gpio_func6_in_sel_cfg;
#[doc = "GPIO_FUNC7_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func7_in_sel_cfg](gpio_func7_in_sel_cfg) module"]
pub type GPIO_FUNC7_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC7_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC7_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func7_in_sel_cfg::R](gpio_func7_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC7_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func7_in_sel_cfg::W](gpio_func7_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC7_IN_SEL_CFG {}
#[doc = "GPIO_FUNC7_IN_SEL_CFG"]
pub mod gpio_func7_in_sel_cfg;
#[doc = "GPIO_FUNC8_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func8_in_sel_cfg](gpio_func8_in_sel_cfg) module"]
pub type GPIO_FUNC8_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC8_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC8_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func8_in_sel_cfg::R](gpio_func8_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC8_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func8_in_sel_cfg::W](gpio_func8_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC8_IN_SEL_CFG {}
#[doc = "GPIO_FUNC8_IN_SEL_CFG"]
pub mod gpio_func8_in_sel_cfg;
#[doc = "GPIO_FUNC9_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func9_in_sel_cfg](gpio_func9_in_sel_cfg) module"]
pub type GPIO_FUNC9_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC9_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC9_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func9_in_sel_cfg::R](gpio_func9_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC9_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func9_in_sel_cfg::W](gpio_func9_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC9_IN_SEL_CFG {}
#[doc = "GPIO_FUNC9_IN_SEL_CFG"]
pub mod gpio_func9_in_sel_cfg;
#[doc = "GPIO_FUNC10_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func10_in_sel_cfg](gpio_func10_in_sel_cfg) module"]
pub type GPIO_FUNC10_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC10_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC10_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func10_in_sel_cfg::R](gpio_func10_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC10_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func10_in_sel_cfg::W](gpio_func10_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC10_IN_SEL_CFG {}
#[doc = "GPIO_FUNC10_IN_SEL_CFG"]
pub mod gpio_func10_in_sel_cfg;
#[doc = "GPIO_FUNC11_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func11_in_sel_cfg](gpio_func11_in_sel_cfg) module"]
pub type GPIO_FUNC11_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC11_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC11_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func11_in_sel_cfg::R](gpio_func11_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC11_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func11_in_sel_cfg::W](gpio_func11_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC11_IN_SEL_CFG {}
#[doc = "GPIO_FUNC11_IN_SEL_CFG"]
pub mod gpio_func11_in_sel_cfg;
#[doc = "GPIO_FUNC12_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func12_in_sel_cfg](gpio_func12_in_sel_cfg) module"]
pub type GPIO_FUNC12_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC12_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC12_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func12_in_sel_cfg::R](gpio_func12_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC12_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func12_in_sel_cfg::W](gpio_func12_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC12_IN_SEL_CFG {}
#[doc = "GPIO_FUNC12_IN_SEL_CFG"]
pub mod gpio_func12_in_sel_cfg;
#[doc = "GPIO_FUNC13_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func13_in_sel_cfg](gpio_func13_in_sel_cfg) module"]
pub type GPIO_FUNC13_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC13_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC13_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func13_in_sel_cfg::R](gpio_func13_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC13_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func13_in_sel_cfg::W](gpio_func13_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC13_IN_SEL_CFG {}
#[doc = "GPIO_FUNC13_IN_SEL_CFG"]
pub mod gpio_func13_in_sel_cfg;
#[doc = "GPIO_FUNC14_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func14_in_sel_cfg](gpio_func14_in_sel_cfg) module"]
pub type GPIO_FUNC14_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC14_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC14_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func14_in_sel_cfg::R](gpio_func14_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC14_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func14_in_sel_cfg::W](gpio_func14_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC14_IN_SEL_CFG {}
#[doc = "GPIO_FUNC14_IN_SEL_CFG"]
pub mod gpio_func14_in_sel_cfg;
#[doc = "GPIO_FUNC15_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func15_in_sel_cfg](gpio_func15_in_sel_cfg) module"]
pub type GPIO_FUNC15_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC15_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC15_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func15_in_sel_cfg::R](gpio_func15_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC15_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func15_in_sel_cfg::W](gpio_func15_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC15_IN_SEL_CFG {}
#[doc = "GPIO_FUNC15_IN_SEL_CFG"]
pub mod gpio_func15_in_sel_cfg;
#[doc = "GPIO_FUNC16_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func16_in_sel_cfg](gpio_func16_in_sel_cfg) module"]
pub type GPIO_FUNC16_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC16_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC16_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func16_in_sel_cfg::R](gpio_func16_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC16_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func16_in_sel_cfg::W](gpio_func16_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC16_IN_SEL_CFG {}
#[doc = "GPIO_FUNC16_IN_SEL_CFG"]
pub mod gpio_func16_in_sel_cfg;
#[doc = "GPIO_FUNC17_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func17_in_sel_cfg](gpio_func17_in_sel_cfg) module"]
pub type GPIO_FUNC17_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC17_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC17_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func17_in_sel_cfg::R](gpio_func17_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC17_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func17_in_sel_cfg::W](gpio_func17_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC17_IN_SEL_CFG {}
#[doc = "GPIO_FUNC17_IN_SEL_CFG"]
pub mod gpio_func17_in_sel_cfg;
#[doc = "GPIO_FUNC18_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func18_in_sel_cfg](gpio_func18_in_sel_cfg) module"]
pub type GPIO_FUNC18_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC18_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC18_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func18_in_sel_cfg::R](gpio_func18_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC18_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func18_in_sel_cfg::W](gpio_func18_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC18_IN_SEL_CFG {}
#[doc = "GPIO_FUNC18_IN_SEL_CFG"]
pub mod gpio_func18_in_sel_cfg;
#[doc = "GPIO_FUNC19_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func19_in_sel_cfg](gpio_func19_in_sel_cfg) module"]
pub type GPIO_FUNC19_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC19_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC19_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func19_in_sel_cfg::R](gpio_func19_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC19_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func19_in_sel_cfg::W](gpio_func19_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC19_IN_SEL_CFG {}
#[doc = "GPIO_FUNC19_IN_SEL_CFG"]
pub mod gpio_func19_in_sel_cfg;
#[doc = "GPIO_FUNC20_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func20_in_sel_cfg](gpio_func20_in_sel_cfg) module"]
pub type GPIO_FUNC20_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC20_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC20_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func20_in_sel_cfg::R](gpio_func20_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC20_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func20_in_sel_cfg::W](gpio_func20_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC20_IN_SEL_CFG {}
#[doc = "GPIO_FUNC20_IN_SEL_CFG"]
pub mod gpio_func20_in_sel_cfg;
#[doc = "GPIO_FUNC21_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func21_in_sel_cfg](gpio_func21_in_sel_cfg) module"]
pub type GPIO_FUNC21_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC21_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC21_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func21_in_sel_cfg::R](gpio_func21_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC21_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func21_in_sel_cfg::W](gpio_func21_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC21_IN_SEL_CFG {}
#[doc = "GPIO_FUNC21_IN_SEL_CFG"]
pub mod gpio_func21_in_sel_cfg;
#[doc = "GPIO_FUNC22_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func22_in_sel_cfg](gpio_func22_in_sel_cfg) module"]
pub type GPIO_FUNC22_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC22_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC22_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func22_in_sel_cfg::R](gpio_func22_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC22_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func22_in_sel_cfg::W](gpio_func22_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC22_IN_SEL_CFG {}
#[doc = "GPIO_FUNC22_IN_SEL_CFG"]
pub mod gpio_func22_in_sel_cfg;
#[doc = "GPIO_FUNC23_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func23_in_sel_cfg](gpio_func23_in_sel_cfg) module"]
pub type GPIO_FUNC23_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC23_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC23_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func23_in_sel_cfg::R](gpio_func23_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC23_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func23_in_sel_cfg::W](gpio_func23_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC23_IN_SEL_CFG {}
#[doc = "GPIO_FUNC23_IN_SEL_CFG"]
pub mod gpio_func23_in_sel_cfg;
#[doc = "GPIO_FUNC24_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func24_in_sel_cfg](gpio_func24_in_sel_cfg) module"]
pub type GPIO_FUNC24_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC24_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC24_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func24_in_sel_cfg::R](gpio_func24_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC24_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func24_in_sel_cfg::W](gpio_func24_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC24_IN_SEL_CFG {}
#[doc = "GPIO_FUNC24_IN_SEL_CFG"]
pub mod gpio_func24_in_sel_cfg;
#[doc = "GPIO_FUNC25_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func25_in_sel_cfg](gpio_func25_in_sel_cfg) module"]
pub type GPIO_FUNC25_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC25_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC25_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func25_in_sel_cfg::R](gpio_func25_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC25_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func25_in_sel_cfg::W](gpio_func25_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC25_IN_SEL_CFG {}
#[doc = "GPIO_FUNC25_IN_SEL_CFG"]
pub mod gpio_func25_in_sel_cfg;
#[doc = "GPIO_FUNC26_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func26_in_sel_cfg](gpio_func26_in_sel_cfg) module"]
pub type GPIO_FUNC26_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC26_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC26_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func26_in_sel_cfg::R](gpio_func26_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC26_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func26_in_sel_cfg::W](gpio_func26_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC26_IN_SEL_CFG {}
#[doc = "GPIO_FUNC26_IN_SEL_CFG"]
pub mod gpio_func26_in_sel_cfg;
#[doc = "GPIO_FUNC27_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func27_in_sel_cfg](gpio_func27_in_sel_cfg) module"]
pub type GPIO_FUNC27_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC27_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC27_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func27_in_sel_cfg::R](gpio_func27_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC27_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func27_in_sel_cfg::W](gpio_func27_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC27_IN_SEL_CFG {}
#[doc = "GPIO_FUNC27_IN_SEL_CFG"]
pub mod gpio_func27_in_sel_cfg;
#[doc = "GPIO_FUNC28_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func28_in_sel_cfg](gpio_func28_in_sel_cfg) module"]
pub type GPIO_FUNC28_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC28_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC28_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func28_in_sel_cfg::R](gpio_func28_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC28_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func28_in_sel_cfg::W](gpio_func28_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC28_IN_SEL_CFG {}
#[doc = "GPIO_FUNC28_IN_SEL_CFG"]
pub mod gpio_func28_in_sel_cfg;
#[doc = "GPIO_FUNC29_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func29_in_sel_cfg](gpio_func29_in_sel_cfg) module"]
pub type GPIO_FUNC29_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC29_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC29_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func29_in_sel_cfg::R](gpio_func29_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC29_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func29_in_sel_cfg::W](gpio_func29_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC29_IN_SEL_CFG {}
#[doc = "GPIO_FUNC29_IN_SEL_CFG"]
pub mod gpio_func29_in_sel_cfg;
#[doc = "GPIO_FUNC30_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func30_in_sel_cfg](gpio_func30_in_sel_cfg) module"]
pub type GPIO_FUNC30_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC30_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC30_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func30_in_sel_cfg::R](gpio_func30_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC30_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func30_in_sel_cfg::W](gpio_func30_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC30_IN_SEL_CFG {}
#[doc = "GPIO_FUNC30_IN_SEL_CFG"]
pub mod gpio_func30_in_sel_cfg;
#[doc = "GPIO_FUNC31_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func31_in_sel_cfg](gpio_func31_in_sel_cfg) module"]
pub type GPIO_FUNC31_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC31_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC31_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func31_in_sel_cfg::R](gpio_func31_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC31_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func31_in_sel_cfg::W](gpio_func31_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC31_IN_SEL_CFG {}
#[doc = "GPIO_FUNC31_IN_SEL_CFG"]
pub mod gpio_func31_in_sel_cfg;
#[doc = "GPIO_FUNC32_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func32_in_sel_cfg](gpio_func32_in_sel_cfg) module"]
pub type GPIO_FUNC32_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC32_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC32_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func32_in_sel_cfg::R](gpio_func32_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC32_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func32_in_sel_cfg::W](gpio_func32_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC32_IN_SEL_CFG {}
#[doc = "GPIO_FUNC32_IN_SEL_CFG"]
pub mod gpio_func32_in_sel_cfg;
#[doc = "GPIO_FUNC33_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func33_in_sel_cfg](gpio_func33_in_sel_cfg) module"]
pub type GPIO_FUNC33_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC33_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC33_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func33_in_sel_cfg::R](gpio_func33_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC33_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func33_in_sel_cfg::W](gpio_func33_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC33_IN_SEL_CFG {}
#[doc = "GPIO_FUNC33_IN_SEL_CFG"]
pub mod gpio_func33_in_sel_cfg;
#[doc = "GPIO_FUNC34_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func34_in_sel_cfg](gpio_func34_in_sel_cfg) module"]
pub type GPIO_FUNC34_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC34_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC34_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func34_in_sel_cfg::R](gpio_func34_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC34_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func34_in_sel_cfg::W](gpio_func34_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC34_IN_SEL_CFG {}
#[doc = "GPIO_FUNC34_IN_SEL_CFG"]
pub mod gpio_func34_in_sel_cfg;
#[doc = "GPIO_FUNC35_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func35_in_sel_cfg](gpio_func35_in_sel_cfg) module"]
pub type GPIO_FUNC35_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC35_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC35_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func35_in_sel_cfg::R](gpio_func35_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC35_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func35_in_sel_cfg::W](gpio_func35_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC35_IN_SEL_CFG {}
#[doc = "GPIO_FUNC35_IN_SEL_CFG"]
pub mod gpio_func35_in_sel_cfg;
#[doc = "GPIO_FUNC36_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func36_in_sel_cfg](gpio_func36_in_sel_cfg) module"]
pub type GPIO_FUNC36_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC36_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC36_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func36_in_sel_cfg::R](gpio_func36_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC36_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func36_in_sel_cfg::W](gpio_func36_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC36_IN_SEL_CFG {}
#[doc = "GPIO_FUNC36_IN_SEL_CFG"]
pub mod gpio_func36_in_sel_cfg;
#[doc = "GPIO_FUNC37_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func37_in_sel_cfg](gpio_func37_in_sel_cfg) module"]
pub type GPIO_FUNC37_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC37_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC37_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func37_in_sel_cfg::R](gpio_func37_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC37_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func37_in_sel_cfg::W](gpio_func37_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC37_IN_SEL_CFG {}
#[doc = "GPIO_FUNC37_IN_SEL_CFG"]
pub mod gpio_func37_in_sel_cfg;
#[doc = "GPIO_FUNC38_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func38_in_sel_cfg](gpio_func38_in_sel_cfg) module"]
pub type GPIO_FUNC38_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC38_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC38_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func38_in_sel_cfg::R](gpio_func38_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC38_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func38_in_sel_cfg::W](gpio_func38_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC38_IN_SEL_CFG {}
#[doc = "GPIO_FUNC38_IN_SEL_CFG"]
pub mod gpio_func38_in_sel_cfg;
#[doc = "GPIO_FUNC39_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func39_in_sel_cfg](gpio_func39_in_sel_cfg) module"]
pub type GPIO_FUNC39_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC39_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC39_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func39_in_sel_cfg::R](gpio_func39_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC39_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func39_in_sel_cfg::W](gpio_func39_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC39_IN_SEL_CFG {}
#[doc = "GPIO_FUNC39_IN_SEL_CFG"]
pub mod gpio_func39_in_sel_cfg;
#[doc = "GPIO_FUNC40_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func40_in_sel_cfg](gpio_func40_in_sel_cfg) module"]
pub type GPIO_FUNC40_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC40_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC40_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func40_in_sel_cfg::R](gpio_func40_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC40_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func40_in_sel_cfg::W](gpio_func40_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC40_IN_SEL_CFG {}
#[doc = "GPIO_FUNC40_IN_SEL_CFG"]
pub mod gpio_func40_in_sel_cfg;
#[doc = "GPIO_FUNC41_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func41_in_sel_cfg](gpio_func41_in_sel_cfg) module"]
pub type GPIO_FUNC41_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC41_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC41_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func41_in_sel_cfg::R](gpio_func41_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC41_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func41_in_sel_cfg::W](gpio_func41_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC41_IN_SEL_CFG {}
#[doc = "GPIO_FUNC41_IN_SEL_CFG"]
pub mod gpio_func41_in_sel_cfg;
#[doc = "GPIO_FUNC42_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func42_in_sel_cfg](gpio_func42_in_sel_cfg) module"]
pub type GPIO_FUNC42_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC42_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC42_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func42_in_sel_cfg::R](gpio_func42_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC42_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func42_in_sel_cfg::W](gpio_func42_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC42_IN_SEL_CFG {}
#[doc = "GPIO_FUNC42_IN_SEL_CFG"]
pub mod gpio_func42_in_sel_cfg;
#[doc = "GPIO_FUNC43_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func43_in_sel_cfg](gpio_func43_in_sel_cfg) module"]
pub type GPIO_FUNC43_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC43_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC43_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func43_in_sel_cfg::R](gpio_func43_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC43_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func43_in_sel_cfg::W](gpio_func43_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC43_IN_SEL_CFG {}
#[doc = "GPIO_FUNC43_IN_SEL_CFG"]
pub mod gpio_func43_in_sel_cfg;
#[doc = "GPIO_FUNC44_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func44_in_sel_cfg](gpio_func44_in_sel_cfg) module"]
pub type GPIO_FUNC44_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC44_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC44_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func44_in_sel_cfg::R](gpio_func44_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC44_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func44_in_sel_cfg::W](gpio_func44_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC44_IN_SEL_CFG {}
#[doc = "GPIO_FUNC44_IN_SEL_CFG"]
pub mod gpio_func44_in_sel_cfg;
#[doc = "GPIO_FUNC45_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func45_in_sel_cfg](gpio_func45_in_sel_cfg) module"]
pub type GPIO_FUNC45_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC45_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC45_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func45_in_sel_cfg::R](gpio_func45_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC45_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func45_in_sel_cfg::W](gpio_func45_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC45_IN_SEL_CFG {}
#[doc = "GPIO_FUNC45_IN_SEL_CFG"]
pub mod gpio_func45_in_sel_cfg;
#[doc = "GPIO_FUNC46_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func46_in_sel_cfg](gpio_func46_in_sel_cfg) module"]
pub type GPIO_FUNC46_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC46_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC46_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func46_in_sel_cfg::R](gpio_func46_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC46_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func46_in_sel_cfg::W](gpio_func46_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC46_IN_SEL_CFG {}
#[doc = "GPIO_FUNC46_IN_SEL_CFG"]
pub mod gpio_func46_in_sel_cfg;
#[doc = "GPIO_FUNC47_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func47_in_sel_cfg](gpio_func47_in_sel_cfg) module"]
pub type GPIO_FUNC47_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC47_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC47_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func47_in_sel_cfg::R](gpio_func47_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC47_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func47_in_sel_cfg::W](gpio_func47_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC47_IN_SEL_CFG {}
#[doc = "GPIO_FUNC47_IN_SEL_CFG"]
pub mod gpio_func47_in_sel_cfg;
#[doc = "GPIO_FUNC48_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func48_in_sel_cfg](gpio_func48_in_sel_cfg) module"]
pub type GPIO_FUNC48_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC48_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC48_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func48_in_sel_cfg::R](gpio_func48_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC48_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func48_in_sel_cfg::W](gpio_func48_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC48_IN_SEL_CFG {}
#[doc = "GPIO_FUNC48_IN_SEL_CFG"]
pub mod gpio_func48_in_sel_cfg;
#[doc = "GPIO_FUNC49_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func49_in_sel_cfg](gpio_func49_in_sel_cfg) module"]
pub type GPIO_FUNC49_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC49_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC49_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func49_in_sel_cfg::R](gpio_func49_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC49_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func49_in_sel_cfg::W](gpio_func49_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC49_IN_SEL_CFG {}
#[doc = "GPIO_FUNC49_IN_SEL_CFG"]
pub mod gpio_func49_in_sel_cfg;
#[doc = "GPIO_FUNC50_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func50_in_sel_cfg](gpio_func50_in_sel_cfg) module"]
pub type GPIO_FUNC50_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC50_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC50_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func50_in_sel_cfg::R](gpio_func50_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC50_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func50_in_sel_cfg::W](gpio_func50_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC50_IN_SEL_CFG {}
#[doc = "GPIO_FUNC50_IN_SEL_CFG"]
pub mod gpio_func50_in_sel_cfg;
#[doc = "GPIO_FUNC51_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func51_in_sel_cfg](gpio_func51_in_sel_cfg) module"]
pub type GPIO_FUNC51_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC51_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC51_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func51_in_sel_cfg::R](gpio_func51_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC51_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func51_in_sel_cfg::W](gpio_func51_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC51_IN_SEL_CFG {}
#[doc = "GPIO_FUNC51_IN_SEL_CFG"]
pub mod gpio_func51_in_sel_cfg;
#[doc = "GPIO_FUNC52_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func52_in_sel_cfg](gpio_func52_in_sel_cfg) module"]
pub type GPIO_FUNC52_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC52_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC52_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func52_in_sel_cfg::R](gpio_func52_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC52_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func52_in_sel_cfg::W](gpio_func52_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC52_IN_SEL_CFG {}
#[doc = "GPIO_FUNC52_IN_SEL_CFG"]
pub mod gpio_func52_in_sel_cfg;
#[doc = "GPIO_FUNC53_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func53_in_sel_cfg](gpio_func53_in_sel_cfg) module"]
pub type GPIO_FUNC53_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC53_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC53_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func53_in_sel_cfg::R](gpio_func53_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC53_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func53_in_sel_cfg::W](gpio_func53_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC53_IN_SEL_CFG {}
#[doc = "GPIO_FUNC53_IN_SEL_CFG"]
pub mod gpio_func53_in_sel_cfg;
#[doc = "GPIO_FUNC54_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func54_in_sel_cfg](gpio_func54_in_sel_cfg) module"]
pub type GPIO_FUNC54_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC54_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC54_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func54_in_sel_cfg::R](gpio_func54_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC54_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func54_in_sel_cfg::W](gpio_func54_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC54_IN_SEL_CFG {}
#[doc = "GPIO_FUNC54_IN_SEL_CFG"]
pub mod gpio_func54_in_sel_cfg;
#[doc = "GPIO_FUNC55_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func55_in_sel_cfg](gpio_func55_in_sel_cfg) module"]
pub type GPIO_FUNC55_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC55_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC55_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func55_in_sel_cfg::R](gpio_func55_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC55_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func55_in_sel_cfg::W](gpio_func55_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC55_IN_SEL_CFG {}
#[doc = "GPIO_FUNC55_IN_SEL_CFG"]
pub mod gpio_func55_in_sel_cfg;
#[doc = "GPIO_FUNC56_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func56_in_sel_cfg](gpio_func56_in_sel_cfg) module"]
pub type GPIO_FUNC56_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC56_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC56_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func56_in_sel_cfg::R](gpio_func56_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC56_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func56_in_sel_cfg::W](gpio_func56_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC56_IN_SEL_CFG {}
#[doc = "GPIO_FUNC56_IN_SEL_CFG"]
pub mod gpio_func56_in_sel_cfg;
#[doc = "GPIO_FUNC57_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func57_in_sel_cfg](gpio_func57_in_sel_cfg) module"]
pub type GPIO_FUNC57_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC57_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC57_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func57_in_sel_cfg::R](gpio_func57_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC57_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func57_in_sel_cfg::W](gpio_func57_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC57_IN_SEL_CFG {}
#[doc = "GPIO_FUNC57_IN_SEL_CFG"]
pub mod gpio_func57_in_sel_cfg;
#[doc = "GPIO_FUNC58_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func58_in_sel_cfg](gpio_func58_in_sel_cfg) module"]
pub type GPIO_FUNC58_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC58_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC58_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func58_in_sel_cfg::R](gpio_func58_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC58_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func58_in_sel_cfg::W](gpio_func58_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC58_IN_SEL_CFG {}
#[doc = "GPIO_FUNC58_IN_SEL_CFG"]
pub mod gpio_func58_in_sel_cfg;
#[doc = "GPIO_FUNC59_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func59_in_sel_cfg](gpio_func59_in_sel_cfg) module"]
pub type GPIO_FUNC59_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC59_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC59_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func59_in_sel_cfg::R](gpio_func59_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC59_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func59_in_sel_cfg::W](gpio_func59_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC59_IN_SEL_CFG {}
#[doc = "GPIO_FUNC59_IN_SEL_CFG"]
pub mod gpio_func59_in_sel_cfg;
#[doc = "GPIO_FUNC60_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func60_in_sel_cfg](gpio_func60_in_sel_cfg) module"]
pub type GPIO_FUNC60_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC60_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC60_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func60_in_sel_cfg::R](gpio_func60_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC60_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func60_in_sel_cfg::W](gpio_func60_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC60_IN_SEL_CFG {}
#[doc = "GPIO_FUNC60_IN_SEL_CFG"]
pub mod gpio_func60_in_sel_cfg;
#[doc = "GPIO_FUNC61_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func61_in_sel_cfg](gpio_func61_in_sel_cfg) module"]
pub type GPIO_FUNC61_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC61_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC61_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func61_in_sel_cfg::R](gpio_func61_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC61_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func61_in_sel_cfg::W](gpio_func61_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC61_IN_SEL_CFG {}
#[doc = "GPIO_FUNC61_IN_SEL_CFG"]
pub mod gpio_func61_in_sel_cfg;
#[doc = "GPIO_FUNC62_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func62_in_sel_cfg](gpio_func62_in_sel_cfg) module"]
pub type GPIO_FUNC62_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC62_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC62_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func62_in_sel_cfg::R](gpio_func62_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC62_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func62_in_sel_cfg::W](gpio_func62_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC62_IN_SEL_CFG {}
#[doc = "GPIO_FUNC62_IN_SEL_CFG"]
pub mod gpio_func62_in_sel_cfg;
#[doc = "GPIO_FUNC63_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func63_in_sel_cfg](gpio_func63_in_sel_cfg) module"]
pub type GPIO_FUNC63_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC63_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC63_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func63_in_sel_cfg::R](gpio_func63_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC63_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func63_in_sel_cfg::W](gpio_func63_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC63_IN_SEL_CFG {}
#[doc = "GPIO_FUNC63_IN_SEL_CFG"]
pub mod gpio_func63_in_sel_cfg;
#[doc = "GPIO_FUNC64_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func64_in_sel_cfg](gpio_func64_in_sel_cfg) module"]
pub type GPIO_FUNC64_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC64_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC64_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func64_in_sel_cfg::R](gpio_func64_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC64_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func64_in_sel_cfg::W](gpio_func64_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC64_IN_SEL_CFG {}
#[doc = "GPIO_FUNC64_IN_SEL_CFG"]
pub mod gpio_func64_in_sel_cfg;
#[doc = "GPIO_FUNC65_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func65_in_sel_cfg](gpio_func65_in_sel_cfg) module"]
pub type GPIO_FUNC65_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC65_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC65_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func65_in_sel_cfg::R](gpio_func65_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC65_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func65_in_sel_cfg::W](gpio_func65_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC65_IN_SEL_CFG {}
#[doc = "GPIO_FUNC65_IN_SEL_CFG"]
pub mod gpio_func65_in_sel_cfg;
#[doc = "GPIO_FUNC66_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func66_in_sel_cfg](gpio_func66_in_sel_cfg) module"]
pub type GPIO_FUNC66_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC66_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC66_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func66_in_sel_cfg::R](gpio_func66_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC66_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func66_in_sel_cfg::W](gpio_func66_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC66_IN_SEL_CFG {}
#[doc = "GPIO_FUNC66_IN_SEL_CFG"]
pub mod gpio_func66_in_sel_cfg;
#[doc = "GPIO_FUNC67_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func67_in_sel_cfg](gpio_func67_in_sel_cfg) module"]
pub type GPIO_FUNC67_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC67_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC67_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func67_in_sel_cfg::R](gpio_func67_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC67_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func67_in_sel_cfg::W](gpio_func67_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC67_IN_SEL_CFG {}
#[doc = "GPIO_FUNC67_IN_SEL_CFG"]
pub mod gpio_func67_in_sel_cfg;
#[doc = "GPIO_FUNC68_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func68_in_sel_cfg](gpio_func68_in_sel_cfg) module"]
pub type GPIO_FUNC68_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC68_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC68_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func68_in_sel_cfg::R](gpio_func68_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC68_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func68_in_sel_cfg::W](gpio_func68_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC68_IN_SEL_CFG {}
#[doc = "GPIO_FUNC68_IN_SEL_CFG"]
pub mod gpio_func68_in_sel_cfg;
#[doc = "GPIO_FUNC69_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func69_in_sel_cfg](gpio_func69_in_sel_cfg) module"]
pub type GPIO_FUNC69_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC69_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC69_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func69_in_sel_cfg::R](gpio_func69_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC69_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func69_in_sel_cfg::W](gpio_func69_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC69_IN_SEL_CFG {}
#[doc = "GPIO_FUNC69_IN_SEL_CFG"]
pub mod gpio_func69_in_sel_cfg;
#[doc = "GPIO_FUNC70_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func70_in_sel_cfg](gpio_func70_in_sel_cfg) module"]
pub type GPIO_FUNC70_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC70_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC70_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func70_in_sel_cfg::R](gpio_func70_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC70_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func70_in_sel_cfg::W](gpio_func70_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC70_IN_SEL_CFG {}
#[doc = "GPIO_FUNC70_IN_SEL_CFG"]
pub mod gpio_func70_in_sel_cfg;
#[doc = "GPIO_FUNC71_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func71_in_sel_cfg](gpio_func71_in_sel_cfg) module"]
pub type GPIO_FUNC71_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC71_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC71_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func71_in_sel_cfg::R](gpio_func71_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC71_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func71_in_sel_cfg::W](gpio_func71_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC71_IN_SEL_CFG {}
#[doc = "GPIO_FUNC71_IN_SEL_CFG"]
pub mod gpio_func71_in_sel_cfg;
#[doc = "GPIO_FUNC72_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func72_in_sel_cfg](gpio_func72_in_sel_cfg) module"]
pub type GPIO_FUNC72_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC72_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC72_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func72_in_sel_cfg::R](gpio_func72_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC72_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func72_in_sel_cfg::W](gpio_func72_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC72_IN_SEL_CFG {}
#[doc = "GPIO_FUNC72_IN_SEL_CFG"]
pub mod gpio_func72_in_sel_cfg;
#[doc = "GPIO_FUNC73_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func73_in_sel_cfg](gpio_func73_in_sel_cfg) module"]
pub type GPIO_FUNC73_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC73_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC73_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func73_in_sel_cfg::R](gpio_func73_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC73_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func73_in_sel_cfg::W](gpio_func73_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC73_IN_SEL_CFG {}
#[doc = "GPIO_FUNC73_IN_SEL_CFG"]
pub mod gpio_func73_in_sel_cfg;
#[doc = "GPIO_FUNC74_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func74_in_sel_cfg](gpio_func74_in_sel_cfg) module"]
pub type GPIO_FUNC74_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC74_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC74_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func74_in_sel_cfg::R](gpio_func74_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC74_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func74_in_sel_cfg::W](gpio_func74_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC74_IN_SEL_CFG {}
#[doc = "GPIO_FUNC74_IN_SEL_CFG"]
pub mod gpio_func74_in_sel_cfg;
#[doc = "GPIO_FUNC75_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func75_in_sel_cfg](gpio_func75_in_sel_cfg) module"]
pub type GPIO_FUNC75_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC75_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC75_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func75_in_sel_cfg::R](gpio_func75_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC75_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func75_in_sel_cfg::W](gpio_func75_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC75_IN_SEL_CFG {}
#[doc = "GPIO_FUNC75_IN_SEL_CFG"]
pub mod gpio_func75_in_sel_cfg;
#[doc = "GPIO_FUNC76_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func76_in_sel_cfg](gpio_func76_in_sel_cfg) module"]
pub type GPIO_FUNC76_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC76_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC76_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func76_in_sel_cfg::R](gpio_func76_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC76_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func76_in_sel_cfg::W](gpio_func76_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC76_IN_SEL_CFG {}
#[doc = "GPIO_FUNC76_IN_SEL_CFG"]
pub mod gpio_func76_in_sel_cfg;
#[doc = "GPIO_FUNC77_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func77_in_sel_cfg](gpio_func77_in_sel_cfg) module"]
pub type GPIO_FUNC77_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC77_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC77_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func77_in_sel_cfg::R](gpio_func77_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC77_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func77_in_sel_cfg::W](gpio_func77_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC77_IN_SEL_CFG {}
#[doc = "GPIO_FUNC77_IN_SEL_CFG"]
pub mod gpio_func77_in_sel_cfg;
#[doc = "GPIO_FUNC78_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func78_in_sel_cfg](gpio_func78_in_sel_cfg) module"]
pub type GPIO_FUNC78_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC78_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC78_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func78_in_sel_cfg::R](gpio_func78_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC78_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func78_in_sel_cfg::W](gpio_func78_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC78_IN_SEL_CFG {}
#[doc = "GPIO_FUNC78_IN_SEL_CFG"]
pub mod gpio_func78_in_sel_cfg;
#[doc = "GPIO_FUNC79_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func79_in_sel_cfg](gpio_func79_in_sel_cfg) module"]
pub type GPIO_FUNC79_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC79_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC79_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func79_in_sel_cfg::R](gpio_func79_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC79_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func79_in_sel_cfg::W](gpio_func79_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC79_IN_SEL_CFG {}
#[doc = "GPIO_FUNC79_IN_SEL_CFG"]
pub mod gpio_func79_in_sel_cfg;
#[doc = "GPIO_FUNC80_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func80_in_sel_cfg](gpio_func80_in_sel_cfg) module"]
pub type GPIO_FUNC80_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC80_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC80_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func80_in_sel_cfg::R](gpio_func80_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC80_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func80_in_sel_cfg::W](gpio_func80_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC80_IN_SEL_CFG {}
#[doc = "GPIO_FUNC80_IN_SEL_CFG"]
pub mod gpio_func80_in_sel_cfg;
#[doc = "GPIO_FUNC81_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func81_in_sel_cfg](gpio_func81_in_sel_cfg) module"]
pub type GPIO_FUNC81_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC81_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC81_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func81_in_sel_cfg::R](gpio_func81_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC81_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func81_in_sel_cfg::W](gpio_func81_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC81_IN_SEL_CFG {}
#[doc = "GPIO_FUNC81_IN_SEL_CFG"]
pub mod gpio_func81_in_sel_cfg;
#[doc = "GPIO_FUNC82_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func82_in_sel_cfg](gpio_func82_in_sel_cfg) module"]
pub type GPIO_FUNC82_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC82_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC82_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func82_in_sel_cfg::R](gpio_func82_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC82_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func82_in_sel_cfg::W](gpio_func82_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC82_IN_SEL_CFG {}
#[doc = "GPIO_FUNC82_IN_SEL_CFG"]
pub mod gpio_func82_in_sel_cfg;
#[doc = "GPIO_FUNC83_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func83_in_sel_cfg](gpio_func83_in_sel_cfg) module"]
pub type GPIO_FUNC83_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC83_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC83_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func83_in_sel_cfg::R](gpio_func83_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC83_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func83_in_sel_cfg::W](gpio_func83_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC83_IN_SEL_CFG {}
#[doc = "GPIO_FUNC83_IN_SEL_CFG"]
pub mod gpio_func83_in_sel_cfg;
#[doc = "GPIO_FUNC84_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func84_in_sel_cfg](gpio_func84_in_sel_cfg) module"]
pub type GPIO_FUNC84_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC84_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC84_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func84_in_sel_cfg::R](gpio_func84_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC84_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func84_in_sel_cfg::W](gpio_func84_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC84_IN_SEL_CFG {}
#[doc = "GPIO_FUNC84_IN_SEL_CFG"]
pub mod gpio_func84_in_sel_cfg;
#[doc = "GPIO_FUNC85_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func85_in_sel_cfg](gpio_func85_in_sel_cfg) module"]
pub type GPIO_FUNC85_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC85_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC85_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func85_in_sel_cfg::R](gpio_func85_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC85_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func85_in_sel_cfg::W](gpio_func85_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC85_IN_SEL_CFG {}
#[doc = "GPIO_FUNC85_IN_SEL_CFG"]
pub mod gpio_func85_in_sel_cfg;
#[doc = "GPIO_FUNC86_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func86_in_sel_cfg](gpio_func86_in_sel_cfg) module"]
pub type GPIO_FUNC86_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC86_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC86_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func86_in_sel_cfg::R](gpio_func86_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC86_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func86_in_sel_cfg::W](gpio_func86_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC86_IN_SEL_CFG {}
#[doc = "GPIO_FUNC86_IN_SEL_CFG"]
pub mod gpio_func86_in_sel_cfg;
#[doc = "GPIO_FUNC87_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func87_in_sel_cfg](gpio_func87_in_sel_cfg) module"]
pub type GPIO_FUNC87_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC87_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC87_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func87_in_sel_cfg::R](gpio_func87_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC87_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func87_in_sel_cfg::W](gpio_func87_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC87_IN_SEL_CFG {}
#[doc = "GPIO_FUNC87_IN_SEL_CFG"]
pub mod gpio_func87_in_sel_cfg;
#[doc = "GPIO_FUNC88_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func88_in_sel_cfg](gpio_func88_in_sel_cfg) module"]
pub type GPIO_FUNC88_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC88_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC88_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func88_in_sel_cfg::R](gpio_func88_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC88_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func88_in_sel_cfg::W](gpio_func88_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC88_IN_SEL_CFG {}
#[doc = "GPIO_FUNC88_IN_SEL_CFG"]
pub mod gpio_func88_in_sel_cfg;
#[doc = "GPIO_FUNC89_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func89_in_sel_cfg](gpio_func89_in_sel_cfg) module"]
pub type GPIO_FUNC89_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC89_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC89_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func89_in_sel_cfg::R](gpio_func89_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC89_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func89_in_sel_cfg::W](gpio_func89_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC89_IN_SEL_CFG {}
#[doc = "GPIO_FUNC89_IN_SEL_CFG"]
pub mod gpio_func89_in_sel_cfg;
#[doc = "GPIO_FUNC90_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func90_in_sel_cfg](gpio_func90_in_sel_cfg) module"]
pub type GPIO_FUNC90_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC90_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC90_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func90_in_sel_cfg::R](gpio_func90_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC90_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func90_in_sel_cfg::W](gpio_func90_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC90_IN_SEL_CFG {}
#[doc = "GPIO_FUNC90_IN_SEL_CFG"]
pub mod gpio_func90_in_sel_cfg;
#[doc = "GPIO_FUNC91_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func91_in_sel_cfg](gpio_func91_in_sel_cfg) module"]
pub type GPIO_FUNC91_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC91_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC91_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func91_in_sel_cfg::R](gpio_func91_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC91_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func91_in_sel_cfg::W](gpio_func91_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC91_IN_SEL_CFG {}
#[doc = "GPIO_FUNC91_IN_SEL_CFG"]
pub mod gpio_func91_in_sel_cfg;
#[doc = "GPIO_FUNC92_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func92_in_sel_cfg](gpio_func92_in_sel_cfg) module"]
pub type GPIO_FUNC92_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC92_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC92_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func92_in_sel_cfg::R](gpio_func92_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC92_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func92_in_sel_cfg::W](gpio_func92_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC92_IN_SEL_CFG {}
#[doc = "GPIO_FUNC92_IN_SEL_CFG"]
pub mod gpio_func92_in_sel_cfg;
#[doc = "GPIO_FUNC93_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func93_in_sel_cfg](gpio_func93_in_sel_cfg) module"]
pub type GPIO_FUNC93_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC93_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC93_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func93_in_sel_cfg::R](gpio_func93_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC93_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func93_in_sel_cfg::W](gpio_func93_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC93_IN_SEL_CFG {}
#[doc = "GPIO_FUNC93_IN_SEL_CFG"]
pub mod gpio_func93_in_sel_cfg;
#[doc = "GPIO_FUNC94_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func94_in_sel_cfg](gpio_func94_in_sel_cfg) module"]
pub type GPIO_FUNC94_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC94_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC94_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func94_in_sel_cfg::R](gpio_func94_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC94_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func94_in_sel_cfg::W](gpio_func94_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC94_IN_SEL_CFG {}
#[doc = "GPIO_FUNC94_IN_SEL_CFG"]
pub mod gpio_func94_in_sel_cfg;
#[doc = "GPIO_FUNC95_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func95_in_sel_cfg](gpio_func95_in_sel_cfg) module"]
pub type GPIO_FUNC95_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC95_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC95_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func95_in_sel_cfg::R](gpio_func95_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC95_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func95_in_sel_cfg::W](gpio_func95_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC95_IN_SEL_CFG {}
#[doc = "GPIO_FUNC95_IN_SEL_CFG"]
pub mod gpio_func95_in_sel_cfg;
#[doc = "GPIO_FUNC96_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func96_in_sel_cfg](gpio_func96_in_sel_cfg) module"]
pub type GPIO_FUNC96_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC96_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC96_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func96_in_sel_cfg::R](gpio_func96_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC96_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func96_in_sel_cfg::W](gpio_func96_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC96_IN_SEL_CFG {}
#[doc = "GPIO_FUNC96_IN_SEL_CFG"]
pub mod gpio_func96_in_sel_cfg;
#[doc = "GPIO_FUNC97_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func97_in_sel_cfg](gpio_func97_in_sel_cfg) module"]
pub type GPIO_FUNC97_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC97_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC97_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func97_in_sel_cfg::R](gpio_func97_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC97_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func97_in_sel_cfg::W](gpio_func97_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC97_IN_SEL_CFG {}
#[doc = "GPIO_FUNC97_IN_SEL_CFG"]
pub mod gpio_func97_in_sel_cfg;
#[doc = "GPIO_FUNC98_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func98_in_sel_cfg](gpio_func98_in_sel_cfg) module"]
pub type GPIO_FUNC98_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC98_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC98_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func98_in_sel_cfg::R](gpio_func98_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC98_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func98_in_sel_cfg::W](gpio_func98_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC98_IN_SEL_CFG {}
#[doc = "GPIO_FUNC98_IN_SEL_CFG"]
pub mod gpio_func98_in_sel_cfg;
#[doc = "GPIO_FUNC99_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func99_in_sel_cfg](gpio_func99_in_sel_cfg) module"]
pub type GPIO_FUNC99_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC99_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC99_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func99_in_sel_cfg::R](gpio_func99_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC99_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func99_in_sel_cfg::W](gpio_func99_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC99_IN_SEL_CFG {}
#[doc = "GPIO_FUNC99_IN_SEL_CFG"]
pub mod gpio_func99_in_sel_cfg;
#[doc = "GPIO_FUNC100_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func100_in_sel_cfg](gpio_func100_in_sel_cfg) module"]
pub type GPIO_FUNC100_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC100_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC100_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func100_in_sel_cfg::R](gpio_func100_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC100_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func100_in_sel_cfg::W](gpio_func100_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC100_IN_SEL_CFG {}
#[doc = "GPIO_FUNC100_IN_SEL_CFG"]
pub mod gpio_func100_in_sel_cfg;
#[doc = "GPIO_FUNC101_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func101_in_sel_cfg](gpio_func101_in_sel_cfg) module"]
pub type GPIO_FUNC101_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC101_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC101_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func101_in_sel_cfg::R](gpio_func101_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC101_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func101_in_sel_cfg::W](gpio_func101_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC101_IN_SEL_CFG {}
#[doc = "GPIO_FUNC101_IN_SEL_CFG"]
pub mod gpio_func101_in_sel_cfg;
#[doc = "GPIO_FUNC102_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func102_in_sel_cfg](gpio_func102_in_sel_cfg) module"]
pub type GPIO_FUNC102_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC102_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC102_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func102_in_sel_cfg::R](gpio_func102_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC102_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func102_in_sel_cfg::W](gpio_func102_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC102_IN_SEL_CFG {}
#[doc = "GPIO_FUNC102_IN_SEL_CFG"]
pub mod gpio_func102_in_sel_cfg;
#[doc = "GPIO_FUNC103_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func103_in_sel_cfg](gpio_func103_in_sel_cfg) module"]
pub type GPIO_FUNC103_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC103_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC103_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func103_in_sel_cfg::R](gpio_func103_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC103_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func103_in_sel_cfg::W](gpio_func103_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC103_IN_SEL_CFG {}
#[doc = "GPIO_FUNC103_IN_SEL_CFG"]
pub mod gpio_func103_in_sel_cfg;
#[doc = "GPIO_FUNC104_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func104_in_sel_cfg](gpio_func104_in_sel_cfg) module"]
pub type GPIO_FUNC104_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC104_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC104_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func104_in_sel_cfg::R](gpio_func104_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC104_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func104_in_sel_cfg::W](gpio_func104_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC104_IN_SEL_CFG {}
#[doc = "GPIO_FUNC104_IN_SEL_CFG"]
pub mod gpio_func104_in_sel_cfg;
#[doc = "GPIO_FUNC105_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func105_in_sel_cfg](gpio_func105_in_sel_cfg) module"]
pub type GPIO_FUNC105_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC105_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC105_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func105_in_sel_cfg::R](gpio_func105_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC105_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func105_in_sel_cfg::W](gpio_func105_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC105_IN_SEL_CFG {}
#[doc = "GPIO_FUNC105_IN_SEL_CFG"]
pub mod gpio_func105_in_sel_cfg;
#[doc = "GPIO_FUNC106_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func106_in_sel_cfg](gpio_func106_in_sel_cfg) module"]
pub type GPIO_FUNC106_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC106_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC106_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func106_in_sel_cfg::R](gpio_func106_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC106_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func106_in_sel_cfg::W](gpio_func106_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC106_IN_SEL_CFG {}
#[doc = "GPIO_FUNC106_IN_SEL_CFG"]
pub mod gpio_func106_in_sel_cfg;
#[doc = "GPIO_FUNC107_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func107_in_sel_cfg](gpio_func107_in_sel_cfg) module"]
pub type GPIO_FUNC107_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC107_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC107_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func107_in_sel_cfg::R](gpio_func107_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC107_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func107_in_sel_cfg::W](gpio_func107_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC107_IN_SEL_CFG {}
#[doc = "GPIO_FUNC107_IN_SEL_CFG"]
pub mod gpio_func107_in_sel_cfg;
#[doc = "GPIO_FUNC108_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func108_in_sel_cfg](gpio_func108_in_sel_cfg) module"]
pub type GPIO_FUNC108_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC108_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC108_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func108_in_sel_cfg::R](gpio_func108_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC108_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func108_in_sel_cfg::W](gpio_func108_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC108_IN_SEL_CFG {}
#[doc = "GPIO_FUNC108_IN_SEL_CFG"]
pub mod gpio_func108_in_sel_cfg;
#[doc = "GPIO_FUNC109_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func109_in_sel_cfg](gpio_func109_in_sel_cfg) module"]
pub type GPIO_FUNC109_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC109_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC109_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func109_in_sel_cfg::R](gpio_func109_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC109_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func109_in_sel_cfg::W](gpio_func109_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC109_IN_SEL_CFG {}
#[doc = "GPIO_FUNC109_IN_SEL_CFG"]
pub mod gpio_func109_in_sel_cfg;
#[doc = "GPIO_FUNC110_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func110_in_sel_cfg](gpio_func110_in_sel_cfg) module"]
pub type GPIO_FUNC110_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC110_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC110_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func110_in_sel_cfg::R](gpio_func110_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC110_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func110_in_sel_cfg::W](gpio_func110_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC110_IN_SEL_CFG {}
#[doc = "GPIO_FUNC110_IN_SEL_CFG"]
pub mod gpio_func110_in_sel_cfg;
#[doc = "GPIO_FUNC111_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func111_in_sel_cfg](gpio_func111_in_sel_cfg) module"]
pub type GPIO_FUNC111_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC111_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC111_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func111_in_sel_cfg::R](gpio_func111_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC111_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func111_in_sel_cfg::W](gpio_func111_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC111_IN_SEL_CFG {}
#[doc = "GPIO_FUNC111_IN_SEL_CFG"]
pub mod gpio_func111_in_sel_cfg;
#[doc = "GPIO_FUNC112_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func112_in_sel_cfg](gpio_func112_in_sel_cfg) module"]
pub type GPIO_FUNC112_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC112_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC112_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func112_in_sel_cfg::R](gpio_func112_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC112_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func112_in_sel_cfg::W](gpio_func112_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC112_IN_SEL_CFG {}
#[doc = "GPIO_FUNC112_IN_SEL_CFG"]
pub mod gpio_func112_in_sel_cfg;
#[doc = "GPIO_FUNC113_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func113_in_sel_cfg](gpio_func113_in_sel_cfg) module"]
pub type GPIO_FUNC113_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC113_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC113_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func113_in_sel_cfg::R](gpio_func113_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC113_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func113_in_sel_cfg::W](gpio_func113_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC113_IN_SEL_CFG {}
#[doc = "GPIO_FUNC113_IN_SEL_CFG"]
pub mod gpio_func113_in_sel_cfg;
#[doc = "GPIO_FUNC114_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func114_in_sel_cfg](gpio_func114_in_sel_cfg) module"]
pub type GPIO_FUNC114_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC114_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC114_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func114_in_sel_cfg::R](gpio_func114_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC114_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func114_in_sel_cfg::W](gpio_func114_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC114_IN_SEL_CFG {}
#[doc = "GPIO_FUNC114_IN_SEL_CFG"]
pub mod gpio_func114_in_sel_cfg;
#[doc = "GPIO_FUNC115_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func115_in_sel_cfg](gpio_func115_in_sel_cfg) module"]
pub type GPIO_FUNC115_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC115_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC115_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func115_in_sel_cfg::R](gpio_func115_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC115_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func115_in_sel_cfg::W](gpio_func115_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC115_IN_SEL_CFG {}
#[doc = "GPIO_FUNC115_IN_SEL_CFG"]
pub mod gpio_func115_in_sel_cfg;
#[doc = "GPIO_FUNC116_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func116_in_sel_cfg](gpio_func116_in_sel_cfg) module"]
pub type GPIO_FUNC116_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC116_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC116_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func116_in_sel_cfg::R](gpio_func116_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC116_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func116_in_sel_cfg::W](gpio_func116_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC116_IN_SEL_CFG {}
#[doc = "GPIO_FUNC116_IN_SEL_CFG"]
pub mod gpio_func116_in_sel_cfg;
#[doc = "GPIO_FUNC117_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func117_in_sel_cfg](gpio_func117_in_sel_cfg) module"]
pub type GPIO_FUNC117_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC117_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC117_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func117_in_sel_cfg::R](gpio_func117_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC117_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func117_in_sel_cfg::W](gpio_func117_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC117_IN_SEL_CFG {}
#[doc = "GPIO_FUNC117_IN_SEL_CFG"]
pub mod gpio_func117_in_sel_cfg;
#[doc = "GPIO_FUNC118_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func118_in_sel_cfg](gpio_func118_in_sel_cfg) module"]
pub type GPIO_FUNC118_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC118_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC118_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func118_in_sel_cfg::R](gpio_func118_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC118_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func118_in_sel_cfg::W](gpio_func118_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC118_IN_SEL_CFG {}
#[doc = "GPIO_FUNC118_IN_SEL_CFG"]
pub mod gpio_func118_in_sel_cfg;
#[doc = "GPIO_FUNC119_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func119_in_sel_cfg](gpio_func119_in_sel_cfg) module"]
pub type GPIO_FUNC119_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC119_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC119_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func119_in_sel_cfg::R](gpio_func119_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC119_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func119_in_sel_cfg::W](gpio_func119_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC119_IN_SEL_CFG {}
#[doc = "GPIO_FUNC119_IN_SEL_CFG"]
pub mod gpio_func119_in_sel_cfg;
#[doc = "GPIO_FUNC120_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func120_in_sel_cfg](gpio_func120_in_sel_cfg) module"]
pub type GPIO_FUNC120_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC120_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC120_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func120_in_sel_cfg::R](gpio_func120_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC120_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func120_in_sel_cfg::W](gpio_func120_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC120_IN_SEL_CFG {}
#[doc = "GPIO_FUNC120_IN_SEL_CFG"]
pub mod gpio_func120_in_sel_cfg;
#[doc = "GPIO_FUNC121_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func121_in_sel_cfg](gpio_func121_in_sel_cfg) module"]
pub type GPIO_FUNC121_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC121_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC121_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func121_in_sel_cfg::R](gpio_func121_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC121_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func121_in_sel_cfg::W](gpio_func121_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC121_IN_SEL_CFG {}
#[doc = "GPIO_FUNC121_IN_SEL_CFG"]
pub mod gpio_func121_in_sel_cfg;
#[doc = "GPIO_FUNC122_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func122_in_sel_cfg](gpio_func122_in_sel_cfg) module"]
pub type GPIO_FUNC122_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC122_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC122_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func122_in_sel_cfg::R](gpio_func122_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC122_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func122_in_sel_cfg::W](gpio_func122_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC122_IN_SEL_CFG {}
#[doc = "GPIO_FUNC122_IN_SEL_CFG"]
pub mod gpio_func122_in_sel_cfg;
#[doc = "GPIO_FUNC123_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func123_in_sel_cfg](gpio_func123_in_sel_cfg) module"]
pub type GPIO_FUNC123_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC123_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC123_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func123_in_sel_cfg::R](gpio_func123_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC123_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func123_in_sel_cfg::W](gpio_func123_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC123_IN_SEL_CFG {}
#[doc = "GPIO_FUNC123_IN_SEL_CFG"]
pub mod gpio_func123_in_sel_cfg;
#[doc = "GPIO_FUNC124_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func124_in_sel_cfg](gpio_func124_in_sel_cfg) module"]
pub type GPIO_FUNC124_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC124_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC124_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func124_in_sel_cfg::R](gpio_func124_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC124_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func124_in_sel_cfg::W](gpio_func124_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC124_IN_SEL_CFG {}
#[doc = "GPIO_FUNC124_IN_SEL_CFG"]
pub mod gpio_func124_in_sel_cfg;
#[doc = "GPIO_FUNC125_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func125_in_sel_cfg](gpio_func125_in_sel_cfg) module"]
pub type GPIO_FUNC125_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC125_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC125_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func125_in_sel_cfg::R](gpio_func125_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC125_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func125_in_sel_cfg::W](gpio_func125_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC125_IN_SEL_CFG {}
#[doc = "GPIO_FUNC125_IN_SEL_CFG"]
pub mod gpio_func125_in_sel_cfg;
#[doc = "GPIO_FUNC126_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func126_in_sel_cfg](gpio_func126_in_sel_cfg) module"]
pub type GPIO_FUNC126_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC126_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC126_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func126_in_sel_cfg::R](gpio_func126_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC126_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func126_in_sel_cfg::W](gpio_func126_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC126_IN_SEL_CFG {}
#[doc = "GPIO_FUNC126_IN_SEL_CFG"]
pub mod gpio_func126_in_sel_cfg;
#[doc = "GPIO_FUNC127_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func127_in_sel_cfg](gpio_func127_in_sel_cfg) module"]
pub type GPIO_FUNC127_IN_SEL_CFG = crate::Reg<u32, _GPIO_FUNC127_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC127_IN_SEL_CFG;
#[doc = "`read()` method returns [gpio_func127_in_sel_cfg::R](gpio_func127_in_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC127_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func127_in_sel_cfg::W](gpio_func127_in_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC127_IN_SEL_CFG {}
#[doc = "GPIO_FUNC127_IN_SEL_CFG"]
pub mod gpio_func127_in_sel_cfg;
#[doc = "GPIO_FUNC0_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func0_out_sel_cfg](gpio_func0_out_sel_cfg) module"]
pub type GPIO_FUNC0_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC0_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC0_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func0_out_sel_cfg::R](gpio_func0_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC0_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func0_out_sel_cfg::W](gpio_func0_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC0_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC0_OUT_SEL_CFG"]
pub mod gpio_func0_out_sel_cfg;
#[doc = "GPIO_FUNC1_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func1_out_sel_cfg](gpio_func1_out_sel_cfg) module"]
pub type GPIO_FUNC1_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC1_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC1_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func1_out_sel_cfg::R](gpio_func1_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC1_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func1_out_sel_cfg::W](gpio_func1_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC1_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC1_OUT_SEL_CFG"]
pub mod gpio_func1_out_sel_cfg;
#[doc = "GPIO_FUNC2_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func2_out_sel_cfg](gpio_func2_out_sel_cfg) module"]
pub type GPIO_FUNC2_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC2_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC2_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func2_out_sel_cfg::R](gpio_func2_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC2_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func2_out_sel_cfg::W](gpio_func2_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC2_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC2_OUT_SEL_CFG"]
pub mod gpio_func2_out_sel_cfg;
#[doc = "GPIO_FUNC3_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func3_out_sel_cfg](gpio_func3_out_sel_cfg) module"]
pub type GPIO_FUNC3_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC3_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC3_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func3_out_sel_cfg::R](gpio_func3_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC3_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func3_out_sel_cfg::W](gpio_func3_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC3_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC3_OUT_SEL_CFG"]
pub mod gpio_func3_out_sel_cfg;
#[doc = "GPIO_FUNC4_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func4_out_sel_cfg](gpio_func4_out_sel_cfg) module"]
pub type GPIO_FUNC4_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC4_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC4_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func4_out_sel_cfg::R](gpio_func4_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC4_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func4_out_sel_cfg::W](gpio_func4_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC4_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC4_OUT_SEL_CFG"]
pub mod gpio_func4_out_sel_cfg;
#[doc = "GPIO_FUNC5_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func5_out_sel_cfg](gpio_func5_out_sel_cfg) module"]
pub type GPIO_FUNC5_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC5_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC5_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func5_out_sel_cfg::R](gpio_func5_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC5_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func5_out_sel_cfg::W](gpio_func5_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC5_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC5_OUT_SEL_CFG"]
pub mod gpio_func5_out_sel_cfg;
#[doc = "GPIO_FUNC6_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func6_out_sel_cfg](gpio_func6_out_sel_cfg) module"]
pub type GPIO_FUNC6_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC6_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC6_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func6_out_sel_cfg::R](gpio_func6_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC6_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func6_out_sel_cfg::W](gpio_func6_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC6_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC6_OUT_SEL_CFG"]
pub mod gpio_func6_out_sel_cfg;
#[doc = "GPIO_FUNC7_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func7_out_sel_cfg](gpio_func7_out_sel_cfg) module"]
pub type GPIO_FUNC7_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC7_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC7_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func7_out_sel_cfg::R](gpio_func7_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC7_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func7_out_sel_cfg::W](gpio_func7_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC7_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC7_OUT_SEL_CFG"]
pub mod gpio_func7_out_sel_cfg;
#[doc = "GPIO_FUNC8_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func8_out_sel_cfg](gpio_func8_out_sel_cfg) module"]
pub type GPIO_FUNC8_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC8_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC8_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func8_out_sel_cfg::R](gpio_func8_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC8_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func8_out_sel_cfg::W](gpio_func8_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC8_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC8_OUT_SEL_CFG"]
pub mod gpio_func8_out_sel_cfg;
#[doc = "GPIO_FUNC9_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func9_out_sel_cfg](gpio_func9_out_sel_cfg) module"]
pub type GPIO_FUNC9_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC9_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC9_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func9_out_sel_cfg::R](gpio_func9_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC9_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func9_out_sel_cfg::W](gpio_func9_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC9_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC9_OUT_SEL_CFG"]
pub mod gpio_func9_out_sel_cfg;
#[doc = "GPIO_FUNC10_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func10_out_sel_cfg](gpio_func10_out_sel_cfg) module"]
pub type GPIO_FUNC10_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC10_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC10_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func10_out_sel_cfg::R](gpio_func10_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC10_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func10_out_sel_cfg::W](gpio_func10_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC10_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC10_OUT_SEL_CFG"]
pub mod gpio_func10_out_sel_cfg;
#[doc = "GPIO_FUNC11_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func11_out_sel_cfg](gpio_func11_out_sel_cfg) module"]
pub type GPIO_FUNC11_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC11_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC11_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func11_out_sel_cfg::R](gpio_func11_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC11_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func11_out_sel_cfg::W](gpio_func11_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC11_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC11_OUT_SEL_CFG"]
pub mod gpio_func11_out_sel_cfg;
#[doc = "GPIO_FUNC12_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func12_out_sel_cfg](gpio_func12_out_sel_cfg) module"]
pub type GPIO_FUNC12_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC12_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC12_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func12_out_sel_cfg::R](gpio_func12_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC12_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func12_out_sel_cfg::W](gpio_func12_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC12_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC12_OUT_SEL_CFG"]
pub mod gpio_func12_out_sel_cfg;
#[doc = "GPIO_FUNC13_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func13_out_sel_cfg](gpio_func13_out_sel_cfg) module"]
pub type GPIO_FUNC13_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC13_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC13_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func13_out_sel_cfg::R](gpio_func13_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC13_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func13_out_sel_cfg::W](gpio_func13_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC13_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC13_OUT_SEL_CFG"]
pub mod gpio_func13_out_sel_cfg;
#[doc = "GPIO_FUNC14_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func14_out_sel_cfg](gpio_func14_out_sel_cfg) module"]
pub type GPIO_FUNC14_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC14_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC14_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func14_out_sel_cfg::R](gpio_func14_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC14_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func14_out_sel_cfg::W](gpio_func14_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC14_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC14_OUT_SEL_CFG"]
pub mod gpio_func14_out_sel_cfg;
#[doc = "GPIO_FUNC15_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func15_out_sel_cfg](gpio_func15_out_sel_cfg) module"]
pub type GPIO_FUNC15_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC15_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC15_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func15_out_sel_cfg::R](gpio_func15_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC15_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func15_out_sel_cfg::W](gpio_func15_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC15_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC15_OUT_SEL_CFG"]
pub mod gpio_func15_out_sel_cfg;
#[doc = "GPIO_FUNC16_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func16_out_sel_cfg](gpio_func16_out_sel_cfg) module"]
pub type GPIO_FUNC16_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC16_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC16_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func16_out_sel_cfg::R](gpio_func16_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC16_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func16_out_sel_cfg::W](gpio_func16_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC16_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC16_OUT_SEL_CFG"]
pub mod gpio_func16_out_sel_cfg;
#[doc = "GPIO_FUNC17_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func17_out_sel_cfg](gpio_func17_out_sel_cfg) module"]
pub type GPIO_FUNC17_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC17_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC17_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func17_out_sel_cfg::R](gpio_func17_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC17_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func17_out_sel_cfg::W](gpio_func17_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC17_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC17_OUT_SEL_CFG"]
pub mod gpio_func17_out_sel_cfg;
#[doc = "GPIO_FUNC18_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func18_out_sel_cfg](gpio_func18_out_sel_cfg) module"]
pub type GPIO_FUNC18_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC18_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC18_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func18_out_sel_cfg::R](gpio_func18_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC18_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func18_out_sel_cfg::W](gpio_func18_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC18_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC18_OUT_SEL_CFG"]
pub mod gpio_func18_out_sel_cfg;
#[doc = "GPIO_FUNC19_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func19_out_sel_cfg](gpio_func19_out_sel_cfg) module"]
pub type GPIO_FUNC19_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC19_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC19_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func19_out_sel_cfg::R](gpio_func19_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC19_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func19_out_sel_cfg::W](gpio_func19_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC19_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC19_OUT_SEL_CFG"]
pub mod gpio_func19_out_sel_cfg;
#[doc = "GPIO_FUNC20_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func20_out_sel_cfg](gpio_func20_out_sel_cfg) module"]
pub type GPIO_FUNC20_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC20_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC20_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func20_out_sel_cfg::R](gpio_func20_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC20_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func20_out_sel_cfg::W](gpio_func20_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC20_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC20_OUT_SEL_CFG"]
pub mod gpio_func20_out_sel_cfg;
#[doc = "GPIO_FUNC21_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func21_out_sel_cfg](gpio_func21_out_sel_cfg) module"]
pub type GPIO_FUNC21_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC21_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC21_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func21_out_sel_cfg::R](gpio_func21_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC21_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func21_out_sel_cfg::W](gpio_func21_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC21_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC21_OUT_SEL_CFG"]
pub mod gpio_func21_out_sel_cfg;
#[doc = "GPIO_FUNC22_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func22_out_sel_cfg](gpio_func22_out_sel_cfg) module"]
pub type GPIO_FUNC22_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC22_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC22_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func22_out_sel_cfg::R](gpio_func22_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC22_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func22_out_sel_cfg::W](gpio_func22_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC22_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC22_OUT_SEL_CFG"]
pub mod gpio_func22_out_sel_cfg;
#[doc = "GPIO_FUNC23_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func23_out_sel_cfg](gpio_func23_out_sel_cfg) module"]
pub type GPIO_FUNC23_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC23_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC23_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func23_out_sel_cfg::R](gpio_func23_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC23_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func23_out_sel_cfg::W](gpio_func23_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC23_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC23_OUT_SEL_CFG"]
pub mod gpio_func23_out_sel_cfg;
#[doc = "GPIO_FUNC24_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func24_out_sel_cfg](gpio_func24_out_sel_cfg) module"]
pub type GPIO_FUNC24_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC24_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC24_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func24_out_sel_cfg::R](gpio_func24_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC24_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func24_out_sel_cfg::W](gpio_func24_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC24_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC24_OUT_SEL_CFG"]
pub mod gpio_func24_out_sel_cfg;
#[doc = "GPIO_FUNC25_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_func25_out_sel_cfg](gpio_func25_out_sel_cfg) module"]
pub type GPIO_FUNC25_OUT_SEL_CFG = crate::Reg<u32, _GPIO_FUNC25_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC25_OUT_SEL_CFG;
#[doc = "`read()` method returns [gpio_func25_out_sel_cfg::R](gpio_func25_out_sel_cfg::R) reader structure"]
impl crate::Readable for GPIO_FUNC25_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [gpio_func25_out_sel_cfg::W](gpio_func25_out_sel_cfg::W) writer structure"]
impl crate::Writable for GPIO_FUNC25_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC25_OUT_SEL_CFG"]
pub mod gpio_func25_out_sel_cfg;
#[doc = "GPIO_CLOCK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_clock_gate](gpio_clock_gate) module"]
pub type GPIO_CLOCK_GATE = crate::Reg<u32, _GPIO_CLOCK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CLOCK_GATE;
#[doc = "`read()` method returns [gpio_clock_gate::R](gpio_clock_gate::R) reader structure"]
impl crate::Readable for GPIO_CLOCK_GATE {}
#[doc = "`write(|w| ..)` method takes [gpio_clock_gate::W](gpio_clock_gate::W) writer structure"]
impl crate::Writable for GPIO_CLOCK_GATE {}
#[doc = "GPIO_CLOCK_GATE"]
pub mod gpio_clock_gate;
#[doc = "GPIO_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_date](gpio_date) module"]
pub type GPIO_DATE = crate::Reg<u32, _GPIO_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_DATE;
#[doc = "`read()` method returns [gpio_date::R](gpio_date::R) reader structure"]
impl crate::Readable for GPIO_DATE {}
#[doc = "`write(|w| ..)` method takes [gpio_date::W](gpio_date::W) writer structure"]
impl crate::Writable for GPIO_DATE {}
#[doc = "GPIO_DATE"]
pub mod gpio_date;
