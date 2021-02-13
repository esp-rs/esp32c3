#[doc = "Reader of register LEDC_LSTIMER3_VALUE"]
pub type R = crate::R<u32, super::LEDC_LSTIMER3_VALUE>;
#[doc = "Reader of field `LEDC_LSTIMER3_CNT`"]
pub type LEDC_LSTIMER3_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn ledc_lstimer3_cnt(&self) -> LEDC_LSTIMER3_CNT_R {
        LEDC_LSTIMER3_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
