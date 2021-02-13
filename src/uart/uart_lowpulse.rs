#[doc = "Reader of register UART_LOWPULSE"]
pub type R = crate::R<u32, super::UART_LOWPULSE>;
#[doc = "Reader of field `UART_LOWPULSE_MIN_CNT`"]
pub type UART_LOWPULSE_MIN_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn uart_lowpulse_min_cnt(&self) -> UART_LOWPULSE_MIN_CNT_R {
        UART_LOWPULSE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
