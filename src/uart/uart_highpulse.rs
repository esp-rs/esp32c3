#[doc = "Reader of register UART_HIGHPULSE"]
pub type R = crate::R<u32, super::UART_HIGHPULSE>;
#[doc = "Reader of field `UART_HIGHPULSE_MIN_CNT`"]
pub type UART_HIGHPULSE_MIN_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn uart_highpulse_min_cnt(&self) -> UART_HIGHPULSE_MIN_CNT_R {
        UART_HIGHPULSE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
