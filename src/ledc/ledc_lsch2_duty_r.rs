#[doc = "Reader of register LEDC_LSCH2_DUTY_R"]
pub type R = crate::R<u32, super::LEDC_LSCH2_DUTY_R>;
#[doc = "Reader of field `LEDC_DUTY_LSCH2`"]
pub type LEDC_DUTY_LSCH2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn ledc_duty_lsch2(&self) -> LEDC_DUTY_LSCH2_R {
        LEDC_DUTY_LSCH2_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
