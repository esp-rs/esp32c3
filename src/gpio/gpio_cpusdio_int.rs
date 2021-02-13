#[doc = "Reader of register GPIO_CPUSDIO_INT"]
pub type R = crate::R<u32, super::GPIO_CPUSDIO_INT>;
#[doc = "Reader of field `GPIO_SDIO_INT`"]
pub type GPIO_SDIO_INT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpio_sdio_int(&self) -> GPIO_SDIO_INT_R {
        GPIO_SDIO_INT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
