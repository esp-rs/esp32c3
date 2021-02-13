#[doc = "Reader of register LEDC_LSCH1_DUTY_R"]
pub type R = crate::R<u32, super::LEDC_LSCH1_DUTY_R>;
#[doc = "Reader of field `LEDC_DUTY_LSCH1`"]
pub type LEDC_DUTY_LSCH1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn ledc_duty_lsch1(&self) -> LEDC_DUTY_LSCH1_R {
        LEDC_DUTY_LSCH1_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
