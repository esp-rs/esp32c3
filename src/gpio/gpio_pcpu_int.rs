#[doc = "Reader of register GPIO_PCPU_INT"]
pub type R = crate::R<u32, super::GPIO_PCPU_INT>;
#[doc = "Reader of field `GPIO_PROCPU_INT`"]
pub type GPIO_PROCPU_INT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpio_procpu_int(&self) -> GPIO_PROCPU_INT_R {
        GPIO_PROCPU_INT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
