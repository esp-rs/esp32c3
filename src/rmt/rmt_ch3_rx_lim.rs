#[doc = "Reader of register RMT_CH3_RX_LIM"]
pub type R = crate::R<u32, super::RMT_CH3_RX_LIM>;
#[doc = "Writer for register RMT_CH3_RX_LIM"]
pub type W = crate::W<u32, super::RMT_CH3_RX_LIM>;
#[doc = "Register RMT_CH3_RX_LIM `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_CH3_RX_LIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_RX_LIM_CH3`"]
pub type RMT_RX_LIM_CH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_RX_LIM_CH3`"]
pub struct RMT_RX_LIM_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_RX_LIM_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rmt_rx_lim_ch3(&self) -> RMT_RX_LIM_CH3_R {
        RMT_RX_LIM_CH3_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rmt_rx_lim_ch3(&mut self) -> RMT_RX_LIM_CH3_W {
        RMT_RX_LIM_CH3_W { w: self }
    }
}
