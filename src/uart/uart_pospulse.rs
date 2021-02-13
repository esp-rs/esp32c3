#[doc = "Reader of register UART_POSPULSE"]
pub type R = crate::R<u32, super::UART_POSPULSE>;
#[doc = "Reader of field `UART_POSEDGE_MIN_CNT`"]
pub type UART_POSEDGE_MIN_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn uart_posedge_min_cnt(&self) -> UART_POSEDGE_MIN_CNT_R {
        UART_POSEDGE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
