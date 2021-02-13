#[doc = "Reader of register UART_NEGPULSE"]
pub type R = crate::R<u32, super::UART_NEGPULSE>;
#[doc = "Reader of field `UART_NEGEDGE_MIN_CNT`"]
pub type UART_NEGEDGE_MIN_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn uart_negedge_min_cnt(&self) -> UART_NEGEDGE_MIN_CNT_R {
        UART_NEGEDGE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
