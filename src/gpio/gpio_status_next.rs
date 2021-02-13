#[doc = "Reader of register GPIO_STATUS_NEXT"]
pub type R = crate::R<u32, super::GPIO_STATUS_NEXT>;
#[doc = "Reader of field `GPIO_STATUS_INTERRUPT_NEXT`"]
pub type GPIO_STATUS_INTERRUPT_NEXT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpio_status_interrupt_next(&self) -> GPIO_STATUS_INTERRUPT_NEXT_R {
        GPIO_STATUS_INTERRUPT_NEXT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
