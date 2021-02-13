#[doc = "Reader of register GPIO_STRAP"]
pub type R = crate::R<u32, super::GPIO_STRAP>;
#[doc = "Reader of field `GPIO_STRAPPING`"]
pub type GPIO_STRAPPING_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gpio_strapping(&self) -> GPIO_STRAPPING_R {
        GPIO_STRAPPING_R::new((self.bits & 0xffff) as u16)
    }
}
