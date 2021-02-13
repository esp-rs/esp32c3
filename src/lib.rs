#![doc = "Peripheral access API for ESP32C3 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "APB_CTRL"]
pub struct APB_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_CTRL {}
impl APB_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_ctrl::RegisterBlock {
        0x6002_6000 as *const _
    }
}
impl Deref for APB_CTRL {
    type Target = apb_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*APB_CTRL::ptr() }
    }
}
#[doc = "APB_CTRL"]
pub mod apb_ctrl;
#[doc = "APB_SARADC"]
pub struct APB_SARADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_SARADC {}
impl APB_SARADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_saradc::RegisterBlock {
        0x6004_0000 as *const _
    }
}
impl Deref for APB_SARADC {
    type Target = apb_saradc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*APB_SARADC::ptr() }
    }
}
#[doc = "APB_SARADC"]
pub mod apb_saradc;
#[doc = "ASSIST_DEBUG"]
pub struct ASSIST_DEBUG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ASSIST_DEBUG {}
impl ASSIST_DEBUG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const assist_debug::RegisterBlock {
        0x600c_e000 as *const _
    }
}
impl Deref for ASSIST_DEBUG {
    type Target = assist_debug::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ASSIST_DEBUG::ptr() }
    }
}
#[doc = "ASSIST_DEBUG"]
pub mod assist_debug;
#[doc = "EFUSE"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        0x6000_8800 as *const _
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFUSE::ptr() }
    }
}
#[doc = "EFUSE"]
pub mod efuse;
#[doc = "EXTMEM"]
pub struct EXTMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTMEM {}
impl EXTMEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const extmem::RegisterBlock {
        0x600c_4000 as *const _
    }
}
impl Deref for EXTMEM {
    type Target = extmem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTMEM::ptr() }
    }
}
#[doc = "EXTMEM"]
pub mod extmem;
#[doc = "GDMA"]
pub struct GDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GDMA {}
impl GDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gdma::RegisterBlock {
        0x6003_f000 as *const _
    }
}
impl Deref for GDMA {
    type Target = gdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GDMA::ptr() }
    }
}
#[doc = "GDMA"]
pub mod gdma;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x6000_4000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "GPIO_SD"]
pub struct GPIO_SD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_SD {}
impl GPIO_SD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_sd::RegisterBlock {
        0 as *const _
    }
}
impl Deref for GPIO_SD {
    type Target = gpio_sd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_SD::ptr() }
    }
}
#[doc = "GPIO_SD"]
pub mod gpio_sd;
#[doc = "I2C"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        0 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C::ptr() }
    }
}
#[doc = "I2C"]
pub mod i2c;
#[doc = "I2S"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        0 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "I2S"]
pub mod i2s;
#[doc = "INTERRUPT_CORE0"]
pub struct INTERRUPT_CORE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INTERRUPT_CORE0 {}
impl INTERRUPT_CORE0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const interrupt_core0::RegisterBlock {
        0 as *const _
    }
}
impl Deref for INTERRUPT_CORE0 {
    type Target = interrupt_core0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*INTERRUPT_CORE0::ptr() }
    }
}
#[doc = "INTERRUPT_CORE0"]
pub mod interrupt_core0;
#[doc = "LEDC"]
pub struct LEDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDC {}
impl LEDC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ledc::RegisterBlock {
        0x6001_9000 as *const _
    }
}
impl Deref for LEDC {
    type Target = ledc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LEDC::ptr() }
    }
}
#[doc = "LEDC"]
pub mod ledc;
#[doc = "RMT"]
pub struct RMT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMT {}
impl RMT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rmt::RegisterBlock {
        0x6001_6000 as *const _
    }
}
impl Deref for RMT {
    type Target = rmt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RMT::ptr() }
    }
}
#[doc = "RMT"]
pub mod rmt;
#[doc = "RTCCNTL"]
pub struct RTCCNTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCCNTL {}
impl RTCCNTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtccntl::RegisterBlock {
        0x6000_8000 as *const _
    }
}
impl Deref for RTCCNTL {
    type Target = rtccntl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTCCNTL::ptr() }
    }
}
#[doc = "RTCCNTL"]
pub mod rtccntl;
#[doc = "RTC_I2C"]
pub struct RTC_I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_I2C {}
impl RTC_I2C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_i2c::RegisterBlock {
        0x6000_e000 as *const _
    }
}
impl Deref for RTC_I2C {
    type Target = rtc_i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC_I2C::ptr() }
    }
}
#[doc = "RTC_I2C"]
pub mod rtc_i2c;
#[doc = "SENSITIVE"]
pub struct SENSITIVE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SENSITIVE {}
impl SENSITIVE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sensitive::RegisterBlock {
        0x600c_1000 as *const _
    }
}
impl Deref for SENSITIVE {
    type Target = sensitive::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SENSITIVE::ptr() }
    }
}
#[doc = "SENSITIVE"]
pub mod sensitive;
#[doc = "SPI"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "SPI"]
pub mod spi;
#[doc = "SPI_MEM"]
pub struct SPI_MEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI_MEM {}
impl SPI_MEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi_mem::RegisterBlock {
        0 as *const _
    }
}
impl Deref for SPI_MEM {
    type Target = spi_mem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI_MEM::ptr() }
    }
}
#[doc = "SPI_MEM"]
pub mod spi_mem;
#[doc = "SYSCON"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscon::RegisterBlock {
        0x6002_6000 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "SYSCON"]
pub mod syscon;
#[doc = "SYSTEM"]
pub struct SYSTEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM {}
impl SYSTEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system::RegisterBlock {
        0x600c_0000 as *const _
    }
}
impl Deref for SYSTEM {
    type Target = system::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEM::ptr() }
    }
}
#[doc = "SYSTEM"]
pub mod system;
#[doc = "SYS_TIMER"]
pub struct SYS_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS_TIMER {}
impl SYS_TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_timer::RegisterBlock {
        0x6002_3000 as *const _
    }
}
impl Deref for SYS_TIMER {
    type Target = sys_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYS_TIMER::ptr() }
    }
}
#[doc = "SYS_TIMER"]
pub mod sys_timer;
#[doc = "TIMG"]
pub struct TIMG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMG {}
impl TIMG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timg::RegisterBlock {
        0 as *const _
    }
}
impl Deref for TIMG {
    type Target = timg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMG::ptr() }
    }
}
#[doc = "TIMG"]
pub mod timg;
#[doc = "UART"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0x6000_0000 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "UART"]
pub mod uart;
#[doc = "UHCI"]
pub struct UHCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UHCI {}
impl UHCI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uhci::RegisterBlock {
        0 as *const _
    }
}
impl Deref for UHCI {
    type Target = uhci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UHCI::ptr() }
    }
}
#[doc = "UHCI"]
pub mod uhci;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "APB_CTRL"]
    pub APB_CTRL: APB_CTRL,
    #[doc = "APB_SARADC"]
    pub APB_SARADC: APB_SARADC,
    #[doc = "ASSIST_DEBUG"]
    pub ASSIST_DEBUG: ASSIST_DEBUG,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "EXTMEM"]
    pub EXTMEM: EXTMEM,
    #[doc = "GDMA"]
    pub GDMA: GDMA,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "GPIO_SD"]
    pub GPIO_SD: GPIO_SD,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "INTERRUPT_CORE0"]
    pub INTERRUPT_CORE0: INTERRUPT_CORE0,
    #[doc = "LEDC"]
    pub LEDC: LEDC,
    #[doc = "RMT"]
    pub RMT: RMT,
    #[doc = "RTCCNTL"]
    pub RTCCNTL: RTCCNTL,
    #[doc = "RTC_I2C"]
    pub RTC_I2C: RTC_I2C,
    #[doc = "SENSITIVE"]
    pub SENSITIVE: SENSITIVE,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "SPI_MEM"]
    pub SPI_MEM: SPI_MEM,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "SYSTEM"]
    pub SYSTEM: SYSTEM,
    #[doc = "SYS_TIMER"]
    pub SYS_TIMER: SYS_TIMER,
    #[doc = "TIMG"]
    pub TIMG: TIMG,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "UHCI"]
    pub UHCI: UHCI,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            APB_CTRL: APB_CTRL {
                _marker: PhantomData,
            },
            APB_SARADC: APB_SARADC {
                _marker: PhantomData,
            },
            ASSIST_DEBUG: ASSIST_DEBUG {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            EXTMEM: EXTMEM {
                _marker: PhantomData,
            },
            GDMA: GDMA {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            GPIO_SD: GPIO_SD {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            INTERRUPT_CORE0: INTERRUPT_CORE0 {
                _marker: PhantomData,
            },
            LEDC: LEDC {
                _marker: PhantomData,
            },
            RMT: RMT {
                _marker: PhantomData,
            },
            RTCCNTL: RTCCNTL {
                _marker: PhantomData,
            },
            RTC_I2C: RTC_I2C {
                _marker: PhantomData,
            },
            SENSITIVE: SENSITIVE {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            SPI_MEM: SPI_MEM {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            SYSTEM: SYSTEM {
                _marker: PhantomData,
            },
            SYS_TIMER: SYS_TIMER {
                _marker: PhantomData,
            },
            TIMG: TIMG {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            UHCI: UHCI {
                _marker: PhantomData,
            },
        }
    }
}
