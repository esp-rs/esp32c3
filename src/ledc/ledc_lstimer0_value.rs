#[doc = "Reader of register LEDC_LSTIMER0_VALUE"]
pub type R = crate::R<u32, super::LEDC_LSTIMER0_VALUE>;
#[doc = "Reader of field `LEDC_LSTIMER0_CNT`"]
pub type LEDC_LSTIMER0_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn ledc_lstimer0_cnt(&self) -> LEDC_LSTIMER0_CNT_R {
        LEDC_LSTIMER0_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
