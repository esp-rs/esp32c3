#[doc = "Reader of register UART_FSM_STATUS"]
pub type R = crate::R<u32, super::UART_FSM_STATUS>;
#[doc = "Reader of field `UART_ST_UTX_OUT`"]
pub type UART_ST_UTX_OUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `UART_ST_URX_OUT`"]
pub type UART_ST_URX_OUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_st_utx_out(&self) -> UART_ST_UTX_OUT_R {
        UART_ST_UTX_OUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_st_urx_out(&self) -> UART_ST_URX_OUT_R {
        UART_ST_URX_OUT_R::new((self.bits & 0x0f) as u8)
    }
}
