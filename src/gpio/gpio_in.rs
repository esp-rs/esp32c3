#[doc = "Reader of register GPIO_IN"]
pub type R = crate::R<u32, super::GPIO_IN>;
#[doc = "Reader of field `GPIO_IN_DATA`"]
pub type GPIO_IN_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpio_in_data(&self) -> GPIO_IN_DATA_R {
        GPIO_IN_DATA_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
